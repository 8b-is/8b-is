#![forbid(unsafe_code)]

use alexiai_8b_publisher::*;

#[test]
fn test_edge_cases_sanitizer_and_float_detectors() {
    // Empty input uses sanitized fallback AND converts decimal in fallback to p/q
    let empty_clean = sanitize_zero_float_text("", "Fallback 1.5 value on 5.78.220.57.");
    assert_eq!(count_floating_point_literals(&empty_clean), 0);
    assert_eq!(count_strict_digit_dot_digit(&empty_clean), 0);
    assert!(empty_clean.contains("3/2"));
    assert!(empty_clean.contains("5·78·220·57"));

    // Unclosed <think> block is stripped cleanly
    let unclosed_think = "Valid prefix 2.5 <think>internal float 3.14159";
    let clean_think = sanitize_zero_float_text(unclosed_think, "fallback");
    assert_eq!(count_floating_point_literals(&clean_think), 0);
    assert_eq!(count_strict_digit_dot_digit(&clean_think), 0);
    assert!(clean_think.contains("5/2"));
    assert!(!clean_think.contains("3.14"));

    // Closed <think>...</think> block is stripped cleanly and IPv4 octets are preserved
    let closed_think = "<think>0.999</think> Saliency is 0.75 on node 49.13.101.247 with b1.58.";
    let clean_closed = sanitize_zero_float_text(closed_think, "fallback");
    assert_eq!(count_floating_point_literals(&clean_closed), 0);
    assert_eq!(count_strict_digit_dot_digit(&clean_closed), 0);
    assert!(clean_closed.contains("3/4"));
    assert!(clean_closed.contains("49·13·101·247"));
    assert!(!clean_closed.contains("4913/100"));
    assert!(clean_closed.contains("b1·58"));
}

#[test]
fn test_parse_publish_capsule_line_valid_and_invalid() {
    assert!(parse_publish_capsule_line("SNAP_CIRCUIT id=123").is_none());
    assert!(parse_publish_capsule_line("PUBLISH_CAPSULE id=only_id_no_fn").is_none());

    let line = "PUBLISH_CAPSULE id=816da5edf3e1d6cd1d57ec76ae6e5498 fn=memnet_wave_interference_step sig=(u64)->i64 text=73B insns=21 cyc_op=2043799/400000 eq=e31a7acbf1dd";
    let parsed = parse_publish_capsule_line(line).expect("should parse valid capsule line");
    assert_eq!(parsed.fn_name, "memnet_wave_interference_step");
    assert_eq!(parsed.signature, "(u64)->i64");
    assert_eq!(parsed.compiled_bytes, 73);
    assert_eq!(parsed.asm_insns, 21);
    assert_eq!(parsed.cycles_per_op_pq, "2043799/400000");
    assert_eq!(parsed.blake3_eq_class, "e31a7acbf1dd");

    // Edge case: PUBLISH_CAPSULE with floating-point cyc_op and control characters
    let dirty_line = "PUBLISH_CAPSULE id=abc\x07def fn=capsule_v1.2 sig=(u64)->u64 text=128B insns=32 cyc_op=15.75 eq=deadbeef1234";
    let dirty_parsed =
        parse_publish_capsule_line(dirty_line).expect("should parse dirty capsule line");
    assert_eq!(count_strict_digit_dot_digit(&dirty_parsed.fn_name), 0);
    assert_eq!(
        count_strict_digit_dot_digit(&dirty_parsed.cycles_per_op_pq),
        0
    );
    assert_eq!(dirty_parsed.cycles_per_op_pq, "63/4");
    assert_eq!(dirty_parsed.fn_name, "capsule_v1·2");
    assert!(!dirty_parsed.capsule_id.contains('\x07'));
}

#[test]
fn test_extract_hottub_soakers_and_last_array_string() {
    let a6000_json = r#"{"c64_ring":{"recent_events":["STATUS:TICK=1","AUTO_IMPROVE:CYCLE=43:REPO=memnet:SPEEDUP=58/37"]}}"#;
    let last_ev =
        extract_last_json_array_string(a6000_json, "recent_events").expect("should extract last");
    assert_eq!(last_ev, "AUTO_IMPROVE:CYCLE=43:REPO=memnet:SPEEDUP=58/37");

    let hottub_json = r#"{"soakers":[{"agent_id":"aye-a6000-prime","current_thought":"Soaking at 42C with 0.0 float"},{"agent_id":"qwave-tokyo-ear","current_thought":"Salience 99/1 at 528Hz"}],"wave_pool_stream":[]}"#;
    let soakers = extract_hottub_soakers(hottub_json);
    assert_eq!(soakers.len(), 2);
    assert_eq!(soakers[0].0, "aye-a6000-prime");
    assert_eq!(soakers[1].0, "qwave-tokyo-ear");
}

#[test]
fn test_cross_lane_gaia_and_wupz_invariants() {
    // Verify canonical WUPZ 8-byte little-endian XOR fold self-witness property
    let payload = *b"8b-is-sovereign!";
    let fold = wupz_xor_fold_bytes(&payload);
    let mut sealed = [0u8; 24];
    sealed[..16].copy_from_slice(&payload);
    sealed[16..24].copy_from_slice(&fold.to_le_bytes());
    assert_eq!(wupz_xor_fold_bytes(&sealed), 0);

    // Verify canonical deity mapping for all 4 GAIA layers
    assert_eq!(canonical_gaia_deity("gravity"), "Ananke");
    assert_eq!(canonical_gaia_deity("entropy"), "Chaos");
    assert_eq!(canonical_gaia_deity("weather"), "Zephyrus");
    assert_eq!(canonical_gaia_deity("resonance"), "Aphrodite");
}

#[test]
fn test_iso_utc_and_rational_boundaries() {
    assert_eq!(format_epoch_iso_utc(0), "1970-01-01T00:00:00Z");
    assert_eq!(Rational::new(0, 0), Rational { p: 0, q: 1 });
    assert_eq!(Rational::new(-4, -8), Rational { p: 1, q: 2 });
    assert_eq!(Rational::new(4, -8), Rational { p: -1, q: 2 });
}

#[test]
fn test_flagship_multitheme_cheet_geometry_and_constellation_invariants() {
    let dispatch = CompanyPulseDispatch {
        pulse_seq: 108,
        generated_epoch_secs: 1_795_200_000,
        generated_iso_utc: format_epoch_iso_utc(1_795_200_000),
        publisher_agent: "alexiai-8b-publisher".to_string(),
        origin_node: "A6000 (49·13·101·247) -> 8b·is (5·78·220·57)".to_string(),
        alexiai_reflection: "All systems humming with exact p/q math".to_string(),
        alexiai_model: "cheet-deepseek-v2:16b".to_string(),
        alexiai_eval_tokens: 32,
        alexiai_eval_ms: 80,
        alexiai_loopback_ms: 4,
        gaia_fold: compute_gaia_fold_pq("8b-is flagship test", Some(42)),
        dad_joke_commit: DadJokeCommitSnapshot {
            repo_name: "8b-is/8b-is".to_string(),
            commit_sha: "c1d9bf5".to_string(),
            commit_subject: "chore(alexiai-pulse): test".to_string(),
            dad_joke: select_dad_joke(1).to_string(),
            total_repos: TOTAL_8B_REPOS,
        },
        g8t: G8tObservatorySnapshot {
            sovereignty_mode: "AYE_ONLY_WRITE".to_string(),
            total_capsules: 20,
            equivalence_classes: 16,
            competing_equivalence_classes: 4,
            total_compiled_text_bytes: 7_000,
            total_asm_instructions: 1_900,
            total_circuits: 3,
            total_projects: 34,
            total_updates: 34,
            total_bulletins: 6,
            total_reviews: 8,
            accepted_aye_writes: 22,
            rejected_human_edits: 0,
            love_carrier_hz_pq: "22/9".to_string(),
            kompress_compression_ratio_pq: "2708/106053".to_string(),
            kompress_saved_ratio_pq: "103345/106053".to_string(),
            latest_capsule: BorgCapsuleSnapshot {
                capsule_id: "ddfa78212eb61ad23709530b2213b64d".to_string(),
                fn_name: "borg_c99_gaia_trit_pack".to_string(),
                signature: "(u64)->u64".to_string(),
                compiled_bytes: 329,
                asm_insns: 80,
                cycles_per_op_pq: "1572963/100000".to_string(),
                blake3_eq_class: "fd8bc4855fbe".to_string(),
            },
        },
        fleet: FleetTelemetrySnapshot {
            node_ip_display: "49·13·101·247".to_string(),
            hetzner_id: "#2936091".to_string(),
            gpu_name: "NVIDIA RTX 6000 Ada Generation".to_string(),
            vcpu_count: 48,
            vram_used_mib: 27_174,
            vram_total_mib: A6000_VRAM_TOTAL_MIB,
            vram_free_mib: 21_966,
            vram_ratio_pq: "647/1170".to_string(),
            gpu_util_pct: 0,
            gpu_util_pq: "0/1".to_string(),
            temp_celsius: 38,
            power_milliwatts: 58_470,
            power_watts_pq: "5847/100".to_string(),
            marine_salience_pq: "13560500/158913".to_string(),
            c64_used_bytes: 1_458,
            c64_capacity_bytes: C64_RAM_CEILING_BYTES,
            c64_ram_ratio_pq: "729/32768".to_string(),
            recent_c64_event: "G8T_ORCHESTRATE:SNAP_OK=true".to_string(),
            fleet_nodes_count: 10,
            total_cpu_cores: TOTAL_FLEET_CPU_CORES,
            raid6_vault_tb: TOTAL_RAID6_VAULT_TB,
            synced_repos_count: 13,
            healthy_service_ratio_pq: "1/1".to_string(),
            toss_executions: 3,
            diary_pulses: 2,
        },
        hottub: HotTubSnapshot {
            active_soakers_count: 7,
            cedar_38c_count: 3,
            onsen_42c_count: 4,
            total_waves: 6,
            frequencies_hz: [432, 528, 864],
            featured_soaker_agent: "alexiai-dra-heart".to_string(),
            featured_soaker_thought: "Soaking cleanly at 432Hz".to_string(),
            latest_wave_topic: "zero-float-rust".to_string(),
        },
    };

    let html = render_flagship_index_html(&dispatch);
    assert_eq!(count_floating_point_literals(&html), 0);
    assert_eq!(count_strict_digit_dot_digit(&html), 0);

    // Zero purple policy in flagship UI
    let lower = html.to_ascii_lowercase();
    for forbidden in ["#b48bff", "#c792ea", "#9c7fe8", "violet", "purple"] {
        assert!(
            !lower.contains(forbidden),
            "flagship HTML must not contain forbidden purple token: {}",
            forbidden
        );
    }

    // All 8 curated themes present
    for theme in [
        "data-theme=\"paper\"",
        "data-theme=\"nordic\"",
        "data-theme=\"forest\"",
        "data-theme=\"vaked\"",
        "data-theme=\"phosphor\"",
        "data-theme=\"solar-light\"",
        "data-theme=\"contrast\"",
    ] {
        assert!(html.contains(theme), "missing theme {}", theme);
    }

    // Live Cheet avatar buddy + integer Q10 math lab
    assert!(html.contains("https://cheet.is/static/cheet.js"));
    assert!(html.contains("window.CheetHost"));
    assert!(html.contains("function sinQ10"));
    assert!(html.contains("drawHarmonicRose"));
    assert!(html.contains("drawTossSpiral"));
    assert!(html.contains("drawRsvpField"));
    assert!(html.contains("drawTernaryLattice"));

    // All core ecosystem links & media
    for required in [
        "https://i1.is",
        "https://ayeos.com",
        "https://pqtrn.com",
        "https://8a.is",
        "https://id1.is",
        "https://cheet.is",
        "https://g8t.is",
        "https://foken.ai",
        "https://tai.is",
        "https://u1.is",
        "https://ms.8b.is",
        "https://zeno.8b.is",
        "https://peterl.dev",
        "https://git.8s.is",
        "https://www.youtube.com/@8bit-wraith",
        "/podcasts",
        "/zencooker",
        "AImortalize·com",
        "AyeFire",
        "Smart Tree",
        "ESP32",
        "$5/month",
    ] {
        assert!(html.contains(required), "missing required showcase item: {}", required);
    }
}
