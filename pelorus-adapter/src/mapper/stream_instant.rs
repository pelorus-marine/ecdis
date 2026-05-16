/// Monotonic-friendly observation time carried alongside telemetry (nanoseconds since Unix epoch).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamInstant {
    pub unix_nanos: u128,
    pub source_label: Option<String>,
}
