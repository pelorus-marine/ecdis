/// **F**eature **O**bject **ID**entifier: registering agency, feature number, feature subdivision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeatureObjectId {
    pub agency: u16,
    pub fidn: u32,
    pub fids: u16,
}

impl FeatureObjectId {
    #[must_use]
    pub const fn new(agency: u16, fidn: u32, fids: u16) -> Self {
        Self { agency, fidn, fids }
    }
}
