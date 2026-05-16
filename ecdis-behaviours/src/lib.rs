//! ECDIS-style **navigation behaviours** (incremental).
//!
//! Alarm semantics here are **non-certifying** stubs — formal IMO/IEC evidence belongs in
//! dedicated validation programs.
//!
//! Each public type lives in its own file; this `lib.rs` is just the namespace assembly point.

#![forbid(unsafe_code)]

mod alarm_sink;
mod nav_alert_kind;
mod stderr_alarm_sink;

pub use alarm_sink::AlarmSink;
pub use nav_alert_kind::NavAlertKind;
pub use stderr_alarm_sink::StderrAlarmSink;

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
