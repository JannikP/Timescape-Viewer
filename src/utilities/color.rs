use glam::Vec4;
use iced::Color;

pub trait ColorExt {
    fn to_vec4(&self) -> Vec4;
}

impl ColorExt for Color {
    fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
}

// ---------------------------------------------------------------------------
// HSV <-> RGB helpers
// ---------------------------------------------------------------------------

/// Convert iced::Color to HSV (h: 0-360, s: 0-1, v: 0-1).
pub fn color_to_hsv(c: Color) -> (f32, f32, f32) {
    let r = c.r;
    let g = c.g;
    let b = c.b;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;

    let v = max;
    let s = if max.abs() < 1e-6 { 0.0 } else { d / max };

    let h = if d.abs() < 1e-6 {
        0.0
    } else if (max - r).abs() < 1e-6 {
        60.0 * (((g - b) / d) % 6.0)
    } else if (max - g).abs() < 1e-6 {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };

    let h = ((h % 360.0) + 360.0) % 360.0;
    (h, s, v)
}

/// Convert HSV (h: 0-360, s: 0-1, v: 0-1) to iced::Color.
pub fn hsv_to_color(h: f32, s: f32, v: f32) -> Color {
    let h = ((h % 360.0) + 360.0) % 360.0;
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::from_rgb(r + m, g + m, b + m)
}

/// Convert HSV to hex string (#RRGGBB).
pub fn hsv_to_hex(h: f32, s: f32, v: f32) -> String {
    let c = hsv_to_color(h, s, v);
    format!(
        "#{:02X}{:02X}{:02X}",
        (c.r * 255.0).round() as u8,
        (c.g * 255.0).round() as u8,
        (c.b * 255.0).round() as u8,
    )
}

/// Parse a hex string (#RRGGBB or RRGGBB) to iced::Color.
pub fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::from_rgb8(r, g, b))
}
