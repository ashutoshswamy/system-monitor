//! Number formatting for readouts.

pub fn pct(used: u64, total: u64) -> f32 {
    if total == 0 { 0. } else { used as f32 / total as f32 * 100. }
}

/// Decimal units (macOS Finder convention).
pub fn bytes(b: f64) -> String {
    const U: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut v = b;
    let mut i = 0;
    while v >= 1000. && i < U.len() - 1 {
        v /= 1000.;
        i += 1;
    }
    if i == 0 || v >= 100. { format!("{v:.0} {}", U[i]) } else { format!("{v:.1} {}", U[i]) }
}

pub fn uptime(s: u64) -> String {
    let (d, h, m) = (s / 86400, s / 3600 % 24, s / 60 % 60);
    if d > 0 { format!("{d}d {h}h {m}m") } else { format!("{h}h {m}m") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting() {
        assert_eq!(bytes(512.), "512 B");
        assert_eq!(bytes(1_500_000.), "1.5 MB");
        assert_eq!(bytes(250_000_000.), "250 MB");
        assert_eq!(uptime(90_061), "1d 1h 1m");
        assert_eq!(uptime(3_660), "1h 1m");
        assert_eq!(pct(1, 0), 0.);
        assert_eq!(pct(1, 4), 25.);
    }
}
