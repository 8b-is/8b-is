#![forbid(unsafe_code)]
//! `alexiai-8b-publisher` — Autonomous Zero-Float Publisher for `https://8b.is`
//! (`5.78.220.57:/var/www/8b-is/public/{index.html,live-pulse.json}`) and `/home/hue/8b-is/8b-is/README.md`.

use alexiai_8b_publisher::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn write_file_atomic(target_path: &str, content: &str) -> Result<(), String> {
    let tmp_path = format!("{}.tmp", target_path);
    fs::write(&tmp_path, content.as_bytes()).map_err(|e| format!("write {}: {}", tmp_path, e))?;
    fs::rename(&tmp_path, target_path)
        .map_err(|e| format!("rename {} -> {}: {}", tmp_path, target_path, e))?;
    Ok(())
}

fn push_to_remote_8b_edge(local_json: &str, local_html: &str) -> bool {
    let remote_host = env::var("SITE_8B_SSH_HOST").unwrap_or_else(|_| "root@5.78.220.57".to_string());
    let scp_status = Command::new("scp")
        .args([
            "-q",
            "-o",
            "ConnectTimeout=6",
            "-o",
            "BatchMode=yes",
            local_json,
            local_html,
            &format!("{}:/var/www/8b-is/public/", remote_host),
        ])
        .status();

    match scp_status {
        Ok(st) if st.success() => true,
        _ => false,
    }
}

fn update_and_commit_readme(readme_md: &str, pulse: &CompanyPulseDispatch, do_commit: bool) -> bool {
    let mut updated_any = false;
    for repo_dir in ["/home/hue/8b-is/8b-is", "/data/8b-is/8b-is"] {
        let dir_path = Path::new(repo_dir);
        if dir_path.is_dir() {
            let readme_path = format!("{}/README.md", repo_dir);
            if write_file_atomic(&readme_path, readme_md).is_ok() {
                updated_any = true;
                if do_commit && dir_path.join(".git").exists() {
                    let _ = Command::new("git")
                        .args(["-C", repo_dir, "add", "README.md"])
                        .status();
                    let commit_msg = format!(
                        "chore(alexiai-pulse): update 8b-is live company pulse [#{}] — {}",
                        pulse.pulse_seq, pulse.dad_joke_commit.dad_joke
                    );
                    let _ = Command::new("git")
                        .args(["-C", repo_dir, "commit", "-m", &commit_msg])
                        .status();
                }
            }
        }
    }
    updated_any
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcmd = args.get(1).map(|s| s.as_str()).unwrap_or("publish");
    let no_commit = args.iter().any(|a| a == "--no-commit");

    let pulse = collect_company_pulse_dispatch(None);
    let pulse_json = render_live_pulse_json(&pulse);
    let index_html = render_flagship_index_html(&pulse);
    let readme_md = render_readme_markdown(&pulse);

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
        if write_file_atomic(&json_target, &pulse_json).is_ok()
            && write_file_atomic(&html_target, &index_html).is_ok()
        {
            local_site_written = true;
        }
    }

    let tmp_json = "/tmp/live-pulse.json";
    let tmp_html = "/tmp/index.html";
    let _ = write_file_atomic(tmp_json, &pulse_json);
    let _ = write_file_atomic(tmp_html, &index_html);

    let remote_pushed = if !local_site_written {
        push_to_remote_8b_edge(tmp_json, tmp_html)
    } else {
        true
    };

    let readme_updated = update_and_commit_readme(&readme_md, &pulse, !no_commit);

    println!(
        "{{\"ok\":true,\"sovereign_law\":\"strict-8b-is-no-float\",\"float_count\":0,\"strict_digit_dot_digit\":0,\"pulse_seq\":{},\"generated_iso_utc\":\"{}\",\"local_site_written\":{},\"remote_edge_pushed\":{},\"readme_updated\":{},\"json_bytes\":{},\"html_bytes\":{},\"readme_bytes\":{}}}",
        pulse.pulse_seq,
        pulse.generated_iso_utc,
        local_site_written,
        remote_pushed,
        readme_updated,
        pulse_json.len(),
        index_html.len(),
        readme_md.len()
    );
}
