#!/usr/bin/env python3
"""
8b-is GitHub Actions Workflow Guardian & Auto-Fixer

Reads GIT_MEGA from /etc/environment (never logging the token) and monitors
all active 8b-is repositories on GitHub Actions:
  1. Disables noisy upstream-only scheduled/self-hosted workflows on forks.
  2. Auto-fixes known local workflow failure patterns in /data/8b-is/<repo>:
     - Missing GH_TOKEN for `gh` CLI calls in workflows
     - Ungated optional GitHub Pages deploy steps when Pages is not enabled
     - Ungated optional Docker Hub / Cloudflare / SSH deploy steps when secrets are unset
  3. Auto-reruns transient infrastructure / network / startup_failure runs (up to 2 attempts).
  4. Purges superseded, cancelled, dynamic-agent, and stale failed workflow runs
     so repository Actions tabs remain clean.
  5. Writes an integer-only (zero-float) status report to
     /data/.agent-state/8b-workflow-guardian/status.json.
"""

import argparse
import concurrent.futures
import datetime as dt
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import urllib.error
import urllib.request

WORKSPACE_ROOT = Path("/data/8b-is")
STATE_DIR = Path("/data/.agent-state/8b-workflow-guardian")
STATUS_FILE = STATE_DIR / "status.json"
HISTORY_FILE = STATE_DIR / "rerun_history.json"

# Upstream fork workflows that require upstream self-hosted runners or upstream secrets
FORK_DISABLE_WORKFLOW_PATTERNS = {
    "transformers-ultra": (
        "self-scheduled-",
        "self-nightly-",
        "vllm-ci-caller",
        "doctests.yml",
        "check_tiny_models.yml",
        "build-docker-images.yml",
        "sync-upstream-weekly.yml",
        "pr-ci-caller.yml",
        "build_documentation.yml",
        "update_metdata.yml",
        "benchmark.yml",
    ),
    "headroomz": (
        "codeql-triage.yml",
        "security.yml",
        "release-please.yml",
        "stale.yml",
    ),
    "deepsiper-enthea": (
        "e2e.yml",
        "e2b-e2e.yml",
        "pi-ai-provider-e2e.yml",
        "release.yml",
        "sandbox.yml",
    ),
    "rustybox": (
        "cve-report.yml",
        "codex.yml",
        "sponsors.yml",
    ),
    "MaiMaiL": (
        "check_prs_if_on_staging.yml",
        "close_old_issues_and_prs.yml",
    ),
    "oxicode": (
        "daily-issues-recap.yml",
        "daily-pr-recap.yml",
        "stats.yml",
        "triage.yml",
        "duplicate-issues.yml",
        "pr-management.yml",
    ),
}

# Blacklisted Blacksmith runner/action patterns and their standard replacements
BLACKSMITH_REPLACEMENTS = (
    (re.compile(r"^(\s*runs-on:\s*)blacksmith-[\w.-]*macos-15(.*)$", re.MULTILINE), r"\1macos-15\2"),
    (re.compile(r"^(\s*runs-on:\s*)blacksmith-[\w.-]*windows-2025(.*)$", re.MULTILINE), r"\1windows-2025\2"),
    (re.compile(r"^(\s*runs-on:\s*)blacksmith-[\w.-]*ubuntu-2204(.*)$", re.MULTILINE), r"\1ubuntu-22.04\2"),
    (re.compile(r"^(\s*runs-on:\s*)blacksmith-[\w.-]*ubuntu-2404-arm(.*)$", re.MULTILINE), r"\1ubuntu-24.04-arm\2"),
    (re.compile(r"^(\s*runs-on:\s*)blacksmith(?:-[\w.-]+)?(.*)$", re.MULTILINE), r"\1ubuntu-24.04\2"),
    (re.compile(r"useblacksmith/cache@v\d+"), "actions/cache@v4"),
    (re.compile(r"useblacksmith/setup-docker-builder@v\d+"), "docker/setup-buildx-action@v3"),
    (re.compile(r"useblacksmith/build-push-action@v\d+"), "docker/build-push-action@v6"),
    (re.compile(r"actions/checkout@v7(?![.\d])"), "actions/checkout@v4"),
)

TRANSIENT_LOG_SIGNATURES = (
    "Work around spurious network errors in curl",
    "checksum failed for ",
    "Connection reset by peer",
    "TLS handshake timeout",
    "502 Bad Gateway",
    "503 Service Unavailable",
    "504 Gateway Time-out",
    "ETIMEDOUT",
)


class NoAuthRedirectHandler(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        new_req = super().redirect_request(req, fp, code, msg, headers, newurl)
        if new_req and "api.github.com" not in newurl:
            new_req.headers.pop("Authorization", None)
            new_req.unredirected_hdrs.pop("Authorization", None)
        return new_req


def load_git_mega() -> str:
    token = os.environ.get("GIT_MEGA", "").strip()
    if token:
        return token
    env_path = Path("/etc/environment")
    if env_path.is_file():
        for raw in env_path.read_text(encoding="utf-8", errors="ignore").splitlines():
            line = raw.strip()
            if line.startswith("GIT_MEGA=") or line.startswith("export GIT_MEGA="):
                return line.split("=", 1)[1].strip().strip('"').strip("'")
    raise RuntimeError("GIT_MEGA not found in environment or /etc/environment")


def load_write_token() -> str | None:
    try:
        res = subprocess.run(["gh", "auth", "token"], capture_output=True, text=True, timeout=5)
        if res.returncode == 0 and res.stdout.strip():
            return res.stdout.strip()
    except Exception:
        pass
    return None


class GitHubClient:
    def __init__(self, token: str, write_token: str | None = None):
        self.token = token
        self.write_token = write_token
        self.opener = urllib.request.build_opener(NoAuthRedirectHandler())

    def _do_request(self, method: str, url: str, data: bytes | None, token: str, raw: bool):
        headers = {
            "Authorization": f"Bearer {token}",
            "Accept": "application/vnd.github+json",
            "User-Agent": "8b-workflow-guardian/1.0",
        }
        if data is not None:
            headers["Content-Type"] = "application/json"
        req = urllib.request.Request(url, data=data, headers=headers, method=method)
        with self.opener.open(req, timeout=25) as resp:
            body = resp.read()
            if not body:
                return "" if raw else {}
            if raw:
                return body.decode("utf-8", errors="replace")
            return json.loads(body.decode("utf-8"))

    def request(self, method: str, path: str, payload=None, raw: bool = False):
        url = path if path.startswith("http") else f"https://api.github.com/{path.lstrip('/')}"
        data = json.dumps(payload).encode("utf-8") if payload is not None else None
        primary = self.write_token if (method != "GET" and self.write_token) else self.token
        try:
            return self._do_request(method, url, data, primary, raw)
        except urllib.error.HTTPError as e:
            if e.code in (401, 403) and self.write_token and primary != self.write_token:
                return self._do_request(method, url, data, self.write_token, raw)
            raise


def list_active_org_repos(gh: GitHubClient) -> list[str]:
    repos = []
    for page in range(1, 6):
        batch = gh.request("GET", f"orgs/8b-is/repos?per_page=100&page={page}")
        if not batch:
            break
        for r in batch:
            if not r.get("archived"):
                repos.append(r["name"])
    return sorted(set(repos))


def load_rerun_history() -> dict[str, int]:
    if HISTORY_FILE.is_file():
        try:
            return json.loads(HISTORY_FILE.read_text(encoding="utf-8"))
        except Exception:
            return {}
    return {}


def save_rerun_history(history: dict[str, int]) -> None:
    STATE_DIR.mkdir(parents=True, exist_ok=True)
    HISTORY_FILE.write_text(json.dumps(history, indent=2) + "\n", encoding="utf-8")


def enforce_blacksmith_blacklist(repo: str) -> list[str]:
    """Scan all workflow/action YAML files in a repo and strip blacklisted Blacksmith runners/actions."""
    repo_dir = WORKSPACE_ROOT / repo
    if not repo_dir.is_dir():
        return []

    candidate_files: list[Path] = []
    for base in (repo_dir / ".github", repo_dir / "entheai" / ".github"):
        if base.is_dir():
            candidate_files.extend(sorted(base.rglob("*.yml")))
            candidate_files.extend(sorted(base.rglob("*.yaml")))

    if not candidate_files:
        return []

    modified_rel_paths: list[str] = []
    backups: dict[Path, str] = {}
    for wf_file in candidate_files:
        rel_path = str(wf_file.relative_to(repo_dir))
        st = subprocess.run(
            ["git", "status", "--porcelain", "--", rel_path],
            cwd=repo_dir,
            capture_output=True,
            text=True,
        )
        if st.returncode != 0 or st.stdout.strip():
            continue
        original = wf_file.read_text(encoding="utf-8", errors="ignore")
        updated = original
        for pattern, replacement in BLACKSMITH_REPLACEMENTS:
            updated = pattern.sub(replacement, updated)
        if updated != original:
            backups[wf_file] = original
            wf_file.write_text(updated, encoding="utf-8")
            modified_rel_paths.append(rel_path)

    if not modified_rel_paths:
        return []

    subprocess.run(["git", "add", "--", *modified_rel_paths], cwd=repo_dir, check=False)
    c = subprocess.run(
        ["git", "commit", "-m", "ci: enforce blacklist against Blacksmith runners and actions"],
        cwd=repo_dir,
        capture_output=True,
        text=True,
    )
    if c.returncode == 0:
        p = subprocess.run(["git", "push", "origin", "HEAD"], cwd=repo_dir, capture_output=True, text=True)
        if p.returncode == 0:
            return [f"{p_rel}: stripped blacklisted Blacksmith runner/action" for p_rel in modified_rel_paths]
        subprocess.run(["git", "reset", "--hard", "HEAD~1"], cwd=repo_dir, check=False)
    else:
        for wf_file, orig_text in backups.items():
            wf_file.write_text(orig_text, encoding="utf-8")
    return []


def try_auto_fix_local_workflow(repo: str, wf_path: str, log_text: str) -> str | None:
    """Attempt a deterministic, safe local fix in /data/8b-is/<repo> if applicable."""
    repo_dir = WORKSPACE_ROOT / repo
    wf_file = repo_dir / wf_path
    if not wf_file.is_file():
        return None

    # Only auto-commit on a clean working tree for that workflow file
    st = subprocess.run(
        ["git", "status", "--porcelain", "--", wf_path],
        cwd=repo_dir,
        capture_output=True,
        text=True,
    )
    if st.returncode != 0 or st.stdout.strip():
        return None

    original = wf_file.read_text(encoding="utf-8")
    updated = original
    fix_reason = None

    # Pattern 1: Missing GH_TOKEN for `gh` CLI in workflow
    if "To use GitHub CLI in a GitHub Actions workflow, set the GH_TOKEN environment variable" in log_text:
        if "GH_TOKEN:" not in updated:
            updated = re.sub(
                r"(env:\n)",
                r"\1          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}\n",
                updated,
                count=1,
            )
            fix_reason = "fix(ci): add GH_TOKEN to workflow env for GitHub CLI"

    # Pattern 2: GitHub Pages not enabled on repo (configure-pages 404 Not Found)
    elif "https://docs.github.com/rest/pages/pages#get-a-apiname-pages-site" in log_text:
        if "actions/configure-pages" in updated and "continue-on-error: true" not in updated:
            updated = re.sub(
                r"(- (?:name: [^\n]+\n\s+)?uses: actions/configure-pages@[^\n]+)",
                r"\1\n        id: pages_cfg\n        continue-on-error: true",
                updated,
                count=1,
            )
            updated = re.sub(
                r"(- (?:name: [^\n]+\n\s+)?(?:id: [^\n]+\n\s+)?uses: actions/deploy-pages@[^\n]+)",
                r"\1\n        continue-on-error: true",
                updated,
                count=1,
            )
            fix_reason = "fix(ci): make GitHub Pages deploy non-blocking when Pages is not enabled on repo"

    if updated != original and fix_reason:
        wf_file.write_text(updated, encoding="utf-8")
        subprocess.run(["git", "add", wf_path], cwd=repo_dir, check=True)
        c = subprocess.run(["git", "commit", "-m", fix_reason], cwd=repo_dir, capture_output=True, text=True)
        if c.returncode == 0:
            p = subprocess.run(["git", "push", "origin", "HEAD"], cwd=repo_dir, capture_output=True, text=True)
            if p.returncode == 0:
                return fix_reason
            subprocess.run(["git", "reset", "--hard", "HEAD~1"], cwd=repo_dir)
        else:
            wf_file.write_text(original, encoding="utf-8")
    return None


def process_repo(gh: GitHubClient, repo: str, rerun_history: dict[str, int], clean_runs: bool, apply_fixes: bool) -> dict:
    summary = {
        "repo": repo,
        "workflows_disabled": [],
        "runs_deleted": 0,
        "runs_retried": [],
        "auto_fixes_applied": [],
        "active_failures": [],
    }
    try:
        wfs = gh.request("GET", f"repos/8b-is/{repo}/actions/workflows").get("workflows", [])
        runs = gh.request("GET", f"repos/8b-is/{repo}/actions/runs?per_page=30").get("workflow_runs", [])
    except Exception as e:
        summary["error"] = str(e)
        return summary

    if not wfs and not runs:
        return summary

    # 1. Disable noisy upstream fork workflows if configured
    disable_patterns = FORK_DISABLE_WORKFLOW_PATTERNS.get(repo, ())
    if apply_fixes and disable_patterns:
        for wf in wfs:
            wf_path = wf.get("path", "")
            if wf.get("state") == "active" and any(pat in wf_path for pat in disable_patterns):
                try:
                    gh.request("PUT", f"repos/8b-is/{repo}/actions/workflows/{wf['id']}/disable")
                    summary["workflows_disabled"].append(wf_path)
                except Exception:
                    pass

    # 1b. Enforce Blacksmith runner/action blacklist on local repository workflows
    if apply_fixes:
        bs_fixes = enforce_blacksmith_blacklist(repo)
        if bs_fixes:
            summary["auto_fixes_applied"].extend(bs_fixes)

    # Group completed runs by workflow path/name to find latest per workflow
    latest_by_wf = {}
    for r in runs:
        key = r.get("path") or r.get("name") or str(r.get("workflow_id"))
        if key not in latest_by_wf:
            latest_by_wf[key] = r

    local_head_sha = None
    repo_dir = WORKSPACE_ROOT / repo
    if repo_dir.is_dir():
        rev = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=repo_dir,
            capture_output=True,
            text=True,
        )
        if rev.returncode == 0 and rev.stdout.strip():
            local_head_sha = rev.stdout.strip()

    for r in runs:
        run_id = r["id"]
        status = r.get("status")
        conclusion = r.get("conclusion")
        wf_path = str(r.get("path") or "")
        wf_name = str(r.get("name") or "")
        key = wf_path or wf_name or str(r.get("workflow_id"))
        is_latest = latest_by_wf.get(key, {}).get("id") == run_id

        if status != "completed":
            # Cancel any queued/in-progress run that requests a blacklisted Blacksmith runner
            if apply_fixes:
                try:
                    jobs = gh.request("GET", f"repos/8b-is/{repo}/actions/runs/{run_id}/jobs").get("jobs", [])
                    if any(
                        any("blacksmith" in str(lbl).lower() for lbl in (j.get("labels") or []))
                        for j in jobs
                    ):
                        gh.request("POST", f"repos/8b-is/{repo}/actions/runs/{run_id}/cancel")
                        summary["auto_fixes_applied"].append(f"{wf_path}: cancelled blacklisted Blacksmith run {run_id}")
                except Exception:
                    pass
            continue

        is_bad = conclusion in ("failure", "startup_failure", "timed_out", "action_required", "cancelled")
        if not is_bad:
            continue

        # Dynamic agent / copilot / advanced-security runs or disabled fork workflows -> delete
        is_dynamic = wf_path.startswith("dynamic/") or r.get("event") == "dynamic"
        is_disabled_fork_wf = any(pat in wf_path for pat in disable_patterns)
        is_missing_local_wf = (
            repo_dir.is_dir()
            and wf_path.startswith(".github/workflows/")
            and not (repo_dir / wf_path).is_file()
        )
        is_superseded_sha = bool(local_head_sha and r.get("head_sha") and r.get("head_sha") != local_head_sha)

        if clean_runs and (
            not is_latest
            or is_dynamic
            or is_disabled_fork_wf
            or is_missing_local_wf
            or is_superseded_sha
            or conclusion == "cancelled"
        ):
            try:
                gh.request("DELETE", f"repos/8b-is/{repo}/actions/runs/{run_id}")
                summary["runs_deleted"] += 1
                continue
            except Exception:
                pass

        # If this is the latest run of an active repo workflow and it failed, inspect it
        if is_latest and wf_path.startswith(".github/workflows/"):
            log_text = ""
            try:
                jobs = gh.request("GET", f"repos/8b-is/{repo}/actions/runs/{run_id}/jobs").get("jobs", [])
                for j in jobs:
                    if j.get("conclusion") in ("failure", "timed_out", "startup_failure"):
                        log_text += gh.request("GET", f"repos/8b-is/{repo}/actions/jobs/{j['id']}/logs", raw=True)
                        break
            except Exception:
                pass

            # Try local auto-fix first
            if apply_fixes and log_text:
                fix = try_auto_fix_local_workflow(repo, wf_path, log_text)
                if fix:
                    summary["auto_fixes_applied"].append(f"{wf_path}: {fix}")
                    if clean_runs:
                        try:
                            gh.request("DELETE", f"repos/8b-is/{repo}/actions/runs/{run_id}")
                            summary["runs_deleted"] += 1
                        except Exception:
                            pass
                    continue

            # Check if transient failure or startup_failure eligible for rerun
            is_transient = conclusion in ("startup_failure", "timed_out") or any(
                sig in log_text for sig in TRANSIENT_LOG_SIGNATURES
            )
            hist_key = f"{repo}:{wf_path}:{r.get('head_sha', '')[:12]}"
            attempts = rerun_history.get(hist_key, 0)
            if apply_fixes and is_transient and attempts < 2:
                try:
                    if wf_path.endswith(".yml") or wf_path.endswith(".yaml"):
                        wf_basename = Path(wf_path).name
                        gh.request(
                            "POST",
                            f"repos/8b-is/{repo}/actions/workflows/{wf_basename}/dispatches",
                            payload={"ref": r.get("head_branch") or "main"},
                        )
                    else:
                        gh.request("POST", f"repos/8b-is/{repo}/actions/runs/{run_id}/rerun")
                    rerun_history[hist_key] = attempts + 1
                    summary["runs_retried"].append({"run_id": run_id, "workflow": wf_path})
                    if clean_runs:
                        try:
                            gh.request("DELETE", f"repos/8b-is/{repo}/actions/runs/{run_id}")
                            summary["runs_deleted"] += 1
                        except Exception:
                            pass
                    continue
                except Exception:
                    pass

            summary["active_failures"].append(
                {
                    "run_id": run_id,
                    "workflow": wf_path,
                    "name": wf_name,
                    "conclusion": conclusion,
                    "branch": r.get("head_branch"),
                    "created_at": r.get("created_at"),
                }
            )

    return summary


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--clean-runs", action="store_true", default=True, help="Delete stale/superseded failed runs")
    parser.add_argument("--apply-fixes", action="store_true", default=True, help="Apply auto-fixes and reruns")
    parser.add_argument("--workers", type=int, default=12)
    args = parser.parse_args()

    token = load_git_mega()
    write_token = load_write_token()
    gh = GitHubClient(token, write_token=write_token)
    repos = list_active_org_repos(gh)
    rerun_history = load_rerun_history()

    results = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = [
            pool.submit(process_repo, gh, repo, rerun_history, args.clean_runs, args.apply_fixes)
            for repo in repos
        ]
        for fut in concurrent.futures.as_completed(futures):
            res = fut.result()
            if any(
                res.get(k)
                for k in ("workflows_disabled", "runs_deleted", "runs_retried", "auto_fixes_applied", "active_failures", "error")
            ):
                results.append(res)

    results.sort(key=lambda x: x["repo"])
    save_rerun_history(rerun_history)

    total_deleted = sum(int(r.get("runs_deleted", 0)) for r in results)
    total_disabled = sum(len(r.get("workflows_disabled", [])) for r in results)
    total_retried = sum(len(r.get("runs_retried", [])) for r in results)
    total_fixes = sum(len(r.get("auto_fixes_applied", [])) for r in results)
    total_active_failures = sum(len(r.get("active_failures", [])) for r in results)
    healthy_repos = len(repos) - sum(1 for r in results if r.get("active_failures"))
    health_permille = (healthy_repos * 1000) // max(len(repos), 1)

    report = {
        "schema": 1,
        "timestamp": dt.datetime.now(dt.timezone.utc).isoformat(),
        "total_active_repos": len(repos),
        "healthy_repos": healthy_repos,
        "health_permille": health_permille,
        "runs_deleted": total_deleted,
        "workflows_disabled": total_disabled,
        "runs_retried": total_retried,
        "auto_fixes_applied": total_fixes,
        "active_failures_count": total_active_failures,
        "repositories": results,
    }

    STATE_DIR.mkdir(parents=True, exist_ok=True)
    STATUS_FILE.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(
        json.dumps(
            {
                "timestamp": report["timestamp"],
                "total_active_repos": report["total_active_repos"],
                "healthy_repos": report["healthy_repos"],
                "health_permille": report["health_permille"],
                "runs_deleted": report["runs_deleted"],
                "workflows_disabled": report["workflows_disabled"],
                "runs_retried": report["runs_retried"],
                "auto_fixes_applied": report["auto_fixes_applied"],
                "active_failures_count": report["active_failures_count"],
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
