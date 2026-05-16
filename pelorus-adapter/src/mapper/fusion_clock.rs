use std::time::{Duration, Instant};

/// Cheap monotonic clock anchor for correlating samples before GNSS time is available.
#[derive(Debug, Clone)]
pub struct FusionClock {
    origin: Instant,
}

impl Default for FusionClock {
    fn default() -> Self {
        Self::new()
    }
}

impl FusionClock {
    #[must_use]
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }

    pub fn elapsed_monotonic(&self) -> Duration {
        self.origin.elapsed()
    }
}
