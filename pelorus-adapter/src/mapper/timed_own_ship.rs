use crate::OwnShip;

use super::StreamInstant;

/// Own-ship snapshot plus optional Stream/Core observation time.
#[derive(Debug, Clone, PartialEq)]
pub struct TimedOwnShip {
    pub ship: OwnShip,
    pub observed_at: Option<StreamInstant>,
}
