use iced::animation::{Float, Interpolable};

/// The [Domain] of an axis with either linear or logarithmic scaling.
/// This structure is intended to be used for handling the physical axis (y-axis) range of line
/// charts and provides manipulation functions to support common navigation actions such as pan
/// and zoom.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Domain {
    Linear { minimum: f32, maximum: f32 },

    Logarithmic { minimum: f32, maximum: f32 },
}

impl Domain {
    /// Creates a linear scaled [Domain]. The maximum value must be strictly larger than the
    /// minimum value or this function panics.
    #[must_use]
    pub fn linear(minimum: f32, maximum: f32) -> Self {
        assert!(maximum > minimum);
        Self::Linear { minimum, maximum }
    }

    /// Creates a logarithmically scaled [Domain]. The maximum value must be strictly larger than
    /// the minimum value and the minimum value must be larger than zero or this function panics.
    #[must_use]
    pub fn logarithmic(minimum: f32, maximum: f32) -> Self {
        assert!(maximum > minimum);
        assert!(minimum > 0.0);
        Self::Logarithmic { minimum, maximum }
    }

    /// The smallest value included in this [Domain].
    /// (inclusive minimum)
    #[must_use]
    pub fn minimum(&self) -> f32 {
        match self {
            Domain::Linear {
                minimum,
                maximum: _,
            } => *minimum,
            Domain::Logarithmic {
                minimum,
                maximum: _,
            } => *minimum,
        }
    }

    /// The largest value just no longer included in this [Domain].
    /// (exclusive maximum)
    #[must_use]
    pub fn maximum(&self) -> f32 {
        match self {
            Domain::Linear {
                minimum: _,
                maximum,
            } => *maximum,
            Domain::Logarithmic {
                minimum: _,
                maximum,
            } => *maximum,
        }
    }

    /// Returns `true` if this [Domain] is linear scaled.
    #[must_use]
    pub fn is_linear(&self) -> bool {
        match self {
            Domain::Linear {
                minimum: _,
                maximum: _,
            } => true,
            Domain::Logarithmic {
                minimum: _,
                maximum: _,
            } => false,
        }
    }

    /// Returns `true` if this [Domain] is logarithmically scaled.
    #[must_use]
    pub fn is_logarithmic(&self) -> bool {
        match self {
            Domain::Linear {
                minimum: _,
                maximum: _,
            } => false,
            Domain::Logarithmic {
                minimum: _,
                maximum: _,
            } => true,
        }
    }

    /// Toggles between linear and logarithmic scaling. Returns `None` if that's not possible, most
    /// likely in the case the linear [Domain] contains zero or negative numbers which is impossible
    /// logarithmic scaling.
    #[must_use]
    pub fn toggle(&self) -> Option<Domain> {
        match self {
            Domain::Linear { minimum, maximum } => {
                if *minimum > 0.0 {
                    Some(Domain::logarithmic(*minimum, *maximum))
                } else {
                    None
                }
            }
            Domain::Logarithmic { minimum, maximum } => Some(Domain::linear(*minimum, *maximum)),
        }
    }

    /// Checks if the [Domain]'s scaling can be toggled.
    /// Logarithmic scaling can always be switched to linear scaling, but linear scaling can only
    /// be changed to logarithmic scaling if the minimum value is larger than zero.
    #[must_use]
    pub fn can_toggle(&self) -> bool {
        match self {
            Domain::Linear {
                minimum,
                maximum: _,
            } => *minimum > 0.0,
            Domain::Logarithmic {
                minimum: _,
                maximum: _,
            } => true,
        }
    }

    /// Performs a zoom action on this [Domain]. The `pivot` point (physical value) value will
    /// remain stationary while the range zooms in or out around it. A scaling `factor` of 0.0..1.0
    /// zooms in, so a smaller [Domain] will be visible and a factor larger than 1.0 zooms out.
    /// If the function is called with a `factor` of 1.0, nothing changes.
    pub fn zoom(&mut self, pivot: f32, factor: f32) {}

    /// How much to shift the physical [Domain] (y-axis). A positive value increases
    /// the minimum and maximum values, causing the trace lines to move down on screen.
    /// The translation between mouse motion and physical values is handled by the widget.
    pub fn pan(&mut self, delta: f32) {}

    /// Returns `true` if the physical `value` is contained by this [Domain].
    #[must_use]
    pub fn contains(&self, value: f32) -> bool {
        self.minimum() <= value && value < self.maximum()
    }

    /// Maps a physical `value` to relative pixel coordinates. In order to do so, the screen `size`
    /// in logical pixels is needed.
    #[must_use]
    pub fn map_physical_to_pixel(&self, value: f32, size: f32) -> f32 {
        todo!()
    }

    /// Maps a relative pixel `position` to physical values covered by this range. In order to do
    /// so, the screen `size` in logical pixels is needed.
    #[must_use]
    pub fn map_pixel_to_physical(&self, position: f32, size: f32) -> f32 {
        todo!()
    }
}

impl Interpolable for Domain {
    fn interpolated(&self, other: Self, ratio: f32) -> Self {
        match (self, other) {
            (
                Domain::Linear {
                    minimum: s_min,
                    maximum: s_max,
                },
                Domain::Linear {
                    minimum: o_min,
                    maximum: o_max,
                },
            ) => {
                let minimum = s_min.interpolated(o_min, ratio);
                let maximum = s_max.interpolated(o_max, ratio);
                Domain::linear(minimum, maximum)
            }
            (
                Domain::Linear {
                    minimum: s_min,
                    maximum: s_max,
                },
                Domain::Logarithmic {
                    minimum: o_min,
                    maximum: o_max,
                },
            ) => {
                let minimum = s_min.interpolated(o_min, ratio);
                let maximum = s_max.interpolated(o_max, ratio);
                if ratio < 0.5 || minimum <= 0.0 {
                    Domain::linear(minimum, maximum)
                } else {
                    Domain::logarithmic(minimum, maximum)
                }
            }
            (
                Domain::Logarithmic {
                    minimum: s_min,
                    maximum: s_max,
                },
                Domain::Logarithmic {
                    minimum: o_min,
                    maximum: o_max,
                },
            ) => {
                let minimum = s_min.interpolated(o_min, ratio);
                let maximum = s_max.interpolated(o_max, ratio);
                Domain::logarithmic(minimum, maximum)
            }
            (
                Domain::Logarithmic {
                    minimum: s_min,
                    maximum: s_max,
                },
                Domain::Linear {
                    minimum: o_min,
                    maximum: o_max,
                },
            ) => {
                let minimum = s_min.interpolated(o_min, ratio);
                let maximum = s_max.interpolated(o_max, ratio);
                if ratio > 0.5 || minimum <= 0.0 {
                    Domain::linear(minimum, maximum)
                } else {
                    Domain::logarithmic(minimum, maximum)
                }
            }
        }
    }
}

impl From<std::ops::Range<f32>> for Domain {
    fn from(value: std::ops::Range<f32>) -> Self {
        Self::linear(value.start, value.end)
    }
}

impl Into<std::ops::Range<f32>> for Domain {
    fn into(self) -> std::ops::Range<f32> {
        self.minimum()..self.maximum()
    }
}

impl Default for Domain {
    fn default() -> Self {
        Self::linear(0.0, 1.0)
    }
}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Domain::Linear { minimum, maximum } => write!(f, "{}..{}", minimum, maximum),
            Domain::Logarithmic { minimum, maximum } => write!(f, "{}..{} (log)", minimum, maximum),
        }
    }
}

// FIXME: This is absolutely stupid. A domain can't be represented by a single f32, but this is for
// some reason required by iced::animation::Animation.
impl Float for Domain {
    fn float_value(&self) -> f32 {
        self.minimum()
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f32_near;

    use super::*;

    #[test]
    fn toggle_valid_domain() {
        let domain = Domain::linear(2.0, 4.0);
        let result = domain.toggle();
        assert!(result.is_some_and(|r| r.is_logarithmic()));
    }

    #[test]
    fn do_not_toggle_invalid_domain() {
        let domain = Domain::linear(-2.0, 4.0);
        let result = domain.toggle();
        assert!(result.is_none());
    }

    #[test]
    fn can_toggle_valid_domain() {
        assert!(Domain::linear(2.0, 4.0).can_toggle())
    }

    #[test]
    fn can_not_toggle_invalid_domain() {
        assert!(!Domain::linear(-2.0, 4.0).can_toggle())
    }

    #[test]
    fn interpolate_linear_to_linear() {
        let a = Domain::linear(2.0, 4.0);
        let b = Domain::linear(-2.0, 8.0);

        let s1 = a.interpolated(b.clone(), 0.25);
        assert!(s1.is_linear());
        assert_f32_near!(s1.minimum(), 1.0);
        assert_f32_near!(s1.maximum(), 5.0);

        let s2 = a.interpolated(b.clone(), 0.50);
        assert!(s2.is_linear());
        assert_f32_near!(s2.minimum(), 0.0);
        assert_f32_near!(s2.maximum(), 6.0);

        let s3 = a.interpolated(b.clone(), 0.75);
        assert!(s3.is_linear());
        assert_f32_near!(s3.minimum(), -1.0);
        assert_f32_near!(s3.maximum(), 7.0);
    }
}
