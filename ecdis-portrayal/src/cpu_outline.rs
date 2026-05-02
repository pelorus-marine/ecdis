//! CPU-side outline portrayal from **C2IL** polylines (WGS84 degrees).

use s_101::{extract_c2il_polylines_wgs84, IntegerCrsParameters, S101Dataset};

use crate::{ChartViewportState, PortrayError, PortrayalPipeline};

const MAX_DECODE_VERTICES: usize = 200_000;
const MAX_DRAW_SEGMENTS: usize = 40_000;

/// Builds screen polylines by decoding **C2IL** chains from an ENC dataset.
#[derive(Debug, Clone)]
pub struct CpuOutlinePortrayal {
    crs: IntegerCrsParameters,
    chains_wgs84: Vec<Vec<(f64, f64)>>,
    /// BBox centre used to initialise mariner pan before user edits.
    pub anchor_lat_deg: f64,
    pub anchor_lon_deg: f64,
}

impl Default for CpuOutlinePortrayal {
    fn default() -> Self {
        Self {
            crs: IntegerCrsParameters::default(),
            chains_wgs84: Vec::new(),
            anchor_lat_deg: 0.0,
            anchor_lon_deg: 0.0,
        }
    }
}

/// Scale and offsets shared by [`CpuOutlinePortrayal::segments_screen_px`] and [`CpuOutlinePortrayal::project_wgs84_to_screen_px`].
struct ChartPixelTransform {
    scale: f64,
    cos_vp: f64,
    w: f64,
    h: f64,
}

impl CpuOutlinePortrayal {
    #[must_use]
    pub fn crs(&self) -> &IntegerCrsParameters {
        &self.crs
    }

    #[must_use]
    pub fn chain_count(&self) -> usize {
        self.chains_wgs84.len()
    }

    fn pixel_transform(
        &self,
        vp: &ChartViewportState,
        width_px: f32,
        height_px: f32,
    ) -> Option<ChartPixelTransform> {
        if self.chains_wgs84.is_empty() {
            return None;
        }

        let w = f64::from(width_px.max(32.0));
        let h = f64::from(height_px.max(32.0));

        let mut mn_lat = f64::MAX;
        let mut mx_lat = f64::MIN;
        let mut mn_lon = f64::MAX;
        let mut mx_lon = f64::MIN;
        for ch in &self.chains_wgs84 {
            for (lat, lon) in ch {
                mn_lat = mn_lat.min(*lat);
                mx_lat = mx_lat.max(*lat);
                mn_lon = mn_lon.min(*lon);
                mx_lon = mx_lon.max(*lon);
            }
        }

        let lat_span = (mx_lat - mn_lat).max(1e-8);
        let lon_span = (mx_lon - mn_lon).max(1e-8);
        let mid_lat = (mn_lat + mx_lat) * 0.5;
        let cos_mid = mid_lat.to_radians().cos().abs().max(0.12);

        let fit_lon = lon_span * cos_mid;
        let fit_scale = (w / fit_lon).min(h / lat_span) * 0.88;
        let denom_zoom =
            (22_000f64 / f64::from(vp.scale_denominator.max(500))).sqrt().clamp(0.35, 6.0);
        let scale = fit_scale * denom_zoom;

        let cos_vp = vp.center_lat_deg.to_radians().cos().abs().max(0.12);

        Some(ChartPixelTransform {
            scale,
            cos_vp,
            w,
            h,
        })
    }

    /// Project WGS84 degrees to **same** chart pixel space as [`Self::segments_screen_px`].
    #[must_use]
    pub fn project_wgs84_to_screen_px(
        &self,
        vp: &ChartViewportState,
        lat_deg: f64,
        lon_deg: f64,
        width_px: f32,
        height_px: f32,
    ) -> Option<(f32, f32)> {
        let t = self.pixel_transform(vp, width_px, height_px)?;
        let x = ((lon_deg - vp.center_lon_deg) * t.scale * t.cos_vp + t.w * 0.5) as f32;
        let y = (t.h * 0.5 - (lat_deg - vp.center_lat_deg) * t.scale) as f32;
        Some((x, y))
    }

    fn bbox_center_deg(chains: &[Vec<(f64, f64)>]) -> Option<(f64, f64)> {
        let mut mn_lat = f64::MAX;
        let mut mx_lat = f64::MIN;
        let mut mn_lon = f64::MAX;
        let mut mx_lon = f64::MIN;
        for ch in chains {
            for (lat, lon) in ch {
                mn_lat = mn_lat.min(*lat);
                mx_lat = mx_lat.max(*lat);
                mn_lon = mn_lon.min(*lon);
                mx_lon = mx_lon.max(*lon);
            }
        }
        if !(mn_lat <= mx_lat && mn_lon <= mx_lon) {
            return None;
        }
        Some(((mn_lat + mx_lat) * 0.5, (mn_lon + mx_lon) * 0.5))
    }

    /// Map WGS84 vertices to chart-area pixels using viewport centre + scale denominator.
    #[must_use]
    pub fn segments_screen_px(
        &self,
        vp: &ChartViewportState,
        width_px: f32,
        height_px: f32,
    ) -> Vec<(f32, f32, f32, f32)> {
        let Some(t) = self.pixel_transform(vp, width_px, height_px) else {
            return Vec::new();
        };

        let mut out = Vec::new();
        for ch in &self.chains_wgs84 {
            if out.len() >= MAX_DRAW_SEGMENTS {
                break;
            }
            for win in ch.windows(2) {
                if out.len() >= MAX_DRAW_SEGMENTS {
                    break;
                }
                let (lat1, lon1) = win[0];
                let (lat2, lon2) = win[1];
                let x1 = ((lon1 - vp.center_lon_deg) * t.scale * t.cos_vp + t.w * 0.5) as f32;
                let y1 = (t.h * 0.5 - (lat1 - vp.center_lat_deg) * t.scale) as f32;
                let x2 = ((lon2 - vp.center_lon_deg) * t.scale * t.cos_vp + t.w * 0.5) as f32;
                let y2 = (t.h * 0.5 - (lat2 - vp.center_lat_deg) * t.scale) as f32;
                out.push((x1, y1, x2, y2));
            }
        }
        out
    }
}

impl PortrayalPipeline for CpuOutlinePortrayal {
    fn reset_for_chart(&mut self, chart: &S101Dataset) -> Result<(), PortrayError> {
        let (crs, chains) = extract_c2il_polylines_wgs84(chart, MAX_DECODE_VERTICES);
        self.crs = crs;
        self.chains_wgs84 = chains;
        if let Some((lat, lon)) = Self::bbox_center_deg(&self.chains_wgs84) {
            self.anchor_lat_deg = lat;
            self.anchor_lon_deg = lon;
        } else {
            self.anchor_lat_deg = 0.0;
            self.anchor_lon_deg = 0.0;
        }
        Ok(())
    }

    fn set_display_scale(&mut self, _scale_denominator: u32) -> Result<(), PortrayError> {
        // LOD / cartographic generalisation not implemented — viewport handles zoom visually.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use s_101::S101Dataset;

    #[test]
    fn outline_populates_when_env_enc_present() {
        let Ok(p) = std::env::var("S101_TEST_ENC") else {
            return;
        };
        if !std::path::Path::new(&p).exists() {
            return;
        }
        let chart = S101Dataset::load(p).unwrap();
        let mut p = CpuOutlinePortrayal::default();
        p.reset_for_chart(&chart).unwrap();
        assert!(p.chain_count() > 0);
        let vp = ChartViewportState {
            center_lat_deg: p.anchor_lat_deg,
            center_lon_deg: p.anchor_lon_deg,
            scale_denominator: 22_000,
        };
        let seg = p.segments_screen_px(&vp, 800.0, 600.0);
        assert!(!seg.is_empty());
        let proj = p
            .project_wgs84_to_screen_px(&vp, p.anchor_lat_deg, p.anchor_lon_deg, 800.0, 600.0)
            .expect("anchor projects");
        assert!(
            (proj.0 - 400.0).abs() < 0.5 && (proj.1 - 300.0).abs() < 0.5,
            "viewport centre projects to canvas centre: {proj:?}"
        );
    }
}
