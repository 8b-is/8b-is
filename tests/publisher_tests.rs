#![forbid(unsafe_code)]

use alexiai_8b_publisher::*;

#[test]
fn test_edge_cases_sanitizer_and_float_detectors() {
    // Empty input uses sanitized fallback
    let empty_clean = sanitize_zero_float_text("", "Fallback 1.5 value.");
    assert_eq!(count_floating_point_literals(&empty_clean), 0);
    assert_eq!(count_strict_digit_dot_digit(&empty_clean), 0);

    // Unclosed <think> block is stripped cleanly
    let unclosed_think = "Valid prefix 2.5 <think>internal float 3.14159";
    let clean_think = sanitize_zero_float_text(unclosed_think, "fallback");
    assert_eq!(count_floating_point_literals(&clean_think), 0);
    assert_eq!(count_strict_digit_dot_digit(&clean_think), 0);
    assert!(clean_think.contains("5/2"));
    assert!(!clean_think.contains("3.14"));

    // Closed <think>...</think> block is stripped cleanly
    let closed_think = "<think>0.999</think> Saliency is 0.75 on node 49.13.101.247.";
    let clean_closed = sanitize_zero_float_text(closed_think, "fallback");
    assert_eq!(count_floating_point_literals(&clean_closed), 0);
    assert_eq!(count_strict_digit_dot_digit(&clean_closed), 0);
    assert!(clean_closed.contains("3/4"));
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
}

#[test]
fn test_iso_utc_and_rational_boundaries() {
    assert_eq!(format_epoch_iso_utc(0), "1970-01-01T00:00:00Z");
    assert_eq!(Rational::new(0, 0), Rational { p: 0, q: 1 });
    assert_eq!(Rational::new(-4, -8), Rational { p: 1, q: 2 });
    assert_eq!(Rational::new(4, -8), Rational { p: -1, q: 2 });
}
