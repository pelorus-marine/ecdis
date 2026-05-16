use super::NavAlertKind;

/// Consumes navigation alerts (bridge UI, logging, Stream publishers, …).
pub trait AlarmSink {
    fn emit(&mut self, kind: NavAlertKind, message: &str);
}
