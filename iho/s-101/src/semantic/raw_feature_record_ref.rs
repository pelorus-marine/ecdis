/// Reference to one raw feature-shaped row (still bytes — FC assigns meaning).
#[derive(Debug, Clone, Copy)]
pub struct RawFeatureRecordRef<'a> {
    pub record_index: usize,
    pub frid_payload: Option<&'a [u8]>,
}
