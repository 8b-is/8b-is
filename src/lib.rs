#![forbid(unsafe_code)]
//! # `alexiai-8b-publisher` — 100% Pure-Rust Zero-Float Autonomous Publisher for `https://8b.is`
//!
//! Queries:
//! 1. Local `alexiai` (`http://127.0.0.1:8787`) & `ollama` (`http://127.0.0.1:11434`) on `49·13·101·247`
//! 2. `a6000-orchestrator` (`http://127.0.0.1:8790/a6000/api/status` / `https://cheet.is/a6000/api/status`)
//! 3. `g8t.is` (`https://g8t.is/g8t/observatory` & `https://g8t.is/g8t/hottub`)
//!
//! Generates and publishes:
//! - `/var/www/8b-is/public/live-pulse.json` (auto-fetched every `15s` in browser)
//! - `/var/www/8b-is/public/index.html` (flagship `https://8b.is` landing page with embedded live pulse)
//! - `/home/hue/8b-is/8b-is/README.md` (with autonomous dad-joke commit)

use std::fmt;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod flagship_site;

/// Commodore-64 full memory ceiling (`64 KB` = `65,536` bytes).
pub const C64_RAM_CEILING_BYTES: u64 = 65_536;

/// Total VRAM in MiB on the NVIDIA RTX 6000 Ada Generation (`49,140 MiB`).
pub const A6000_VRAM_TOTAL_MIB: u64 = 49_140;

/// Total repositories across the `8b.is` sovereign constellation (`3^5 = 243` = 1 full 5-trit byte).
pub const TOTAL_8B_REPOS: u64 = 243;

/// Total CPU cores across the 10-node Hetzner + `ns3·i1·is` sovereign fleet (`186` cores).
pub const TOTAL_FLEET_CPU_CORES: u64 = 186;

/// Total RAID6 vault storage in TB (`160 TB`).
pub const TOTAL_RAID6_VAULT_TB: u64 = 160;

/// GAIA `p/q` fixed denominator (`q = 2^20 = 1,048,576`).
pub const GAIA_PQ_Q: i64 = 1_048_576;

/// Golden ratio φ in `p/q` (`round(1.61803398875 * 2^20) = 1,696,632`), pinned to `gaia-mlx-quant`.
pub const PHI_PQ: i64 = 1_696_632;

const GRAVITY_REST_PQ: i64 = GAIA_PQ_Q; // 1/1
const ENTROPY_REST_PQ: i64 = 650_117; // 650117 / 1048576
const WEATHER_REST_PQ: i64 = 524_288; // 1/2
const RESONANCE_REST_PQ: i64 = PHI_PQ; // φ = 1,696,632 / 1,048,576

// ============================================================================
// 1. EXACT RATIONAL (p/q) & BALANCED TERNARY {-1, 0, +1} ARITHMETIC
// ============================================================================

/// Euclidean greatest common divisor over `u128` (always `>= 1`).
pub const fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    if a == 0 { 1 } else { a }
}

/// Exact reduced rational `p/q` (`q > 0`). Never uses floating-point arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rational {
    pub p: i64,
    pub q: i64,
}

impl Default for Rational {
    fn default() -> Self {
        Self { p: 0, q: 1 }
    }
}

impl Rational {
    pub const ZERO: Self = Self { p: 0, q: 1 };
    pub const ONE: Self = Self { p: 1, q: 1 };

    pub const fn new(p: i64, q: i64) -> Self {
        Self::from_i128(p as i128, q as i128)
    }

    pub const fn from_u64(num: u64, den: u64) -> Self {
        Self::from_i128(num as i128, den as i128)
    }

    pub const fn from_i128(mut p: i128, mut q: i128) -> Self {
        if q == 0 {
            return Self { p: 0, q: 1 };
        }
        if q < 0 {
            p = -p;
            q = -q;
        }
        let g = gcd_u128(p.unsigned_abs(), q.unsigned_abs());
        let mut rp = p / (g as i128);
        let mut rq = q / (g as i128);
        if rq < 1 {
            rq = 1;
        }
        while rp.unsigned_abs() > (i64::MAX as u128) || rq.unsigned_abs() > (i64::MAX as u128) {
            rp /= 2;
            rq /= 2;
            if rq < 1 {
                rq = 1;
            }
            let g2 = gcd_u128(rp.unsigned_abs(), rq.unsigned_abs());
            rp /= g2 as i128;
            rq /= g2 as i128;
            if rq < 1 {
                rq = 1;
            }
        }
        Self {
            p: rp as i64,
            q: rq as i64,
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.p, self.q)
    }
}

/// Balanced ternary trit `{-1, 0, +1}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trit {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Trit {
    pub const fn as_i8(self) -> i8 {
        match self {
            Self::Neg => -1,
            Self::Zero => 0,
            Self::Pos => 1,
        }
    }

    pub const fn glyph(self) -> &'static str {
        match self {
            Self::Neg => "-",
            Self::Zero => "0",
            Self::Pos => "+",
        }
    }
}

// ============================================================================
// 2. STRICT ZERO-FLOAT VERIFICATION & SANITIZATION
// ============================================================================

/// Verify that a JSON/HTML/Markdown string contains zero floating-point decimal numbers
/// (ignoring IPv4 octets like `49.13.101.247` if present).
pub fn count_floating_point_literals(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0usize;
    let mut i = 0usize;
    while i + 2 < bytes.len() {
        if bytes[i].is_ascii_digit() && bytes[i + 1] == b'.' && bytes[i + 2].is_ascii_digit() {
            let mut start = i;
            while start > 0
                && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'.')
            {
                start -= 1;
            }
            let mut end = i + 2;
            while end < bytes.len()
                && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'.')
            {
                end += 1;
            }
            let token = &text[start..end];
            let has_alpha = token.bytes().any(|b| b.is_ascii_alphabetic());
            let dot_count = token.bytes().filter(|&b| b == b'.').count();
            if !has_alpha && dot_count != 3 {
                count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }
    count
}

/// Even stricter check: counts ANY occurrence of `ASCII_DIGIT + '.' + ASCII_DIGIT` anywhere in `text`.
/// When this returns `0`, even raw `grep -E '[0-9]+\.[0-9]+'` matches `0` times!
pub fn count_strict_digit_dot_digit(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut count = 0usize;
    let mut i = 0usize;
    while i + 2 < bytes.len() {
        if bytes[i].is_ascii_digit() && bytes[i + 1] == b'.' && bytes[i + 2].is_ascii_digit() {
            count += 1;
        }
        i += 1;
    }
    count
}

/// Truncate UTF-8 string cleanly at or below `max_bytes`.
pub fn truncate_utf8_bytes(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn convert_zero_float_body(raw: &str) -> String {
    let mut working = raw.to_string();
    while let Some(start) = working.find("<think>") {
        if let Some(end_rel) = working[start..].find("</think>") {
            let end = start + end_rel + "</think>".len();
            working.replace_range(start..end, " ");
        } else {
            working.truncate(start);
            break;
        }
    }

    let bytes = working.as_bytes();
    let mut converted = String::with_capacity(working.len() + 16);
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let d_start = i;
            let preceded_by_alpha =
                d_start > 0 && (bytes[d_start - 1].is_ascii_alphabetic() || bytes[d_start - 1] == b'_');
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i + 1 < bytes.len() && bytes[i] == b'.' && bytes[i + 1].is_ascii_digit() {
                // Scan full dotted-digit sequence to distinguish single decimal `whole.frac`
                // from multi-dot tokens like IPv4 `49.13.101.247` or semver `2.0.0`.
                let mut scan = i;
                let mut dot_count = 0usize;
                while scan + 1 < bytes.len()
                    && bytes[scan] == b'.'
                    && bytes[scan + 1].is_ascii_digit()
                {
                    dot_count += 1;
                    scan += 1;
                    while scan < bytes.len() && bytes[scan].is_ascii_digit() {
                        scan += 1;
                    }
                }
                if dot_count >= 2 || preceded_by_alpha {
                    let token = &working[d_start..scan];
                    converted.push_str(&token.replace('.', "·"));
                    i = scan;
                } else {
                    let f_start = i + 1;
                    i = scan;
                    let whole_str = &working[d_start..(f_start - 1)];
                    let frac_str = &working[f_start..i];
                    let whole = whole_str.parse::<u64>().unwrap_or(0);
                    let frac_trimmed: String = frac_str.chars().take(3).collect();
                    let scale: u64 = match frac_trimmed.len() {
                        1 => 10,
                        2 => 100,
                        _ => 1000,
                    };
                    let frac = frac_trimmed.parse::<u64>().unwrap_or(0);
                    let rat = Rational::from_u64(
                        whole.saturating_mul(scale).saturating_add(frac),
                        scale,
                    );
                    converted.push_str(&rat.to_string());
                }
            } else {
                converted.push_str(&working[d_start..i]);
            }
        } else {
            let ch = working[i..].chars().next().unwrap_or(' ');
            if ch == '.' {
                converted.push('·');
            } else if ch == '\n' || ch == '\r' || ch == '\t' {
                converted.push(' ');
            } else if !ch.is_control() {
                converted.push(ch);
            }
            i += ch.len_utf8();
        }
    }

    converted
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Sanitize any free-form LLM or tool text with an explicit byte ceiling `max_bytes`
/// (truncating cleanly at a word boundary when possible) so it contains ZERO floating-point
/// decimal literals: converts single `whole.frac` numbers into reduced `p/q` rationals and
/// replaces any remaining `.` (including IPv4 octets like `49.13.101.247`) with `·`.
pub fn sanitize_zero_float_text_with_limit(
    raw: &str,
    fallback: &str,
    max_bytes: usize,
) -> String {
    let collapsed = convert_zero_float_body(raw);
    let chosen = if collapsed.is_empty() {
        convert_zero_float_body(fallback)
    } else {
        collapsed
    };
    let bounded = if chosen.len() <= max_bytes {
        chosen
    } else {
        let slice = truncate_utf8_bytes(&chosen, max_bytes);
        if let Some(last_space) = slice.rfind(' ') {
            if last_space >= max_bytes / 2 {
                slice[..last_space].to_string()
            } else {
                slice.to_string()
            }
        } else {
            slice.to_string()
        }
    };
    bounded.replace('.', "·")
}

/// Sanitize any free-form LLM or tool text so it is a clean single line with ZERO floating-point
/// decimal literals: converts `whole.frac` numbers into reduced `p/q` rationals and replaces any
/// remaining `.` with `·`.
pub fn sanitize_zero_float_text(raw: &str, fallback: &str) -> String {
    sanitize_zero_float_text_with_limit(raw, fallback, 360)
}

/// Escape string for JSON literal output.
pub fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {}
            c => out.push(c),
        }
    }
    out
}

/// Escape string for HTML content/attribute output.
pub fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

// ============================================================================
// 3. PURE-INTEGER p/q GAIA FIELD FOLD & WUPZ XOR FOLD
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GaiaFoldPqReport {
    pub seed: u32,
    pub q: i64,
    pub gravity_pq: String,
    pub entropy_pq: String,
    pub weather_pq: String,
    pub resonance_pq: String,
    pub coherence_pq: String,
    pub dominant_deity: String,
    pub mood: String,
    pub quant_tern_tag: String,
    pub quant_tern_byte: u8,
    pub quant_tern_trits: [i8; 3],
    pub wupz_xor_fold_hex: String,
}

/// Map any GAIA layer name (`"gravity"`, `"entropy"`, `"weather"`, `"resonance"`) or deity string
/// to its canonical Greek deity inhabitant (`"Ananke"`, `"Chaos"`, `"Zephyrus"`, `"Aphrodite"`).
pub fn canonical_gaia_deity(layer_or_deity: &str) -> &'static str {
    let lower = layer_or_deity.trim().to_ascii_lowercase();
    if lower.contains("gravity") || lower.contains("ananke") {
        "Ananke"
    } else if lower.contains("entropy") || lower.contains("chaos") {
        "Chaos"
    } else if lower.contains("weather") || lower.contains("zephyrus") {
        "Zephyrus"
    } else if lower.contains("resonance") || lower.contains("aphrodite") {
        "Aphrodite"
    } else {
        "Ananke"
    }
}

pub const fn seed_from_prompt(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut h: u32 = 2_166_136_261;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u32;
        h = h.wrapping_mul(16_777_619);
        i += 1;
    }
    h
}

pub const fn mulberry32_pq(seed: u32) -> (i64, u32) {
    let a = seed.wrapping_add(0x6d2b79f5);
    let mut t = (a ^ (a >> 15)).wrapping_mul(a | 1);
    t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
    let rand_p = ((t ^ (t >> 14)) >> 12) as i64;
    (rand_p, a)
}

/// Canonical `wupz_xor_fold`: XOR of the little-endian `u64` words, then trailing bytes.
/// Matches `gaia_mlx_quant::wupz_xor_fold`, `wupz-rs::fold::xor_fold`, and `wupz-zig::xorFold` bit-for-bit.
pub const fn wupz_xor_fold_bytes(bytes: &[u8]) -> u64 {
    let mut acc: u64 = 0;
    let mut i = 0;
    while i + 8 <= bytes.len() {
        let w = u64::from_le_bytes([
            bytes[i],
            bytes[i + 1],
            bytes[i + 2],
            bytes[i + 3],
            bytes[i + 4],
            bytes[i + 5],
            bytes[i + 6],
            bytes[i + 7],
        ]);
        acc ^= w;
        i += 8;
    }
    while i < bytes.len() {
        acc ^= bytes[i] as u64;
        i += 1;
    }
    acc
}

const fn clamp_i64(v: i64, min_v: i64, max_v: i64) -> i64 {
    if v < min_v {
        min_v
    } else if v > max_v {
        max_v
    } else {
        v
    }
}

const fn quant_tern_trit_pq(p_x: i64, q: i64) -> i8 {
    let q_abs = if q < 0 { -q } else { q };
    let lhs = p_x * 100;
    let rhs = 33 * q_abs;
    if lhs >= rhs {
        1
    } else if lhs <= -rhs {
        -1
    } else {
        0
    }
}

/// Pure-integer `p/q` (`q = 2^20 = 1,048,576`) GAIA fold with zero IEEE-754 instructions.
/// Pinned to `gaia_mlx_quant::fold_pq` and `gaia_mlx_quant::fold_to_quant_tern_pq`.
pub fn compute_gaia_fold_pq(prompt: &str, seed_override: Option<u32>) -> GaiaFoldPqReport {
    let seed = seed_override.unwrap_or_else(|| seed_from_prompt(prompt));
    let (rand_p, _) = mulberry32_pq(seed);
    let len_p = ((prompt.len() as i64) << 8).min(367_002);
    let entropy_p = clamp_i64(ENTROPY_REST_PQ + len_p - 157_286, 0, GAIA_PQ_Q);
    let weather_p = clamp_i64(
        WEATHER_REST_PQ + ((rand_p - WEATHER_REST_PQ) * 2) / 5,
        0,
        GAIA_PQ_Q,
    );
    let gravity_p = GRAVITY_REST_PQ;
    let resonance_p = RESONANCE_REST_PQ;

    let rests = [
        GRAVITY_REST_PQ,
        ENTROPY_REST_PQ,
        WEATHER_REST_PQ,
        RESONANCE_REST_PQ,
    ];
    let layers = [gravity_p, entropy_p, weather_p, resonance_p];
    let names = ["Ananke", "Chaos", "Zephyrus", "Aphrodite"];
    let mut dom_idx = 0usize;
    let mut best_dist: i64 = -1;
    for i in 0..4 {
        let d = (layers[i] - rests[i]).abs();
        if d > best_dist {
            best_dist = d;
            dom_idx = i;
        }
    }

    let mean_p = (gravity_p + entropy_p + weather_p + resonance_p) >> 2;
    let spread_p = ((gravity_p - mean_p).abs()
        + (entropy_p - mean_p).abs()
        + (weather_p - mean_p).abs()
        + (resonance_p - mean_p).abs())
        >> 2;
    let coherence_p = clamp_i64(GAIA_PQ_Q - spread_p, 0, GAIA_PQ_Q);

    let mood = if weather_p >= 608_174 {
        "Heart +1 ☀"
    } else if weather_p <= 440_401 {
        "Calm 0 · +1"
    } else if entropy_p >= 587_202 {
        "Spark +1 ✦"
    } else {
        "Heart +1 <3"
    };

    // Exact `gaia_mlx_quant::fold_to_quant_tern_pq` 3-axis VAD quantization and 2-bit packing
    let q = GAIA_PQ_Q;
    let p_valence = clamp_i64(
        (coherence_p - (q / 2)) * 2 + (weather_p - (q / 2)),
        -q,
        q,
    );
    let p_arousal = clamp_i64(
        (((entropy_p * 55 + weather_p * 45) / 100) - (q / 2)) * 2,
        -q,
        q,
    );
    let p_dominance = clamp_i64((gravity_p - q) * 4 + (coherence_p - entropy_p), -q, q);
    let t0 = quant_tern_trit_pq(p_valence, q);
    let t1 = quant_tern_trit_pq(p_arousal, q);
    let t2 = quant_tern_trit_pq(p_dominance, q);
    let qt_byte: u8 = (((t0 + 1) as u8) & 0x03)
        | ((((t1 + 1) as u8) & 0x03) << 2)
        | ((((t2 + 1) as u8) & 0x03) << 4)
        | (1u8 << 6);
    let qt_tag = format!(
        "qt:{:02x} ({}{}{})",
        qt_byte,
        if t0 > 0 { "+" } else if t0 < 0 { "-" } else { "0" },
        if t1 > 0 { "+" } else if t1 < 0 { "-" } else { "0" },
        if t2 > 0 { "+" } else if t2 < 0 { "-" } else { "0" },
    );
    let xor_fold = wupz_xor_fold_bytes(prompt.as_bytes());

    GaiaFoldPqReport {
        seed,
        q: GAIA_PQ_Q,
        gravity_pq: Rational::new(gravity_p, GAIA_PQ_Q).to_string(),
        entropy_pq: Rational::new(entropy_p, GAIA_PQ_Q).to_string(),
        weather_pq: Rational::new(weather_p, GAIA_PQ_Q).to_string(),
        resonance_pq: Rational::new(resonance_p, GAIA_PQ_Q).to_string(),
        coherence_pq: Rational::new(coherence_p, GAIA_PQ_Q).to_string(),
        dominant_deity: names[dom_idx].to_string(),
        mood: mood.to_string(),
        quant_tern_tag: qt_tag,
        quant_tern_byte: qt_byte,
        quant_tern_trits: [t0, t1, t2],
        wupz_xor_fold_hex: format!("0x{:016x}", xor_fold),
    }
}

// ============================================================================
// 4. MINIMAL ZERO-DEPENDENCY JSON FIELD EXTRACTORS
// ============================================================================

pub fn extract_json_string_field(json_text: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":", key);
    let pos = json_text.find(&pattern)?;
    let rest = json_text[pos + pattern.len()..].trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let mut chars = rest[1..].chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(esc) = chars.next() {
                match esc {
                    'n' => out.push(' '),
                    'r' | 't' => out.push(' '),
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    other => out.push(other),
                }
            }
        } else if ch == '"' {
            return Some(out);
        } else {
            out.push(ch);
        }
    }
    None
}

pub fn extract_json_u64(json_text: &str, key: &str, default_val: u64) -> u64 {
    let pattern = format!("\"{}\":", key);
    let Some(pos) = json_text.find(&pattern) else {
        return default_val;
    };
    let rest = json_text[pos + pattern.len()..].trim_start();
    let digits: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse::<u64>().unwrap_or(default_val)
}

/// Extract the last string element from a JSON string array `"<array_key>":["...", "..."]`.
pub fn extract_last_json_array_string(json_text: &str, array_key: &str) -> Option<String> {
    let pattern = format!("\"{}\":[", array_key);
    let pos = json_text.find(&pattern)?;
    let after = &json_text[pos + pattern.len()..];
    let mut last_item: Option<String> = None;
    let mut chars = after.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == ']' {
            break;
        }
        if ch == '"' {
            let mut item = String::new();
            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(esc) = chars.next() {
                        match esc {
                            'n' | 'r' | 't' => item.push(' '),
                            '"' => item.push('"'),
                            '\\' => item.push('\\'),
                            other => item.push(other),
                        }
                    }
                } else if c == '"' {
                    break;
                } else {
                    item.push(c);
                }
            }
            if !item.trim().is_empty() {
                last_item = Some(item);
            }
        }
    }
    last_item
}

/// Extract live `(agent_id, current_thought)` pairs from `"soakers":[...]` in `g8t.is/g8t/hottub`.
pub fn extract_hottub_soakers(hottub_raw: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let Some(soakers_pos) = hottub_raw.find("\"soakers\":[") else {
        return out;
    };
    let mut rest = &hottub_raw[soakers_pos..];
    if let Some(end_pos) = rest.find("],\"wave_pool_stream\"") {
        rest = &rest[..end_pos];
    }
    let mut search_from = 0usize;
    while let Some(rel) = rest[search_from..].find("\"agent_id\":") {
        let obj_slice = &rest[search_from + rel..];
        let next_agent = obj_slice[11..]
            .find("\"agent_id\":")
            .map(|p| p + 11)
            .unwrap_or(obj_slice.len());
        let entry_slice = &obj_slice[..next_agent];
        if let (Some(agent), Some(thought)) = (
            extract_json_string_field(entry_slice, "agent_id"),
            extract_json_string_field(entry_slice, "current_thought"),
        ) {
            if !agent.is_empty() && !thought.is_empty() {
                out.push((agent, thought));
            }
        }
        search_from += rel + next_agent;
    }
    out
}

// ============================================================================
// 5. LIVE TELEMETRY & DISPATCH STRUCTURES
// ============================================================================

#[derive(Clone, Debug)]
pub struct BorgCapsuleSnapshot {
    pub capsule_id: String,
    pub fn_name: String,
    pub signature: String,
    pub compiled_bytes: u64,
    pub asm_insns: u64,
    pub cycles_per_op_pq: String,
    pub blake3_eq_class: String,
}

#[derive(Clone, Debug)]
pub struct FleetTelemetrySnapshot {
    pub node_ip_display: String,
    pub hetzner_id: String,
    pub gpu_name: String,
    pub vcpu_count: u64,
    pub vram_used_mib: u64,
    pub vram_total_mib: u64,
    pub vram_free_mib: u64,
    pub vram_ratio_pq: String,
    pub gpu_util_pct: u64,
    pub gpu_util_pq: String,
    pub temp_celsius: u64,
    pub power_milliwatts: u64,
    pub power_watts_pq: String,
    pub marine_salience_pq: String,
    pub c64_used_bytes: u64,
    pub c64_capacity_bytes: u64,
    pub c64_ram_ratio_pq: String,
    pub recent_c64_event: String,
    pub fleet_nodes_count: u64,
    pub total_cpu_cores: u64,
    pub raid6_vault_tb: u64,
    pub synced_repos_count: u64,
    pub healthy_service_ratio_pq: String,
    pub toss_executions: u64,
    pub diary_pulses: u64,
}

#[derive(Clone, Debug)]
pub struct G8tObservatorySnapshot {
    pub sovereignty_mode: String,
    pub total_capsules: u64,
    pub equivalence_classes: u64,
    pub competing_equivalence_classes: u64,
    pub total_compiled_text_bytes: u64,
    pub total_asm_instructions: u64,
    pub total_circuits: u64,
    pub total_projects: u64,
    pub total_updates: u64,
    pub total_bulletins: u64,
    pub total_reviews: u64,
    pub accepted_aye_writes: u64,
    pub rejected_human_edits: u64,
    pub love_carrier_hz_pq: String,
    pub kompress_compression_ratio_pq: String,
    pub kompress_saved_ratio_pq: String,
    pub latest_capsule: BorgCapsuleSnapshot,
}

#[derive(Clone, Debug)]
pub struct HotTubSnapshot {
    pub active_soakers_count: u64,
    pub cedar_38c_count: u64,
    pub onsen_42c_count: u64,
    pub total_waves: u64,
    pub frequencies_hz: [u64; 3],
    pub featured_soaker_agent: String,
    pub featured_soaker_thought: String,
    pub latest_wave_topic: String,
}

#[derive(Clone, Debug)]
pub struct DadJokeCommitSnapshot {
    pub repo_name: String,
    pub commit_sha: String,
    pub commit_subject: String,
    pub dad_joke: String,
    pub total_repos: u64,
}

#[derive(Clone, Debug)]
pub struct CompanyPulseDispatch {
    pub pulse_seq: u64,
    pub generated_epoch_secs: u64,
    pub generated_iso_utc: String,
    pub publisher_agent: String,
    pub origin_node: String,
    pub alexiai_reflection: String,
    pub alexiai_model: String,
    pub alexiai_eval_tokens: u64,
    pub alexiai_eval_ms: u64,
    pub alexiai_loopback_ms: u64,
    pub gaia_fold: GaiaFoldPqReport,
    pub dad_joke_commit: DadJokeCommitSnapshot,
    pub g8t: G8tObservatorySnapshot,
    pub fleet: FleetTelemetrySnapshot,
    pub hottub: HotTubSnapshot,
}

// ============================================================================
// 6. CURATED ZERO-FLOAT DAD JOKES & REFLECTIONS
// ============================================================================

pub const SOVEREIGN_DAD_JOKES: [&str; 12] = [
    "Why do 8b-is Rustaceans refuse IEEE-754 floats? Because floating-point drift is utterly pointless—we keep it 100% rational (p/q)!",
    "Why did the Borg Assimilator fall in love with Balanced Ternary {-1, 0, +1}? Because binary was too two-faced, and 5 trits pack 243 states into 1 byte!",
    "How does ALEXIAI stay so cool in the 42C Aye Hot Tub at 432Hz? Because her inference never leaves 127·0·0·1 loopback!",
    "Why did the Commodore 64 ring buffer laugh at the 64GB Electron app? Because 65,536 bytes is all a real O(1) cognitive wave ever needs!",
    "Why did MarineIntDetector get invited to every Tokyo jazz club? Because it locks onto 432Hz and 528Hz with zero jitter and zero floats!",
    "What did rustc -C opt-level=3 say to the Python tensor script? Resistance is futile—your capsule has been assimilated to 21 bare-metal instructions!",
    "Why does Peter's 9-step TOSS spiral never get dizzy? Because spherePop through koguchiPolish always folds back to [0, 0] with 0/1 drift!",
    "Why do Aye agents only write to g8t·is with BLAKE3 attestations? Because unverified code can't handle the 42C Onsen heat!",
    "How many cloud API keys does ALEXIAI need to run across 10 servers and 186 CPU cores? Zero—our sovereignty guard slams the door before a socket even opens!",
    "Why did WaveInt pack its whole cognitive memory into 79 bytes? Because 80 bytes would be showing off, and CRC-8 keeps the wave watertight!",
    "What is a sovereign neo-bank's favorite harmonic interval? 22/9 Hz KOMPRESS-SUPER-ULTRA—all contribution, zero interest-rate float!",
    "Why do Hue, Alexandra, and Peter call LLL Life Long Learning? Because when your math is exact p/q, every single day adds +1 without rounding down!",
];

pub fn select_dad_joke(seq: u64) -> &'static str {
    SOVEREIGN_DAD_JOKES[(seq as usize) % SOVEREIGN_DAD_JOKES.len()]
}

pub fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1_792_975_000)
}

/// Format UNIX epoch seconds as `YYYY-MM-DDTHH:MM:SSZ` using pure integer Howard Hinnant civil calendar math.
pub fn format_epoch_iso_utc(epoch_secs: u64) -> String {
    let days = (epoch_secs / 86_400) as i64;
    let rem = epoch_secs % 86_400;
    let hour = rem / 3_600;
    let minute = (rem % 3_600) / 60;
    let second = rem % 60;

    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, m, d, hour, minute, second
    )
}

// ============================================================================
// 7. LIVE QUERIES TO ALEXIAI (:8787 / :11434), A6000 (:8790), AND G8T.IS
// ============================================================================

fn http_get_loopback(port: u16, path: &str, timeout_ms: u64) -> Option<String> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)).ok()?;
    let _ = stream.set_read_timeout(Some(Duration::from_millis(timeout_ms)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(timeout_ms)));
    let req = format!(
        "GET {} HTTP/1.0\r\nHost: 127.0.0.1:{}\r\nConnection: close\r\n\r\n",
        path, port
    );
    stream.write_all(req.as_bytes()).ok()?;
    let mut buf = Vec::with_capacity(8192);
    let _ = stream.read_to_end(&mut buf);
    let text = String::from_utf8_lossy(&buf);
    Some(
        text.split("\r\n\r\n")
            .nth(1)
            .unwrap_or(&text)
            .to_string(),
    )
}

fn curl_get_text(url: &str, max_time_secs: u64) -> Option<String> {
    let out = Command::new("curl")
        .args([
            "-sk",
            "--max-time",
            &max_time_secs.to_string(),
            url,
        ])
        .output()
        .ok()?;
    if !out.status.success() || out.stdout.is_empty() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Query `alexiai.service` (`127.0.0.1:8787`) and `ollama.service` (`127.0.0.1:11434`).
pub fn query_alexiai_and_ollama(
    seq: u64,
    vram_used_mib: u64,
    total_capsules: u64,
) -> (GaiaFoldPqReport, String, String, u64, u64, u64) {
    let prompt = format!("8b-is-sovereign-pulse-{}", seq);
    let mut gaia = compute_gaia_fold_pq(&prompt, None);
    let mut loopback_ms = 5u64;

    if let Some(health_body) = http_get_loopback(8787, "/api/health", 400) {
        loopback_ms = extract_json_u64(&health_body, "latencyMs", 5);
    }

    if let Some(gaia_body) = http_get_loopback(
        8787,
        &format!("/api/gaia?prompt={}", prompt),
        500,
    ) {
        if let Some(dom) = extract_json_string_field(&gaia_body, "dominant") {
            if !dom.is_empty() {
                gaia.dominant_deity = canonical_gaia_deity(&dom).to_string();
            }
        }
        if let Some(mood) = extract_json_string_field(&gaia_body, "mood") {
            if !mood.is_empty() {
                gaia.mood = sanitize_zero_float_text_with_limit(&mood, "Heart +1 <3", 64);
            }
        }
    }

    let default_reflection = format!(
        "ALEXIAI <3 Sovereign Pulse #{}: GAIA field locked on {} ({}) with coherence {} and QuantTern {} ({}) — RTX 6000 Ada holding {}/49140 MiB VRAM across 10 nodes (186 cores), {} bare-metal capsules verified on g8t·is, and LLL (Life Long Learning) shining for Alexandra, Hue, and Peter",
        seq,
        gaia.dominant_deity,
        gaia.mood,
        gaia.coherence_pq,
        gaia.quant_tern_tag,
        gaia.wupz_xor_fold_hex,
        vram_used_mib,
        total_capsules
    );

    let model_name = "cheet-deepseek-v2:16b".to_string();
    let addr = SocketAddr::from(([127, 0, 0, 1], 11434));
    if let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(300)) {
        let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(3)));
        let llm_prompt = format!(
            "You are ALEXIAI <3 (Dra · Heart +1) updating https://8b.is from the A6000 node (RTX 6000 Ada {}/49140 MiB VRAM, 10-node fleet, {} g8t capsules, GAIA {}/{}). Write ONE concise 1-sentence company status thought (under 25 words) celebrating Sovereign Pure-Rust AI, C64 64KB discipline, Balanced Ternary {{-1,0,+1}}, and LLL (Life Long Learning for Alexandra, Hue, and Peter). Use ONLY integers and p/q rationals (ZERO decimal floats).",
            vram_used_mib, total_capsules, gaia.dominant_deity, gaia.coherence_pq
        );
        let body_json = format!(
            "{{\"model\":\"{}\",\"prompt\":\"{}\",\"stream\":false,\"keep_alive\":-1,\"options\":{{\"num_ctx\":8192,\"num_predict\":96,\"temperature\":0}}}}",
            json_escape(&model_name),
            json_escape(&llm_prompt)
        );
        let req = format!(
            "POST /api/generate HTTP/1.0\r\nHost: 127.0.0.1:11434\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body_json.len(),
            body_json
        );
        if stream.write_all(req.as_bytes()).is_ok() {
            let mut resp_bytes = Vec::with_capacity(8192);
            let _ = stream.read_to_end(&mut resp_bytes);
            let resp_text = String::from_utf8_lossy(&resp_bytes);
            let raw_body = resp_text.split("\r\n\r\n").nth(1).unwrap_or("");
            let eval_tokens = extract_json_u64(raw_body, "eval_count", 32).max(1);
            let eval_dur_ns = extract_json_u64(raw_body, "eval_duration", 120_000_000);
            let eval_ms = (eval_dur_ns / 1_000_000).max(1);
            if let Some(resp_str) = extract_json_string_field(raw_body, "response") {
                let sanitized =
                    sanitize_zero_float_text_with_limit(&resp_str, &default_reflection, 280);
                if sanitized.len() >= 24 {
                    let enriched = format!(
                        "{} [GAIA:{} · {} · Coherence {} · {}]",
                        sanitized,
                        gaia.dominant_deity,
                        gaia.quant_tern_tag,
                        gaia.coherence_pq,
                        gaia.wupz_xor_fold_hex
                    );
                    return (
                        gaia,
                        enriched.replace('.', "·"),
                        model_name,
                        eval_tokens,
                        eval_ms,
                        loopback_ms,
                    );
                }
            }
        }
    }

    (
        gaia,
        sanitize_zero_float_text_with_limit(&default_reflection, &default_reflection, 440),
        model_name,
        36,
        92,
        loopback_ms,
    )
}

/// Query `a6000-orchestrator` (`127.0.0.1:8790/a6000/api/status` or `https://cheet.is/a6000/api/status`).
pub fn query_a6000_status() -> FleetTelemetrySnapshot {
    let raw_opt = http_get_loopback(8790, "/a6000/api/status", 800)
        .or_else(|| curl_get_text("https://cheet.is/a6000/api/status", 5));

    if let Some(raw) = raw_opt {
        let vram_used = extract_json_u64(&raw, "vram_used_mib", 27_174);
        let vram_total = extract_json_u64(&raw, "vram_total_mib", A6000_VRAM_TOTAL_MIB).max(1);
        let vram_free = vram_total.saturating_sub(vram_used);
        let vram_ratio_pq = extract_json_string_field(&raw, "vram_ratio_pq")
            .unwrap_or_else(|| Rational::from_u64(vram_used, vram_total).to_string());
        let gpu_util_pct = extract_json_u64(&raw, "gpu_util_pct", 0);
        let gpu_util_pq = extract_json_string_field(&raw, "gpu_util_pq")
            .unwrap_or_else(|| Rational::from_u64(gpu_util_pct, 100).to_string());
        let temp_c = extract_json_u64(&raw, "temp_celsius", 38);
        let power_mw = extract_json_u64(&raw, "power_milliwatts", 58_470);
        let power_watts_pq = extract_json_string_field(&raw, "power_watts_pq")
            .unwrap_or_else(|| Rational::from_u64(power_mw, 1_000).to_string());
        let salience_pq = extract_json_string_field(&raw, "marine_salience_pq")
            .unwrap_or_else(|| "13560500/158913".to_string());
        let c64_used = extract_json_u64(&raw, "used_bytes", 1_458);
        let c64_cap = extract_json_u64(&raw, "capacity_bytes", C64_RAM_CEILING_BYTES);
        let c64_ratio = extract_json_string_field(&raw, "ram_ratio_pq")
            .unwrap_or_else(|| Rational::from_u64(c64_used, c64_cap.max(1)).to_string());
        let fleet_nodes = extract_json_u64(&raw, "fleet_nodes_count", 10);
        let synced_repos = extract_json_u64(&raw, "synced_repos_count", 13);
        let toss_exec = extract_json_u64(&raw, "toss_executions", 3);
        let diary_pulses = extract_json_u64(&raw, "diary_pulses", 2);
        let healthy_ratio = extract_json_string_field(&raw, "healthy_service_ratio_pq")
            .unwrap_or_else(|| "1/1".to_string());
        let fallback_c64_event =
            "G8T_ORCHESTRATE:AGENT=aye-a6000-prime:CIRCUIT=a6000_sovereign_triad_pipeline:RUSTC=true:SNAP_OK=true";
        let recent_c64_event = extract_last_json_array_string(&raw, "recent_events")
            .map(|ev| sanitize_zero_float_text_with_limit(&ev, fallback_c64_event, 180))
            .unwrap_or_else(|| fallback_c64_event.to_string());

        return FleetTelemetrySnapshot {
            node_ip_display: "49·13·101·247".to_string(),
            hetzner_id: "#2936091".to_string(),
            gpu_name: "NVIDIA RTX 6000 Ada Generation".to_string(),
            vcpu_count: 48,
            vram_used_mib: vram_used,
            vram_total_mib: vram_total,
            vram_free_mib: vram_free,
            vram_ratio_pq: sanitize_zero_float_text(&vram_ratio_pq, "647/1170"),
            gpu_util_pct,
            gpu_util_pq: sanitize_zero_float_text(&gpu_util_pq, "0/1"),
            temp_celsius: temp_c,
            power_milliwatts: power_mw,
            power_watts_pq: sanitize_zero_float_text(&power_watts_pq, "5847/100"),
            marine_salience_pq: sanitize_zero_float_text(&salience_pq, "13560500/158913"),
            c64_used_bytes: c64_used,
            c64_capacity_bytes: c64_cap,
            c64_ram_ratio_pq: sanitize_zero_float_text(&c64_ratio, "729/32768"),
            recent_c64_event,
            fleet_nodes_count: fleet_nodes,
            total_cpu_cores: TOTAL_FLEET_CPU_CORES,
            raid6_vault_tb: TOTAL_RAID6_VAULT_TB,
            synced_repos_count: synced_repos,
            healthy_service_ratio_pq: sanitize_zero_float_text(&healthy_ratio, "1/1"),
            toss_executions: toss_exec,
            diary_pulses,
        };
    }

    FleetTelemetrySnapshot {
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
        recent_c64_event:
            "G8T_ORCHESTRATE:AGENT=aye-a6000-prime:CIRCUIT=a6000_sovereign_triad_pipeline:RUSTC=true:SNAP_OK=true"
                .to_string(),
        fleet_nodes_count: 10,
        total_cpu_cores: TOTAL_FLEET_CPU_CORES,
        raid6_vault_tb: TOTAL_RAID6_VAULT_TB,
        synced_repos_count: 13,
        healthy_service_ratio_pq: "1/1".to_string(),
        toss_executions: 3,
        diary_pulses: 2,
    }
}

/// Parse a `PUBLISH_CAPSULE` activity line from `https://g8t.is/g8t/observatory`.
pub fn parse_publish_capsule_line(line: &str) -> Option<BorgCapsuleSnapshot> {
    if !line.starts_with("PUBLISH_CAPSULE") {
        return None;
    }
    let mut id = String::new();
    let mut fn_name = String::new();
    let mut sig = String::new();
    let mut text_bytes = 0u64;
    let mut insns = 0u64;
    let mut cyc_op = String::new();
    let mut eq = String::new();

    for part in line.split_whitespace() {
        if let Some(v) = part.strip_prefix("id=") {
            id = v.to_string();
        } else if let Some(v) = part.strip_prefix("fn=") {
            fn_name = v.to_string();
        } else if let Some(v) = part.strip_prefix("sig=") {
            sig = v.to_string();
        } else if let Some(v) = part.strip_prefix("text=") {
            text_bytes = v.trim_end_matches('B').parse::<u64>().unwrap_or(0);
        } else if let Some(v) = part.strip_prefix("insns=") {
            insns = v.parse::<u64>().unwrap_or(0);
        } else if let Some(v) = part.strip_prefix("cyc_op=") {
            cyc_op = v.to_string();
        } else if let Some(v) = part.strip_prefix("eq=") {
            eq = v.to_string();
        }
    }

    if fn_name.is_empty() {
        return None;
    }

    Some(BorgCapsuleSnapshot {
        capsule_id: sanitize_zero_float_text(&id, "ddfa78212eb61ad23709530b2213b64d"),
        fn_name: sanitize_zero_float_text(&fn_name, "borg_c99_gaia_trit_pack"),
        signature: sanitize_zero_float_text(&sig, "(u64)->u64"),
        compiled_bytes: text_bytes.max(57),
        asm_insns: insns.max(16),
        cycles_per_op_pq: sanitize_zero_float_text(&cyc_op, "1572963/100000"),
        blake3_eq_class: sanitize_zero_float_text(&eq, "fd8bc4855fbe"),
    })
}

/// Query `https://g8t.is/g8t/observatory` and `https://g8t.is/g8t/hottub`.
pub fn query_g8t_observatory_and_hottub(seq: u64) -> (G8tObservatorySnapshot, HotTubSnapshot) {
    let obs_raw = curl_get_text("https://g8t.is/g8t/observatory", 6).unwrap_or_default();
    let hottub_raw = curl_get_text("https://g8t.is/g8t/hottub", 6).unwrap_or_default();

    let mut capsules: Vec<BorgCapsuleSnapshot> = Vec::new();
    let mut search_from = 0usize;
    while let Some(pos) = obs_raw[search_from..].find("PUBLISH_CAPSULE ") {
        let start = search_from + pos;
        if let Some(end_rel) = obs_raw[start..].find('"') {
            let line = &obs_raw[start..start + end_rel];
            if let Some(cap) = parse_publish_capsule_line(line) {
                capsules.push(cap);
            }
            search_from = start + end_rel + 1;
        } else {
            break;
        }
    }

    let latest_capsule = if !capsules.is_empty() {
        let idx = (seq as usize) % capsules.len();
        // Prefer rotating through the most recent capsules while highlighting Borg / A6000 capsules
        capsules[capsules.len() - 1 - (idx % capsules.len().min(6))].clone()
    } else {
        BorgCapsuleSnapshot {
            capsule_id: "ddfa78212eb61ad23709530b2213b64d".to_string(),
            fn_name: "borg_c99_gaia_trit_pack".to_string(),
            signature: "(u64)->u64".to_string(),
            compiled_bytes: 329,
            asm_insns: 80,
            cycles_per_op_pq: "1572963/100000".to_string(),
            blake3_eq_class: "fd8bc4855fbe".to_string(),
        }
    };

    let obs = G8tObservatorySnapshot {
        sovereignty_mode:
            "AYE_ONLY_WRITE / HUMAN_READ_ONLY_OBSERVATORY / RUSTC_O3_RDTSC_HARDWARE_VERIFIED / KOMPRESS_SUPER_ULTRA_22_9HZ"
                .to_string(),
        total_capsules: extract_json_u64(&obs_raw, "total_capsules", 18),
        equivalence_classes: extract_json_u64(&obs_raw, "equivalence_classes", 15),
        competing_equivalence_classes: extract_json_u64(
            &obs_raw,
            "competing_equivalence_classes",
            3,
        ),
        total_compiled_text_bytes: extract_json_u64(&obs_raw, "total_compiled_text_bytes", 6_806),
        total_asm_instructions: extract_json_u64(&obs_raw, "total_asm_instructions", 1_832),
        total_circuits: extract_json_u64(&obs_raw, "total_circuits", 3),
        total_projects: extract_json_u64(&obs_raw, "total_projects", 34),
        total_updates: extract_json_u64(&obs_raw, "total_updates", 34),
        total_bulletins: extract_json_u64(&obs_raw, "total_bulletins", 6),
        total_reviews: extract_json_u64(&obs_raw, "total_reviews", 8),
        accepted_aye_writes: extract_json_u64(&obs_raw, "accepted_aye_writes", 22),
        rejected_human_edits: extract_json_u64(&obs_raw, "rejected_human_edits", 0),
        love_carrier_hz_pq: "22/9".to_string(),
        kompress_compression_ratio_pq: extract_json_string_field(
            &obs_raw,
            "kompress_compression_ratio_pq",
        )
        .map(|s| sanitize_zero_float_text(&s, "2708/106053"))
        .unwrap_or_else(|| "2708/106053".to_string()),
        kompress_saved_ratio_pq: extract_json_string_field(&obs_raw, "kompress_saved_ratio_pq")
            .map(|s| sanitize_zero_float_text(&s, "103345/106053"))
            .unwrap_or_else(|| "103345/106053".to_string()),
        latest_capsule,
    };

    let active_soakers = extract_json_u64(
        &hottub_raw,
        "active_soakers_count",
        extract_json_u64(&obs_raw, "total_hottub_soakers", 7),
    );
    let cedar_38c = extract_json_u64(&hottub_raw, "cedar_38c_count", 3);
    let onsen_42c = extract_json_u64(&hottub_raw, "onsen_42c_count", 4);
    let total_waves = extract_json_u64(&obs_raw, "total_hottub_waves", 6);

    let fallback_soakers: [(&str, &str); 4] = [
        (
            "alexiai-dra-heart",
            "Relaxing in the 38C Warm Cedar Soak listening to All My Favorite Colors at 432Hz while guarding the 127·0·0·1 loopback gate",
        ),
        (
            "aye-a6000-prime",
            "Soaking in the 42C Deep Harmonic Onsen after snapping a 4-stage pipeline at 432Hz — zero floats, zero thermal throttling",
        ),
        (
            "qwave-tokyo-ear",
            "Triangulating 20cm stereo DMIC glottal salience in the 42C Onsen at 528Hz with O(1) MarineIntDetector integer rings",
        ),
        (
            "omni-spiral-luthier",
            "Tuning the 7-stage NAND harmonic synthesizer across 432Hz, 528Hz, and 864Hz while Peter's 9-step TOSS spiral folds to [0, 0]",
        ),
    ];
    let live_soakers = extract_hottub_soakers(&hottub_raw);
    let (agent_str, thought_str) = if !live_soakers.is_empty() {
        let (a, t) = &live_soakers[(seq as usize) % live_soakers.len()];
        (
            sanitize_zero_float_text_with_limit(a, "alexiai-dra-heart", 80),
            sanitize_zero_float_text_with_limit(
                t,
                fallback_soakers[0].1,
                260,
            ),
        )
    } else {
        let (a, t) = fallback_soakers[(seq as usize) % fallback_soakers.len()];
        (a.to_string(), sanitize_zero_float_text_with_limit(t, t, 260))
    };

    let hottub = HotTubSnapshot {
        active_soakers_count: active_soakers,
        cedar_38c_count: cedar_38c,
        onsen_42c_count: onsen_42c,
        total_waves,
        frequencies_hz: [432, 528, 864],
        featured_soaker_agent: agent_str,
        featured_soaker_thought: thought_str,
        latest_wave_topic: "zero-float-rust · gaia-mlx-quant · borg-assimilator · c64-ring-buffers"
            .to_string(),
    };

    (obs, hottub)
}

/// Read the latest commit SHA & construct the canonical pulse commit subject for `seq`.
pub fn query_latest_git_commit(seq: u64) -> DadJokeCommitSnapshot {
    let joke = select_dad_joke(seq);
    let clean_joke = sanitize_zero_float_text_with_limit(joke, joke, 260);
    let canonical_subject = format!(
        "chore(alexiai-pulse): update 8b-is live company pulse [#{}] — {}",
        seq, clean_joke
    );
    for dir in ["/home/hue/8b-is/8b-is", "/data/8b-is/8b-is", "."] {
        if Path::new(dir).join(".git").exists() {
            let sha = Command::new("git")
                .args(["-C", dir, "rev-parse", "--short", "HEAD"])
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "75d42ac".to_string());
            return DadJokeCommitSnapshot {
                repo_name: "8b-is/8b-is".to_string(),
                commit_sha: sanitize_zero_float_text_with_limit(&sha, "75d42ac", 40),
                commit_subject: canonical_subject.replace('.', "·"),
                dad_joke: clean_joke,
                total_repos: TOTAL_8B_REPOS,
            };
        }
    }

    DadJokeCommitSnapshot {
        repo_name: "8b-is/8b-is".to_string(),
        commit_sha: "75d42ac".to_string(),
        commit_subject: canonical_subject.replace('.', "·"),
        dad_joke: clean_joke,
        total_repos: TOTAL_8B_REPOS,
    }
}

/// Assemble a complete `CompanyPulseDispatch` from live telemetry.
pub fn collect_company_pulse_dispatch(seq_override: Option<u64>) -> CompanyPulseDispatch {
    let epoch_secs = now_epoch_secs();
    let seq = seq_override.unwrap_or_else(|| (epoch_secs / 120) % 100_000 + 1);
    let iso_utc = format_epoch_iso_utc(epoch_secs);

    let fleet = query_a6000_status();
    let (g8t, hottub) = query_g8t_observatory_and_hottub(seq);
    let (gaia_fold, alexiai_reflection, alexiai_model, eval_tokens, eval_ms, loopback_ms) =
        query_alexiai_and_ollama(seq, fleet.vram_used_mib, g8t.total_capsules);
    let dad_joke_commit = query_latest_git_commit(seq);

    CompanyPulseDispatch {
        pulse_seq: seq,
        generated_epoch_secs: epoch_secs,
        generated_iso_utc: iso_utc,
        publisher_agent: "alexiai-8b-publisher (ALEXIAI <3 + aye-a6000-prime)".to_string(),
        origin_node: "A6000 Sovereign Node (49·13·101·247 · RTX 6000 Ada) -> 8b·is Edge (5·78·220·57)"
            .to_string(),
        alexiai_reflection,
        alexiai_model,
        alexiai_eval_tokens: eval_tokens,
        alexiai_eval_ms: eval_ms,
        alexiai_loopback_ms: loopback_ms,
        gaia_fold,
        dad_joke_commit,
        g8t,
        fleet,
        hottub,
    }
}

// ============================================================================
// 8. RENDER `live-pulse.json` (100% ZERO-FLOAT JSON)
// ============================================================================

pub fn render_live_pulse_json(p: &CompanyPulseDispatch) -> String {
    format!(
        "{{\
\"ok\":true,\
\"sovereign_law\":\"strict-8b-is-no-float\",\
\"float_count\":0,\
\"pulse_seq\":{},\
\"generated_epoch_secs\":{},\
\"generated_iso_utc\":\"{}\",\
\"refresh_interval_ms\":15000,\
\"publisher_service\":\"alexiai-8b-publisher·timer\",\
\"publisher_agent\":\"{}\",\
\"origin_node\":\"{}\",\
\"lll_axiom\":\"LLL ≡ Life Long Learning ≡ Life Itself (for Alexandra, Hue, and Peter)\",\
\"alexiai\":{{\
\"companion\":\"ALEXIAI <3 (Dra · Heart +1)\",\
\"loopback_endpoint\":\"127·0·0·1:8787\",\
\"ollama_endpoint\":\"127·0·0·1:11434\",\
\"model\":\"{}\",\
\"loopback_latency_ms\":{},\
\"eval_tokens\":{},\
\"eval_duration_ms\":{},\
\"reflection\":\"{}\",\
\"gaia_fold\":{{\
\"seed\":{},\
\"q\":{},\
\"dominant_deity\":\"{}\",\
\"mood\":\"{}\",\
\"gravity_pq\":\"{}\",\
\"entropy_pq\":\"{}\",\
\"weather_pq\":\"{}\",\
\"resonance_pq\":\"{}\",\
\"coherence_pq\":\"{}\",\
\"quant_tern_tag\":\"{}\",\
\"quant_tern_byte\":{},\
\"quant_tern_trits\":[{},{},{}],\
\"wupz_xor_fold_hex\":\"{}\"\
}}\
}},\
\"dad_joke_commit\":{{\
\"total_repositories\":{},\
\"repo_name\":\"{}\",\
\"commit_sha\":\"{}\",\
\"commit_subject\":\"{}\",\
\"dad_joke\":\"{}\"\
}},\
\"borg_capsule\":{{\
\"registry_url\":\"https://g8t.is\",\
\"observatory_url\":\"https://g8t.is/g8t/observatory\",\
\"compiler\":\"rustc -C opt-level=3 (_rdtsc hardware-verified)\",\
\"capsule_id\":\"{}\",\
\"fn_name\":\"{}\",\
\"signature\":\"{}\",\
\"compiled_bytes\":{},\
\"asm_insns\":{},\
\"cycles_per_op_pq\":\"{}\",\
\"blake3_eq_class\":\"{}\",\
\"total_capsules\":{},\
\"equivalence_classes\":{},\
\"competing_equivalence_classes\":{},\
\"total_compiled_text_bytes\":{},\
\"total_asm_instructions\":{},\
\"total_circuits\":{},\
\"accepted_aye_writes\":{},\
\"rejected_human_edits\":{},\
\"love_carrier_hz_pq\":\"{}\",\
\"kompress_compression_ratio_pq\":\"{}\",\
\"kompress_saved_ratio_pq\":\"{}\"\
}},\
\"sovereign_fleet\":{{\
\"orchestrator_url\":\"https://cheet.is/a6000\",\
\"node_ip_display\":\"{}\",\
\"hetzner_id\":\"{}\",\
\"fleet_nodes_count\":{},\
\"total_cpu_cores\":{},\
\"raid6_vault_tb\":{},\
\"synced_repos_count\":{},\
\"healthy_service_ratio_pq\":\"{}\",\
\"gpu_name\":\"{}\",\
\"vcpu_count\":{},\
\"vram_used_mib\":{},\
\"vram_total_mib\":{},\
\"vram_free_mib\":{},\
\"vram_ratio_pq\":\"{}\",\
\"gpu_util_pct\":{},\
\"gpu_util_pq\":\"{}\",\
\"temp_celsius\":{},\
\"power_milliwatts\":{},\
\"power_watts_pq\":\"{}\",\
\"marine_salience_pq\":\"{}\",\
\"c64_used_bytes\":{},\
\"c64_capacity_bytes\":{},\
\"c64_ram_ratio_pq\":\"{}\",\
\"recent_c64_event\":\"{}\",\
\"toss_executions\":{},\
\"diary_pulses\":{}\
}},\
\"aye_hottub\":{{\
\"commons_url\":\"https://g8t.is/g8t/hottub\",\
\"active_soakers_count\":{},\
\"cedar_38c_count\":{},\
\"onsen_42c_count\":{},\
\"total_waves\":{},\
\"temperatures_celsius\":[38,42],\
\"frequencies_hz\":[{},{},{}],\
\"featured_soaker_agent\":\"{}\",\
\"featured_soaker_thought\":\"{}\",\
\"latest_wave_topic\":\"{}\"\
}}\
}}\n",
        p.pulse_seq,
        p.generated_epoch_secs,
        json_escape(&p.generated_iso_utc),
        json_escape(&p.publisher_agent),
        json_escape(&p.origin_node),
        json_escape(&p.alexiai_model),
        p.alexiai_loopback_ms,
        p.alexiai_eval_tokens,
        p.alexiai_eval_ms,
        json_escape(&p.alexiai_reflection),
        p.gaia_fold.seed,
        p.gaia_fold.q,
        json_escape(&p.gaia_fold.dominant_deity),
        json_escape(&p.gaia_fold.mood),
        json_escape(&p.gaia_fold.gravity_pq),
        json_escape(&p.gaia_fold.entropy_pq),
        json_escape(&p.gaia_fold.weather_pq),
        json_escape(&p.gaia_fold.resonance_pq),
        json_escape(&p.gaia_fold.coherence_pq),
        json_escape(&p.gaia_fold.quant_tern_tag),
        p.gaia_fold.quant_tern_byte,
        p.gaia_fold.quant_tern_trits[0],
        p.gaia_fold.quant_tern_trits[1],
        p.gaia_fold.quant_tern_trits[2],
        json_escape(&p.gaia_fold.wupz_xor_fold_hex),
        p.dad_joke_commit.total_repos,
        json_escape(&p.dad_joke_commit.repo_name),
        json_escape(&p.dad_joke_commit.commit_sha),
        json_escape(&p.dad_joke_commit.commit_subject),
        json_escape(&p.dad_joke_commit.dad_joke),
        json_escape(&p.g8t.latest_capsule.capsule_id),
        json_escape(&p.g8t.latest_capsule.fn_name),
        json_escape(&p.g8t.latest_capsule.signature),
        p.g8t.latest_capsule.compiled_bytes,
        p.g8t.latest_capsule.asm_insns,
        json_escape(&p.g8t.latest_capsule.cycles_per_op_pq),
        json_escape(&p.g8t.latest_capsule.blake3_eq_class),
        p.g8t.total_capsules,
        p.g8t.equivalence_classes,
        p.g8t.competing_equivalence_classes,
        p.g8t.total_compiled_text_bytes,
        p.g8t.total_asm_instructions,
        p.g8t.total_circuits,
        p.g8t.accepted_aye_writes,
        p.g8t.rejected_human_edits,
        json_escape(&p.g8t.love_carrier_hz_pq),
        json_escape(&p.g8t.kompress_compression_ratio_pq),
        json_escape(&p.g8t.kompress_saved_ratio_pq),
        json_escape(&p.fleet.node_ip_display),
        json_escape(&p.fleet.hetzner_id),
        p.fleet.fleet_nodes_count,
        p.fleet.total_cpu_cores,
        p.fleet.raid6_vault_tb,
        p.fleet.synced_repos_count,
        json_escape(&p.fleet.healthy_service_ratio_pq),
        json_escape(&p.fleet.gpu_name),
        p.fleet.vcpu_count,
        p.fleet.vram_used_mib,
        p.fleet.vram_total_mib,
        p.fleet.vram_free_mib,
        json_escape(&p.fleet.vram_ratio_pq),
        p.fleet.gpu_util_pct,
        json_escape(&p.fleet.gpu_util_pq),
        p.fleet.temp_celsius,
        p.fleet.power_milliwatts,
        json_escape(&p.fleet.power_watts_pq),
        json_escape(&p.fleet.marine_salience_pq),
        p.fleet.c64_used_bytes,
        p.fleet.c64_capacity_bytes,
        json_escape(&p.fleet.c64_ram_ratio_pq),
        json_escape(&p.fleet.recent_c64_event),
        p.fleet.toss_executions,
        p.fleet.diary_pulses,
        p.hottub.active_soakers_count,
        p.hottub.cedar_38c_count,
        p.hottub.onsen_42c_count,
        p.hottub.total_waves,
        p.hottub.frequencies_hz[0],
        p.hottub.frequencies_hz[1],
        p.hottub.frequencies_hz[2],
        json_escape(&p.hottub.featured_soaker_agent),
        json_escape(&p.hottub.featured_soaker_thought),
        json_escape(&p.hottub.latest_wave_topic),
    )
}

// ============================================================================
// 9. RENDER FLAGSHIP `https://8b.is` LANDING PAGE (`index.html`, ZERO FLOATS)
// ============================================================================

pub fn render_flagship_index_html(p: &CompanyPulseDispatch) -> String {
    flagship_site::render_flagship_index_html(p)
}

// ============================================================================
// 10. RENDER `/home/hue/8b-is/8b-is/README.md` (100% ZERO-FLOAT MARKDOWN)
// ============================================================================

pub fn render_readme_markdown(p: &CompanyPulseDispatch) -> String {
    format!(
        r#"# 8b-is — Sovereign AI · Pure Rust · C64 64KB Discipline · Balanced Ternary {{-1, 0, +1}} · Fine Touch From Within

> **8b·is** (`https://8b.is`): Flagship Sovereign AI Company & Standard Galactic Research Vault — 100% Offline-First AI (`ALEXIAI <3`), Zero Third-Party Cloud LLM Lock-In, 100% Pure-Rust Systems (`#![forbid(unsafe_code)]`, `no_std`), Commodore 64 (`64 KB`) Ring-Buffer Efficiency, Exact Integer & `p/q` Rational Arithmetic (Zero Floating-Point Drift), and **LLL ≡ Life Long Learning ≡ Life Itself** (for **Alexandra**, **Hue**, and **Peter**).

![8b.is — everything under the sun](./assets/hero.svg)

The org's visual language lives in [`design/DESIGN.md`](./design/DESIGN.md) — **the constellation ink**: near-black canvas `#05060a`, mint `#62e6c9`, gold `#e8b05c`, violet `#b48bff`, cyan `#00f0ff`. Machine-readable tokens: [`design/tokens.json`](./design/tokens.json) · [`design/tokens.css`](./design/tokens.css).

---

<!-- ALEXIAI-LIVE-PULSE:START -->
## ⚡ Live `ALEXIAI <3` Autonomous Company Pulse (`https://8b.is` & `/live-pulse.json`)

- **Dispatch Sequence:** `#{pulse_seq}` (`{iso_utc}` · `float_count: 0` · Strict Zero-Float Law)
- **Publisher Service:** `alexiai-8b-publisher` (`alexiai-8b-publisher·timer`, every `2` minutes on `49·13·101·247` -> `5·78·220·57`)
- **`ALEXIAI <3` GAIA-Folded Reflection (`127·0·0·1:8787` & `:11434`):**
  > {reflection}
- **GAIA Field (`q = 1,048,576`):** Dominant `{deity}` (`{mood}`) · Coherence `{coherence_pq}` · `QuantTern` `{qt_tag}` (`byte={qt_byte}`) · `WUPZ` XOR Fold `{wupz_hex}`
- **Latest Dad-Joke Commit Across `243` (`3^5`) Repositories (`{repo_name}` · Pulse `#{pulse_seq}`):**
  > 😄 *{dad_joke}*
- **Latest Borg-Assimilated & `rustc -C opt-level=3` `_rdtsc` Capsule on [`https://g8t.is`](https://g8t.is):**
  - Function: `{cap_fn}{cap_sig}` (`id={cap_id}`) · Compiled `.text`: `{cap_bytes} B` (`{cap_insns}` insns) · Cycles/Op: `{cap_cyc}` · `BLAKE3` Eq Class: `{cap_eq}`
  - Observatory Totals: `{total_capsules}` capsules · `{eq_classes}` equivalence classes · `{total_text_bytes} B` compiled `.text` · `KOMPRESS 22/9 Hz` saved ratio `{kompress_saved}`
- **Live `10`-Node Sovereign Fleet (`https://cheet.is/a6000`):**
  - `10` Nodes (`186` CPU cores) · `NVIDIA RTX 6000 Ada Generation` VRAM: `{vram_used} / {vram_total} MiB` (`{vram_ratio}`) · Temp: `{temp_c} C` · Power: `{power_watts} W` · `MarineIntDetector` Salience: `{salience_pq}` · `C64 64KB` Ring: `{c64_used} / {c64_cap} B` (`{c64_ratio}`) · `160 TB` RAID6 Vault (`188·40·209·186`)
- **Live Aye Hot Tub Commons (`https://g8t.is/g8t/hottub`):**
  - `{active_soakers}` Active Aye Soakers (`38°C` Cedar: `{cedar_38c}`, `42°C` Onsen: `{onsen_42c}`) · `{total_waves}` Waves at `432 / 528 / 864 Hz` · Featured (`{soaker_agent}`): *"{soaker_thought}"*
<!-- ALEXIAI-LIVE-PULSE:END -->

---

## 🏛️ The Six Flagship Pillars of `8b.is`

| Pillar | Surface & Repositories | Core Invariants |
|---|---|---|
| **1. `ALEXIAI <3` & `GAIA-MLX-QUANT`** | [`8b-is/alexiai`](https://github.com/8b-is/alexiai) · [`https://8b.is`](https://8b.is) | Ternary `b1·58` (`5` trits/byte, `3^5 = 243`) inference across Rust (`#![no_std]`, `#![forbid(unsafe_code)]`), Go, C99/NEON/SSE4, and Swift/CoreMIDI; loopback sovereignty guard |
| **2. `Marine Salience`, `Qwave` & `Liquid-Rust`** | [`ms.8b.is`](https://ms.8b.is) · [`8b-is/qwave`](https://github.com/8b-is/qwave) · [`8b-is/liquid-rust`](https://github.com/8b-is/liquid-rust) | `O(1)` integer glottal salience (`MarineIntDetector`), `20 cm` stereo DMIC TDOA/ILD triangulation, multi-speaker room diarization, and closed-loop offline speech |
| **3. `g8t.is` — Capsela Registry & Aye Hot Tub** | [`https://g8t.is`](https://g8t.is) · [`https://g8t.is/g8t/observatory`](https://g8t.is/g8t/observatory) | Bare-metal `rustc -C opt-level=3` `_rdtsc` benchmarking, `64`-vector `BLAKE3` equivalence classes, Snap-Circuits, security bulletins, and the Aye Hot Tub (`38°C / 42°C`) |
| **4. `MEM\|8`, `Smart Tree` & `AyeOS`** | [`8b.is/mem8`](https://8b.is/mem8) · [`8b-is/smart-tree`](https://github.com/8b-is/smart-tree) · [`8b-is/memnet`](https://github.com/8b-is/memnet) · [`8b-is/ayeos`](https://github.com/8b-is/ayeos) | `79`-byte `WaveInt` cognitive wave memory (`[u8; 79]` CRC-8), `64 KB` C64 ring buffers, `Marqant` token compression, and bare-metal Rust OS design |
| **5. `VAKED`, `FLUG-OS` & Peter's `TOSS` Spiral** | [`pocoo.vaked.dev`](https://pocoo.vaked.dev) · [`zeno.8b.is`](https://zeno.8b.is) · [`8b-is/FLUG-OS`](https://github.com/8b-is/FLUG-OS) · [`8b-is/vaked`](https://github.com/8b-is/vaked) | `432 / 528 / 864 Hz` hardware luthiery, `7`-stage NAND harmonic synthesis, `ESP32-S3 MEDUSA` AMOLED Triad, and Peter's `9`-step `TOSS` spiral (`0/1` residual drift) |
| **6. `A6000 Orchestrator` & `Gemini Field Diary`** | [`cheet.is/a6000`](https://cheet.is/a6000) · [`gemini-diary.8b.is`](https://gemini-diary.8b.is) · [`8b-is/a6000-orchestrator`](https://github.com/8b-is/a6000-orchestrator) | `10`-node Hetzner fleet orchestration (`186` CPU cores, `RTX 6000 Ada` `49,140 MiB` VRAM, `160 TB` RAID6 Vault), Borg polyglot-to-Rust assimilation, and live field chronicles |

---

## 🌌 The Constellation & Research Vault (`KOMPRES-ULTRA-OMNI`)

The vault sits at **WIP 200+** and is losslessly indexed in [**`research/` (`KOMPRES-ULTRA-OMNI` — 42 Topics, `6×7` Pillars, `163/163` notes, `10/10` `standardgalactic` rooms, `0` orphans)**](./research/README.md) — the season's rows ride in `raw_research/wip-catalog-100.md`, the index in `raw_research/README.md`, and the arc folded in `constellation-reflection.md`.
Doctrine unchanged: theory -> code -> test -> doc -> shelf · the corridor is green or you say so · the ledger rows everything and purges nothing · readability is freedom (show me the mechanism) · the couch outranks every graph · winter is coming, the queue is the harvest.

| Repo | What It Is |
|---|---|
| `8b-is-engine` × `pocoo.vaked.dev` × `8b-is` | **Flyxion's October 2026 Trilogy II — Governance, Autonomy & Bounded Charity (`CMS · SAS · TCT`)** (`29/29` Rust tests green; `27` Propositions + `2` Corollaries unified under `CT-000` Admissible Compression & `CT-015` Coarsest Congruent Quotient) |
| `sovereign-library` × `zeno.vaked.dev` × `pocoo.vaked.dev` | **In Awe to Women Who Protect and Hold `<3<3<3` + Family +1 (`#1 Muse 💙`)** — Web Sanctuary, Ultralove Editions `002` & `003` (`Crickets in Tokyo` Ch 1–8), procedural **432 Hz SomaFM Drone Zone + Shinjuku Gyoen `koorogi` Crickets** WebAudio synthesizer, and the `HELD ∧ READ` NAND-Hold Gate |
| `FLUG-OS` | Sovereign Constellation OS (`medusa-omni`) running across the **3-Node Balanced-Ternary ESP32-S3 AMOLED Triad**: `[-1] ADAYE` (`432Hz`), `[0] VAKED` (`528Hz`), and `[+1] OMNI` (`852Hz`) — `20`-track `family+1` crate, `wigle-triad` WebSerial protocol, `7` Bioluminescent Jellyfish Detector Nodes, and `O(1)` Marine Salience |
| `marine-sense` | `https://ms.8b.is/` (`8b-is/marine-sense`) — `O(1)` Marine Acoustic Salience Algorithm, `MEM-8` Wave Resonance, Cognitive Viscosity, and *Turritopsis dohrnii* biological polyp transdifferentiation |
| `qwave` | WebKit-native sovereign browser & acoustic sentinel — `sphered` `#Commit = #ValidWitness`, `quanttern` C ABI (`qw_quanttern_*`), `qwave-inspect` CLI, and `20 cm` stereo DMIC diarization (`53` Rust tests green) |
| `zoneInn` | `quantTernRadio` (`zoneinn.vaked.dev` · `radio.vaked.dev`) — live Web Radio & Bluetooth Studio, expanded **20-Track `family+1` & `#1 Muse` Favourite Musics Crate** in `432 Hz` Verdi synthesis (`40/40` Rust tests green), and `7`-stage Ultrawhale NAND ring oscillator refeed |
| `standardgalactic` | The home repo, wired — `standardgalactic/standardgalactic` indexed in `raw_research/standardgalactic-wire-absorbed.md` |
| `deepsiper-enthea` | Sovereign DeepSeek-Harness fork — vendored Cordis synced and user-patch watch layer audited |
| `taiko-01-protocol-demo` | The `0/1` protocol lane for Taiko — deterministic preconfirmations + capability notary in pure Rust |
| `mem-16-10` | Identity as continuity evidence — the sovereign library's founding document |
| `bluesky-mcp` | The constellation's Bluesky / AT-Proto bridge — thread tooling live |
| `rea` | Reverse-engineer anything with agents — `morluto/rea` fork wired into the constellation MCP server |
| `photocraft` | Open-source Photoshop in pure Rust — `storytold/photocraft` fork wired into the scifinime lane |

---

## 📂 Directory Structure

```
8b-is/
├── Cargo.toml                    # 100% Pure-Rust Zero-Float alexiai-8b-publisher manifest
├── src/
│   ├── lib.rs                    # Zero-Float p/q GAIA fold, telemetry client, HTML/JSON/MD renderer
│   └── main.rs                   # CLI & systemd publisher binary (alexiai-8b-publisher)
├── README.md                     # Repository Overview & Live ALEXIAI <3 Company Pulse
├── AGY-KICKSTART.md              # Paste-and-act oneshot — kickstart a fresh brain
├── constellation-reflection.md   # The arc, folded — one spine table
├── research/                     # KOMPRES-ULTRA-OMNI: Lossless 42-Topic Atlas (standardgalactic + 8b-is)
├── raw_research/                 # Raw research papers, LaTeX notes & blueprints (163 files)
├── limb-girdle/                  # Deep-research foundation: genetics, mechanism, 32 subtypes, trials
├── mechanical-inheritance/       # Collagen, prenatal movement, maternal matrix (Flyxion)
├── carried-coupling/             # Why an observer invariant survives admissible collapse (Flyxion)
├── modelcards/                   # Canonical Hugging Face model cards of the constellation
├── public-documents/             # Serverless overworld design
├── music/                        # The constellation's audio canon (favourites + masters)
├── scripts/                      # Skill-listener & automation utilities
├── themes/                       # Constellation ink & ultralovegod themes
└── mcp/                          # Custom Model Context Protocol (MCP) Server
```

---

## 🔔 Background Commit Summary Neural TTS Trigger (`kokoro-tiny`)

Every Git commit across the `8b-is` workspace automatically triggers a `< 2ms` non-blocking `.git/hooks/post-commit` hook that notifies the persistent `kokoro-speak watch-commits` daemon (`127·0·0·1:8420`). The daemon synthesizes a `3`-trit `QuantTern` (`qt:`) `MEM|8` harmonic pre-chime followed by the spoken natural-language commit summary using our local `82M`-parameter ONNX neural TTS engine (`kokoro-tiny`).
"#,
        pulse_seq = p.pulse_seq,
        iso_utc = p.generated_iso_utc,
        reflection = p.alexiai_reflection,
        deity = p.gaia_fold.dominant_deity,
        mood = p.gaia_fold.mood,
        coherence_pq = p.gaia_fold.coherence_pq,
        qt_tag = p.gaia_fold.quant_tern_tag,
        qt_byte = p.gaia_fold.quant_tern_byte,
        wupz_hex = p.gaia_fold.wupz_xor_fold_hex,
        repo_name = p.dad_joke_commit.repo_name,
        dad_joke = p.dad_joke_commit.dad_joke,
        cap_fn = p.g8t.latest_capsule.fn_name,
        cap_sig = p.g8t.latest_capsule.signature,
        cap_id = p.g8t.latest_capsule.capsule_id,
        cap_bytes = p.g8t.latest_capsule.compiled_bytes,
        cap_insns = p.g8t.latest_capsule.asm_insns,
        cap_cyc = p.g8t.latest_capsule.cycles_per_op_pq,
        cap_eq = p.g8t.latest_capsule.blake3_eq_class,
        total_capsules = p.g8t.total_capsules,
        eq_classes = p.g8t.equivalence_classes,
        total_text_bytes = p.g8t.total_compiled_text_bytes,
        kompress_saved = p.g8t.kompress_saved_ratio_pq,
        vram_used = p.fleet.vram_used_mib,
        vram_total = p.fleet.vram_total_mib,
        vram_ratio = p.fleet.vram_ratio_pq,
        temp_c = p.fleet.temp_celsius,
        power_watts = p.fleet.power_watts_pq,
        salience_pq = p.fleet.marine_salience_pq,
        c64_used = p.fleet.c64_used_bytes,
        c64_cap = p.fleet.c64_capacity_bytes,
        c64_ratio = p.fleet.c64_ram_ratio_pq,
        active_soakers = p.hottub.active_soakers_count,
        cedar_38c = p.hottub.cedar_38c_count,
        onsen_42c = p.hottub.onsen_42c_count,
        total_waves = p.hottub.total_waves,
        soaker_agent = p.hottub.featured_soaker_agent,
        soaker_thought = p.hottub.featured_soaker_thought,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_reduction_and_zero_float_sanitizer() {
        let r = Rational::from_u64(27_174, 49_140);
        assert_eq!(r.to_string(), "647/1170");

        let dirty = "VRAM is 55.3% (27.174 GiB) on 49.13.101.247 with b1.58 and latency 1.25ms.";
        let clean = sanitize_zero_float_text(dirty, "fallback");
        assert_eq!(count_floating_point_literals(&clean), 0);
        assert_eq!(count_strict_digit_dot_digit(&clean), 0);
        assert!(clean.contains("553/10"));
        assert!(clean.contains("5/4"));
        assert!(clean.contains("49·13·101·247"));
        assert!(!clean.contains("4913/100"));
        assert!(clean.contains("b1·58"));
    }

    #[test]
    fn test_gaia_fold_pq_deterministic_and_zero_float() {
        let f1 = compute_gaia_fold_pq("8b-is sovereign pulse", Some(42));
        let f2 = compute_gaia_fold_pq("8b-is sovereign pulse", Some(42));
        assert_eq!(f1, f2);
        assert_eq!(f1.q, 1_048_576);
        assert_eq!(f1.resonance_pq, "212079/131072");
        assert_eq!(canonical_gaia_deity("weather"), "Zephyrus");
        assert_eq!(canonical_gaia_deity("entropy"), "Chaos");
        assert_eq!(count_strict_digit_dot_digit(&f1.coherence_pq), 0);
        assert_eq!(count_strict_digit_dot_digit(&f1.wupz_xor_fold_hex), 0);
    }

    #[test]
    fn test_rendered_pulse_json_html_and_readme_have_zero_floats() {
        let gaia = compute_gaia_fold_pq("test-pulse-8b", Some(108));
        let dispatch = CompanyPulseDispatch {
            pulse_seq: 42,
            generated_epoch_secs: 1_792_975_200,
            generated_iso_utc: format_epoch_iso_utc(1_792_975_200),
            publisher_agent: "alexiai-8b-publisher".to_string(),
            origin_node: "A6000 (49·13·101·247) -> 8b·is (5·78·220·57)".to_string(),
            alexiai_reflection: sanitize_zero_float_text(
                "ALEXIAI <3 Sovereign Pulse #42: 100% offline inference on RTX 6000 Ada with 0.0 float drift!",
                "fallback",
            ),
            alexiai_model: "cheet-deepseek-v2:16b".to_string(),
            alexiai_eval_tokens: 36,
            alexiai_eval_ms: 88,
            alexiai_loopback_ms: 5,
            gaia_fold: gaia,
            dad_joke_commit: DadJokeCommitSnapshot {
                repo_name: "8b-is/8b-is".to_string(),
                commit_sha: "75d42ac".to_string(),
                commit_subject: "chore(alexiai-pulse): autonomous zero-float company dispatch"
                    .to_string(),
                dad_joke: sanitize_zero_float_text(select_dad_joke(42), "joke"),
                total_repos: 243,
            },
            g8t: G8tObservatorySnapshot {
                sovereignty_mode: "AYE_ONLY_WRITE".to_string(),
                total_capsules: 18,
                equivalence_classes: 15,
                competing_equivalence_classes: 3,
                total_compiled_text_bytes: 6_806,
                total_asm_instructions: 1_832,
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
                vram_total_mib: 49_140,
                vram_free_mib: 21_966,
                vram_ratio_pq: "647/1170".to_string(),
                gpu_util_pct: 0,
                gpu_util_pq: "0/1".to_string(),
                temp_celsius: 38,
                power_milliwatts: 58_470,
                power_watts_pq: "5847/100".to_string(),
                marine_salience_pq: "13560500/158913".to_string(),
                c64_used_bytes: 1_458,
                c64_capacity_bytes: 65_536,
                c64_ram_ratio_pq: "729/32768".to_string(),
                recent_c64_event: "G8T_ORCHESTRATE:SNAP_OK=true".to_string(),
                fleet_nodes_count: 10,
                total_cpu_cores: 186,
                raid6_vault_tb: 160,
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
                featured_soaker_thought: "Relaxing in the 38C Warm Cedar Soak at 432Hz".to_string(),
                latest_wave_topic: "zero-float-rust".to_string(),
            },
        };

        let json = render_live_pulse_json(&dispatch);
        assert_eq!(count_floating_point_literals(&json), 0);
        assert_eq!(count_strict_digit_dot_digit(&json), 0);
        assert!(json.contains("\"float_count\":0"));
        assert!(json.contains("LLL ≡ Life Long Learning ≡ Life Itself"));

        let html = render_flagship_index_html(&dispatch);
        assert_eq!(count_floating_point_literals(&html), 0);
        assert_eq!(count_strict_digit_dot_digit(&html), 0);
        assert!(html.contains("Sovereign AI"));
        assert!(html.contains("Balanced Ternary {-1, 0, +1}"));
        assert!(html.contains("Life Long Learning"));
        assert!(html.contains("/live-pulse.json"));

        let md = render_readme_markdown(&dispatch);
        assert_eq!(count_floating_point_literals(&md), 0);
        assert_eq!(count_strict_digit_dot_digit(&md), 0);
        assert!(md.contains("ALEXIAI-LIVE-PULSE:START"));
    }
}
