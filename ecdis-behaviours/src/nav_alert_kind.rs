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
