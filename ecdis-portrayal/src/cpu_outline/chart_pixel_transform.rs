/// Scale and offsets shared by [`super::CpuOutlinePortrayal::segments_screen_px`] and [`super::CpuOutlinePortrayal::project_wgs84_to_screen_px`].
pub(super) struct ChartPixelTransform {
    pub(super) scale: f64,
    pub(super) cos_vp: f64,
    pub(super) w: f64,
    pub(super) h: f64,
}
