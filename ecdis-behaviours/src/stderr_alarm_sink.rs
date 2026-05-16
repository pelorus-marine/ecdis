use super::{AlarmSink, NavAlertKind};

/// Writes alarms to **stderr** — useful for demos and CLI composition roots.
#[derive(Debug, Default, Clone, Copy)]
pub struct StderrAlarmSink;

impl AlarmSink for StderrAlarmSink {
    fn emit(&mut self, kind: NavAlertKind, message: &str) {
        eprintln!("nav-alert [{kind}]: {message}");
    }
}
