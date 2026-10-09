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

    /// Computes the midpoint of this [Domain].
    #[must_use]
    pub fn middle(&self) -> f32 {
        match self {
            Domain::Linear { minimum, maximum } => 0.5 * (*minimum + *maximum),
            Domain::Logarithmic { minimum, maximum } => {
                let log_min = minimum.log10();
                let log_max = maximum.log10();
                let log_mid = 0.5 * (log_min + log_max);
                10.0f32.powf(log_mid)
            }
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
    pub fn zoom(&self, pivot: f32, factor: f32) -> Domain {
        match self {
            Domain::Linear { minimum, maximum } => {
                let above = (maximum - pivot) * factor;
                let below = (minimum - pivot) * factor;
                Domain::linear(pivot + below, pivot + above)
            }
            Domain::Logarithmic { minimum, maximum } => {
                let log_min = minimum.log10();
                let log_max = maximum.log10();
                let log_piv = pivot.log10();
                let above = (log_max - log_piv) * factor;
                let below = (log_min - log_piv) * factor;
                let log_min = log_piv + below;
                let log_max = log_piv + above;
                Domain::logarithmic(10f32.powf(log_min), 10f32.powf(log_max))
            }
        }
    }

    /// How much to shift the physical [Domain] (y-axis). A positive value increases
    /// the minimum and maximum values, causing the trace lines to move down on screen.
    /// The translation between mouse motion and physical values is handled by the widget.
    pub fn pan(&self, delta: f32) -> Domain {
        match self {
            Domain::Linear { minimum, maximum } => {
                Domain::linear(*minimum + delta, *maximum + delta)
            }
            Domain::Logarithmic { minimum, maximum } => {
                Domain::logarithmic(*minimum + delta, *maximum + delta)
            }
        }
    }

    /// Returns `true` if the physical `value` is contained by this [Domain].
    #[must_use]
    pub fn contains(&self, value: f32) -> bool {
        self.minimum() <= value && value < self.maximum()
    }

    /// Maps a physical `value` to relative pixel coordinates. In order to do so, the screen `size`
    /// in logical pixels is needed.
    ///
    /// ```text
    /// physical maximum ─┰─ pixel 0
    ///    :              ┃    :
    /// physical value   ─╂─ return value
    /// physical minimum ─┸─ pixel size
    /// ```
    #[must_use]
    pub fn map_physical_to_pixel(&self, value: f32, size: f32) -> f32 {
        match self {
            Domain::Linear { minimum, maximum } => {
                size - (value - minimum) * size / (maximum - minimum)
            }
            Domain::Logarithmic { minimum, maximum } => {
                let log_min = minimum.log10();
                let log_max = maximum.log10();
                let log_val = value.log10();
                size - (log_val - log_min) * size / (log_max - log_min)
            }
        }
    }

    /// Maps a relative pixel `position` to physical values covered by this range. In order to do
    /// so, the screen `size` in logical pixels is needed.
    /// ///
    /// ```text
    /// pixel 0        ─┰─ physical maximum
    ///   :             ┃    :
    /// pixel position ─╂─ return value
    /// pixel size     ─┸─ physical minimum
    /// ```
    #[must_use]
    pub fn map_pixel_to_physical(&self, position: f32, size: f32) -> f32 {
        match self {
            Domain::Linear { minimum, maximum } => {
                (size - position) * (maximum - minimum) / size + minimum
            }
            Domain::Logarithmic { minimum, maximum } => {
                let log_min = minimum.log10();
                let log_max = maximum.log10();
                let log_val = (size - position) * (log_max - log_min) / size + log_min;
                10.0f32.powf(log_val)
            }
        }
    }

    #[must_use]
    pub fn ticks(
        &self,
        size: f32,
        major_tick_spacing: f32,
        minor_tick_spacing: f32,
        extra_ticks: &[f32],
    ) -> Vec<Tick> {
        let minimum = self.minimum();
        let maximum = self.maximum();

        // Find nice major ticks
        let major_ticks = (size / major_tick_spacing).floor();
        let ideal_tick = (maximum - minimum) / major_ticks;
        let nice_tick = next_nice_number(ideal_tick);
        let top = (maximum / nice_tick).ceil() * nice_tick;
        let bottom = (minimum / nice_tick).floor() * nice_tick;
        let naive_count = (top - bottom) / nice_tick + 1.0;

        // Distributing the major ticks
        // TODO: The distribution of major ticks needs to be reworked for logarithmic scaling.
        let mut ticks = Vec::with_capacity(naive_count as usize);
        let mut value = bottom + nice_tick;
        while value < top {
            let position = self.map_physical_to_pixel(value, size);
            if position >= 0.0 && position < size {
                let tick = Tick {
                    value,
                    position,
                    significance: Significance::Major,
                };
                ticks.push(tick);
            }
            value += nice_tick;
        }

        // Distribute the minor ticks
        // TODO: The distribution of minor ticks needs to be reworked for logarithmic scaling.
        let actual_major_tick_spacing = nice_tick * size / (maximum - minimum);
        let minor_tick = nice_minor_ticks(nice_tick, actual_major_tick_spacing, minor_tick_spacing);
        let naive_count = (maximum - minimum) / minor_tick + 1.0;
        let mut minor_ticks = Vec::with_capacity(naive_count as usize);
        let mut previous_major_value = bottom;
        for major_tick in ticks.iter() {
            value = previous_major_value + minor_tick;
            while value < major_tick.value {
                let position = self.map_physical_to_pixel(value, size);
                if position >= 0.0 && position < size {
                    let tick = Tick {
                        value,
                        position,
                        significance: Significance::Minor,
                    };
                    minor_ticks.push(tick);
                }
                value += minor_tick;
            }
            previous_major_value = major_tick.value;
        }
        ticks.extend(minor_ticks);

        // Add extra ticks and remove ordinary ticks to close to the extra ones.
        let spacing = |significance| match significance {
            Significance::Minor => minor_tick_spacing,
            Significance::Major | Significance::Extra => major_tick_spacing,
        };
        extra_ticks
            .iter()
            .map(|value| Tick {
                value: *value,
                position: self.map_physical_to_pixel(*value, size),
                significance: Significance::Extra,
            })
            .for_each(|extra| {
                // Check for close major ticks and remove them.
                remove_close(&extra, &mut ticks, spacing);
                // Add the extra tick as major tick
                ticks.push(extra);
            });

        ticks
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
        Self::linear(-5.0, 105.0)
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Significance {
    #[default]
    Minor,
    Major,
    Extra,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    pub value: f32,
    pub position: f32,
    pub significance: Significance,
}

fn next_power_of_ten(number: f32) -> f32 {
    if number > 0.0 {
        10f32.powf(number.log10().ceil())
    } else if number < 0.0 {
        -1.0 * 10f32.powf((-number).log10().ceil())
    } else {
        0.0
    }
}

fn next_nice_number(number: f32) -> f32 {
    let nice_tick = next_power_of_ten(number);
    if nice_tick > 4.0 * number {
        nice_tick / 4.0
    } else if nice_tick > 2.0 * number {
        nice_tick / 2.0
    } else {
        nice_tick
    }
}

fn nice_minor_ticks(major_tick: f32, major_tick_spacing: f32, minor_tick_spacing: f32) -> f32 {
    let ideal_ticks = major_tick_spacing / minor_tick_spacing;
    let nice_ticks = if ideal_ticks >= 10.0 {
        10.0
    } else if ideal_ticks >= 5.0 {
        5.0
    } else if ideal_ticks >= 2.0 {
        2.0
    } else {
        1.0
    };
    major_tick / nice_ticks
}

fn remove_close<F>(extra: &Tick, ticks: &mut Vec<Tick>, spacing: F)
where
    F: Fn(Significance) -> f32,
{
    ticks.retain(|tick| {
        let distance = (extra.position - tick.position).abs();
        distance > spacing(tick.significance)
    });
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
    fn zoom_linear() {
        let domain = Domain::linear(2.0, 4.0);
        let result = domain.zoom(3.0, 1.5);
        assert_f32_near!(result.minimum(), 1.5);
        assert_f32_near!(result.maximum(), 4.5);
    }

    #[test]
    fn zoom_linear_pivot_at_ceiling() {
        let domain = Domain::linear(2.0, 4.0);
        let result = domain.zoom(4.0, 1.5);
        assert_f32_near!(result.minimum(), 1.0);
        assert_f32_near!(result.maximum(), 4.0);
    }

    #[test]
    fn map_physical_to_pixel_linear() {
        let domain = Domain::linear(2.0, 4.0);
        let result = domain.map_physical_to_pixel(3.0, 200.0);
        assert_f32_near!(result, 100.0);
    }

    #[test]
    fn map_physical_to_pixel_logarithmic() {
        let domain = Domain::logarithmic(1.0, 100.0);
        let result = domain.map_physical_to_pixel(10.0, 200.0);
        assert_f32_near!(result, 100.0);
    }

    #[test]
    fn map_pixel_to_physical_linear() {
        let domain = Domain::linear(2.0, 4.0);
        let result = domain.map_pixel_to_physical(100.0, 200.0);
        assert_f32_near!(result, 3.0);
    }

    #[test]
    fn map_pixel_to_physical_logarithmic() {
        let domain = Domain::logarithmic(1.0, 100.0);
        let result = domain.map_pixel_to_physical(100.0, 200.0);
        assert_f32_near!(result, 10.0);
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

    #[test]
    fn distribute_linear_ticks_simple_example() {
        let domain = Domain::linear(-5.0, 105.0);
        let ticks = domain.ticks(300.0, 64.0, 16.0, &[42.0]);
        assert_eq!(ticks.len(), 10);
        // Two major ticks
        assert_eq!(ticks[0].value, 0.0);
        assert_eq!(ticks[1].value, 100.0);
        // One extra tick at the end
        assert_eq!(ticks[9].value, 42.0);
    }

    #[test]
    fn distribute_linear_ticks_default_line_chart() {
        let domain = Domain::linear(-5.0, 105.0);
        let ticks = domain.ticks(150.0, 64.0, 16.0, &[]);
        assert_eq!(ticks.len(), 6);
        // Two major ticks
        assert_eq!(ticks[0].value, 0.0);
        assert_eq!(ticks[1].value, 100.0);
        // Many minor ticks
    }

    #[test]
    fn next_power_of_ten_positive_numbers() {
        assert_f32_near!(next_power_of_ten(987.0), 1000.0);
        assert_f32_near!(next_power_of_ten(99_999.0), 100_000.0);
        assert_f32_near!(next_power_of_ten(999_999_999.0), 1_000_000_000.0);
    }

    #[test]
    fn next_power_of_ten_negative_numbers() {
        assert_f32_near!(next_power_of_ten(-987.0), -1000.0);
        assert_f32_near!(next_power_of_ten(-99_999.0), -100_000.0);
        assert_f32_near!(next_power_of_ten(-999_999_999.0), -1_000_000_000.0);
    }

    #[test]
    fn next_power_of_ten_zero() {
        assert_f32_near!(next_power_of_ten(0.0), 0.0);
    }

    #[test]
    fn nice_tick_small() {
        assert_f32_near!(next_nice_number(9.0), 10.0);
        assert_f32_near!(next_nice_number(4.0), 5.0);
        assert_f32_near!(next_nice_number(1.8), 2.5);
        assert_f32_near!(next_nice_number(0.9), 1.0);
    }

    #[test]
    fn ten_nice_minor_ticks() {
        assert_eq!(nice_minor_ticks(100.0, 160.0, 12.0), 10.0);
        assert_eq!(nice_minor_ticks(100.0, 160.0, 15.9), 10.0);
        assert_eq!(nice_minor_ticks(0.1, 160.0, 12.0), 0.01);
    }

    #[test]
    fn five_nice_minor_ticks() {
        assert_eq!(nice_minor_ticks(100.0, 160.0, 18.0), 20.0);
        assert_eq!(nice_minor_ticks(100.0, 160.0, 31.9), 20.0);
        assert_eq!(nice_minor_ticks(0.1, 160.0, 18.0), 0.02);
    }

    #[test]
    fn two_nice_minor_ticks() {
        assert_eq!(nice_minor_ticks(100.0, 160.0, 70.0), 50.0);
        assert_eq!(nice_minor_ticks(100.0, 160.0, 79.9), 50.0);
        assert_eq!(nice_minor_ticks(0.1, 160.0, 70.0), 0.05);
    }
}
