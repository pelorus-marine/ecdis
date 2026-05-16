/// CRS parameters from the **DSSI** field on dataset record 0 (empirical layout for current test cells).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntegerCrsParameters {
    pub dco_x: f64,
    pub dco_y: f64,
    pub cmf_x: u32,
    pub cmf_y: u32,
}

impl Default for IntegerCrsParameters {
    fn default() -> Self {
        Self {
            dco_x: 0.0,
            dco_y: 0.0,
            cmf_x: 10_000_000,
            cmf_y: 10_000_000,
        }
    }
}
