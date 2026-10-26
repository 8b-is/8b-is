#![forbid(unsafe_code)]
//! `alexiai-8b-publisher` — Autonomous Zero-Float Publisher for `https://8b.is`
//! (`5.78.220.57:/var/www/8b-is/public/{index.html,live-pulse.json}`) and `/home/hue/8b-is/8b-is/README.md`.

use alexiai_8b_publisher::*;
use std::env;
use std::fs;
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

fn write_file_atomic(target_path: &str, content: &str) -> Result<(), String> {
    let tmp_path = format!("{}.tmp", target_path);
    fs::write(&tmp_path, content.as_bytes()).map_err(|e| format!("write {}: {}", tmp_path, e))?;
    fs::rename(&tmp_path, target_path)
        .map_err(|e| format!("rename {} -> {}: {}", tmp_path, target_path, e))?;
    Ok(())
}

fn push_to_remote_8b_edge(local_json_tmp: &str, local_html_tmp: &str) -> bool {
    let remote_host =
        env::var("SITE_8B_SSH_HOST").unwrap_or_else(|_| "root@5.78.220.57".to_string());
    let scp_status = Command::new("scp")
        .args([
            "-q",
            "-o",
            "ConnectTimeout=6",
            "-o",
            "BatchMode=yes",
            local_json_tmp,
            local_html_tmp,
            &format!("{}:/var/www/8b-is/public/", remote_host),
        ])
        .status();

    if !matches!(scp_status, Ok(st) if st.success()) {
        return false;
    }

    let mv_status = Command::new("ssh")
        .args([
            "-o",
            "ConnectTimeout=6",
            "-o",
            "BatchMode=yes",
            &remote_host,
            "mv -f /var/www/8b-is/public/live-pulse.json.tmp /var/www/8b-is/public/live-pulse.json && mv -f /var/www/8b-is/public/index.html.tmp /var/www/8b-is/public/index.html",
        ])
        .status();

    matches!(mv_status, Ok(st) if st.success())
}

fn unique_repo_dirs() -> Vec<PathBuf> {
    let mut seen = Vec::new();
    for repo_dir in ["/home/hue/8b-is/8b-is", "/data/8b-is/8b-is"] {
        let p = Path::new(repo_dir);
        if p.is_dir() {
            let canon = fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
            if !seen.contains(&canon) {
                seen.push(canon);
            }
        }
    }
    seen
}

fn update_and_commit_readme(
    readme_md: &str,
    pulse: &mut CompanyPulseDispatch,
    do_commit: bool,
) -> bool {
    let mut updated_any = false;
    for dir_path in unique_repo_dirs() {
        let Some(repo_dir) = dir_path.to_str() else {
            continue;
        };
        let readme_path = format!("{}/README.md", repo_dir);
        if write_file_atomic(&readme_path, readme_md).is_ok() {
            updated_any = true;
            if do_commit && dir_path.join(".git").exists() {
                let _ = Command::new("git")
                    .args(["-C", repo_dir, "add", "README.md"])
                    .status();
                let commit_msg = pulse.dad_joke_commit.commit_subject.clone();
                let _ = Command::new("git")
                    .args(["-C", repo_dir, "commit", "-m", &commit_msg])
                    .status();
                if let Ok(out) = Command::new("git")
                    .args(["-C", repo_dir, "rev-parse", "--short", "HEAD"])
                    .output()
                {
                    if out.status.success() {
                        let new_sha = String::from_utf8_lossy(&out.stdout).trim().to_string();
                        if !new_sha.is_empty() {
                            pulse.dad_joke_commit.commit_sha =
                                sanitize_zero_float_text_with_limit(&new_sha, "75d42ac", 40);
                        }
                    }
                }
            }
        }
    }
    updated_any
}

fn has_local_ollama() -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], 11434));
    TcpStream::connect_timeout(&addr, Duration::from_millis(150)).is_ok()
}

fn is_edge_pulse_fresh(json_path: &str, max_age_secs: u64) -> bool {
    let Ok(meta) = fs::metadata(json_path) else {
        return false;
    };
    let Ok(mtime) = meta.modified() else {
        return false;
    };
    let Ok(elapsed) = SystemTime::now().duration_since(mtime) else {
        return false;
    };
    elapsed.as_secs() < max_age_secs
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcmd = args.get(1).map(|s| s.as_str()).unwrap_or("publish");
    let no_commit = args.iter().any(|a| a == "--no-commit");
    let force_local = args.iter().any(|a| a == "--force-local");

    let mut pulse = collect_company_pulse_dispatch(None);
    let readme_md = render_readme_markdown(&pulse);

    let is_publish = !matches!(
        subcmd,
        "render-json" | "render-html" | "render-readme" | "verify"
    );

    let readme_updated = if is_publish {
        update_and_commit_readme(&readme_md, &mut pulse, !no_commit)
    } else {
        false
    };

    let pulse_json = render_live_pulse_json(&pulse);
    let index_html = render_flagship_index_html(&pulse);

    let json_floats = count_floating_point_literals(&pulse_json);
    let html_floats = count_floating_point_literals(&index_html);
    let md_floats = count_floating_point_literals(&readme_md);
    let strict_json_dots = count_strict_digit_dot_digit(&pulse_json);
    let strict_html_dots = count_strict_digit_dot_digit(&index_html);
    let strict_md_dots = count_strict_digit_dot_digit(&readme_md);

    if json_floats != 0
        || html_floats != 0
        || md_floats != 0
        || strict_json_dots != 0
        || strict_html_dots != 0
        || strict_md_dots != 0
    {
        eprintln!(
            "FATAL ZERO-FLOAT LAW VIOLATION: json_floats={} html_floats={} md_floats={} strict_json={} strict_html={} strict_md={}",
            json_floats, html_floats, md_floats, strict_json_dots, strict_html_dots, strict_md_dots
        );
        std::process::exit(1);
    }

    match subcmd {
        "render-json" => {
            print!("{}", pulse_json);
            return;
        }
        "render-html" => {
            print!("{}", index_html);
            return;
        }
        "render-readme" => {
            print!("{}", readme_md);
            return;
        }
        "verify" => {
            println!(
                "{{\"ok\":true,\"sovereign_law\":\"strict-8b-is-no-float\",\"float_count\":0,\"strict_digit_dot_digit\":0,\"pulse_seq\":{},\"json_bytes\":{},\"html_bytes\":{},\"readme_bytes\":{}}}",
                pulse.pulse_seq,
                pulse_json.len(),
                index_html.len(),
                readme_md.len()
            );
            return;
        }
        _ => {}
    }

    let site_root =
        env::var("SITE_8B_ROOT").unwrap_or_else(|_| "/var/www/8b-is/public".to_string());
    let mut local_site_written = false;
    if Path::new(&site_root).is_dir() {
        let json_target = format!("{}/live-pulse.json", site_root);
        let html_target = format!("{}/index.html", site_root);
        // If running on the 5.78.220.57 edge node without local Ollama, do not clobber a fresh
        // (< 210s old) A6000 dispatch that was pushed from 49.13.101.247.
        let skip_clobber =
            !force_local && !has_local_ollama() && is_edge_pulse_fresh(&json_target, 210);
        if skip_clobber
            || (write_file_atomic(&json_target, &pulse_json).is_ok()
                && write_file_atomic(&html_target, &index_html).is_ok())
        {
            local_site_written = true;
        }
    }

    let tmp_json = "/tmp/live-pulse.json.tmp";
    let tmp_html = "/tmp/index.html.tmp";
    let _ = fs::write(tmp_json, pulse_json.as_bytes());
    let _ = fs::write(tmp_html, index_html.as_bytes());

    let remote_pushed = if !local_site_written {
        push_to_remote_8b_edge(tmp_json, tmp_html)
    } else {
        true
    };

    println!(
        "{{\"ok\":true,\"sovereign_law\":\"strict-8b-is-no-float\",\"float_count\":0,\"strict_digit_dot_digit\":0,\"pulse_seq\":{},\"generated_iso_utc\":\"{}\",\"commit_sha\":\"{}\",\"local_site_written\":{},\"remote_edge_pushed\":{},\"readme_updated\":{},\"json_bytes\":{},\"html_bytes\":{},\"readme_bytes\":{}}}",
        pulse.pulse_seq,
        pulse.generated_iso_utc,
        pulse.dad_joke_commit.commit_sha,
        local_site_written,
        remote_pushed,
        readme_updated,
        pulse_json.len(),
        index_html.len(),
        readme_md.len()
    );
}
