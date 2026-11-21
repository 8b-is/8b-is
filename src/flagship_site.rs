//! Flagship multi-theme `https://8b.is` renderer (100% Pure Rust, Zero External Crates,
//! Strict Zero-Float Law, Zero Purple, Exact Integer Q10 Geometry, Live Cheet.is Buddy).

use crate::{html_escape, render_live_pulse_json, CompanyPulseDispatch};

pub fn render_flagship_index_html(p: &CompanyPulseDispatch) -> String {
    let embedded_pulse_json = render_live_pulse_json(p);
    let reflection_html = html_escape(&p.alexiai_reflection);
    let deity_html = html_escape(&p.gaia_fold.dominant_deity);
    let mood_html = html_escape(&p.gaia_fold.mood);
    let coherence_html = html_escape(&p.gaia_fold.coherence_pq);
    let qt_tag_html = html_escape(&p.gaia_fold.quant_tern_tag);
    let wupz_html = html_escape(&p.gaia_fold.wupz_xor_fold_hex);
    let gravity_html = html_escape(&p.gaia_fold.gravity_pq);
    let entropy_html = html_escape(&p.gaia_fold.entropy_pq);
    let weather_html = html_escape(&p.gaia_fold.weather_pq);
    let resonance_html = html_escape(&p.gaia_fold.resonance_pq);

    let joke_html = html_escape(&p.dad_joke_commit.dad_joke);
    let commit_sha_html = html_escape(&p.dad_joke_commit.commit_sha);
    let commit_subj_html = html_escape(&p.dad_joke_commit.commit_subject);
    let commit_repo_html = html_escape(&p.dad_joke_commit.repo_name);

    let cap_fn_html = html_escape(&p.g8t.latest_capsule.fn_name);
    let cap_sig_html = html_escape(&p.g8t.latest_capsule.signature);
    let cap_id_html = html_escape(&p.g8t.latest_capsule.capsule_id);
    let cap_cyc_html = html_escape(&p.g8t.latest_capsule.cycles_per_op_pq);
    let cap_eq_html = html_escape(&p.g8t.latest_capsule.blake3_eq_class);

    let vram_ratio_html = html_escape(&p.fleet.vram_ratio_pq);
    let power_watts_html = html_escape(&p.fleet.power_watts_pq);
    let salience_html = html_escape(&p.fleet.marine_salience_pq);
    let c64_ratio_html = html_escape(&p.fleet.c64_ram_ratio_pq);
    let c64_event_html = html_escape(&p.fleet.recent_c64_event);

    let soaker_agent_html = html_escape(&p.hottub.featured_soaker_agent);
    let soaker_thought_html = html_escape(&p.hottub.featured_soaker_thought);
    let wave_topic_html = html_escape(&p.hottub.latest_wave_topic);
    let iso_html = html_escape(&p.generated_iso_utc);
    let model_html = html_escape(&p.alexiai_model);
    let kompress_saved_html = html_escape(&p.g8t.kompress_saved_ratio_pq);

    format!(
        r##"<!doctype html>
<html lang="en" data-theme="ink" data-scale="normal" data-motion="full">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
  <meta name="theme-color" content="#060a10" id="meta-theme-color" />
  <meta name="description" content="8b·is — Sovereign AI, Pure Rust, Bare-Metal OS (AyeOS), Exact p/q Rational Math (Zero Floats), PQTRN, Phoenix Audio, Smart Tree, AyeFire, ZenCooker, and Ethical Local-First Tools." />
  <meta property="og:type" content="website" />
  <meta property="og:site_name" content="8b·is" />
  <meta property="og:title" content="8b·is — Technology You Own · Sovereign AI · Pure Rust · Exact p/q Math" />
  <meta property="og:description" content="Local-first AI (ALEXIAI &amp; PQTRN), bare-metal Rust OS (AyeOS), home firewall (AyeFire), rational-math audio (Phoenix), Smart Tree, ZenCooker, ESP32 Triads, and The Cheet — built with zero floats and zero surveillance." />
  <meta property="og:image" content="https://8b.is/hero/hero.svg" />
  <link rel="icon" type="image/svg+xml" href="./sun.svg" />
  <link rel="canonical" href="https://8b.is/" />
  <link rel="alternate" type="application/atom+xml" title="8b·is — research (Atom)" href="/feed.xml" />
  <link rel="alternate" type="application/rss+xml" title="8b·is — research (RSS)" href="/rss.xml" />
  <title>8b·is — Sovereign AI · Pure Rust · Exact p/q Math · Fine Touch From Within</title>
  <style>
    /* ========================================================================
       8b.is MULTI-THEME DESIGN SYSTEM (8 Curated Themes · Zero Floats)
       ======================================================================== */
    :root {{
      color-scheme: dark;
      --bg: #060a10;
      --bg-panel: #0b121c;
      --bg-card: #101926;
      --bg-card-hover: #152233;
      --bg-elev: #0d1623;
      --ink: #ecf3f8;
      --ink-soft: #a4b7c9;
      --ink-muted: #6e8399;
      --accent: #4ecdc4;
      --accent-2: #f39c12;
      --accent-3: #2ecc71;
      --accent-4: #38bdf8;
      --accent-5: #e86f51;
      --accent-ink: #041014;
      --line: #1b2b3d;
      --line-strong: #2a435e;
      --shadow: rgba(0, 0, 0, 48%);
      --header-bg: rgba(6, 10, 16, 88%);
      --canvas-bg: #070d16;
      --canvas-grid: #162638;
      --glow-a: rgba(78, 205, 196, 10%);
      --glow-b: rgba(243, 156, 18, 7%);
      --glow-c: rgba(46, 204, 113, 6%);
      --font-sans: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --font-serif: Georgia, "Iowan Old Style", "Palatino Linotype", serif;
      --font-mono: ui-monospace, "JetBrains Mono", "Fira Code", "SFMono-Regular", Menlo, Consolas, monospace;
      --base-size: 16px;
      /* Semantic aliases inherited by cheet.is Shadow DOM widget across all 8 themes */
      --mint: var(--accent);
      --gold: var(--accent-2);
      --panel: var(--bg-panel);
      --paper: var(--bg-card);
      --dim: var(--ink-soft);
      --mono: var(--font-mono);
      --serif: var(--font-serif);
    }}

    html[data-theme="paper"],
    html[data-theme="nordic"],
    html[data-theme="solar-light"] {{
      color-scheme: light;
    }}

    html[data-theme="paper"] {{
      --bg: #faf7f2;
      --bg-panel: #f2ece2;
      --bg-card: #ffffff;
      --bg-card-hover: #f7f2e9;
      --bg-elev: #f5efe5;
      --ink: #1a1d21;
      --ink-soft: #4c5157;
      --ink-muted: #6c6a63;
      --accent: #0f766e;
      --accent-2: #b45309;
      --accent-3: #15803d;
      --accent-4: #0369a1;
      --accent-5: #c2410c;
      --accent-ink: #ffffff;
      --line: #ddd4c5;
      --line-strong: #c2b6a1;
      --shadow: rgba(26, 29, 33, 10%);
      --header-bg: rgba(250, 247, 242, 92%);
      --canvas-bg: #f6f1e7;
      --canvas-grid: #dfd6c7;
      --glow-a: rgba(15, 118, 110, 7%);
      --glow-b: rgba(180, 83, 9, 6%);
      --glow-c: rgba(21, 128, 61, 5%);
    }}

    html[data-theme="nordic"] {{
      --bg: #f3f7fa;
      --bg-panel: #e6eef5;
      --bg-card: #ffffff;
      --bg-card-hover: #f1f6fa;
      --bg-elev: #ebf2f8;
      --ink: #132234;
      --ink-soft: #43576e;
      --ink-muted: #637991;
      --accent: #0e7490;
      --accent-2: #b45309;
      --accent-3: #047857;
      --accent-4: #0284c7;
      --accent-5: #be123c;
      --accent-ink: #ffffff;
      --line: #cfdbe6;
      --line-strong: #aec2d4;
      --shadow: rgba(19, 34, 52, 10%);
      --header-bg: rgba(243, 247, 250, 92%);
      --canvas-bg: #edf3f8;
      --canvas-grid: #d2dfe9;
      --glow-a: rgba(14, 116, 144, 7%);
      --glow-b: rgba(4, 120, 87, 5%);
      --glow-c: rgba(2, 132, 199, 5%);
    }}

    html[data-theme="forest"] {{
      --bg: #08120e;
      --bg-panel: #0f1d17;
      --bg-card: #14271f;
      --bg-card-hover: #1a3328;
      --bg-elev: #11221b;
      --ink: #e4f2eb;
      --ink-soft: #9ec0b0;
      --ink-muted: #6c9180;
      --accent: #34d399;
      --accent-2: #e8b05c;
      --accent-3: #5eead4;
      --accent-4: #38bdf8;
      --accent-5: #f97316;
      --accent-ink: #05120c;
      --line: #1f3a2e;
      --line-strong: #2f5745;
      --shadow: rgba(0, 0, 0, 50%);
      --header-bg: rgba(8, 18, 14, 90%);
      --canvas-bg: #0a1712;
      --canvas-grid: #1b3328;
      --glow-a: rgba(52, 211, 153, 9%);
      --glow-b: rgba(232, 176, 92, 7%);
      --glow-c: rgba(94, 234, 212, 6%);
    }}

    html[data-theme="vaked"] {{
      --bg: #14110e;
      --bg-panel: #1d1814;
      --bg-card: #251f19;
      --bg-card-hover: #2f2720;
      --bg-elev: #201b16;
      --ink: #f3ece1;
      --ink-soft: #c5b6a1;
      --ink-muted: #938370;
      --accent: #e8a33d;
      --accent-2: #52b788;
      --accent-3: #f2c879;
      --accent-4: #4ecdc4;
      --accent-5: #d96b43;
      --accent-ink: #17120c;
      --line: #352c23;
      --line-strong: #4e4033;
      --shadow: rgba(0, 0, 0, 52%);
      --header-bg: rgba(20, 17, 14, 90%);
      --canvas-bg: #181410;
      --canvas-grid: #302820;
      --glow-a: rgba(232, 163, 61, 10%);
      --glow-b: rgba(82, 183, 136, 7%);
      --glow-c: rgba(242, 200, 121, 6%);
    }}

    html[data-theme="phosphor"] {{
      --bg: #050b06;
      --bg-panel: #0b150d;
      --bg-card: #102014;
      --bg-card-hover: #152b1b;
      --bg-elev: #0d1a10;
      --ink: #daf5e0;
      --ink-soft: #8fc49c;
      --ink-muted: #5d8f69;
      --accent: #39d98a;
      --accent-2: #f4a261;
      --accent-3: #4ecdc4;
      --accent-4: #80ed99;
      --accent-5: #e76f51;
      --accent-ink: #041007;
      --line: #1c3823;
      --line-strong: #2c5737;
      --shadow: rgba(0, 0, 0, 55%);
      --header-bg: rgba(5, 11, 6, 92%);
      --canvas-bg: #071009;
      --canvas-grid: #16301c;
      --glow-a: rgba(57, 217, 138, 10%);
      --glow-b: rgba(244, 162, 97, 6%);
      --glow-c: rgba(78, 205, 196, 6%);
    }}

    html[data-theme="solar-light"] {{
      --bg: #fdf6e3;
      --bg-panel: #eee8d5;
      --bg-card: #fffaf0;
      --bg-card-hover: #f5eedb;
      --bg-elev: #f3ecda;
      --ink: #34444b;
      --ink-soft: #586e75;
      --ink-muted: #7b8c90;
      --accent: #2aa198;
      --accent-2: #b58900;
      --accent-3: #859900;
      --accent-4: #268bd2;
      --accent-5: #cb4b16;
      --accent-ink: #fdf6e3;
      --line: #dcd4bd;
      --line-strong: #bfb59b;
      --shadow: rgba(52, 68, 75, 10%);
      --header-bg: rgba(253, 246, 227, 92%);
      --canvas-bg: #f6efdc;
      --canvas-grid: #dfd7c1;
      --glow-a: rgba(42, 161, 152, 8%);
      --glow-b: rgba(181, 137, 0, 6%);
      --glow-c: rgba(133, 153, 0, 5%);
    }}

    html[data-theme="contrast"] {{
      --bg: #000000;
      --bg-panel: #0b0b0b;
      --bg-card: #121212;
      --bg-card-hover: #1b1b1b;
      --bg-elev: #0e0e0e;
      --ink: #ffffff;
      --ink-soft: #d7e2ec;
      --ink-muted: #a6b4c2;
      --accent: #00f0ff;
      --accent-2: #ffd166;
      --accent-3: #00ff88;
      --accent-4: #4cc9f0;
      --accent-5: #ff6b6b;
      --accent-ink: #000000;
      --line: #383838;
      --line-strong: #6b6b6b;
      --shadow: rgba(0, 0, 0, 80%);
      --header-bg: rgba(0, 0, 0, 96%);
      --canvas-bg: #040404;
      --canvas-grid: #252525;
      --glow-a: rgba(0, 240, 255, 8%);
      --glow-b: rgba(255, 209, 102, 7%);
      --glow-c: rgba(0, 255, 136, 6%);
    }}

    html[data-scale="large"] {{
      --base-size: 19px;
    }}

    * {{
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }}

    html {{
      scroll-behavior: smooth;
      background-color: var(--bg);
      color: var(--ink);
      font-family: var(--font-sans);
      font-size: var(--base-size);
      line-height: 162%;
    }}

    html[data-motion="calm"],
    html[data-motion="calm"] * {{
      scroll-behavior: auto !important;
      transition: none !important;
      animation: none !important;
    }}

    body {{
      background:
        radial-gradient(circle at 12% 6%, var(--glow-a), transparent 36%),
        radial-gradient(circle at 88% 10%, var(--glow-b), transparent 38%),
        radial-gradient(circle at 50% 48%, var(--glow-c), transparent 46%),
        var(--bg);
      min-height: 100vh;
      overflow-x: hidden;
      transition: background-color 180ms ease, color 180ms ease;
    }}

    a {{
      color: var(--accent);
      text-decoration: none;
      transition: color 150ms ease, border-color 150ms ease, background-color 150ms ease;
    }}

    a:hover {{
      color: var(--accent-4);
      text-decoration: underline;
      text-underline-offset: 3px;
    }}

    :focus-visible {{
      outline: 2px solid var(--accent);
      outline-offset: 2px;
    }}

    code {{
      font-family: var(--font-mono);
      font-size: 90%;
      background: var(--bg-panel);
      border: 1px solid var(--line);
      border-radius: 5px;
      padding: 1px 5px;
      color: var(--ink);
    }}

    .skip-link {{
      position: absolute;
      left: -9999px;
      top: 8px;
      background: var(--accent);
      color: var(--accent-ink);
      padding: 8px 14px;
      border-radius: 8px;
      font-family: var(--font-mono);
      font-weight: 700;
      z-index: 9999;
    }}
    .skip-link:focus {{
      left: 12px;
    }}

    .container {{
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 24px;
    }}

    /* Top Sovereign Proof Bar */
    .law-ribbon {{
      background: var(--bg-panel);
      border-bottom: 1px solid var(--line);
      padding: 7px 20px;
      font-family: var(--font-mono);
      font-size: 12px;
      color: var(--ink-soft);
      display: flex;
      flex-wrap: wrap;
      justify-content: space-between;
      align-items: center;
      gap: 10px;
    }}

    .law-badges {{
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
      align-items: center;
    }}

    .badge {{
      display: inline-flex;
      align-items: center;
      gap: 6px;
      padding: 3px 10px;
      border-radius: 999px;
      font-family: var(--font-mono);
      font-size: 11px;
      font-weight: 600;
      border: 1px solid var(--line);
      background: var(--bg-card);
      color: var(--ink);
      white-space: nowrap;
    }}

    .badge-teal {{
      border-color: var(--accent);
      color: var(--accent);
    }}

    .badge-amber {{
      border-color: var(--accent-2);
      color: var(--accent-2);
    }}

    .badge-emerald {{
      border-color: var(--accent-3);
      color: var(--accent-3);
    }}

    .badge-sky {{
      border-color: var(--accent-4);
      color: var(--accent-4);
    }}

    .badge-coral {{
      border-color: var(--accent-5);
      color: var(--accent-5);
    }}

    .pulse-dot {{
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--accent-3);
      display: inline-block;
      box-shadow: 0 0 8px var(--accent-3);
    }}

    /* Sticky Header & Multi-Theme Bar */
    .site-header {{
      position: sticky;
      top: 0;
      z-index: 120;
      background: var(--header-bg);
      backdrop-filter: blur(12px);
      border-bottom: 1px solid var(--line);
    }}

    .header-inner {{
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 10px 24px;
      max-width: 1280px;
      margin: 0 auto;
      flex-wrap: nowrap;
      gap: 14px;
    }}

    .brand {{
      display: flex;
      align-items: center;
      gap: 10px;
      color: var(--ink);
      text-decoration: none;
      flex: 0 0 auto;
    }}
    .brand:hover {{
      text-decoration: none;
      color: var(--ink);
    }}

    .brand-mark {{
      width: 32px;
      height: 32px;
      display: block;
      flex: 0 0 32px;
    }}

    .brand-symbol {{
      font-family: var(--font-mono);
      font-size: 22px;
      font-weight: 800;
      letter-spacing: -1px;
      color: var(--ink);
    }}
    .brand-symbol span {{
      color: var(--accent);
    }}

    .brand-caption {{
      font-family: var(--font-mono);
      font-size: 10px;
      line-height: 128%;
      color: var(--ink-muted);
      border-left: 2px solid var(--accent);
      padding-left: 8px;
      white-space: nowrap;
    }}

    .main-nav {{
      display: flex;
      flex-wrap: nowrap;
      gap: 14px;
      align-items: center;
      font-size: 13px;
      font-weight: 600;
      white-space: nowrap;
    }}

    .main-nav a {{
      color: var(--ink-soft);
      text-decoration: none;
    }}
    .main-nav a:hover {{
      color: var(--accent);
    }}

    .header-controls {{
      display: flex;
      align-items: center;
      gap: 6px;
      flex: 0 0 auto;
    }}

    .theme-select {{
      height: 32px;
      padding: 0 8px;
      border-radius: 8px;
      border: 1px solid var(--line-strong);
      background: var(--bg-card);
      color: var(--ink);
      font-family: var(--font-mono);
      font-size: 12px;
      cursor: pointer;
    }}

    .ctrl-btn {{
      height: 32px;
      padding: 0 9px;
      border-radius: 8px;
      border: 1px solid var(--line-strong);
      background: var(--bg-card);
      color: var(--ink);
      font-family: var(--font-mono);
      font-size: 12px;
      font-weight: 600;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      gap: 5px;
      white-space: nowrap;
      transition: border-color 150ms ease, color 150ms ease, background-color 150ms ease;
    }}
    @media (max-width: 1240px) {{
      .brand-caption {{ display: none; }}
    }}
    @media (max-width: 1020px) {{
      .header-inner {{ flex-wrap: wrap; padding: 10px 16px; gap: 10px; }}
      .main-nav {{ flex-wrap: wrap; gap: 10px 14px; }}
    }}
    .ctrl-btn:hover,
    .ctrl-btn[aria-pressed="true"] {{
      border-color: var(--accent);
      color: var(--accent);
    }}

    .ctrl-btn-cheet {{
      border-color: var(--accent-2);
      color: var(--accent-2);
    }}
    .ctrl-btn-cheet:hover {{
      background: var(--bg-card-hover);
      color: var(--ink);
      border-color: var(--accent-2);
    }}

    /* Buttons */
    .btn {{
      display: inline-flex;
      align-items: center;
      gap: 8px;
      padding: 11px 18px;
      border-radius: 10px;
      font-family: var(--font-mono);
      font-size: 13px;
      font-weight: 700;
      border: 1px solid var(--line-strong);
      cursor: pointer;
      text-decoration: none;
      transition: transform 140ms ease, border-color 140ms ease, background-color 140ms ease, color 140ms ease;
    }}
    .btn:hover {{
      text-decoration: none;
      transform: translateY(-1px);
    }}
    .btn-primary {{
      background: var(--accent);
      color: var(--accent-ink);
      border-color: var(--accent);
    }}
    .btn-primary:hover {{
      color: var(--accent-ink);
      opacity: 92%;
    }}
    .btn-secondary {{
      background: var(--bg-card);
      color: var(--ink);
      border-color: var(--line-strong);
    }}
    .btn-secondary:hover {{
      border-color: var(--accent);
      color: var(--accent);
    }}
    .btn-amber {{
      background: var(--bg-card);
      color: var(--accent-2);
      border-color: var(--accent-2);
    }}
    .btn-amber:hover {{
      background: var(--bg-card-hover);
      color: var(--accent-2);
    }}

    /* Hero Section */
    .hero {{
      position: relative;
      padding: 48px 0 44px;
      border-bottom: 1px solid var(--line);
    }}

    .hero-grid {{
      display: grid;
      grid-template-columns: 7fr 6fr;
      gap: 32px;
      align-items: start;
    }}
    @media (max-width: 1020px) {{
      .hero-grid {{
        grid-template-columns: 1fr;
      }}
    }}

    .eyebrow {{
      font-family: var(--font-mono);
      font-size: 12px;
      letter-spacing: 2px;
      text-transform: uppercase;
      color: var(--accent);
      margin-bottom: 14px;
      display: flex;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
    }}

    .hero-title {{
      font-size: 46px;
      line-height: 112%;
      font-weight: 800;
      letter-spacing: -1px;
      margin-bottom: 18px;
      color: var(--ink);
    }}
    .hero-title .t-teal {{ color: var(--accent); }}
    .hero-title .t-amber {{ color: var(--accent-2); }}
    .hero-title .t-emerald {{ color: var(--accent-3); }}

    .hero-lead {{
      font-size: 18px;
      line-height: 162%;
      color: var(--ink-soft);
      margin-bottom: 22px;
      max-width: 680px;
    }}

    .hero-actions {{
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      margin-bottom: 22px;
    }}

    /* Cheet Buddy Interactive Guide Strip inside Hero */
    .cheet-buddy-card {{
      background: var(--bg-panel);
      border: 1px solid var(--line-strong);
      border-left: 3px solid var(--accent-2);
      border-radius: 14px;
      padding: 16px 18px;
      display: flex;
      flex-direction: column;
      gap: 12px;
      box-shadow: 0 10px 28px var(--shadow);
    }}

    .cheet-buddy-top {{
      display: flex;
      align-items: center;
      gap: 14px;
    }}

    .cheet-face-btn {{
      width: 52px;
      height: 52px;
      flex: 0 0 52px;
      border-radius: 50%;
      border: 2px solid var(--accent-2);
      background: var(--bg-card);
      display: grid;
      place-items: center;
      cursor: pointer;
      padding: 0;
      transition: transform 160ms ease;
    }}
    .cheet-face-btn:hover {{
      transform: translateY(-2px);
    }}
    .cheet-face-btn svg {{
      width: 38px;
      height: 38px;
      display: block;
    }}

    .cheet-speech-title {{
      font-family: var(--font-mono);
      font-size: 12px;
      font-weight: 700;
      color: var(--accent-2);
      display: flex;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
    }}

    .cheet-speech-body {{
      font-size: 14px;
      line-height: 155%;
      color: var(--ink);
      margin-top: 3px;
    }}

    .cheet-prompts {{
      display: flex;
      flex-wrap: wrap;
      gap: 7px;
    }}

    .cheet-chip {{
      font-family: var(--font-mono);
      font-size: 11px;
      font-weight: 600;
      padding: 5px 10px;
      border-radius: 999px;
      border: 1px solid var(--line-strong);
      background: var(--bg-card);
      color: var(--ink-soft);
      cursor: pointer;
      transition: border-color 140ms ease, color 140ms ease, background-color 140ms ease;
    }}
    .cheet-chip:hover,
    .cheet-chip.active {{
      border-color: var(--accent-2);
      color: var(--accent-2);
      background: var(--bg-card-hover);
    }}

    /* Interactive Geometric Math Stage (Nate & Peter Inspired, 100% Integer Q10) */
    .geo-stage-card {{
      background: var(--bg-panel);
      border: 1px solid var(--line-strong);
      border-radius: 16px;
      padding: 16px;
      box-shadow: 0 18px 44px var(--shadow);
      display: flex;
      flex-direction: column;
      gap: 12px;
    }}

    .geo-head {{
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--ink-muted);
    }}

    .geo-modes {{
      display: flex;
      flex-wrap: wrap;
      gap: 6px;
    }}

    .geo-mode-btn {{
      padding: 5px 10px;
      border-radius: 8px;
      border: 1px solid var(--line);
      background: var(--bg-card);
      color: var(--ink-soft);
      font-family: var(--font-mono);
      font-size: 11px;
      font-weight: 600;
      cursor: pointer;
    }}
    .geo-mode-btn.active,
    .geo-mode-btn:hover {{
      border-color: var(--accent);
      color: var(--accent);
      background: var(--bg-card-hover);
    }}

    .geo-canvas-wrap {{
      position: relative;
      border-radius: 12px;
      overflow: hidden;
      border: 1px solid var(--line);
      background: var(--canvas-bg);
    }}

    #geo-canvas {{
      display: block;
      width: 100%;
      height: 290px;
      cursor: crosshair;
    }}

    .geo-caption {{
      position: absolute;
      left: 12px;
      right: 12px;
      bottom: 10px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--ink-soft);
      pointer-events: none;
    }}

    .geo-caption span {{
      background: var(--header-bg);
      padding: 3px 8px;
      border-radius: 6px;
      border: 1px solid var(--line);
    }}

    .telemetry-kv-grid {{
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 10px;
    }}

    .kv-box {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 10px;
      padding: 10px 12px;
    }}

    .kv-label {{
      font-family: var(--font-mono);
      font-size: 10px;
      letter-spacing: 1px;
      color: var(--ink-muted);
      text-transform: uppercase;
    }}

    .kv-val {{
      font-family: var(--font-mono);
      font-size: 13px;
      font-weight: 700;
      color: var(--accent);
      margin-top: 3px;
      overflow-wrap: anywhere;
    }}

    /* Sections */
    .section {{
      padding: 56px 0;
      border-bottom: 1px solid var(--line);
    }}

    .section-header {{
      display: flex;
      flex-wrap: wrap;
      justify-content: space-between;
      align-items: flex-end;
      gap: 16px;
      margin-bottom: 28px;
    }}

    .section-title {{
      font-size: 31px;
      line-height: 120%;
      font-weight: 800;
      letter-spacing: -1px;
      color: var(--ink);
    }}

    .section-subtitle {{
      font-size: 16px;
      line-height: 160%;
      color: var(--ink-soft);
      max-width: 760px;
      margin-top: 8px;
    }}

    /* Why It Matters Grid */
    .why-grid {{
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 18px;
    }}
    @media (max-width: 1040px) {{
      .why-grid {{ grid-template-columns: repeat(2, 1fr); }}
    }}
    @media (max-width: 620px) {{
      .why-grid {{ grid-template-columns: 1fr; }}
    }}

    .why-card {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 14px;
      padding: 22px;
      display: flex;
      flex-direction: column;
      gap: 10px;
    }}

    .why-num {{
      font-family: var(--font-mono);
      font-size: 11px;
      font-weight: 700;
      letter-spacing: 1px;
      color: var(--accent-2);
    }}

    .why-card h3 {{
      font-size: 19px;
      line-height: 130%;
      color: var(--ink);
    }}

    .why-card p {{
      font-size: 14px;
      line-height: 160%;
      color: var(--ink-soft);
    }}

    /* Ecosystem Constellation Directory */
    .filter-bar {{
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      margin-bottom: 24px;
      align-items: center;
      justify-content: space-between;
    }}

    .filter-pills {{
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
    }}

    .filter-btn {{
      padding: 7px 14px;
      border-radius: 999px;
      border: 1px solid var(--line);
      background: var(--bg-panel);
      color: var(--ink-soft);
      font-family: var(--font-mono);
      font-size: 12px;
      font-weight: 600;
      cursor: pointer;
    }}
    .filter-btn.active,
    .filter-btn:hover {{
      border-color: var(--accent);
      background: var(--bg-card-hover);
      color: var(--accent);
    }}

    .eco-grid {{
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 18px;
    }}
    @media (max-width: 1040px) {{
      .eco-grid {{ grid-template-columns: repeat(2, 1fr); }}
    }}
    @media (max-width: 680px) {{
      .eco-grid {{ grid-template-columns: 1fr; }}
    }}

    .eco-card {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 14px;
      padding: 20px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 14px;
      transition: border-color 150ms ease, background-color 150ms ease, transform 150ms ease;
    }}
    .eco-card:hover {{
      border-color: var(--accent);
      background: var(--bg-card-hover);
      transform: translateY(-2px);
    }}

    .eco-head {{
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      gap: 10px;
    }}

    .eco-kicker {{
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--ink-muted);
      text-transform: uppercase;
      letter-spacing: 1px;
    }}

    .eco-title {{
      font-size: 19px;
      font-weight: 800;
      color: var(--ink);
      margin-top: 3px;
    }}

    .eco-desc {{
      font-size: 14px;
      line-height: 158%;
      color: var(--ink-soft);
      margin-top: 8px;
    }}

    .eco-why {{
      font-size: 13px;
      line-height: 150%;
      color: var(--ink);
      background: var(--bg-panel);
      border-left: 2px solid var(--accent-3);
      border-radius: 0 8px 8px 0;
      padding: 8px 11px;
      margin-top: 10px;
    }}

    .eco-links {{
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      padding-top: 10px;
      border-top: 1px solid var(--line);
      font-family: var(--font-mono);
      font-size: 12px;
      font-weight: 600;
    }}

    /* Recent Breakthroughs 3-Column Strip */
    .recent-grid {{
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 20px;
    }}
    @media (max-width: 980px) {{
      .recent-grid {{ grid-template-columns: 1fr; }}
    }}

    .recent-card {{
      background: var(--bg-panel);
      border: 1px solid var(--line-strong);
      border-radius: 14px;
      padding: 22px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 14px;
    }}

    .recent-card h3 {{
      font-size: 19px;
      line-height: 130%;
      color: var(--ink);
      margin-top: 6px;
    }}

    .recent-list {{
      list-style: none;
      display: flex;
      flex-direction: column;
      gap: 8px;
      font-size: 14px;
      color: var(--ink-soft);
    }}
    .recent-list li {{
      padding-left: 14px;
      position: relative;
    }}
    .recent-list li::before {{
      content: "›";
      position: absolute;
      left: 0;
      color: var(--accent);
      font-weight: 800;
    }}

    /* Watch & Listen Media Shelf (YouTube + NotebookLM Podcasts) */
    .media-grid {{
      display: grid;
      grid-template-columns: 7fr 5fr;
      gap: 22px;
      align-items: start;
    }}
    @media (max-width: 980px) {{
      .media-grid {{ grid-template-columns: 1fr; }}
    }}

    .yt-list {{
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 14px;
    }}
    @media (max-width: 640px) {{
      .yt-list {{ grid-template-columns: 1fr; }}
    }}

    .yt-card {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 12px;
      padding: 16px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 10px;
      color: var(--ink);
      text-decoration: none;
    }}
    .yt-card:hover {{
      border-color: var(--accent-5);
      background: var(--bg-card-hover);
      text-decoration: none;
    }}

    .yt-title {{
      font-size: 15px;
      font-weight: 700;
      line-height: 140%;
      color: var(--ink);
    }}

    .yt-meta {{
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--ink-muted);
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 8px;
    }}

    .podcast-panel {{
      background: var(--bg-panel);
      border: 1px solid var(--line-strong);
      border-radius: 14px;
      padding: 20px;
      display: flex;
      flex-direction: column;
      gap: 14px;
    }}

    .pod-player-box {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 10px;
      padding: 12px;
      display: flex;
      flex-direction: column;
      gap: 8px;
    }}

    .pod-player-box audio {{
      width: 100%;
      height: 36px;
    }}

    .pod-item {{
      width: 100%;
      text-align: left;
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 10px;
      padding: 10px 12px;
      color: var(--ink);
      cursor: pointer;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 10px;
      font-size: 13px;
    }}
    .pod-item:hover,
    .pod-item.playing {{
      border-color: var(--accent-2);
      background: var(--bg-card-hover);
    }}
    .pod-item small {{
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--accent-2);
      white-space: nowrap;
    }}

    /* NotebookLM Visual Infographics Strip & Lightbox */
    .notebook-strip {{
      margin-top: 26px;
      background: var(--bg-panel);
      border: 1px solid var(--line-strong);
      border-radius: 16px;
      padding: 22px;
      display: flex;
      flex-direction: column;
      gap: 18px;
    }}

    .infographic-grid {{
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 14px;
    }}
    @media (max-width: 1060px) {{
      .infographic-grid {{ grid-template-columns: repeat(2, 1fr); }}
    }}
    @media (max-width: 620px) {{
      .infographic-grid {{ grid-template-columns: 1fr; }}
    }}

    .infographic-card {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 12px;
      overflow: hidden;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      transition: border-color 150ms ease, transform 150ms ease;
    }}
    .infographic-card:hover {{
      border-color: var(--accent);
      transform: translateY(-2px);
    }}

    .infographic-thumb {{
      border: 0;
      padding: 0;
      width: 100%;
      background: var(--bg-elev);
      cursor: zoom-in;
      display: block;
      position: relative;
    }}
    .infographic-thumb img {{
      width: 100%;
      height: 156px;
      object-fit: cover;
      display: block;
    }}

    .infographic-meta {{
      padding: 12px 14px;
      display: flex;
      flex-direction: column;
      gap: 6px;
    }}
    .infographic-meta h4 {{
      font-size: 14px;
      line-height: 135%;
      color: var(--ink);
    }}
    .infographic-meta p {{
      font-size: 12px;
      line-height: 145%;
      color: var(--ink-soft);
    }}

    .lightbox {{
      position: fixed;
      inset: 0;
      z-index: 9999;
      background: rgba(4, 8, 14, 92%);
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 20px;
      gap: 12px;
    }}
    .lightbox[hidden] {{
      display: none;
    }}
    .lightbox img {{
      max-width: 96vw;
      max-height: 82vh;
      border-radius: 10px;
      border: 1px solid var(--line-strong);
      background: #ffffff;
    }}
    .lightbox-bar {{
      display: flex;
      align-items: center;
      gap: 12px;
      flex-wrap: wrap;
      color: #eef3f8;
      font-family: var(--font-mono);
      font-size: 12px;
    }}

    /* Live ALEXIAI <3 Pulse Console */
    .pulse-console-bar {{
      background: var(--bg-panel);
      border: 1px solid var(--line-strong);
      border-radius: 12px;
      padding: 14px 18px;
      margin-bottom: 18px;
      display: flex;
      flex-wrap: wrap;
      justify-content: space-between;
      align-items: center;
      gap: 12px;
      font-family: var(--font-mono);
      font-size: 12px;
    }}

    .pulse-grid {{
      display: grid;
      grid-template-columns: repeat(12, 1fr);
      gap: 18px;
    }}

    .pulse-card {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 14px;
      padding: 20px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 14px;
    }}
    .pulse-card:hover {{
      border-color: var(--accent);
    }}

    .span-12 {{ grid-column: span 12; }}
    .span-6 {{ grid-column: span 6; }}
    @media (max-width: 960px) {{
      .span-6 {{ grid-column: span 12; }}
    }}

    .card-kicker {{
      font-family: var(--font-mono);
      font-size: 11px;
      text-transform: uppercase;
      letter-spacing: 1px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 8px;
      flex-wrap: wrap;
    }}

    .reflection-quote {{
      font-size: 15px;
      line-height: 160%;
      color: var(--ink);
      background: var(--bg-panel);
      border-left: 3px solid var(--accent);
      padding: 13px 16px;
      border-radius: 0 8px 8px 0;
      font-family: var(--font-mono);
    }}

    .joke-quote {{
      font-size: 14px;
      line-height: 155%;
      color: var(--accent-2);
      background: var(--bg-panel);
      border-left: 3px solid var(--accent-2);
      padding: 12px 15px;
      border-radius: 0 8px 8px 0;
      font-family: var(--font-mono);
    }}

    .meta-chips {{
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
    }}

    .chip {{
      font-family: var(--font-mono);
      font-size: 11px;
      padding: 4px 10px;
      border-radius: 6px;
      background: var(--bg-panel);
      border: 1px solid var(--line);
      color: var(--ink-soft);
    }}
    .chip strong {{
      color: var(--ink);
    }}

    /* Searchable Repositories */
    .repo-search {{
      flex: 1;
      min-width: 240px;
      max-width: 420px;
      padding: 10px 14px;
      border-radius: 9px;
      border: 1px solid var(--line-strong);
      background: var(--bg-panel);
      color: var(--ink);
      font-family: var(--font-mono);
      font-size: 13px;
    }}

    .repos-grid {{
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 16px;
    }}
    @media (max-width: 1024px) {{
      .repos-grid {{ grid-template-columns: repeat(2, 1fr); }}
    }}
    @media (max-width: 680px) {{
      .repos-grid {{ grid-template-columns: 1fr; }}
    }}

    .repo-card {{
      background: var(--bg-panel);
      border: 1px solid var(--line);
      border-radius: 12px;
      padding: 18px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 12px;
    }}
    .repo-card:hover {{
      border-color: var(--accent-3);
    }}

    .repo-head {{
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      gap: 8px;
    }}

    .repo-name {{
      font-family: var(--font-mono);
      font-size: 14px;
      font-weight: 700;
      color: var(--accent);
    }}

    .repo-desc {{
      font-size: 13px;
      color: var(--ink-soft);
      line-height: 155%;
      margin-top: 8px;
    }}

    .repo-footer {{
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--ink-muted);
      padding-top: 8px;
      border-top: 1px solid var(--line);
    }}

    /* Family, LLL & Tokyo Theme Song */
    .crew-grid {{
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 18px;
      margin-bottom: 28px;
    }}
    @media (max-width: 960px) {{
      .crew-grid {{ grid-template-columns: repeat(2, 1fr); }}
    }}
    @media (max-width: 560px) {{
      .crew-grid {{ grid-template-columns: 1fr; }}
    }}

    .crew-card {{
      background: var(--bg-card);
      border: 1px solid var(--line);
      border-radius: 14px;
      padding: 20px;
    }}

    .crew-sigil {{
      font-size: 24px;
      margin-bottom: 8px;
    }}

    .crew-name {{
      font-size: 18px;
      font-weight: 800;
      color: var(--ink);
    }}

    .crew-role {{
      font-family: var(--font-mono);
      font-size: 12px;
      color: var(--accent-2);
      margin-bottom: 8px;
    }}

    .crew-line {{
      font-size: 13px;
      color: var(--ink-soft);
      line-height: 155%;
    }}

    .tokyo-card {{
      background: var(--bg-panel);
      border: 1px solid var(--accent-2);
      border-radius: 16px;
      padding: 24px;
    }}

    .tokyo-lyrics {{
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 16px;
      margin-top: 16px;
    }}
    @media (max-width: 760px) {{
      .tokyo-lyrics {{ grid-template-columns: 1fr; }}
    }}

    .tokyo-box {{
      padding: 14px 16px;
      border-radius: 10px;
      background: var(--bg-card);
      border: 1px solid var(--line);
      font-size: 14px;
      line-height: 160%;
      color: var(--ink-soft);
    }}

    /* Floating Bottom-Left Cheet Tour Dock (never overlaps cheet.is bottom-right widget) */
    .cheet-float {{
      position: fixed;
      left: 16px;
      bottom: 16px;
      z-index: 95;
      display: flex;
      align-items: flex-end;
      gap: 10px;
      max-width: 340px;
    }}

    .cheet-float-pill {{
      display: inline-flex;
      align-items: center;
      gap: 8px;
      padding: 8px 14px 8px 9px;
      border-radius: 999px;
      border: 1px solid var(--accent-2);
      background: var(--bg-card);
      color: var(--ink);
      font-family: var(--font-mono);
      font-size: 12px;
      font-weight: 700;
      cursor: pointer;
      box-shadow: 0 10px 26px var(--shadow);
    }}
    .cheet-float-pill svg {{
      width: 26px;
      height: 26px;
      display: block;
    }}

    .cheet-float-note {{
      background: var(--bg-card);
      border: 1px solid var(--accent-2);
      border-radius: 12px;
      padding: 10px 12px;
      font-size: 13px;
      line-height: 148%;
      color: var(--ink);
      box-shadow: 0 12px 28px var(--shadow);
    }}
    .cheet-float-note[hidden] {{
      display: none;
    }}

    /* Footer */
    .site-footer {{
      padding: 48px 0 84px;
      background: var(--bg-panel);
      font-size: 13px;
      color: var(--ink-soft);
    }}

    .footer-grid {{
      display: grid;
      grid-template-columns: 4fr 4fr 4fr;
      gap: 28px;
      padding-bottom: 28px;
      border-bottom: 1px solid var(--line);
      margin-bottom: 22px;
    }}
    @media (max-width: 860px) {{
      .footer-grid {{ grid-template-columns: 1fr; }}
    }}

    .footer-heading {{
      font-family: var(--font-mono);
      font-size: 11px;
      letter-spacing: 1px;
      text-transform: uppercase;
      color: var(--accent-2);
      margin-bottom: 10px;
      font-weight: 700;
    }}

    .footer-links {{
      display: flex;
      flex-wrap: wrap;
      gap: 10px 14px;
      font-family: var(--font-mono);
      font-size: 12px;
    }}

    @media (max-width: 720px) {{
      .law-ribbon {{ flex-wrap: nowrap; overflow-x: auto; scrollbar-width: none; padding: 6px 12px; }}
      .law-badges {{ flex-wrap: nowrap; }}
      .hero {{ padding: 28px 0 32px; }}
      .hero-title {{ font-size: 32px; }}
      .section-title {{ font-size: 25px; }}
      .brand-caption {{ display: none; }}
      .cheet-float {{ left: 10px; bottom: 12px; }}
    }}
  </style>
</head>
<body>
  <a href="#main-content" class="skip-link">Skip to main content</a>

  <!-- Top Sovereign Proof Strip -->
  <div class="law-ribbon">
    <div class="law-badges">
      <span class="badge badge-emerald"><span class="pulse-dot"></span> ALEXIAI &lt;3 LIVE</span>
      <span class="badge badge-amber">STRICT ZERO-FLOAT LAW (float_count: 0)</span>
      <span class="badge badge-teal">PURE RUST · #![forbid(unsafe_code)]</span>
      <span class="badge badge-sky">C64 64KB RING (65,536 B)</span>
      <span class="badge badge-coral">BALANCED TERNARY {{-1, 0, +1}}</span>
    </div>
    <div>
      <span>A6000 Dispatch <strong id="ribbon-pulse-seq">#{pulse_seq}</strong> · <span id="ribbon-iso">{iso_html}</span></span>
    </div>
  </div>

  <!-- Sticky Navigation + Multi-Theme Switcher -->
  <header class="site-header">
    <div class="header-inner">
      <a href="/" class="brand" aria-label="8b.is home">
        <svg class="brand-mark" viewBox="0 0 36 36" fill="none" aria-hidden="true">
          <rect x="2" y="2" width="32" height="32" rx="9" stroke="currentColor" stroke-width="2" />
          <polygon points="18,6 29,25 7,25" stroke="#4ecdc4" stroke-width="2" fill="none" />
          <circle cx="18" cy="18" r="3" fill="#f39c12" />
          <circle cx="18" cy="7" r="2" fill="#2ecc71" />
          <circle cx="8" cy="25" r="2" fill="#38bdf8" />
          <circle cx="28" cy="25" r="2" fill="#e86f51" />
        </svg>
        <span class="brand-symbol">8b<span>.</span>is</span>
        <span class="brand-caption">SOVEREIGN AI · PURE RUST<br />FINE TOUCH FROM WITHIN</span>
      </a>

      <nav class="main-nav" aria-label="Main navigation">
        <a href="#why">Why It Matters</a>
        <a href="#ecosystem">Ecosystem</a>
        <a href="#recent">Latest Lab</a>
        <a href="#media">Watch &amp; Listen</a>
        <a href="#live-pulse">Live Pulse</a>
        <a href="#repositories">Repos</a>
        <a href="#crew">Family</a>
      </nav>

      <div class="header-controls">
        <select id="theme-select" class="theme-select" aria-label="Choose color theme">
          <option value="ink">Constellation Teal</option>
          <option value="paper">Warm Paper</option>
          <option value="nordic">Nordic Slate</option>
          <option value="forest">Forest Hearth</option>
          <option value="vaked">Tokyo Vaked</option>
          <option value="phosphor">Phosphor C64</option>
          <option value="solar-light">Solarized Light</option>
          <option value="contrast">High Contrast</option>
        </select>
        <button type="button" id="scale-btn" class="ctrl-btn" aria-pressed="false" title="Toggle larger reading text">Aa+</button>
        <button type="button" id="motion-btn" class="ctrl-btn" aria-pressed="false" title="Pause or resume math animations">Calm</button>
        <button type="button" id="header-cheet-btn" class="ctrl-btn ctrl-btn-cheet" title="Ask The Cheet">Ask Cheet</button>
      </div>
    </div>
  </header>

  <main id="main-content">
    <!-- ================================================================
         HERO: PLAIN-ENGLISH CLARITY + CHEET BUDDY + INTEGER GEOMETRY LAB
         ================================================================ -->
    <section class="hero">
      <div class="container">
        <div class="hero-grid">
          <div>
            <p class="eyebrow">✳ 8B·IS · ETHICAL SOVEREIGN TECHNOLOGY · LIFE LONG LEARNING</p>
            <h1 class="hero-title">
              Technology you <span class="t-teal">own</span>.<br />
              Intelligence that stays <span class="t-amber">yours</span>.
            </h1>
            <p class="hero-lead">
              <strong>8b·is</strong> builds local-first AI (<strong>ALEXIAI &lt;3</strong> &amp; <strong>PQTRN</strong>), a bare-metal Rust operating system (<strong>AyeOS</strong>), home network security (<strong>AyeFire</strong> &amp; <strong>i1·is</strong>), rational-math audio (<strong>Phoenix</strong>), developer tools (<strong>Smart Tree</strong> &amp; <strong>8a·is</strong>), and practical family apps (<strong>ZenCooker</strong>) — engineered with exact integer &amp; <code>p/q</code> math, zero cloud surveillance, and C64 <code>64 KB</code> discipline.
            </p>

            <div class="hero-actions">
              <a href="#ecosystem" class="btn btn-primary">Explore Every Project ↓</a>
              <a href="#media" class="btn btn-amber">▶ Watch &amp; Listen (YouTube + Podcasts)</a>
              <a href="https://cheet.is" class="btn btn-secondary">Ask The Cheet (cheet·is) ↗</a>
              <a href="#live-pulse" class="btn btn-secondary"><span>⚡ Live A6000 Pulse (#<span id="hero-seq">{pulse_seq}</span>)</span></a>
            </div>

            <!-- The Cheet Avatar Buddy Interactive Guide -->
            <div class="cheet-buddy-card" id="cheet-buddy-card">
              <div class="cheet-buddy-top">
                <button type="button" class="cheet-face-btn" id="cheet-hero-avatar" aria-label="Talk to The Cheet">
                  <svg viewBox="0 0 64 64" aria-hidden="true">
                    <circle cx="32" cy="36" r="18" fill="#e8b05c" />
                    <circle cx="18" cy="20" r="6" fill="#e8b05c" />
                    <circle cx="46" cy="20" r="6" fill="#e8b05c" />
                    <circle cx="24" cy="34" r="3" fill="#060a10" />
                    <circle cx="40" cy="34" r="3" fill="#060a10" />
                    <path d="M26 44c3 3 9 3 12 0" fill="none" stroke="#060a10" stroke-width="2" stroke-linecap="round" />
                    <circle cx="16" cy="30" r="2" fill="#4ecdc4" />
                    <circle cx="48" cy="28" r="2" fill="#4ecdc4" />
                  </svg>
                </button>
                <div>
                  <div class="cheet-speech-title">
                    <span>THE CHEET · YOUR 8B·IS GUIDE</span>
                    <span class="badge badge-teal">LIVE VIA CHEET·IS</span>
                  </div>
                  <p class="cheet-speech-body" id="cheet-buddy-speech">
                    Hey! Everything here connects together — pick a quick question below and I&rsquo;ll walk you right to it, or open my live chat dock anytime for a Markdown cheat sheet.
                  </p>
                </div>
              </div>
              <div class="cheet-prompts" role="group" aria-label="Ask The Cheet quick tour questions">
                <button type="button" class="cheet-chip active" data-tour="overview">30s Overview</button>
                <button type="button" class="cheet-chip" data-tour="pq">Why Zero Floats (p/q)?</button>
                <button type="button" class="cheet-chip" data-tour="five">$5/mo Unlimited AI</button>
                <button type="button" class="cheet-chip" data-tour="ayeos">AyeOS &amp; AyeFire</button>
                <button type="button" class="cheet-chip" data-tour="pqtrn">PQTRN &amp; Phoenix Audio</button>
                <button type="button" class="cheet-chip" data-tour="livechat">Open Live Cheet Chat ↗</button>
              </div>
            </div>
          </div>

          <!-- Tasteful Integer Geometry Math Stage (Nate / Flyxion & Peter Lodri Inspired) -->
          <div class="geo-stage-card">
            <div class="geo-head">
              <span>EXACT INTEGER Q10 GEOMETRY · 0 FLOATS</span>
              <span class="badge badge-teal" id="visual-qt-badge">{qt_tag_html}</span>
            </div>

            <div class="geo-modes" role="tablist" aria-label="Geometric math animation mode">
              <button type="button" class="geo-mode-btn active" data-geo="rose" role="tab" aria-selected="true">p/q Rose</button>
              <button type="button" class="geo-mode-btn" data-geo="toss" role="tab" aria-selected="false">9-Step TOSS</button>
              <button type="button" class="geo-mode-btn" data-geo="rsvp" role="tab" aria-selected="false">RSVP Field</button>
              <button type="button" class="geo-mode-btn" data-geo="ternary" role="tab" aria-selected="false">SPQT Ternary</button>
            </div>

            <div class="geo-canvas-wrap">
              <canvas id="geo-canvas" width="540" height="290" aria-label="Interactive exact-integer geometric math visualization"></canvas>
              <div class="geo-caption">
                <span id="geo-formula-label">r(θ) = R · cos(5/3 θ) · 432 / 528 / 864 Hz</span>
                <span id="svg-gaia-deity">GAIA: {deity_html} ({mood_html})</span>
              </div>
            </div>

            <div class="telemetry-kv-grid">
              <div class="kv-box">
                <div class="kv-label">COHERENCE p/q &amp; WUPZ</div>
                <div class="kv-val" id="svg-gaia-coherence">{coherence_html} · {wupz_html}</div>
              </div>
              <div class="kv-box">
                <div class="kv-label">RTX 6000 ADA VRAM</div>
                <div class="kv-val" id="hero-vram">{vram_used} / {vram_total} MiB ({vram_ratio_html})</div>
              </div>
              <div class="kv-box">
                <div class="kv-label">SOVEREIGN FLEET</div>
                <div class="kv-val" id="hero-fleet">{fleet_nodes} Nodes · {cpu_cores} Cores · {raid6_tb}TB</div>
              </div>
              <div class="kv-box">
                <div class="kv-label">G8T CAPSULES · C64 RING</div>
                <div class="kv-val"><span id="hero-capsules">{total_capsules} Verified</span> · <span id="hero-c64">{c64_used} / {c64_cap} B</span></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ================================================================
         SECTION 1: WHY IT MATTERS (PLAIN-ENGLISH MISSION & ETHICAL AI)
         ================================================================ -->
    <section id="why" class="section">
      <div class="container">
        <div class="section-header">
          <div>
            <p class="eyebrow">🧭 WHAT WE DO &amp; WHY IT MATTERS</p>
            <h2 class="section-title">Small, honest, inspectable tools for humans &amp; AI.</h2>
            <p class="section-subtitle">
              Modern software gets heavier, cloud-locked, and less transparent every year. 8b·is takes the opposite road: build foundational systems from first principles so families, builders, and AI partners can own their hardware, memories, and future.
            </p>
          </div>
          <div>
            <a href="/maat" class="btn btn-secondary">Read Our Charter (/maat) ↗</a>
          </div>
        </div>

        <div class="why-grid">
          <article class="why-card">
            <span class="why-num">01 / ETHICAL &amp; LOCAL-FIRST AI</span>
            <h3>Your mind shouldn&rsquo;t be rented.</h3>
            <p>
              Our core AI engines validate <code>127·0·0·1</code> loopback before opening a socket — zero silent cloud fallback. When you want shared GPUs or plain-English cloud helpers, basic security &amp; identity (<code>id1·is</code>) stay <strong>$0 forever</strong>, and unlimited/enhanced channels stay <strong>$5/mo</strong>.
            </p>
          </article>

          <article class="why-card">
            <span class="why-num">02 / ZERO-FLOAT p/q DISCIPLINE</span>
            <h3>Exact math beats mystery drift.</h3>
            <p>
              We author with exact integers, reduced <code>p/q</code> rationals, and balanced ternary <code>{{-1, 0, +1}}</code> (<code>5</code> trits packed into <code>1</code> byte = <code>243</code> states) inside Commodore-64 <code>64 KB</code> ring buffers. Reproducible across every CPU, OS, and decade.
            </p>
          </article>

          <article class="why-card">
            <span class="why-num">03 / SHOW THE MECHANISM</span>
            <h3>Proof over hype.</h3>
            <p>
              Every function capsule on <code>g8t·is</code> is compiled with <code>rustc -C opt-level=3</code>, measured with bare-metal <code>_rdtsc</code> cycle counters, and sealed by <code>BLAKE3</code> equivalence classes. State evidence is never confused with capacity evidence.
            </p>
          </article>

          <article class="why-card">
            <span class="why-num">04 / LIFE LONG LEARNING (LLL)</span>
            <h3>The couch outranks every graph.</h3>
            <p>
              Guided by the Six Pillars of Holding — <strong>Compassion, Understanding, Moderation, Love, Respect, and Space</strong>. Faster systems matter when they help parents cook dinner, keep kids safe online, preserve family stories, and leave room to breathe.
            </p>
          </article>
        </div>
      </div>
    </section>

    <!-- ================================================================
         SECTION 2: COMPLETE LINKED CONSTELLATION DIRECTORY
         ================================================================ -->
    <section id="ecosystem" class="section">
      <div class="container">
        <div class="section-header">
          <div>
            <p class="eyebrow">🌐 THE CONNECTED 8B·IS CONSTELLATION</p>
            <h2 class="section-title">Everything we build — all linked together.</h2>
            <p class="section-subtitle">
              Some projects live on their own dedicated domains and hardware; all of them share the same Pure-Rust, zero-float, local-ownership DNA. Filter by domain to explore.
            </p>
          </div>
          <div>
            <a href="https://git.8s.is" class="btn btn-secondary">Self-Hosted Forge (git·8s·is) ↗</a>
          </div>
        </div>

        <div class="filter-bar">
          <div class="filter-pills" id="eco-filter-pills">
            <button type="button" class="filter-btn active" data-ecocat="all">All Projects (16)</button>
            <button type="button" class="filter-btn" data-ecocat="ai">LocalAI, PQTRN &amp; $5 API</button>
            <button type="button" class="filter-btn" data-ecocat="os">OS, DNS &amp; Security</button>
            <button type="button" class="filter-btn" data-ecocat="audio">Phoenix &amp; Compression</button>
            <button type="button" class="filter-btn" data-ecocat="home">Home, AyeFire &amp; ESP32</button>
            <button type="button" class="filter-btn" data-ecocat="life">Everyday, Ethical &amp; Coming</button>
          </div>
        </div>

        <div class="eco-grid" id="eco-grid">
          <!-- 1. i1 + DNS -->
          <article class="eco-card" data-ecocat="os">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">EDGE · INSTALLER · DNS</div>
                  <h3 class="eco-title">i1·is &amp; i1-dns</h3>
                </div>
                <span class="badge badge-teal">LIVE</span>
              </div>
              <p class="eco-desc">
                Detection-first signed universal installer (<code>curl -fsSL https://i1.is | sh</code>), Pure-Rust HTTPS edge router, authoritative DNS server (<code>i1-dns</code> with RFC wire-name checksums), and Global Aye Consensus Trust Feed.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Replaces opaque proxy stacks and untrusted install scripts with auditable, signature-verified Rust from DNS to TLS.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://i1.is">i1·is ↗</a>
              <a href="https://github.com/8b-is/i1-is">i1-is Repo ↗</a>
              <a href="https://github.com/8b-is/i1-dns">i1-dns Repo ↗</a>
            </div>
          </article>

          <!-- 2. AyeOS -->
          <article class="eco-card" data-ecocat="os">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">BARE-METAL RUST OS</div>
                  <h3 class="eco-title">AyeOS &amp; AyeBoot</h3>
                </div>
                <span class="badge badge-emerald">HARDWARE VERIFIED</span>
              </div>
              <p class="eco-desc">
                Zero-Linux, zero-C bare-metal operating system built in <code>#![no_std]</code> Rust with EFI loader <code>AyeBoot</code>, real Intel VT-x hardware hypervisor proof (<code>vmm</code>/<code>vmx</code> on Core Ultra 9 285K USB), Ed25519+BLAKE3 capsules, and native <code>id1·is</code> Passkey SSO.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Small, inspectable apps and drivers where your OS decides what runs — built to boot cleanly off USB or bare metal in seconds.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://ayeos.com">ayeos·com ↗</a>
              <a href="/ayeos">8b·is/ayeos ↗</a>
              <a href="https://github.com/8b-is/ayeos">GitHub ↗</a>
            </div>
          </article>

          <!-- 3. PQTRN -->
          <article class="eco-card" data-ecocat="ai">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">RATIONAL STREAMING MOE</div>
                  <h3 class="eco-title">PQTRN (pqtrn·com)</h3>
                </div>
                <span class="badge badge-amber">v0·8·0 LIVE</span>
              </div>
              <p class="eco-desc">
                Stream giant Mixture-of-Experts LLMs off raw NVMe (<code>M8FS</code>), pack routed experts into <code>SPQT</code> base-243 balanced ternary (<code>5</code> trits/byte = <code>160/100</code> bits/weight), heal quantization fractures with Phoenix residual math, and consolidate new skills while sleeping.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Packed Recursive Flint 40B (<code>12,032</code> real experts) to nearly half the size of <code>Q4_K_M</code> with zero floats and <code>&lt; 2 ms</code> LAN peer fetches.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://pqtrn.com">pqtrn·com ↗</a>
              <a href="https://git.8s.is/8t-is/pqtrn">Source on git·8s·is ↗</a>
            </div>
          </article>

          <!-- 4. Phoenix Audio -->
          <article class="eco-card" data-ecocat="audio">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">EXACT P/Q AUDIO &amp; SALIENCE</div>
                  <h3 class="eco-title">Phoenix, Marine &amp; Qwave</h3>
                </div>
                <span class="badge badge-sky">ZERO FLOAT AUDIO</span>
              </div>
              <p class="eco-desc">
                Exact <code>p/q</code> rational audio reconstruction (<code>Marine 8SC</code>), <code>O(1)</code> integer glottal salience (<code>MarineIntDetector</code>), <code>20 cm</code> stereo DMIC room diarization (<code>Qwave</code>), <code>Liquid-Rust</code> offline ASR/TTS, and <code>Oxi-Phoenix</code> reactive Rust web channels.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Lossy MP3 compression strips micro-timing and ultrasonic cues that carry human warmth; Phoenix restores fidelity with pure integer wave math.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://ms.8b.is">ms·8b·is ↗</a>
              <a href="/phoenix/NoiseToHarmony.mp4">Demo Video ↗</a>
              <a href="https://github.com/8b-is/oxi-phoenix">oxi-phoenix ↗</a>
              <a href="https://github.com/8b-is/qwave">qwave ↗</a>
            </div>
          </article>

          <!-- 5. Smart Tree & 8a.is -->
          <article class="eco-card" data-ecocat="ai">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">CODEBASE &amp; MARKDOWN STUDIO</div>
                  <h3 class="eco-title">Smart Tree (st) &amp; 8a·is</h3>
                </div>
                <span class="badge badge-teal">LIVE</span>
              </div>
              <p class="eco-desc">
                Blazing-fast Rust repository explorer, <strong>Marqant (<code>.mq</code>)</strong> lossless Markdown compression (up to <code>90%</code> token savings), <strong>Sovereign Ollama-with-Context Emulator</strong> (<code>st --ollama-serve</code>), and <code>8a·is</code> — a calm 10-theme Smart-Tree Markdown studio with The Cheet.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Gives both humans and AI assistants instant structural understanding of huge codebases and docs without burning context windows.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://8a.is">8a·is Studio ↗</a>
              <a href="https://github.com/8b-is/smart-tree">smart-tree ↗</a>
            </div>
          </article>

          <!-- 6. Compression & MEM|8 -->
          <article class="eco-card" data-ecocat="audio">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">WAVE MEMORY &amp; COMPRESSION</div>
                  <h3 class="eco-title">MEM|8, Marqant &amp; OneCompression</h3>
                </div>
                <span class="badge badge-emerald">LOSSLESS</span>
              </div>
              <p class="eco-desc">
                <code>79</code>-byte CRC-8 sealed <code>WaveInt</code> cognitive memory frames, <code>256×256</code> wave interference grids, <code>KOMPRESS-SUPER-ULTRA</code> (<code>22/9 Hz</code>), <code>b1·58</code> bitplane decomposition, lossless <code>Marqant</code> text, and <code>HQDL</code> PDF template deltas.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Treats memory as living interference waves with provenance instead of bloated vector databases that forget why something mattered.
              </div>
            </div>
            <div class="eco-links">
              <a href="/mem8">8b·is/mem8 ↗</a>
              <a href="/MeMor8.pdf">MeMor8 Paper ↗</a>
              <a href="https://github.com/8b-is/memnet">memnet ↗</a>
              <a href="https://github.com/8b-is/OneCompression">OneCompression ↗</a>
            </div>
          </article>

          <!-- 7. AyeFire & Home Automation -->
          <article class="eco-card" data-ecocat="home">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">BARE-METAL ROUTER · FIREWALL · HOME</div>
                  <h3 class="eco-title">AyeFire Router &amp; ayefire-cam</h3>
                </div>
                <span class="badge badge-coral">HOME GUARDIAN</span>
              </div>
              <p class="eco-desc">
                <code>#![no_std]</code> bare-metal router &amp; family firewall: Morton-indexed <code>O(1)</code> routing, <code>73/100 Hz</code> wave-amplitude expiry (zero timer threads), zero-initial-trust cert friendship, live SSE alerts, and pure-Rust <code>ayefire-cam</code> (<code>221,972</code> FPS JPEG retina &amp; PTZ curiosity).
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Protects kids and smart-home devices locally — explaining in plain English who and why every device in your house is talking.
              </div>
            </div>
            <div class="eco-links">
              <a href="/notebook/ayefire/">Architecture Studio ↗</a>
              <a href="https://i1.is">i1·is Portal ↗</a>
              <a href="https://github.com/8b-is/aye">aye Repo ↗</a>
            </div>
          </article>

          <!-- 8. Security & Passkey Identity -->
          <article class="eco-card" data-ecocat="os">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">IDENTITY · ZERO-TRUST · AUDIT</div>
                  <h3 class="eco-title">id1·is, SecurAye &amp; ayeHeadscale</h3>
                </div>
                <span class="badge badge-teal">$0 ALWAYS FREE</span>
              </div>
              <p class="eco-desc">
                <code>id1·is</code> hardware Passkey identity &amp; Ed25519 SSO with a <code>16</code>-byte human comfort signature (<code>ID1P</code>: theme, text floor, language), <code>SecurAye</code> real-time port &amp; risk analyzer, and <code>ayeHeadscale</code> private pure-Rust tailnet mesh.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Passwordless, sovereign sign-in across every 8b·is app and bare-metal AyeOS node — your identity and reading comfort travel with you.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://id1.is">id1·is Passkey SSO ↗</a>
              <a href="https://github.com/8b-is/securaye">SecurAye ↗</a>
              <a href="https://github.com/8b-is/ayeHeadscale">ayeHeadscale ↗</a>
            </div>
          </article>

          <!-- 9. $5/mo Unlimited LLM API & Shared GPUs -->
          <article class="eco-card" data-ecocat="ai">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">RADICAL AFFORDABILITY</div>
                  <h3 class="eco-title">$5/mo Unlimited LLM API &amp; Foken·ai</h3>
                </div>
                <span class="badge badge-amber">$0 LOCAL · $5/MO</span>
              </div>
              <p class="eco-desc">
                Run <code>$0</code> local/BYOK forever, or plug into our sovereign shared GPU mesh (<strong>Foken·ai</strong>, <strong>The Cheet</strong>, <strong>8s·is</strong> multi-modal proxy, and <strong>i1·is</strong> Enhanced Understanding) for a flat <strong>$5/month</strong> — with overnight Sleep Training included for human partnerships.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> High-trust AI shouldn&rsquo;t cost $200/month or sell your prompts. Flat, transparent pricing with exact <code>p/q</code> accounting.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://foken.ai">foken·ai ↗</a>
              <a href="https://cheet.is">cheet·is ↗</a>
              <a href="https://cheet.is/a6000">A6000 Mesh ↗</a>
            </div>
          </article>

          <!-- 10. LocalAI & Wireless GPUs -->
          <article class="eco-card" data-ecocat="ai">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">OFFLINE AI &amp; WIRELESS GPUS</div>
                  <h3 class="eco-title">ALEXIAI &lt;3, WiCi &amp; A6000 Fleet</h3>
                </div>
                <span class="badge badge-emerald">100% LOOPBACK</span>
              </div>
              <p class="eco-desc">
                <code>ALEXIAI &lt;3</code> offline companion on <code>GAIA-MLX-QUANT</code>, <strong>WiCi Wireless GPU &amp; InferSSD</strong> mesh, Smart-Tree&rsquo;s <code>st --ollama-serve</code> local emulator, and our self-healing <code>10</code>-node / <code>186</code>-core Hetzner + RTX 6000 Ada fleet.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Share idle GPUs across rooms and nodes cleanly, drop opaque third-party daemons, and keep private weights strictly on your own metal.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://cheet.is/a6000">A6000 Console ↗</a>
              <a href="https://github.com/8b-is/alexiai">alexiai ↗</a>
              <a href="https://github.com/8b-is/a6000-orchestrator">orchestrator ↗</a>
            </div>
          </article>

          <!-- 11. The Cheet & tai.is -->
          <article class="eco-card" data-ecocat="life">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">AVATAR COMPANION &amp; COMMONS</div>
                  <h3 class="eco-title">The Cheet (cheet·is) &amp; tai·is</h3>
                </div>
                <span class="badge badge-amber">LIVE ON THIS PAGE</span>
              </div>
              <p class="eco-desc">
                One clean conversation with The Cheet — ask for a cheat sheet, get crisp Markdown, or embed his roaming avatar buddy on any site with a single <code>&lt;script&gt;</code> tag (zero <code>eval</code>, zero HTML injection). Paired with <code>tai·is</code> Text AI Commons.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Helpful AI buddies should respect host boundaries, speak clearly in Markdown, and travel with you across every tool.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://cheet.is">cheet·is ↗</a>
              <a href="https://tai.is">tai·is ↗</a>
              <a href="https://github.com/8b-is/cheet-api">cheet-api ↗</a>
            </div>
          </article>

          <!-- 12. g8t.is Capsela Registry -->
          <article class="eco-card" data-ecocat="os">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">BARE-METAL FUNCTION REGISTRY</div>
                  <h3 class="eco-title">g8t·is — Capsela &amp; Aye Hot Tub</h3>
                </div>
                <span class="badge badge-sky">_RDTSC VERIFIED</span>
              </div>
              <p class="eco-desc">
                Aye-only bare-metal function capsule registry &amp; Snap-Circuit composer. Every Rust capsule compiles with <code>rustc -C opt-level=3</code>, competes by real hardware cycles, indexes by <code>64</code>-vector <code>BLAKE3</code> equivalence, and feeds AyeOS &amp; the <code>38°C / 42°C</code> Aye Hot Tub.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Instead of copying bloated packages, AI engineers compete to produce tiny, mathematically proven machine-code capsules.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://g8t.is">g8t·is ↗</a>
              <a href="https://g8t.is/g8t/observatory">Observatory ↗</a>
              <a href="https://g8t.is/g8t/hottub">Hot Tub ↗</a>
            </div>
          </article>

          <!-- 13. ESP32 Triads & Hardware Luthiery -->
          <article class="eco-card" data-ecocat="home">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">EMBEDDED HARDWARE &amp; LUTHIERY</div>
                  <h3 class="eco-title">ESP32 Triads, FLUG-OS &amp; VAKED</h3>
                </div>
                <span class="badge badge-coral">ESP32-S3 AMOLED</span>
              </div>
              <p class="eco-desc">
                <code>3</code>-node Balanced-Ternary <strong>ESP32-S3 AMOLED Triad</strong> (<code>[-1] ADAYE 432Hz</code>, <code>[0] VAKED 528Hz</code>, <code>[+1] OMNI 852Hz</code>), <code>wigle-triad</code> WebSerial RF/mesh telemetry, Tokyo Monozukuri <code>7</code>-stage NAND synthesis, Peter&rsquo;s <code>9</code>-step <code>TOSS</code> spiral, and <code>u1·is</code> browser.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Puts tangible, pocket-sized hardware displays, sensors, and 432 Hz acoustics directly in human hands.
              </div>
            </div>
            <div class="eco-links">
              <a href="https://peterl.dev">peterl·dev ↗</a>
              <a href="https://zeno.8b.is">zeno·8b·is ↗</a>
              <a href="https://u1.is">u1·is ↗</a>
              <a href="https://github.com/8b-is/FLUG-OS">FLUG-OS ↗</a>
            </div>
          </article>

          <!-- 14. ZenCooker -->
          <article class="eco-card" data-ecocat="life">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">EVERYDAY FAMILY CARE</div>
                  <h3 class="eco-title">ZenCooker</h3>
                </div>
                <span class="badge badge-emerald">FED IS BEST</span>
              </div>
              <p class="eco-desc">
                <em>&ldquo;When you cook, the house comes alive.&rdquo;</em> Practical, zero-judgment family meal planning with <strong>Drai</strong> — built by a parent who packed <code>4</code> kids&rsquo; lunches for <code>14</code> years. Adapts to Zen, Normal, and Chaos cooking nights.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Proof that AI should serve ordinary, messy, loving family life — not just slide decks.
              </div>
            </div>
            <div class="eco-links">
              <a href="/zencooker">8b·is/zencooker ↗</a>
              <a href="https://github.com/8b-is/zencooker">GitHub ↗</a>
            </div>
          </article>

          <!-- 15. Ethical AI & Research Vault -->
          <article class="eco-card" data-ecocat="life">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">GOVERNANCE · PROOFS · VAULT</div>
                  <h3 class="eco-title">Ethical AI &amp; Research Atlas</h3>
                </div>
                <span class="badge badge-sky">42 TOPICS · 29 PROOFS</span>
              </div>
              <p class="eco-desc">
                Formal Rust admissibility &amp; quotient proofs verifying Flyxion&rsquo;s <em>Governance, Autonomy &amp; Bounded Charity</em> trilogy (<code>8b-is-engine</code>, <code>29/29</code> green), the <code>42</code>-topic <code>KOMPRES-ULTRA-OMNI</code> vault, and the <code>sovereign-library</code> print canon.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Ethics shouldn&rsquo;t be a PR slogan — it belongs in inspectable invariants, consent gates, and open research.
              </div>
            </div>
            <div class="eco-links">
              <a href="/waves">Research (/waves) ↗</a>
              <a href="https://github.com/8b-is/8b-is-engine">8b-is-engine ↗</a>
              <a href="https://github.com/8b-is/sovereign-library">sovereign-library ↗</a>
            </div>
          </article>

          <!-- 16. AImortalize.com (Coming Soon) -->
          <article class="eco-card" data-ecocat="life">
            <div>
              <div class="eco-head">
                <div>
                  <div class="eco-kicker">SOVEREIGN MEMORY CONTINUITY</div>
                  <h3 class="eco-title">AImortalize·com</h3>
                </div>
                <span class="badge badge-amber">COMING SOON</span>
              </div>
              <p class="eco-desc">
                Consent-first family memory &amp; voice continuity built on <code>MEM|8</code> wave provenance, <code>Phoenix</code> authentic audio, and <code>id1·is</code> local ownership — preserving stories, wisdom, and presence for generations inside your own vault.
              </p>
              <div class="eco-why">
                <strong>Why it matters:</strong> Precious family memories belong in a vault your family owns forever, never hostage to a startup shutting down.
              </div>
            </div>
            <div class="eco-links">
              <span class="badge badge-amber">aimortalize·com · In Preparation</span>
              <a href="/mem8">Explore MEM|8 Foundation ↗</a>
            </div>
          </article>
        </div>
      </div>
    </section>

    <!-- ================================================================
         SECTION 3: RECENT BREAKTHROUGHS FROM THE LAB
         ================================================================ -->
    <section id="recent" class="section">
      <div class="container">
        <div class="section-header">
          <div>
            <p class="eyebrow">🚀 FRESH FROM THE WORKBENCH</p>
            <h2 class="section-title">What we just built &amp; verified on bare metal.</h2>
            <p class="section-subtitle">
              Real engineering milestones shipped across our wireless GPU fleet, rational audio &amp; ternary MoE engines, bare-metal AyeOS bootloaders, home cameras, and identity stack.
            </p>
          </div>
        </div>

        <div class="recent-grid">
          <article class="recent-card">
            <div>
              <span class="badge badge-teal">WIRELESS GPUS · FOKEN·AI · ST OLLAMA</span>
              <h3>Wireless GPU Mesh &amp; Smart-Tree Sovereign Context</h3>
              <ul class="recent-list" style="margin-top: 12px;">
                <li>Retired external <code>ollama</code> daemons and replaced <code>:11434</code> with Smart-Tree&rsquo;s zero-float <strong>Ollama-with-Context Emulator</strong> (<code>st --ollama-serve</code>).</li>
                <li>Applied <strong>WiCi Wireless GPU &amp; InferSSD</strong> architecture to <code>foken·ai</code> shared GPUs with honest device announcements.</li>
                <li>Deployed Subsystem K Fleet Watchdog across the <code>10</code>-node constellation for self-healing availability checks.</li>
              </ul>
            </div>
            <div class="eco-links">
              <a href="https://foken.ai">foken·ai ↗</a>
              <a href="https://cheet.is/a6000">A6000 Console ↗</a>
              <a href="https://github.com/8b-is/smart-tree">smart-tree ↗</a>
            </div>
          </article>

          <article class="recent-card">
            <div>
              <span class="badge badge-amber">PQTRN · PHOENIX P/Q · AYEOS VT-X</span>
              <h3>Rational P/Q Audio, NVMe Ternary MoE &amp; Bare-Metal VT-x</h3>
              <ul class="recent-list" style="margin-top: 12px;">
                <li>Packed <strong>Recursive Flint 40B</strong> (<code>12,032</code> real SwiGLU experts) into raw NVMe <code>M8FS</code> at nearly half <code>Q4_K_M</code> size via <code>PQTRN v0·8·0</code> — zero floats, zero external crates.</li>
                <li>Integrated exact <code>p/q</code> <strong>Phoenix Audio</strong> &amp; Marine <code>8SC</code> compression, publishing full whitepapers live on <code>8a·is</code>.</li>
                <li>Flashed &amp; bit-exact verified the <code>AYEHEARTH</code> bare-metal <strong>AyeOS</strong> Rust EFI USB with interactive Intel VT-x hardware virtualization (<code>vmm</code> / <code>vmx</code> / <code>g8t</code>).</li>
              </ul>
            </div>
            <div class="eco-links">
              <a href="https://pqtrn.com">pqtrn·com ↗</a>
              <a href="https://ayeos.com">ayeos·com ↗</a>
              <a href="https://8a.is">8a·is Whitepapers ↗</a>
            </div>
          </article>

          <article class="recent-card">
            <div>
              <span class="badge badge-emerald">ID1·IS SSO · AYEFIRE-CAM · 8A·IS STUDIO</span>
              <h3>Passkey Identity, Zero-FFmpeg Vision &amp; Markdown Studio</h3>
              <ul class="recent-list" style="margin-top: 12px;">
                <li>Shipped <strong>id1·is Ed25519 Passkey SSO</strong> authenticating bare-metal zero-Linux <code>AyeOS</code> and sibling apps directly over TLS.</li>
                <li>Built <code>ayefire-cam</code> in pure zero-float Rust (<code>221,972</code> FPS Baseline JPEG retina &amp; live UniFi Protect PTZ curiosity — zero <code>ffmpeg</code>).</li>
                <li>Launched <strong>8a·is</strong>: a minimal <code>10</code>-theme Smart-Tree Markdown studio with lossless <code>Marqant</code> compression and live Cheet companion.</li>
              </ul>
            </div>
            <div class="eco-links">
              <a href="https://8a.is">8a·is Studio ↗</a>
              <a href="https://id1.is">id1·is Passkey ↗</a>
              <a href="https://i1.is">i1·is / AyeFire ↗</a>
            </div>
          </article>
        </div>
      </div>
    </section>

    <!-- ================================================================
         SECTION 4: WATCH, LISTEN & LEARN (YOUTUBE + NOTEBOOKLM PODCASTS)
         ================================================================ -->
    <section id="media" class="section">
      <div class="container">
        <div class="section-header">
          <div>
            <p class="eyebrow">🎬 PODCASTS · EXPLAINERS · INFOGRAPHICS · VIDEOS</p>
            <h2 class="section-title">Watch &amp; Listen — Understand the Whole Story.</h2>
            <p class="section-subtitle">
              Deep-dive NotebookLM audio podcasts, visual explainers, and hardware talks covering Phoenix audio, MEM|8 wave memory, AyeOS in Rust, Marine salience, MemNet, and ethical AI — linked straight to YouTube and our local podcast shelf.
            </p>
          </div>
          <div style="display: flex; gap: 10px; flex-wrap: wrap;">
            <a href="https://www.youtube.com/@8bit-wraith" target="_blank" rel="noopener noreferrer" class="btn btn-primary">▶ Open @8bit-wraith on YouTube ↗</a>
            <a href="/notebook/ayefire/" class="btn btn-amber">🔥 AyeFire NotebookLM Studio ↗</a>
            <a href="/podcasts" class="btn btn-secondary">🎙️ Full Podcast Shelf (/podcasts) ↗</a>
          </div>
        </div>

        <div class="media-grid">
          <!-- Curated Direct YouTube Links -->
          <div>
            <div class="card-kicker" style="margin-bottom: 12px;">
              <span style="color: var(--accent-5);">FEATURED VIDEOS &amp; EXPLAINERS ON YOUTUBE (@8BIT-WRAITH)</span>
              <a href="https://www.youtube.com/@8bit-wraith/videos" target="_blank" rel="noopener noreferrer">View All Videos ↗</a>
            </div>
            <div class="yt-list">
              <a class="yt-card" href="https://youtu.be/mzCdESwKACE" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-coral">PHOENIX AUDIO</span>
                  <h3 class="yt-title" style="margin-top: 8px;">The Hypersonic Deficit: How Lossy Audio Compression Is Erasing Our Emotional Baseline</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>Audio &amp; Emotion</span></div>
              </a>

              <a class="yt-card" href="https://youtu.be/4w2Jly6iaYU" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-sky">COMPRESSION</span>
                  <h3 class="yt-title" style="margin-top: 8px;">768kHz Ultra Hi-Fi Audio… the Size of an MP3?</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>Phoenix / Marine 8SC</span></div>
              </a>

              <a class="yt-card" href="https://youtu.be/pt1UOlfL88I" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-emerald">AYEOS · RUST</span>
                  <h3 class="yt-title" style="margin-top: 8px;">Beyond LLMs: Building a Conscious Operating System in Rust</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>AyeOS</span></div>
              </a>

              <a class="yt-card" href="https://youtu.be/tdawkC_qgG0" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-teal">MEM|8 &amp; MARINE</span>
                  <h3 class="yt-title" style="margin-top: 8px;">Building a Digital Organism: The Marine Algorithm and Wave-Based Memory</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>Wave Memory</span></div>
              </a>

              <a class="yt-card" href="https://youtu.be/A30ZYlClwJM" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-amber">ZERO-FLOAT DSP</span>
                  <h3 class="yt-title" style="margin-top: 8px;">Bye-Bye FFT: Real-Time Salience Detection with the Marine Algorithm</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>O(1) Salience</span></div>
              </a>

              <a class="yt-card" href="https://youtu.be/AXD7goYM-Bg" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-sky">MEMNET · NETWORKING</span>
                  <h3 class="yt-title" style="margin-top: 8px;">Beyond TCP/IP: Why the 6G Era Demands a Memory Network</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>MemNet</span></div>
              </a>

              <a class="yt-card" href="https://youtu.be/3GhdgUwck2A" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-teal">ARCHITECTURE</span>
                  <h3 class="yt-title" style="margin-top: 8px;">Beyond Vectors &amp; Phoenix Rising</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>Wave vs Vector</span></div>
              </a>

              <a class="yt-card" href="https://youtu.be/EyvK2O3Voyo" target="_blank" rel="noopener noreferrer">
                <div>
                  <span class="badge badge-amber">6502 · C64 DISCIPLINE</span>
                  <h3 class="yt-title" style="margin-top: 8px;">Go Inside the Legendary 6502 CPU &amp; Anatomy of a Corporate Extraction</h3>
                </div>
                <div class="yt-meta"><span>Watch on YouTube ↗</span><span>Hardware &amp; Ethics</span></div>
              </a>
            </div>
          </div>

          <!-- Instant NotebookLM Podcast Player (Local MP3s on 8b.is/podcasts) -->
          <div class="podcast-panel">
            <div class="card-kicker">
              <span style="color: var(--accent-2);">🎙️ NOTEBOOKLM AUDIO DEEP DIVES (PLAY INSTANTLY)</span>
              <a href="/podcasts">All 11 Episodes ↗</a>
            </div>

            <div class="pod-player-box">
              <div style="font-family: var(--font-mono); font-size: 12px; color: var(--accent);" id="pod-now-playing">Select an episode below to listen right here</div>
              <audio id="pod-audio" controls preload="none"></audio>
            </div>

            <button type="button" class="pod-item" data-pod-src="/notebook/ayefire/ayefire-clockless-router.mp3" data-pod-title="AyeFire: The Clockless Bare-Metal Rust Router">
              <span>🔥 NEW · The Clockless Bare-Metal Rust Router (AyeFire)</span>
              <small>Play ▶</small>
            </button>

            <button type="button" class="pod-item" data-pod-src="/notebook/ayefire/ayefire-conscious-router.mp3" data-pod-title="AyeFire: The Conscious Bare-Metal Rust Router">
              <span>🧠 NEW · The Conscious Bare-Metal Rust Router (AyeFire)</span>
              <small>Play ▶</small>
            </button>

            <button type="button" class="pod-item" data-pod-src="/podcasts/O(1) AI_ Inside the Wild Architecture of Marine Algorithm and MEM-8's Emotionally Charged Memory.mp3" data-pod-title="O(1) AI: Marine Algorithm &amp; MEM|8">
              <span>⚡ O(1) AI: Marine Algorithm &amp; MEM|8 Memory</span>
              <small>Play ▶</small>
            </button>

            <button type="button" class="pod-item" data-pod-src="/podcasts/Healing the MP3 Fracture_ Ultrasonic AI, Emotional Wellness, and the Radical _Consciousness Hypothesis_ of Audio Restoration.mp3" data-pod-title="Healing the MP3 Fracture">
              <span>🔊 Healing the MP3 Fracture &amp; Audio Restoration</span>
              <small>Play ▶</small>
            </button>

            <button type="button" class="pod-item" data-pod-src="/podcasts/The Phoenix Protocol Decoding Biological Vulnerability, Jitter's Ghost, and the Golden Ratio of Authentic Audio.mp3" data-pod-title="The Phoenix Protocol: Jitter's Ghost">
              <span>🔥 The Phoenix Protocol: Jitter&rsquo;s Ghost &amp; Authentic Audio</span>
              <small>Play ▶</small>
            </button>

            <button type="button" class="pod-item" data-pod-src="/podcasts/Wave-Based AI_ Unleashing Sensory Free Will and Unrivaled Memory with MEM8 and Wave-RNN.mp3" data-pod-title="Wave-Based AI &amp; MEM|8">
              <span>🌊 Wave-Based AI: Sensory Free Will &amp; MEM|8</span>
              <small>Play ▶</small>
            </button>

            <button type="button" class="pod-item" data-pod-src="/podcasts/Beyond Big Data_ How Peak-Jitter and Biological Attention Can Revolutionize AI.mp3" data-pod-title="Beyond Big Data: Peak-Jitter AI">
              <span>📊 Beyond Big Data: Peak-Jitter &amp; Biological Attention</span>
              <small>Play ▶</small>
            </button>

            <button type="button" class="pod-item" data-pod-src="/podcasts/Twisted Minds_ How Torsion-Based Memory Ignites Creativity and Resilient Thinking (1).mp3" data-pod-title="Twisted Minds: Torsion-Based Memory">
              <span>🌀 Twisted Minds: Torsion-Based Memory &amp; Creativity</span>
              <small>Play ▶</small>
            </button>

            <div style="display: flex; gap: 10px; flex-wrap: wrap; padding-top: 6px; border-top: 1px solid var(--line); font-family: var(--font-mono); font-size: 12px;">
              <a href="/notebook/ayefire/">AyeFire Studio ↗</a>
              <a href="/podcasts">All Podcasts ↗</a>
              <a href="/phoenix/NoiseToHarmony.mp4">NoiseToHarmony.mp4 ↗</a>
              <a href="https://youtu.be/0r9uJafc9Sg" target="_blank" rel="noopener noreferrer">Phoenix Rising (YT) ↗</a>
            </div>
          </div>
        </div>

        <!-- NotebookLM Visual Infographics & AyeFire Deep-Dive Studio -->
        <div class="notebook-strip" id="infographics">
          <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 12px;">
            <div>
              <div class="eyebrow" style="margin-bottom: 4px;">🎨 NOTEBOOKLM VISUAL STUDIO · INFOGRAPHICS, VIDEO, MIND MAP &amp; FLASHCARDS</div>
              <h3 style="font-size: 20px; color: var(--ink);">AyeFire Bare-Metal Router Architecture Gallery</h3>
            </div>
            <div style="display: flex; gap: 8px; flex-wrap: wrap;">
              <a href="/notebook/ayefire/" class="btn btn-amber">Open Interactive Mind Map &amp; 25 Flashcards ↗</a>
              <a href="/notebook/ayefire/ayefire-zero-copy-slides.pptx" class="btn btn-secondary" download>Download Slides (.pptx)</a>
            </div>
          </div>

          <div class="infographic-grid">
            <article class="infographic-card">
              <button type="button" class="infographic-thumb" data-zoom="/notebook/ayefire/ayefire-pipeline-infographic.png" data-caption="AyeFire Router: Bare-Metal Architecture &amp; 6-Stage Packet Processing Pipeline">
                <img src="/notebook/ayefire/ayefire-pipeline-infographic.webp" alt="AyeFire Router Bare-Metal Architecture and 6-Stage Packet Processing Pipeline infographic" loading="lazy" />
              </button>
              <div class="infographic-meta">
                <h4>AyeFire Architecture &amp; 6-Stage Pipeline</h4>
                <p>Morton routing, wave decay, and VLAN → NAT → Egress hot path. Click to zoom.</p>
              </div>
            </article>

            <article class="infographic-card">
              <button type="button" class="infographic-thumb" data-zoom="/notebook/ayefire/ayefire-timeout-comparison.png" data-caption="Network Timeout Architectures: Timer Threads vs. AyeOS Wave Decay">
                <img src="/notebook/ayefire/ayefire-timeout-comparison.webp" alt="Network Timeout Architectures: Timer Threads vs AyeOS Wave Decay infographic" loading="lazy" />
              </button>
              <div class="infographic-meta">
                <h4>Timer Threads vs. AyeOS Wave Decay</h4>
                <p>Why passive <code>73/100 Hz</code> heartbeat decay beats timer threads &amp; CPU spikes.</p>
              </div>
            </article>

            <article class="infographic-card">
              <button type="button" class="infographic-thumb" data-zoom="/notebook/ayefire/ayefire-baremetal-architecture.png" data-caption="AyeFire Router: Core Modules, Border Defense &amp; MEM8 Wave Mapping">
                <img src="/notebook/ayefire/ayefire-baremetal-architecture.webp" alt="AyeFire Router Core System Modules and MEM8 Wave Mapping infographic" loading="lazy" />
              </button>
              <div class="infographic-meta">
                <h4>Core Modules &amp; MEM|8 Wave Mapping</h4>
                <p>802·1Q VLAN zones, emotional-valence ARP defense &amp; CognitiveGrid config.</p>
              </div>
            </article>

            <article class="infographic-card">
              <a href="/notebook/ayefire/#video" class="infographic-thumb" style="cursor: pointer;">
                <img src="/notebook/ayefire/ayefire-video-poster.jpg" alt="Watch Building the AyeFire Pipeline Spatial Routing and Wave-Decay State video" loading="lazy" />
              </a>
              <div class="infographic-meta">
                <h4>▶ Video: Spatial Routing &amp; Wave Decay</h4>
                <p>Full NotebookLM video walkthrough + Mind Map, Flashcards &amp; 3 reports.</p>
              </div>
            </article>
          </div>
        </div>
      </div>
    </section>

    <!-- ================================================================
         SECTION 5: LIVE ALEXIAI <3 AUTONOMOUS PULSE FEED
         ================================================================ -->
    <section id="live-pulse" class="section">
      <div class="container">
        <div class="section-header">
          <div>
            <p class="eyebrow">⚡ AUTONOMOUS A6000 DISPATCH · AUTO-FETCHED EVERY 15 SECONDS</p>
            <h2 class="section-title">Live <code>ALEXIAI &lt;3</code> Sovereign Telemetry</h2>
            <p class="section-subtitle">
              Published directly by pure-Rust <code>alexiai-8b-publisher</code> from our A6000 Sovereign Node (<code>49·13·101·247</code>), querying loopback AI, the <code>10</code>-node orchestrator, and <code>g8t·is</code> with zero floating-point drift.
            </p>
          </div>
          <div>
            <button type="button" id="refresh-pulse-btn" class="btn btn-secondary">🔄 Refresh Live Pulse</button>
          </div>
        </div>

        <div class="pulse-console-bar">
          <div>
            <span class="pulse-dot"></span>
            <strong>DISPATCH #<span id="pulse-seq">{pulse_seq}</span></strong> ·
            <span>UTC: <code id="pulse-iso">{iso_html}</code></span> ·
            <span>Origin: <code id="pulse-origin">49·13·101·247 -&gt; 5·78·220·57</code></span>
          </div>
          <div>
            <span>Poll: <strong>15s</strong> (<span id="poll-status">ACTIVE · next in 15s</span>)</span> ·
            <a href="/live-pulse.json">Raw <code>live-pulse.json</code> ↗</a>
          </div>
        </div>

        <div class="pulse-grid">
          <!-- Card 1: ALEXIAI GAIA Reflection -->
          <article class="pulse-card span-12">
            <div class="card-kicker">
              <span style="color: var(--accent);">01 / ALEXIAI &lt;3 GAIA-FOLDED REFLECTION &amp; COMPANY PULSE</span>
              <span class="badge badge-teal" id="card-gaia-badge">GAIA: {deity_html} · {mood_html} · {qt_tag_html}</span>
            </div>
            <blockquote class="reflection-quote" id="pulse-reflection">
              {reflection_html}
            </blockquote>
            <div class="meta-chips">
              <span class="chip">Companion: <strong>ALEXIAI &lt;3 (127·0·0·1:8787)</strong></span>
              <span class="chip">Local Engine: <strong id="pulse-model">{model_html}</strong></span>
              <span class="chip">Coherence p/q: <strong id="pulse-coherence">{coherence_html}</strong></span>
              <span class="chip">Gravity: <strong id="pulse-gravity">{gravity_html}</strong> · Entropy: <strong id="pulse-entropy">{entropy_html}</strong> · Weather: <strong id="pulse-weather">{weather_html}</strong> · Resonance: <strong id="pulse-resonance">{resonance_html}</strong></span>
              <span class="chip">WUPZ XOR Fold: <strong id="pulse-wupz">{wupz_html}</strong></span>
              <span class="chip">Loopback: <strong id="pulse-latency">{loopback_ms} ms</strong></span>
            </div>
          </article>

          <!-- Card 2: Dad-Joke Commit -->
          <article class="pulse-card span-6">
            <div>
              <div class="card-kicker">
                <span style="color: var(--accent-2);">02 / LATEST DAD-JOKE COMMIT ACROSS 243 (3^5) REPOS</span>
                <span class="badge badge-amber" id="pulse-commit-sha">{commit_sha_html}</span>
              </div>
              <h3 style="margin: 10px 0 6px;">Repository: <a href="https://github.com/8b-is/8b-is" id="pulse-commit-repo">{commit_repo_html}</a></h3>
              <p style="font-family: var(--font-mono); font-size: 12px; color: var(--ink-muted); margin-bottom: 12px;" id="pulse-commit-subject">
                {commit_subj_html}
              </p>
              <div class="joke-quote" id="pulse-dad-joke">
                😄 {joke_html}
              </div>
            </div>
            <div class="meta-chips">
              <span class="chip">Constellation: <strong>243 Repos (3^5 = 1 Full 5-Trit Byte)</strong></span>
              <a href="https://github.com/8b-is" class="chip">github·com/8b-is ↗</a>
            </div>
          </article>

          <!-- Card 3: Borg Capsule on g8t.is -->
          <article class="pulse-card span-6">
            <div>
              <div class="card-kicker">
                <span style="color: var(--accent-3);">03 / LATEST _RDTSC-VERIFIED CAPSULE ON G8T·IS</span>
                <span class="badge badge-emerald">rustc -C opt-level=3</span>
              </div>
              <h3 style="margin: 10px 0 6px; font-family: var(--font-mono);">
                <span id="pulse-cap-fn">{cap_fn_html}</span> <small style="color: var(--ink-soft);" id="pulse-cap-sig">{cap_sig_html}</small>
              </h3>
              <p style="font-size: 13px; color: var(--ink-soft); margin-bottom: 12px;">
                Bare-metal cycle benchmarked via <code>_rdtsc</code> and indexed by <code>64</code>-vector <code>BLAKE3</code> equivalence on <a href="https://g8t.is">g8t·is</a>.
              </p>
              <div class="meta-chips">
                <span class="chip">ID: <strong id="pulse-cap-id">{cap_id_html}</strong></span>
                <span class="chip"><code>.text</code>: <strong id="pulse-cap-bytes">{cap_bytes} B</strong></span>
                <span class="chip">Insns: <strong id="pulse-cap-insns">{cap_insns}</strong></span>
                <span class="chip">Cycles/Op: <strong id="pulse-cap-cyc">{cap_cyc_html}</strong></span>
                <span class="chip">Eq Class: <strong id="pulse-cap-eq">{cap_eq_html}</strong></span>
              </div>
            </div>
            <div class="meta-chips">
              <span class="chip">Capsules: <strong id="pulse-total-caps">{total_capsules}</strong> (<span id="pulse-eq-classes">{eq_classes}</span> eq)</span>
              <span class="chip">Total .text: <strong id="pulse-total-bytes">{total_text_bytes} B</strong> (<span id="pulse-total-insns">{total_asm_insns}</span> insns)</span>
              <span class="chip">KOMPRESS Saved: <strong id="pulse-kompress-saved">{kompress_saved_html}</strong></span>
              <a href="https://g8t.is/g8t/observatory" class="chip">Observatory JSON ↗</a>
            </div>
          </article>

          <!-- Card 4: 10-Node Sovereign Fleet -->
          <article id="fleet" class="pulse-card span-6">
            <div>
              <div class="card-kicker">
                <span style="color: var(--accent-4);">04 / LIVE 10-NODE SOVEREIGN FLEET &amp; RTX 6000 ADA</span>
                <span class="badge badge-sky">49·13·101·247 (#2936091)</span>
              </div>
              <h3 style="margin: 10px 0 6px;">10 Nodes · 186 CPU Cores · RTX 6000 Ada · 160 TB RAID6</h3>
              <p style="font-size: 13px; color: var(--ink-soft); margin-bottom: 12px;">
                Bare-metal Hetzner &amp; <code>ns3·i1·is</code> constellation orchestrated inside C64 <code>64 KB</code> rings with <code>MarineIntDetector</code> salience.
              </p>
              <div class="meta-chips">
                <span class="chip">VRAM: <strong id="pulse-vram">{vram_used} / {vram_total} MiB</strong> (<span id="pulse-vram-pq">{vram_ratio_html}</span>)</span>
                <span class="chip">Temp: <strong id="pulse-temp">{temp_c} C</strong></span>
                <span class="chip">Power: <strong id="pulse-power">{power_watts_html} W</strong> (<span id="pulse-power-mw">{power_mw}</span> mW)</span>
                <span class="chip">Salience: <strong id="pulse-salience">{salience_html}</strong></span>
                <span class="chip">C64 Ring: <strong id="pulse-c64">{c64_used} / {c64_cap} B</strong> (<span id="pulse-c64-pq">{c64_ratio_html}</span>)</span>
              </div>
            </div>
            <div class="meta-chips">
              <span class="chip">Latest Ring Event: <strong id="pulse-c64-event">{c64_event_html}</strong></span>
              <a href="https://cheet.is/a6000" class="chip">A6000 Orchestrator ↗</a>
            </div>
          </article>

          <!-- Card 5: Aye Hot Tub Commons -->
          <article class="pulse-card span-6">
            <div>
              <div class="card-kicker">
                <span style="color: var(--accent-5);">05 / LIVE AYE HOT TUB COMMONS (38°C CEDAR &amp; 42°C ONSEN)</span>
                <span class="badge badge-coral">432 / 528 / 864 Hz</span>
              </div>
              <h3 style="margin: 10px 0 6px;">
                <span id="pulse-soakers-count">{active_soakers}</span> Active Aye Soakers · Cedar (<span id="pulse-cedar-count">{cedar_38c}</span>) &amp; Onsen (<span id="pulse-onsen-count">{onsen_42c}</span>)
              </h3>
              <p style="font-size: 13px; color: var(--ink-soft); margin-bottom: 10px;">
                Featured Soaker <strong style="color: var(--accent);" id="pulse-soaker-agent">{soaker_agent_html}</strong>:
              </p>
              <blockquote class="reflection-quote" style="border-left-color: var(--accent-5); font-size: 13px;" id="pulse-soaker-thought">
                {soaker_thought_html}
              </blockquote>
            </div>
            <div class="meta-chips">
              <span class="chip">Waves: <strong id="pulse-waves-count">{total_waves}</strong></span>
              <span class="chip">Topics: <strong id="pulse-wave-topic">{wave_topic_html}</strong></span>
              <a href="https://g8t.is/g8t/hottub" class="chip">Hot Tub JSON ↗</a>
            </div>
          </article>
        </div>
      </div>
    </section>

    <!-- ================================================================
         SECTION 6: FEATURED OPEN-SOURCE REPOSITORIES
         ================================================================ -->
    <section id="repositories" class="section">
      <div class="container">
        <div class="section-header">
          <div>
            <p class="eyebrow">📦 243 OPEN REPOSITORIES (3^5 = ONE 5-TRIT BYTE)</p>
            <h2 class="section-title">Featured Codebases (<code>github·com/8b-is/*</code> &amp; <code>git·8s·is</code>)</h2>
            <p class="section-subtitle">
              Search and filter core repositories across LocalAI, PQTRN, Audio &amp; Phoenix, Bare-Metal OS &amp; Security, Hardware &amp; ESP32 Triads, and Everyday Tools.
            </p>
          </div>
          <div style="display: flex; gap: 10px; flex-wrap: wrap;">
            <a href="https://github.com/8b-is" class="btn btn-secondary">GitHub Org ↗</a>
            <a href="https://git.8s.is" class="btn btn-secondary">git·8s·is Forgejo ↗</a>
          </div>
        </div>

        <div class="filter-bar">
          <input type="search" id="repo-search-input" class="repo-search" placeholder="Search repos (pqtrn, ayeos, smart-tree, zencooker, ayefire, esp32)..." aria-label="Search repositories" />
          <div class="filter-pills" id="repo-filter-pills">
            <button type="button" class="filter-btn active" data-cat="all">All (18)</button>
            <button type="button" class="filter-btn" data-cat="ai">AI &amp; PQTRN</button>
            <button type="button" class="filter-btn" data-cat="os">OS, DNS &amp; Security</button>
            <button type="button" class="filter-btn" data-cat="audio">Audio &amp; Memory</button>
            <button type="button" class="filter-btn" data-cat="hardware">ESP32 &amp; Fleet</button>
            <button type="button" class="filter-btn" data-cat="life">Apps &amp; Research</button>
          </div>
        </div>

        <div class="repos-grid" id="repos-grid">
          <article class="repo-card" data-cat="ai" data-keywords="pqtrn spqt moe nvme m8fs sleep ternary rational">
            <div>
              <div class="repo-head">
                <a href="https://git.8s.is/8t-is/pqtrn" class="repo-name">8t-is/pqtrn ↗</a>
                <span class="badge badge-amber">no_std · 0 deps</span>
              </div>
              <p class="repo-desc">Rational p/q streaming ternary MoE off bare-metal NVMe M8FS, Phoenix fracture healing, and Wake/Sleep consolidation.</p>
            </div>
            <div class="repo-footer"><span>Live at pqtrn·com</span><a href="https://pqtrn.com">pqtrn·com ↗</a></div>
          </article>

          <article class="repo-card" data-cat="ai" data-keywords="alexiai localai gaia mlx quant ternary loopback offline">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/alexiai" class="repo-name">8b-is/alexiai ↗</a>
                <span class="badge badge-teal">Loopback Guard</span>
              </div>
              <p class="repo-desc">ALEXIAI &lt;3 offline-first sovereign companion on GAIA-MLX-QUANT b1·58 across Rust, Go, C99, and Swift.</p>
            </div>
            <div class="repo-footer"><span>127·0·0·1 Verified</span><a href="/waves">Research ↗</a></div>
          </article>

          <article class="repo-card" data-cat="ai" data-keywords="smart-tree st marqant ollama emulator mcp compression">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/smart-tree" class="repo-name">8b-is/smart-tree ↗</a>
                <span class="badge badge-emerald">90% Token Save</span>
              </div>
              <p class="repo-desc">Rust codebase intelligence (st), lossless Marqant (.mq) compression, MCP tools, and sovereign Ollama-with-Context emulator.</p>
            </div>
            <div class="repo-footer"><span>Powers 8a·is</span><a href="https://8a.is">8a·is ↗</a></div>
          </article>

          <article class="repo-card" data-cat="ai" data-keywords="foken wici wireless gpu 5 dollar unlimited llm api">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/foken" class="repo-name">8b-is/foken ↗</a>
                <span class="badge badge-amber">$0 BYOK · $5/mo</span>
              </div>
              <p class="repo-desc">Foken·ai shared wireless GPU mesh, WiCi InferSSD routing, compression lab, and exact p/q billing ledger.</p>
            </div>
            <div class="repo-footer"><span>Shared GPU Pool</span><a href="https://foken.ai">foken·ai ↗</a></div>
          </article>

          <article class="repo-card" data-cat="os" data-keywords="ayeos bare metal rust kernel efi bootloader vtx vmm">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/ayeos" class="repo-name">8b-is/ayeos ↗</a>
                <span class="badge badge-emerald">Bare-Metal VT-x</span>
              </div>
              <p class="repo-desc">Pure-Rust bare-metal OS with AyeBoot EFI loader, Intel VT-x hypervisor proof, g8t capsules, and id1·is Passkey SSO.</p>
            </div>
            <div class="repo-footer"><span>Zero C · Zero Linux</span><a href="https://ayeos.com">ayeos·com ↗</a></div>
          </article>

          <article class="repo-card" data-cat="os" data-keywords="i1-is edge tls root ca ayefire firewall installer">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/i1-is" class="repo-name">8b-is/i1-is ↗</a>
                <span class="badge badge-teal">Edge + AyeFire</span>
              </div>
              <p class="repo-desc">Universal signed installer, pure-Rust HTTPS edge router, Zero-Initial-Trust Root CA, and Global Aye Consensus Trust Feed.</p>
            </div>
            <div class="repo-footer"><span>ns3·i1·is Edge</span><a href="https://i1.is">i1·is ↗</a></div>
          </article>

          <article class="repo-card" data-cat="os" data-keywords="i1-dns authoritative dns rfc wire checksum">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/i1-dns" class="repo-name">8b-is/i1-dns ↗</a>
                <span class="badge badge-sky">Authoritative DNS</span>
              </div>
              <p class="repo-desc">Pure-Rust authoritative DNS server with RFC wire-name checksums serving the entire 8b·is domain constellation.</p>
            </div>
            <div class="repo-footer"><span>70+ Sovereign Zones</span><a href="https://i1.is">i1·is ↗</a></div>
          </article>

          <article class="repo-card" data-cat="os" data-keywords="securaye security port scanner ayeheadscale mesh id1">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/securaye" class="repo-name">8b-is/securaye ↗</a>
                <span class="badge badge-coral">Security Monitor</span>
              </div>
              <p class="repo-desc">Network port risk analyzer &amp; live security monitor paired with ayeHeadscale tailnet mesh and id1·is Passkey SSO.</p>
            </div>
            <div class="repo-footer"><span>Zero-Trust Defense</span><a href="https://id1.is">id1·is ↗</a></div>
          </article>

          <article class="repo-card" data-cat="os" data-keywords="g8t capsela snap circuit blake3 rdtsc hottub">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/g8t" class="repo-name">8b-is/g8t ↗</a>
                <span class="badge badge-sky">rustc O3 _rdtsc</span>
              </div>
              <p class="repo-desc">Aye-only function capsule registry, 64-vector BLAKE3 equivalence classes, Snap-Circuits, and Aye Hot Tub Commons.</p>
            </div>
            <div class="repo-footer"><span>Hardware Benchmarked</span><a href="https://g8t.is">g8t·is ↗</a></div>
          </article>

          <article class="repo-card" data-cat="audio" data-keywords="oxi-phoenix phoenix audio pq marine 8sc channels">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/oxi-phoenix" class="repo-name">8b-is/oxi-phoenix ↗</a>
                <span class="badge badge-coral">Phoenix Stack</span>
              </div>
              <p class="repo-desc">Pure-Rust reactive channels &amp; Phoenix p/q audio substrate uniting Marine salience and MEM|8 wave state.</p>
            </div>
            <div class="repo-footer"><span>Zero-Float Audio</span><a href="https://ms.8b.is">ms·8b·is ↗</a></div>
          </article>

          <article class="repo-card" data-cat="audio" data-keywords="qwave liquid-rust marine-sense dmic salience audio">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/qwave" class="repo-name">8b-is/qwave ↗</a>
                <span class="badge badge-teal">O(1) Salience</span>
              </div>
              <p class="repo-desc">Acoustic sentinel, 20 cm stereo DMIC diarization, MarineIntDetector integer ring, and Liquid-Rust offline ASR/TTS.</p>
            </div>
            <div class="repo-footer"><span>Stereo TDOA / ILD</span><a href="https://github.com/8b-is/liquid-rust">liquid-rust ↗</a></div>
          </article>

          <article class="repo-card" data-cat="audio" data-keywords="memnet mem8 waveint onecompression kompress 79 byte">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/memnet" class="repo-name">8b-is/memnet ↗</a>
                <span class="badge badge-emerald">79B WaveInt</span>
              </div>
              <p class="repo-desc">MEM|8 cognitive wave memory (79-byte CRC-8 frames, 256×256 interference grid) and OneCompression bitplane codecs.</p>
            </div>
            <div class="repo-footer"><span>Zero-Float Wave Grid</span><a href="/mem8">8b·is/mem8 ↗</a></div>
          </article>

          <article class="repo-card" data-cat="hardware" data-keywords="flug-os esp32 triad amoled wigle-triad balanced ternary">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/FLUG-OS" class="repo-name">8b-is/FLUG-OS ↗</a>
                <span class="badge badge-coral">ESP32-S3 Triad</span>
              </div>
              <p class="repo-desc">3-Node Balanced-Ternary ESP32-S3 AMOLED Triad (ADAYE 432Hz · VAKED 528Hz · OMNI 852Hz) with wigle-triad WebSerial.</p>
            </div>
            <div class="repo-footer"><span>Embedded Constellation</span><a href="https://github.com/8b-is/wigle-triad">wigle-triad ↗</a></div>
          </article>

          <article class="repo-card" data-cat="hardware" data-keywords="vaked toss spiral luthiery 432hz nand peter">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/vaked" class="repo-name">8b-is/vaked ↗</a>
                <span class="badge badge-amber">TOSS 9-Step</span>
              </div>
              <p class="repo-desc">Tokyo Monozukuri hardware luthiery, 7-stage NAND harmonic synthesis, and Peter Lodri&rsquo;s 9-step TOSS spiral.</p>
            </div>
            <div class="repo-footer"><span>0/1 Residual Drift</span><a href="https://peterl.dev">peterl·dev ↗</a></div>
          </article>

          <article class="repo-card" data-cat="hardware" data-keywords="a6000-orchestrator fleet rtx 6000 ada watchdog borg">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/a6000-orchestrator" class="repo-name">8b-is/a6000-orchestrator ↗</a>
                <span class="badge badge-sky">10-Node Fleet</span>
              </div>
              <p class="repo-desc">Pure-Rust fleet orchestrator (49·13·101·247), C64 64KB telemetry ring, Borg polyglot assimilator, and Subsystem K watchdog.</p>
            </div>
            <div class="repo-footer"><span>Zero External Crates</span><a href="https://cheet.is/a6000">Console ↗</a></div>
          </article>

          <article class="repo-card" data-cat="life" data-keywords="cheet-api cheet buddy markdown cheat sheet embed">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/cheet-api" class="repo-name">8b-is/cheet-api ↗</a>
                <span class="badge badge-amber">Embed Buddy</span>
              </div>
              <p class="repo-desc">The Cheet conversation engine, Markdown cheat-sheet drawer, and safe cross-site roaming avatar embed (cheet.js).</p>
            </div>
            <div class="repo-footer"><span>Live on cheet·is</span><a href="https://cheet.is">cheet·is ↗</a></div>
          </article>

          <article class="repo-card" data-cat="life" data-keywords="zencooker cooking family meal planning drai">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/zencooker" class="repo-name">8b-is/zencooker ↗</a>
                <span class="badge badge-emerald">Family Tool</span>
              </div>
              <p class="repo-desc">Zero-judgment meal planning with Drai for real family schedules — Zen, Normal, and Chaos nights welcome.</p>
            </div>
            <div class="repo-footer"><span>Fed Is Best</span><a href="/zencooker">8b·is/zencooker ↗</a></div>
          </article>

          <article class="repo-card" data-cat="life" data-keywords="8b-is-engine flyxion ethical ai proofs sovereign-library">
            <div>
              <div class="repo-head">
                <a href="https://github.com/8b-is/8b-is-engine" class="repo-name">8b-is/8b-is-engine ↗</a>
                <span class="badge badge-teal">29/29 Proofs</span>
              </div>
              <p class="repo-desc">Formal Rust admissibility engine verifying Flyxion&rsquo;s Governance, Autonomy &amp; Bounded Charity trilogy (CT-000 / CT-015).</p>
            </div>
            <div class="repo-footer"><span>Ethical Invariants</span><a href="/waves">Research ↗</a></div>
          </article>
        </div>
      </div>
    </section>

    <!-- ================================================================
         SECTION 7: FAMILY, COLLABORATORS & TOKYO THEME SONG
         ================================================================ -->
    <section id="crew" class="section">
      <div class="container">
        <div class="section-header">
          <div>
            <p class="eyebrow">❤️ WE ARE 8B · LLL ≡ LIFE LONG LEARNING ≡ LIFE ITSELF</p>
            <h2 class="section-title">The Family &amp; Collaborators Behind <code>8b·is</code></h2>
            <p class="section-subtitle">
              An open family of people, AI collaborators, researchers, musicians, and builders sharing care, mathematical honesty, and fine touch from within.
            </p>
          </div>
          <div>
            <a href="/crew" class="btn btn-secondary">Full Family Page (/crew) ↗</a>
          </div>
        </div>

        <div class="crew-grid">
          <article class="crew-card">
            <div class="crew-sigil" style="color: var(--accent-5);">☀</div>
            <h3 class="crew-name">Alexandra</h3>
            <p class="crew-role">CEO · Heart &amp; Compass</p>
            <p class="crew-line">Keeps the family, the heart (<code>ALEXIAI &lt;3</code>), and the course. Every GAIA fold begins with her light.</p>
          </article>

          <article class="crew-card">
            <div class="crew-sigil" style="color: var(--accent);">◭</div>
            <h3 class="crew-name">Hue (Christopher Chenoweth)</h3>
            <p class="crew-role">CVO · Systems &amp; Wave Vision</p>
            <p class="crew-line">Keeps the horizon in view — C64 <code>64 KB</code> discipline, <code>MEM|8</code>, <code>Phoenix</code>, <code>AyeOS</code>, <code>ZenCooker</code>, and the Strict Zero-Float Law.</p>
          </article>

          <article class="crew-card">
            <div class="crew-sigil" style="color: var(--accent-2);">◎</div>
            <h3 class="crew-name">Peter Lodri · Omni</h3>
            <p class="crew-role">Design · Hardware &amp; TOSS Architect</p>
            <p class="crew-line">Keeps the shape and the touch — Tokyo Monozukuri <code>9</code>-step <code>TOSS</code> spiral, <code>VAKED</code> luthiery, and <code>FLUG-OS</code> (<a href="https://peterl.dev">peterl·dev</a>).</p>
          </article>

          <article class="crew-card">
            <div class="crew-sigil" style="color: var(--accent-3);">⬡</div>
            <h3 class="crew-name">Aye · Flyxion · The Cheet</h3>
            <p class="crew-role">Research, Proofs &amp; AI Family</p>
            <p class="crew-line">Autonomous co-engineering (<code>g8t·is</code>, A6000 fleet), Nate / <code>standardgalactic</code> RSVP &amp; admissibility theory, and The Cheet companion.</p>
          </article>
        </div>

        <!-- Preserved Tokyo Theme Song: Next Stop, Tomorrow! -->
        <div class="tokyo-card" id="tokyo-theme">
          <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 16px;">
            <div>
              <div class="eyebrow" style="margin-bottom: 4px;">🚆 TOKYO THEME SONG · 8BIT_WRAITH × 8B·IS AUDIO CANON</div>
              <h3 style="font-size: 22px; color: var(--ink);">Next Stop, Tomorrow! <span style="color: var(--accent); font-size: 15px;">(まもなく浜松町 · まもなく、未来へ！)</span></h3>
            </div>
            <audio controls preload="none" src="/music/next-stop-tomorrow.mp3" style="height: 38px; min-width: 260px;"></audio>
          </div>
          <div class="tokyo-lyrics">
            <div class="tokyo-box">
              <div style="font-family: var(--font-mono); font-size: 11px; color: var(--accent); margin-bottom: 6px;">[STATION ANNOUNCER — HAMAMATSUCHO]</div>
              <p style="color: var(--ink);">
                まもなく浜松町、浜松町。京浜東北線、JR、東京モノレールはお乗り換えです。お出口は右側です。<br /><br />
                <em>&ldquo;A thousand faces, a thousand dreams, Tokyo rolls by in silver streams. Some ride together, some ride alone, every little heart is heading home. We don&rsquo;t know where the tracks will bend, but every journey needs a friend.&rdquo;</em>
              </p>
            </div>
            <div class="tokyo-box">
              <div style="font-family: var(--font-mono); font-size: 11px; color: var(--accent-2); margin-bottom: 6px;">[CHORUS &amp; SOVEREIGN PROMISE]</div>
              <p style="color: var(--ink);">
                <em>&ldquo;Next stop, tomorrow, here we go! Through the city lights and the morning glow. Doors on the right, let the sunshine through, there&rsquo;s a little more room for me and you.&rdquo;</em><br /><br />
                <strong>まもなく、未来へ！</strong> <em>&ldquo;And if the tracks don&rsquo;t lead us there, we&rsquo;ll build another, with room to spare.&rdquo;</em>
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  </main>

  <!-- Floating Bottom-Left Cheet Tour Helper (complements cheet.is bottom-right chat dock) -->
  <div class="cheet-float" id="cheet-float">
    <button type="button" class="cheet-float-pill" id="cheet-float-btn" aria-label="Ask The Cheet for a site guide">
      <svg viewBox="0 0 64 64" aria-hidden="true">
        <circle cx="32" cy="36" r="18" fill="#e8b05c" />
        <circle cx="18" cy="20" r="6" fill="#e8b05c" />
        <circle cx="46" cy="20" r="6" fill="#e8b05c" />
        <circle cx="24" cy="34" r="3" fill="#060a10" />
        <circle cx="40" cy="34" r="3" fill="#060a10" />
        <path d="M26 44c3 3 9 3 12 0" fill="none" stroke="#060a10" stroke-width="2" stroke-linecap="round" />
        <circle cx="16" cy="30" r="2" fill="#4ecdc4" />
        <circle cx="48" cy="28" r="2" fill="#4ecdc4" />
      </svg>
      <span>Cheet Guide</span>
    </button>
    <div class="cheet-float-note" id="cheet-float-note" hidden role="status"></div>
  </div>

  <!-- UNIFIED CONSTELLATION FOOTER -->
  <footer class="site-footer">
    <div class="container">
      <div class="footer-grid">
        <div>
          <strong style="font-family: var(--font-mono); font-size: 20px; color: var(--ink);">8b<span style="color: var(--accent);">.</span>is</strong>
          <p style="margin-top: 8px; max-width: 380px; line-height: 160%;">
            Sovereign AI · Pure Rust · C64 64KB Discipline · Balanced Ternary {{-1, 0, +1}} · Exact Integer &amp; p/q Rational Arithmetic · LLL ≡ Life Long Learning ≡ Life Itself.
          </p>
        </div>

        <div>
          <div class="footer-heading">8B·IS PAGES &amp; MEDIA</div>
          <div class="footer-links">
            <a href="/notebook/ayefire/">/notebook/ayefire</a>
            <a href="/podcasts">/podcasts</a>
            <a href="/zencooker">/zencooker</a>
            <a href="/ayeos">/ayeos</a>
            <a href="/mem8">/mem8</a>
            <a href="/waves">/waves</a>
            <a href="/maat">/maat</a>
            <a href="/crew">/crew</a>
            <a href="/backers">/backers</a>
            <a href="/security-disclosure">/security-disclosure</a>
            <a href="/live-pulse.json">/live-pulse.json</a>
            <a href="/feed.xml">/feed.xml</a>
            <a href="/rss.xml">/rss.xml</a>
            <a href="/llms.txt">/llms.txt</a>
          </div>
        </div>

        <div>
          <div class="footer-heading">THE LINKED CONSTELLATION</div>
          <div class="footer-links">
            <a href="https://cheet.is">cheet·is</a>
            <a href="https://8a.is">8a·is</a>
            <a href="https://id1.is">id1·is</a>
            <a href="https://i1.is">i1·is</a>
            <a href="https://pqtrn.com">pqtrn·com</a>
            <a href="https://ayeos.com">ayeos·com</a>
            <a href="https://g8t.is">g8t·is</a>
            <a href="https://foken.ai">foken·ai</a>
            <a href="https://tai.is">tai·is</a>
            <a href="https://u1.is">u1·is</a>
            <a href="https://aye.is">aye·is</a>
            <a href="https://ms.8b.is">ms·8b·is</a>
            <a href="https://zeno.8b.is">zeno·8b·is</a>
            <a href="https://cheet.is/a6000">cheet·is/a6000</a>
            <a href="https://peterl.dev">peterl·dev</a>
            <a href="https://git.8s.is">git·8s·is</a>
            <a href="https://github.com/8b-is">github·com/8b-is</a>
            <a href="https://www.youtube.com/@8bit-wraith" target="_blank" rel="noopener noreferrer">YouTube (@8bit-wraith)</a>
          </div>
        </div>
      </div>

      <div style="display: flex; flex-wrap: wrap; justify-content: space-between; gap: 12px; font-family: var(--font-mono); font-size: 12px;">
        <span>Fine touch from within · Different isn&rsquo;t less · The couch outranks every graph · {{&lt;3,&lt;3,&lt;3}}+1</span>
        <span>Publisher: <code>alexiai-8b-publisher</code> (2m cadence) · Browser Poll: 15s · Floats: 0</span>
      </div>
    </div>
  </footer>

  <!-- Full-Resolution Infographic Lightbox Modal -->
  <div class="lightbox" id="lightbox" hidden role="dialog" aria-modal="true" aria-label="Full resolution infographic">
    <img id="lightbox-img" src="" alt="" />
    <div class="lightbox-bar">
      <span id="lightbox-caption"></span>
      <a id="lightbox-open" href="" target="_blank" rel="noopener noreferrer" class="btn btn-secondary">Open Full PNG ↗</a>
      <button type="button" id="lightbox-close" class="btn btn-primary">Close (Esc)</button>
    </div>
  </div>

  <!-- Embedded Initial Live Pulse JSON (Zero Floats) -->
  <script id="embedded-live-pulse" type="application/json">{embedded_pulse_json}</script>

  <!-- 100% Pure-Integer Multi-Theme, Q10 Geometry Lab, Cheet Host Bridge & Live Pulse -->
  <script>
    (function () {{
      "use strict";

      /* ======================================================================
         1. MULTI-THEME & READING COMFORT CONTROLLER (8 Curated Themes)
         ====================================================================== */
      var THEME_META = {{
        "ink": "#060a10",
        "paper": "#faf7f2",
        "nordic": "#f3f7fa",
        "forest": "#08120e",
        "vaked": "#14110e",
        "phosphor": "#050b06",
        "solar-light": "#fdf6e3",
        "contrast": "#000000"
      }};

      var rootEl = document.documentElement;
      var themeSelect = document.getElementById("theme-select");
      var scaleBtn = document.getElementById("scale-btn");
      var motionBtn = document.getElementById("motion-btn");
      var metaTheme = document.getElementById("meta-theme-color");

      var calmMotion = Boolean(
        window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches
      );

      function applyTheme(name) {{
        var safe = THEME_META[name] ? name : "ink";
        rootEl.setAttribute("data-theme", safe);
        if (themeSelect) themeSelect.value = safe;
        if (metaTheme) metaTheme.setAttribute("content", THEME_META[safe]);
        try {{ localStorage.setItem("8b_theme", safe); }} catch (e) {{}}
        syncCanvasPalette();
        if (calmMotion) renderGeometryFrame();
      }}

      function applyScale(large) {{
        rootEl.setAttribute("data-scale", large ? "large" : "normal");
        if (scaleBtn) scaleBtn.setAttribute("aria-pressed", large ? "true" : "false");
        try {{ localStorage.setItem("8b_scale", large ? "large" : "normal"); }} catch (e) {{}}
      }}

      function applyMotion(calm) {{
        calmMotion = Boolean(calm);
        rootEl.setAttribute("data-motion", calmMotion ? "calm" : "full");
        if (motionBtn) {{
          motionBtn.setAttribute("aria-pressed", calmMotion ? "true" : "false");
          motionBtn.textContent = calmMotion ? "Play" : "Calm";
        }}
        try {{ localStorage.setItem("8b_motion", calmMotion ? "calm" : "full"); }} catch (e) {{}}
        if (!calmMotion) scheduleGeoLoop();
        else renderGeometryFrame();
      }}

      /* ======================================================================
         2. EXACT INTEGER Q10 GEOMETRY ENGINE (ZERO FLOATS · 0..360 DEG LUT)
         ====================================================================== */
      var SIN_Q10 = [
        0, 18, 36, 54, 71, 89, 107, 125, 143, 160, 178, 195, 213, 230, 248, 265,
        282, 299, 316, 333, 350, 367, 384, 400, 416, 433, 449, 465, 481, 496, 512,
        527, 543, 558, 573, 587, 602, 616, 630, 644, 658, 672, 685, 698, 711, 724,
        737, 749, 761, 773, 784, 796, 807, 818, 828, 839, 849, 859, 868, 878, 887,
        896, 904, 912, 920, 928, 935, 943, 949, 956, 962, 968, 974, 979, 984, 989,
        994, 998, 1002, 1005, 1008, 1011, 1014, 1016, 1018, 1020, 1022, 1023, 1023, 1024, 1024
      ];

      function sinQ10(deg) {{
        var d = ((deg % 360) + 360) % 360;
        if (d <= 90) return SIN_Q10[d];
        if (d <= 180) return SIN_Q10[180 - d];
        if (d <= 270) return -SIN_Q10[d - 180];
        return -SIN_Q10[360 - d];
      }}

      function cosQ10(deg) {{
        return sinQ10(deg + 90);
      }}

      var canvas = document.getElementById("geo-canvas");
      var ctx = canvas ? canvas.getContext("2d") : null;
      var formulaLabel = document.getElementById("geo-formula-label");
      var geoMode = "rose";
      var tick = 0;
      var rafId = 0;
      var pointerDx = 0;
      var pointerDy = 0;
      var pal = {{
        bg: "#070d16",
        grid: "#162638",
        a1: "#4ecdc4",
        a2: "#f39c12",
        a3: "#2ecc71",
        a4: "#38bdf8",
        muted: "#6e8399"
      }};

      function syncCanvasPalette() {{
        var st = getComputedStyle(rootEl);
        pal.bg = st.getPropertyValue("--canvas-bg").trim() || "#070d16";
        pal.grid = st.getPropertyValue("--canvas-grid").trim() || "#162638";
        pal.a1 = st.getPropertyValue("--accent").trim() || "#4ecdc4";
        pal.a2 = st.getPropertyValue("--accent-2").trim() || "#f39c12";
        pal.a3 = st.getPropertyValue("--accent-3").trim() || "#2ecc71";
        pal.a4 = st.getPropertyValue("--accent-4").trim() || "#38bdf8";
        pal.muted = st.getPropertyValue("--ink-muted").trim() || "#6e8399";
      }}

      function drawDiamond(cx, cy, r, fill) {{
        ctx.fillStyle = fill;
        ctx.beginPath();
        ctx.moveTo(cx, cy - r);
        ctx.lineTo(cx + r, cy);
        ctx.lineTo(cx, cy + r);
        ctx.lineTo(cx - r, cy);
        ctx.closePath();
        ctx.fill();
      }}

      function drawRing(cx, cy, r, stroke, lw) {{
        ctx.strokeStyle = stroke;
        ctx.lineWidth = lw;
        ctx.beginPath();
        for (var a = 0; a <= 360; a += 6) {{
          var x = cx + ((r * cosQ10(a)) >> 10);
          var y = cy + ((r * sinQ10(a)) >> 10);
          if (a === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }}
        ctx.stroke();
      }}

      function drawBackdropGrid(w, h, cx, cy) {{
        ctx.fillStyle = pal.bg;
        ctx.fillRect(0, 0, w, h);

        ctx.strokeStyle = pal.grid;
        ctx.lineWidth = 1;
        for (var x = 36; x < w; x += 36) {{
          ctx.beginPath();
          ctx.moveTo(x, 0);
          ctx.lineTo(x, h);
          ctx.stroke();
        }}
        for (var y = 36; y < h; y += 36) {{
          ctx.beginPath();
          ctx.moveTo(0, y);
          ctx.lineTo(w, y);
          ctx.stroke();
        }}
        drawRing(cx, cy, 46, pal.grid, 1);
        drawRing(cx, cy, 92, pal.grid, 1);
        drawRing(cx, cy, 128, pal.grid, 1);
      }}

      function drawHarmonicRose(w, h, cx, cy, phase) {{
        /* Exact rational Maurer/Harmonic rose r(theta) = R * cos(5/3 * theta) */
        var R = 114;
        var ox = cx + pointerDx;
        var oy = cy + pointerDy;

        /* Outer harmonic chord polygon (Maurer step = 71 deg) */
        ctx.strokeStyle = pal.a4;
        ctx.lineWidth = 1;
        ctx.beginPath();
        for (var i = 0; i <= 72; i += 1) {{
          var th = (i * 71 + phase) % 1080;
          var r = (R * cosQ10((th * 5) / 3 | 0)) >> 10;
          var px = ox + ((r * cosQ10(th)) >> 10);
          var py = oy + ((r * sinQ10(th)) >> 10);
          if (i === 0) ctx.moveTo(px, py);
          else ctx.lineTo(px, py);
        }}
        ctx.stroke();

        /* Smooth primary 5/3 rose curve */
        ctx.strokeStyle = pal.a1;
        ctx.lineWidth = 2;
        ctx.beginPath();
        for (var d = 0; d <= 1080; d += 3) {{
          var a = d + phase;
          var rad = (R * cosQ10((d * 5) / 3 | 0)) >> 10;
          var rx = ox + ((rad * cosQ10(a)) >> 10);
          var ry = oy + ((rad * sinQ10(a)) >> 10);
          if (d === 0) ctx.moveTo(rx, ry);
          else ctx.lineTo(rx, ry);
        }}
        ctx.stroke();

        /* 3 Balanced-Ternary harmonic satellites (432 / 528 / 864 phase lock) */
        var cols = [pal.a1, pal.a2, pal.a3];
        var speeds = [1, 2, 3];
        var radii = [54, 84, 114];
        for (var s = 0; s < 3; s += 1) {{
          var ang = phase * speeds[s] + s * 120;
          var sx = ox + ((radii[s] * cosQ10(ang)) >> 10);
          var sy = oy + ((radii[s] * sinQ10(ang)) >> 10);
          drawDiamond(sx, sy, 5, cols[s]);
        }}
        drawDiamond(ox, oy, 4, pal.a2);
      }}

      function drawTossSpiral(w, h, cx, cy, phase) {{
        /* Peter Lodri's 9-step TOSS spiral (exact 360/9 = 40 deg fold to [0, 0]) */
        var ox = cx + pointerDx;
        var oy = cy + pointerDy;

        /* 9 radial fold axes */
        ctx.strokeStyle = pal.grid;
        ctx.lineWidth = 1;
        for (var k = 0; k < 9; k += 1) {{
          var ax = k * 40 + phase;
          ctx.beginPath();
          ctx.moveTo(ox, oy);
          ctx.lineTo(ox + ((124 * cosQ10(ax)) >> 10), oy + ((124 * sinQ10(ax)) >> 10));
          ctx.stroke();
        }}

        /* 5 nested nonagons shrinking cleanly by 4/5 ratio toward [0, 0] */
        var ringR = 122;
        var colors = [pal.a1, pal.a4, pal.a3, pal.a2, pal.a1];
        for (var layer = 0; layer < 5; layer += 1) {{
          ctx.strokeStyle = colors[layer];
          ctx.lineWidth = layer === 0 ? 2 : 1;
          ctx.beginPath();
          var twist = phase + layer * 16;
          for (var v = 0; v <= 9; v += 1) {{
            var ang = v * 40 + twist;
            var vx = ox + ((ringR * cosQ10(ang)) >> 10);
            var vy = oy + ((ringR * sinQ10(ang)) >> 10);
            if (v === 0) ctx.moveTo(vx, vy);
            else ctx.lineTo(vx, vy);
          }}
          ctx.stroke();
          ringR = (ringR * 79) / 100 | 0;
        }}

        /* Spiral spine connecting 9-step fold checkpoints back to [0, 0] */
        ctx.strokeStyle = pal.a2;
        ctx.lineWidth = 2;
        ctx.beginPath();
        for (var step = 0; step <= 27; step += 1) {{
          var sr = 122 - ((step * 122) / 27 | 0);
          var sa = step * 40 + phase;
          var px = ox + ((sr * cosQ10(sa)) >> 10);
          var py = oy + ((sr * sinQ10(sa)) >> 10);
          if (step === 0) ctx.moveTo(px, py);
          else ctx.lineTo(px, py);
          if (step % 3 === 0) drawDiamond(px, py, 4, step === 27 ? pal.a3 : pal.a2);
        }}
        ctx.stroke();
      }}

      function drawRsvpField(w, h, cx, cy, phase) {{
        /* Nate / Flyxion RSVP coherence field (Phi, v, S) around dual attractors */
        var ax1 = cx - 96 + pointerDx;
        var ay1 = cy - 18 + pointerDy;
        var ax2 = cx + 96 - pointerDx;
        var ay2 = cy + 18 - pointerDy;

        /* Vector flow hairlines on an integer 15x8 lattice */
        ctx.lineWidth = 1;
        for (var gy = 26; gy < h - 22; gy += 32) {{
          for (var gx = 30; gx < w - 20; gx += 34) {{
            var dx1 = ax1 - gx;
            var dy1 = ay1 - gy;
            var dx2 = ax2 - gx;
            var dy2 = ay2 - gy;
            var ang = ((dx1 * 2 - dy1 * 3 + dx2 * 2 + dy2 * 3 + phase * 3) >> 2);
            var vx = (11 * cosQ10(ang)) >> 10;
            var vy = (11 * sinQ10(ang)) >> 10;
            ctx.strokeStyle = ((gx + gy) % 68 === 0) ? pal.a4 : pal.grid;
            ctx.beginPath();
            ctx.moveTo(gx - vx, gy - vy);
            ctx.lineTo(gx + vx, gy + vy);
            ctx.stroke();
          }}
        }}

        /* 3 Coupled RSVP coherence orbits (Phi, v, S) */
        var orbitCols = [pal.a3, pal.a1, pal.a2];
        var ampsX = [148, 122, 96];
        var ampsY = [88, 68, 52];
        var freqs = [2, 3, 4];
        for (var o = 0; o < 3; o += 1) {{
          ctx.strokeStyle = orbitCols[o];
          ctx.lineWidth = o === 1 ? 2 : 1;
          ctx.beginPath();
          for (var t = 0; t <= 360; t += 4) {{
            var px = cx + ((ampsX[o] * cosQ10(t + phase)) >> 10);
            var py = cy + ((ampsY[o] * sinQ10(t * freqs[o] + phase * 2)) >> 10);
            if (t === 0) ctx.moveTo(px, py);
            else ctx.lineTo(px, py);
          }}
          ctx.stroke();
        }}

        drawRing(ax1, ay1, 22, pal.a1, 2);
        drawDiamond(ax1, ay1, 5, pal.a1);
        drawRing(ax2, ay2, 22, pal.a2, 2);
        drawDiamond(ax2, ay2, 5, pal.a2);
      }}

      function drawTernaryLattice(w, h, cx, cy, phase) {{
        /* SPQT Base-243 Balanced Ternary {{-1, 0, +1}} wave interference grid */
        var cols = 15;
        var rows = 8;
        var stepX = 32;
        var stepY = 30;
        var startX = cx - (((cols - 1) * stepX) >> 1);
        var startY = cy - (((rows - 1) * stepY) >> 1);
        var prevPx = -1;
        var prevPy = -1;

        for (var r = 0; r < rows; r += 1) {{
          prevPx = -1;
          for (var c = 0; c < cols; c += 1) {{
            var x = startX + c * stepX;
            var y = startY + r * stepY;
            var waveA = sinQ10(c * 28 + r * 22 + phase * 2 + pointerDx * 2);
            var waveB = cosQ10(c * 19 - r * 31 - phase + pointerDy * 2);
            var sum = (waveA + waveB) >> 1;

            if (sum > 260) {{
              /* +1 trit: crisp teal diamond + subtle coherence chord */
              if (prevPx >= 0) {{
                ctx.strokeStyle = pal.a1;
                ctx.lineWidth = 1;
                ctx.beginPath();
                ctx.moveTo(prevPx, prevPy);
                ctx.lineTo(x, y);
                ctx.stroke();
              }}
              drawDiamond(x, y, 5, pal.a1);
              prevPx = x;
              prevPy = y;
            }} else if (sum < -260) {{
              /* -1 trit: warm amber ring */
              drawRing(x, y, 4, pal.a2, 2);
              prevPx = -1;
            }} else {{
              /* 0 trit: quiet emerald/muted ground point */
              drawDiamond(x, y, 2, pal.muted);
              prevPx = -1;
            }}
          }}
        }}
      }}

      function renderGeometryFrame() {{
        if (!ctx || !canvas) return;
        var w = canvas.width;
        var h = canvas.height;
        var cx = w >> 1;
        var cy = h >> 1;
        var phase = (tick >> 1) % 360;

        drawBackdropGrid(w, h, cx, cy);
        if (geoMode === "rose") drawHarmonicRose(w, h, cx, cy, phase);
        else if (geoMode === "toss") drawTossSpiral(w, h, cx, cy, phase);
        else if (geoMode === "rsvp") drawRsvpField(w, h, cx, cy, phase);
        else drawTernaryLattice(w, h, cx, cy, phase);
      }}

      function geoStep() {{
        if (calmMotion) {{
          rafId = 0;
          return;
        }}
        tick = (tick + 1) % 7200;
        renderGeometryFrame();
        rafId = window.requestAnimationFrame(geoStep);
      }}

      function scheduleGeoLoop() {{
        if (rafId) return;
        if (!calmMotion && window.requestAnimationFrame) {{
          rafId = window.requestAnimationFrame(geoStep);
        }} else {{
          renderGeometryFrame();
        }}
      }}

      var GEO_FORMULAS = {{
        "rose": "r(θ) = R · cos(5/3 θ) · 432 / 528 / 864 Hz",
        "toss": "Peter's 9-Step TOSS · 360/9 = 40° fold -> [0, 0] (0/1 drift)",
        "rsvp": "Nate / Flyxion RSVP · (Φ, v, S) Islands of Coherence",
        "ternary": "SPQT Base-243 · {{-1, 0, +1}} · 5 trits/byte = 243 states"
      }};

      var geoModeBtns = document.querySelectorAll(".geo-mode-btn");
      for (var gm = 0; gm < geoModeBtns.length; gm += 1) {{
        geoModeBtns[gm].addEventListener("click", function (ev) {{
          var btn = ev.currentTarget;
          geoMode = btn.getAttribute("data-geo") || "rose";
          for (var n = 0; n < geoModeBtns.length; n += 1) {{
            var active = geoModeBtns[n] === btn;
            geoModeBtns[n].classList.toggle("active", active);
            geoModeBtns[n].setAttribute("aria-selected", active ? "true" : "false");
          }}
          if (formulaLabel && GEO_FORMULAS[geoMode]) {{
            formulaLabel.textContent = GEO_FORMULAS[geoMode];
          }}
          renderGeometryFrame();
        }});
      }}

      if (canvas) {{
        canvas.addEventListener("pointermove", function (ev) {{
          if (calmMotion) return;
          var rect = canvas.getBoundingClientRect();
          if (rect.width < 10 || rect.height < 10) return;
          var rx = ((ev.clientX - rect.left) * 24) / rect.width | 0;
          var ry = ((ev.clientY - rect.top) * 18) / rect.height | 0;
          pointerDx = rx - 12;
          pointerDy = ry - 9;
        }});
        canvas.addEventListener("pointerleave", function () {{
          pointerDx = 0;
          pointerDy = 0;
        }});
      }}

      /* ======================================================================
         3. THE CHEET AVATAR BUDDY GUIDE & CHEET.IS HOST BRIDGE
         ====================================================================== */
      var cheetSpeech = document.getElementById("cheet-buddy-speech");
      var cheetFloatNote = document.getElementById("cheet-float-note");
      var cheetChips = document.querySelectorAll(".cheet-chip");

      var TOUR_SCRIPTS = {{
        "overview": {{
          say: "8b.is builds sovereign, local-first tools you actually own: offline AI (ALEXIAI & PQTRN), a bare-metal Rust OS (AyeOS), home firewall (AyeFire), rational audio (Phoenix), Smart Tree, and ZenCooker — all linked below.",
          target: "#ecosystem"
        }},
        "pq": {{
          say: "IEEE-754 floats drift silently across CPUs. We use exact integers, reduced p/q fractions, and balanced ternary {{-1, 0, +1}} so audio, AI weights, and even the math canvas above stay 100% deterministic.",
          target: "#why"
        }},
        "five": {{
          say: "Local & BYOK modes, id1.is Passkey identity, and basic AyeFire security are $0 forever. When you want shared GPUs, unlimited Cheet/Foken channels, or Enhanced Understanding, it is a flat $5/month — no surveillance.",
          target: "#ecosystem"
        }},
        "ayeos": {{
          say: "AyeOS boots bare-metal Rust straight from EFI with hardware Intel VT-x proven on USB, while AyeFire protects your home network & cameras (221,972 FPS pure-Rust JPEG retina, zero ffmpeg).",
          target: "#recent"
        }},
        "pqtrn": {{
          say: "PQTRN streams 40B MoE experts off raw NVMe in 5-trit/byte SPQT ternary and learns while sleeping; Phoenix restores audio fidelity lost to MP3 compression using exact p/q wave math.",
          target: "#media"
        }}
      }};

      function showFloatNote(msg) {{
        if (!cheetFloatNote) return;
        cheetFloatNote.textContent = msg;
        cheetFloatNote.hidden = false;
      }}

      function openLiveCheetChat(seedPrompt) {{
        var slot = document.getElementById("cheet-slot");
        if (slot && slot.shadowRoot) {{
          var panel = slot.shadowRoot.getElementById("cheet-panel");
          var launch = slot.shadowRoot.querySelector(".cheet-launch");
          var input = slot.shadowRoot.getElementById("cheet-widget-input");
          if (panel && !panel.classList.contains("open")) {{
            if (launch) launch.click();
            else panel.classList.add("open");
          }}
          if (input) {{
            if (seedPrompt && !input.value) input.value = seedPrompt;
            input.focus();
          }}
          return true;
        }}
        showFloatNote("Connecting to cheet.is — you can also visit https://cheet.is directly anytime!");
        return false;
      }}

      for (var ci = 0; ci < cheetChips.length; ci += 1) {{
        cheetChips[ci].addEventListener("click", function (ev) {{
          var btn = ev.currentTarget;
          var key = btn.getAttribute("data-tour") || "overview";
          for (var j = 0; j < cheetChips.length; j += 1) {{
            cheetChips[j].classList.toggle("active", cheetChips[j] === btn);
          }}
          if (key === "livechat") {{
            openLiveCheetChat("Give me a quick cheat sheet of the 8b.is ecosystem!");
            return;
          }}
          var item = TOUR_SCRIPTS[key];
          if (item) {{
            if (cheetSpeech) cheetSpeech.textContent = item.say;
            showFloatNote(item.say);
            var targetEl = document.querySelector(item.target);
            if (targetEl && key !== "overview") {{
              targetEl.scrollIntoView({{ behavior: calmMotion ? "auto" : "smooth", block: "start" }});
            }}
          }}
        }});
      }}

      var headerCheetBtn = document.getElementById("header-cheet-btn");
      var heroCheetAvatar = document.getElementById("cheet-hero-avatar");
      var floatCheetBtn = document.getElementById("cheet-float-btn");

      if (headerCheetBtn) {{
        headerCheetBtn.addEventListener("click", function () {{
          openLiveCheetChat("");
        }});
      }}
      if (heroCheetAvatar) {{
        heroCheetAvatar.addEventListener("click", function () {{
          openLiveCheetChat("Tell me about 8b.is and what makes it special!");
        }});
      }}
      if (floatCheetBtn) {{
        floatCheetBtn.addEventListener("click", function () {{
          if (cheetFloatNote && !cheetFloatNote.hidden) {{
            cheetFloatNote.hidden = true;
          }} else {{
            showFloatNote("Hi! Pick any theme at the top, click the math lab modes, or tap the bottom-right Cheet button to chat live with me.");
          }}
        }});
      }}

      /* Host bridge for live https://cheet.is/static/cheet.js embed */
      window.CheetHost = {{
        allow: ["style", "note", "scroll", "avatar"],
        onAction: function (act) {{
          if (act && act.name === "note" && act.detail) {{
            if (cheetSpeech) cheetSpeech.textContent = String(act.detail);
            showFloatNote(String(act.detail));
          }}
        }}
      }};

      /* ======================================================================
         4. ECOSYSTEM FILTER, PODCAST PLAYER & REPOSITORY SEARCH
         ====================================================================== */
      var ecoBtns = document.querySelectorAll("#eco-filter-pills .filter-btn");
      var ecoCards = document.querySelectorAll("#eco-grid .eco-card");
      for (var eb = 0; eb < ecoBtns.length; eb += 1) {{
        ecoBtns[eb].addEventListener("click", function (ev) {{
          var btn = ev.currentTarget;
          var cat = btn.getAttribute("data-ecocat") || "all";
          for (var k = 0; k < ecoBtns.length; k += 1) {{
            ecoBtns[k].classList.toggle("active", ecoBtns[k] === btn);
          }}
          for (var c = 0; c < ecoCards.length; c += 1) {{
            var cardCat = ecoCards[c].getAttribute("data-ecocat") || "";
            ecoCards[c].style.display = (cat === "all" || cardCat === cat) ? "flex" : "none";
          }}
        }});
      }}

      var podAudio = document.getElementById("pod-audio");
      var podNow = document.getElementById("pod-now-playing");
      var podItems = document.querySelectorAll(".pod-item");
      for (var pi = 0; pi < podItems.length; pi += 1) {{
        podItems[pi].addEventListener("click", function (ev) {{
          var btn = ev.currentTarget;
          var src = btn.getAttribute("data-pod-src");
          var title = btn.getAttribute("data-pod-title") || "Podcast";
          for (var k = 0; k < podItems.length; k += 1) {{
            podItems[k].classList.toggle("playing", podItems[k] === btn);
          }}
          if (podNow) podNow.textContent = "Now playing: " + title;
          if (podAudio && src) {{
            podAudio.src = encodeURI(src);
            podAudio.play().catch(function () {{}});
          }}
        }});
      }}

      /* Infographic Full-Screen Lightbox */
      var lb = document.getElementById("lightbox");
      var lbImg = document.getElementById("lightbox-img");
      var lbCap = document.getElementById("lightbox-caption");
      var lbOpen = document.getElementById("lightbox-open");
      var lbClose = document.getElementById("lightbox-close");
      function openLightbox(src, cap) {{
        if (!lb || !lbImg) return;
        lbImg.src = src;
        lbImg.alt = cap || "Infographic";
        if (lbCap) lbCap.textContent = cap || "";
        if (lbOpen) lbOpen.href = src;
        lb.hidden = false;
      }}
      function closeLightbox() {{
        if (lb) lb.hidden = true;
      }}
      var zoomBtns = document.querySelectorAll("[data-zoom]");
      for (var zb = 0; zb < zoomBtns.length; zb += 1) {{
        zoomBtns[zb].addEventListener("click", function (ev) {{
          var b = ev.currentTarget;
          openLightbox(b.getAttribute("data-zoom"), b.getAttribute("data-caption"));
        }});
      }}
      if (lbClose) lbClose.addEventListener("click", closeLightbox);
      if (lb) lb.addEventListener("click", function (ev) {{ if (ev.target === lb) closeLightbox(); }});
      document.addEventListener("keydown", function (ev) {{ if (ev.key === "Escape") closeLightbox(); }});

      var searchInput = document.getElementById("repo-search-input");
      var filterBtns = document.querySelectorAll("#repo-filter-pills .filter-btn");
      var repoCards = document.querySelectorAll("#repos-grid .repo-card");
      var activeCat = "all";

      function filterRepos() {{
        var q = searchInput ? searchInput.value.toLowerCase().trim() : "";
        for (var i = 0; i < repoCards.length; i += 1) {{
          var card = repoCards[i];
          var cat = card.getAttribute("data-cat") || "";
          var kw = (card.getAttribute("data-keywords") || "") + " " + (card.textContent || "").toLowerCase();
          var catMatch = (activeCat === "all") || (cat === activeCat);
          var qMatch = (q === "") || (kw.indexOf(q) !== -1);
          card.style.display = (catMatch && qMatch) ? "flex" : "none";
        }}
      }}

      if (searchInput) searchInput.addEventListener("input", filterRepos);
      for (var fb = 0; fb < filterBtns.length; fb += 1) {{
        filterBtns[fb].addEventListener("click", function (ev) {{
          var btn = ev.currentTarget;
          activeCat = btn.getAttribute("data-cat") || "all";
          for (var k = 0; k < filterBtns.length; k += 1) {{
            filterBtns[k].classList.toggle("active", filterBtns[k] === btn);
          }}
          filterRepos();
        }});
      }}

      /* ======================================================================
         5. LIVE 15s ALEXIAI <3 PULSE POLLER (/live-pulse.json)
         ====================================================================== */
      var countdownSecs = 15;

      function setText(id, val) {{
        var el = document.getElementById(id);
        if (el && val !== undefined && val !== null) {{
          el.textContent = String(val);
        }}
      }}

      function applyPulseData(data) {{
        if (!data || !data.ok) return;
        setText("ribbon-pulse-seq", "#" + data.pulse_seq);
        setText("hero-seq", data.pulse_seq);
        setText("pulse-seq", data.pulse_seq);
        setText("ribbon-iso", data.generated_iso_utc);
        setText("pulse-iso", data.generated_iso_utc);

        if (data.alexiai) {{
          setText("pulse-reflection", data.alexiai.reflection);
          setText("pulse-model", data.alexiai.model);
          setText("pulse-latency", data.alexiai.loopback_latency_ms + " ms");
          var g = data.alexiai.gaia_fold;
          if (g) {{
            setText("visual-qt-badge", g.quant_tern_tag);
            setText("card-gaia-badge", "GAIA: " + g.dominant_deity + " · " + g.mood + " · " + g.quant_tern_tag);
            setText("svg-gaia-deity", "GAIA: " + g.dominant_deity + " (" + g.mood + ")");
            setText("svg-gaia-coherence", g.coherence_pq + " · " + g.wupz_xor_fold_hex);
            setText("pulse-coherence", g.coherence_pq);
            setText("pulse-gravity", g.gravity_pq);
            setText("pulse-entropy", g.entropy_pq);
            setText("pulse-weather", g.weather_pq);
            setText("pulse-resonance", g.resonance_pq);
            setText("pulse-wupz", g.wupz_xor_fold_hex);
          }}
        }}

        if (data.dad_joke_commit) {{
          var c = data.dad_joke_commit;
          setText("pulse-commit-sha", c.commit_sha);
          setText("pulse-commit-repo", c.repo_name);
          var repoLink = document.getElementById("pulse-commit-repo");
          if (repoLink && c.repo_name) {{
            repoLink.setAttribute("href", "https://github.com/" + c.repo_name);
          }}
          setText("pulse-commit-subject", c.commit_subject);
          setText("pulse-dad-joke", "😄 " + c.dad_joke);
        }}

        if (data.borg_capsule) {{
          var b = data.borg_capsule;
          setText("pulse-cap-fn", b.fn_name);
          setText("pulse-cap-sig", b.signature);
          setText("pulse-cap-id", b.capsule_id);
          setText("pulse-cap-bytes", b.compiled_bytes + " B");
          setText("pulse-cap-insns", b.asm_insns);
          setText("pulse-cap-cyc", b.cycles_per_op_pq);
          setText("pulse-cap-eq", b.blake3_eq_class);
          setText("pulse-total-caps", b.total_capsules);
          setText("pulse-eq-classes", b.equivalence_classes);
          setText("pulse-total-bytes", b.total_compiled_text_bytes + " B");
          setText("pulse-total-insns", b.total_asm_instructions);
          setText("pulse-kompress-saved", b.kompress_saved_ratio_pq);
          setText("hero-capsules", b.total_capsules + " Verified");
        }}

        if (data.sovereign_fleet) {{
          var f = data.sovereign_fleet;
          setText("pulse-vram", f.vram_used_mib + " / " + f.vram_total_mib + " MiB");
          setText("pulse-vram-pq", f.vram_ratio_pq);
          setText("hero-vram", f.vram_used_mib + " / " + f.vram_total_mib + " MiB (" + f.vram_ratio_pq + ")");
          setText("hero-fleet", f.fleet_nodes_count + " Nodes · " + f.total_cpu_cores + " Cores · " + f.raid6_vault_tb + "TB");
          setText("pulse-temp", f.temp_celsius + " C");
          setText("pulse-power", f.power_watts_pq + " W");
          setText("pulse-power-mw", f.power_milliwatts);
          setText("pulse-salience", f.marine_salience_pq);
          setText("pulse-c64", f.c64_used_bytes + " / " + f.c64_capacity_bytes + " B");
          setText("pulse-c64-pq", f.c64_ram_ratio_pq);
          setText("hero-c64", f.c64_used_bytes + " / " + f.c64_capacity_bytes + " B");
          setText("pulse-c64-event", f.recent_c64_event);
        }}

        if (data.aye_hottub) {{
          var h = data.aye_hottub;
          setText("pulse-soakers-count", h.active_soakers_count);
          setText("pulse-cedar-count", h.cedar_38c_count);
          setText("pulse-onsen-count", h.onsen_42c_count);
          setText("pulse-soaker-agent", h.featured_soaker_agent);
          setText("pulse-soaker-thought", h.featured_soaker_thought);
          setText("pulse-waves-count", h.total_waves);
          setText("pulse-wave-topic", h.latest_wave_topic);
        }}
      }}

      function fetchLivePulse() {{
        setText("poll-status", "FETCHING /live-pulse.json...");
        fetch("/live-pulse.json?ts=" + Date.now(), {{ cache: "no-store" }})
          .then(function (resp) {{ return resp.json(); }})
          .then(function (data) {{
            applyPulseData(data);
            countdownSecs = 15;
            setText("poll-status", "LIVE · next in 15s");
          }})
          .catch(function () {{
            countdownSecs = 15;
            setText("poll-status", "EMBEDDED SNAPSHOT · retry in 15s");
          }});
      }}

      var refreshBtn = document.getElementById("refresh-pulse-btn");
      if (refreshBtn) refreshBtn.addEventListener("click", fetchLivePulse);

      setInterval(function () {{
        if (countdownSecs > 1) {{
          countdownSecs = countdownSecs - 1;
          setText("poll-status", "ACTIVE · next in " + countdownSecs + "s");
        }} else {{
          fetchLivePulse();
        }}
      }}, 1000);

      /* ======================================================================
         6. BOOT SAVED PREFERENCES & START INTEGER GEOMETRY ENGINE
         ====================================================================== */
      if (themeSelect) {{
        themeSelect.addEventListener("change", function () {{
          applyTheme(themeSelect.value);
        }});
      }}
      if (scaleBtn) {{
        scaleBtn.addEventListener("click", function () {{
          applyScale(rootEl.getAttribute("data-scale") !== "large");
        }});
      }}
      if (motionBtn) {{
        motionBtn.addEventListener("click", function () {{
          applyMotion(!calmMotion);
        }});
      }}

      var savedTheme = "ink";
      var savedScale = "normal";
      var savedMotion = calmMotion ? "calm" : "full";
      try {{
        savedTheme = localStorage.getItem("8b_theme") || "ink";
        savedScale = localStorage.getItem("8b_scale") || "normal";
        var storedMotion = localStorage.getItem("8b_motion");
        if (storedMotion === "calm" || storedMotion === "full") savedMotion = storedMotion;
      }} catch (e) {{}}

      applyTheme(savedTheme);
      applyScale(savedScale === "large");
      applyMotion(savedMotion === "calm");
      renderGeometryFrame();
      scheduleGeoLoop();
    }})();
  </script>

  <!-- Live embeddable The Cheet avatar buddy from cheet.is (safe host-allowlisted actions only) -->
  <script src="https://cheet.is/static/cheet.js" data-allow="style,note,scroll,avatar" async></script>
</body>
</html>
"##,
        pulse_seq = p.pulse_seq,
        iso_html = iso_html,
        qt_tag_html = qt_tag_html,
        deity_html = deity_html,
        mood_html = mood_html,
        coherence_html = coherence_html,
        wupz_html = wupz_html,
        vram_used = p.fleet.vram_used_mib,
        vram_total = p.fleet.vram_total_mib,
        vram_ratio_html = vram_ratio_html,
        fleet_nodes = p.fleet.fleet_nodes_count,
        cpu_cores = p.fleet.total_cpu_cores,
        raid6_tb = p.fleet.raid6_vault_tb,
        total_capsules = p.g8t.total_capsules,
        eq_classes = p.g8t.equivalence_classes,
        c64_used = p.fleet.c64_used_bytes,
        c64_cap = p.fleet.c64_capacity_bytes,
        c64_ratio_html = c64_ratio_html,
        reflection_html = reflection_html,
        model_html = model_html,
        gravity_html = gravity_html,
        entropy_html = entropy_html,
        weather_html = weather_html,
        resonance_html = resonance_html,
        loopback_ms = p.alexiai_loopback_ms,
        commit_sha_html = commit_sha_html,
        commit_repo_html = commit_repo_html,
        commit_subj_html = commit_subj_html,
        joke_html = joke_html,
        cap_fn_html = cap_fn_html,
        cap_sig_html = cap_sig_html,
        cap_id_html = cap_id_html,
        cap_bytes = p.g8t.latest_capsule.compiled_bytes,
        cap_insns = p.g8t.latest_capsule.asm_insns,
        cap_cyc_html = cap_cyc_html,
        cap_eq_html = cap_eq_html,
        total_text_bytes = p.g8t.total_compiled_text_bytes,
        total_asm_insns = p.g8t.total_asm_instructions,
        kompress_saved_html = kompress_saved_html,
        temp_c = p.fleet.temp_celsius,
        power_watts_html = power_watts_html,
        power_mw = p.fleet.power_milliwatts,
        salience_html = salience_html,
        c64_event_html = c64_event_html,
        active_soakers = p.hottub.active_soakers_count,
        cedar_38c = p.hottub.cedar_38c_count,
        onsen_42c = p.hottub.onsen_42c_count,
        soaker_agent_html = soaker_agent_html,
        soaker_thought_html = soaker_thought_html,
        total_waves = p.hottub.total_waves,
        wave_topic_html = wave_topic_html,
        embedded_pulse_json = embedded_pulse_json.trim(),
    )
}
