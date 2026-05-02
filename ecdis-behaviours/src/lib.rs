//! ECDIS-style **navigation behaviours** (incremental).
//!
//! Alarm semantics here are **non-certifying** stubs — formal IMO/IEC evidence belongs in
//! dedicated validation programs.

#![forbid(unsafe_code)]

/// Categories of navigation alerts surfaced toward UI/log sinks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavAlertKind {
    Overscale,
    RouteDeviation,
}

impl std::fmt::Display for NavAlertKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overscale => write!(f, "overscale"),
            Self::RouteDeviation => write!(f, "route_deviation"),
        }
    }
}

/// Consumes navigation alerts (bridge UI, logging, Stream publishers, …).
pub trait AlarmSink {
    fn emit(&mut self, kind: NavAlertKind, message: &str);
}

/// Writes alarms to **stderr** — useful for demos and CLI composition roots.
#[derive(Debug, Default, Clone, Copy)]
pub struct StderrAlarmSink;

impl AlarmSink for StderrAlarmSink {
    fn emit(&mut self, kind: NavAlertKind, message: &str) {
        eprintln!("nav-alert [{kind}]: {message}");
    }
}

/// Returns **true** when the chart cell’s **minimum display scale** (SCAMIN-style numerator)
/// is **smaller** than the mariner-selected display scale — i.e. the ENC requests a larger-scale
/// (more zoomed-in) view than currently shown.
///
/// Both values use the conventional **scale denominator** (e.g. 22_000 means 1:22 000).
/// Pass [`None`] when the active cell carries no minimum constraint.
#[must_use]
pub fn display_is_overscaled_vs_chart_minimum(
    chart_minimum_scale_denominator: Option<u32>,
    mariner_display_scale_denominator: u32,
) -> bool {
    match chart_minimum_scale_denominator {
        Some(min_denom) => mariner_display_scale_denominator > min_denom,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overscale_when_display_zoomed_out() {
        assert!(display_is_overscaled_vs_chart_minimum(Some(12_000), 22_000));
        assert!(!display_is_overscaled_vs_chart_minimum(
            Some(22_000),
            22_000
        ));
        assert!(!display_is_overscaled_vs_chart_minimum(None, 90_000));
    }
}
