use iced::mouse::ScrollDelta;

const PIXELS_PER_LINE: f32 = 16.0;

/// Zoom speed. The zoom factor is calculated by $exp{k_z*x}$ where $x$ is the number of pixels
/// scrolled. This factor is computed by solving $(1 + y) = exp{k_z * x}$ for the desired zoom
/// factor $y$ given a scroll delta of $x$ pixels for the [ZOOM_SPEED] $k_z$.
const ZOOM_SPEED: f32 = 0.00198563;

/// Translates [ScrollDelta] steps to how much to zoom the time or physical domain.
/// A value of `1.0` indicates no change, a value 0..1.0 zooms in and a value > 1.0 zooms out
/// (shows more).
pub fn scroll_to_zoom(delta: &ScrollDelta) -> f32 {
    let normalized_pixels = match delta {
        ScrollDelta::Lines { x: _, y } => *y * PIXELS_PER_LINE,
        ScrollDelta::Pixels { x: _, y } => *y,
    };
    (-1.0 * normalized_pixels * ZOOM_SPEED).exp()
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f32_near;

    use super::*;

    #[test]
    fn zoom_out_pixels() {
        let delta = ScrollDelta::Pixels { x: 0.0, y: -48.0 };
        let result = scroll_to_zoom(&delta);
        assert_f32_near!(result, 1.10);
    }

    #[test]
    fn zoom_out_lines() {
        let delta = ScrollDelta::Lines { x: 0.0, y: -3.0 };
        let result = scroll_to_zoom(&delta);
        assert_f32_near!(result, 1.10);
    }

    #[test]
    fn zoom_in_pixels() {
        let delta = ScrollDelta::Pixels { x: 0.0, y: 48.0 };
        let result = scroll_to_zoom(&delta);
        assert_f32_near!(result, 0.9090909);
    }

    #[test]
    fn zoom_in_lines() {
        let delta = ScrollDelta::Lines { x: 0.0, y: 3.0 };
        let result = scroll_to_zoom(&delta);
        assert_f32_near!(result, 0.9090909);
    }
}
