//! Slint UI bindings for the portrayal gallery viewer.

slint::include_modules!();

use ecdis_portrayal::PortrayalFrame;

pub fn apply_frame(ui: &ViewerWindow, frame: &PortrayalFrame) {
    ui.set_chart_background(parse_brush(&frame.theme.background.to_hex()).into());
    ui.set_caption_label(frame.layers.caption.clone().into());

    let mut segments: Vec<LineSeg> = frame
        .layers
        .segments
        .iter()
        .map(|s| LineSeg {
            x1: s.x1,
            y1: s.y1,
            x2: s.x2,
            y2: s.y2,
            color: parse_brush(&s.stroke.to_hex()).into(),
            width: s.width_px,
        })
        .collect();

    for poly in &frame.layers.polygons {
        let stroke = parse_brush(&poly.stroke.to_hex());
        for w in poly.vertices.windows(2) {
            segments.push(LineSeg {
                x1: w[0].0,
                y1: w[0].1,
                x2: w[1].0,
                y2: w[1].1,
                color: stroke.clone(),
                width: 1.5,
            });
        }
        if let (Some(first), Some(last)) = (poly.vertices.first(), poly.vertices.last()) {
            segments.push(LineSeg {
                x1: last.0,
                y1: last.1,
                x2: first.0,
                y2: first.1,
                color: stroke,
                width: 1.5,
            });
        }
    }

    ui.set_line_segments(slint::ModelRc::new(slint::VecModel::from(segments)));

    let swatches: Vec<SwatchItem> = frame
        .layers
        .swatches
        .iter()
        .map(|sw| SwatchItem {
            token: sw.token.clone().into(),
            x: sw.x,
            y: sw.y,
            size: sw.size_px,
            color: parse_brush(&sw.rgb.to_hex()).into(),
        })
        .collect();
    ui.set_swatches(slint::ModelRc::new(slint::VecModel::from(swatches)));

    let points: Vec<PointItem> = frame
        .layers
        .points
        .iter()
        .map(|p| PointItem {
            x: p.x,
            y: p.y,
            radius: p.radius_px,
            fill: parse_brush(&p.fill.to_hex()).into(),
            stroke: parse_brush(&p.stroke.to_hex()).into(),
        })
        .collect();
    ui.set_points(slint::ModelRc::new(slint::VecModel::from(points)));

    if let Some(sym) = frame.layers.symbols.first() {
        let img = rgba_to_image(sym.width_px, sym.height_px, &sym.rgba);
        ui.set_symbol_image(img);
        ui.set_symbol_visible(true);
        ui.set_symbol_x(sym.x);
        ui.set_symbol_y(sym.y);
        ui.set_symbol_id_label(sym.symbol_id.clone().into());
    } else {
        ui.set_symbol_visible(false);
        ui.set_symbol_id_label("".into());
    }
}

fn parse_brush(hex: &str) -> slint::Brush {
    slint::Brush::from(parse_hex_color(hex).unwrap_or(slint::Color::from_rgb_u8(200, 200, 200)))
}

fn parse_hex_color(hex: &str) -> Option<slint::Color> {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    Some(slint::Color::from_rgb_u8(r, g, b))
}

fn rgba_to_image(width: u32, height: u32, rgba: &[u8]) -> slint::Image {
    let mut buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
    let bytes = buffer.make_mut_bytes();
    let len = (width as usize * height as usize * 4).min(rgba.len()).min(bytes.len());
    bytes[..len].copy_from_slice(&rgba[..len]);
    slint::Image::from_rgba8(buffer)
}
