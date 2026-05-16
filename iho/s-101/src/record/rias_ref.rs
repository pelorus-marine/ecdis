#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiasRef {
    pub rrn: u8,
    pub rrid: u32,
    pub ornt: u8,
    pub usag: u8,
    pub raui: u8,
}
