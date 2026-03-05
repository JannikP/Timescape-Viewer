use std::ops::RangeInclusive;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::color;
use iced::{Element, Length, Rectangle, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scaling {
    Linear,
    Logarithmic,
}

impl Scaling {
    pub fn ticks(
        &self,
        size: f32,
        major_tick_spacing: f32,
        minor_tick_spacing: f32,
        range: &RangeInclusive<f64>,
        extra_ticks: &[f64],
    ) -> Vec<Tick> {
        match self {
            Self::Linear => distribute_linear_ticks(
                size,
                major_tick_spacing,
                minor_tick_spacing,
                range,
                extra_ticks,
            ),
            Self::Logarithmic => distribute_logarithmic_ticks(
                size,
                major_tick_spacing,
                minor_tick_spacing,
                range,
                extra_ticks,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Significance {
    Minor,
    Major,
    Extra,
}

pub struct Axis {
    range: RangeInclusive<f64>,
    scaling: Scaling,
    width: Length,
    height: Length,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    value: f64,
    position: f32,
    significance: Significance,
}

impl Axis {
    pub fn new(scaling: Scaling, range: RangeInclusive<f64>) -> Self {
        Self {
            range,
            scaling,
            width: 100.into(),
            height: Length::Fill,
        }
    }

    /// Sets the width of the [`Axis`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Axis`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Axis
where
    Renderer: iced::advanced::renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let ticks = self
            .scaling
            .ticks(bounds.height, 64.0, 16.0, &self.range, &[]);
        let to_pixels = |value: f64| value_to_pixels(value, &self.range, bounds.height);

        // Draw major ticks
        let major_x = (bounds.x + bounds.width - 8.0 - 1.0).max(bounds.x);
        let major_width = 8.0f32.min(bounds.width);
        let minor_x = (bounds.x + bounds.width - 4.0 - 1.0).max(bounds.x);
        let minor_width = 4.0f32.min(bounds.width);
        for tick in ticks.iter() {
            match tick.significance {
                Significance::Minor => {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: minor_x,
                                y: to_pixels(tick.value) - 0.5 + bounds.y,
                                width: minor_width,
                                height: 1.0,
                            },
                            snap: true,
                            ..renderer::Quad::default()
                        },
                        color!(0xffffff),
                    );
                }
                Significance::Major | Significance::Extra => {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle {
                                x: major_x,
                                y: to_pixels(tick.value) - 1.0 + bounds.y,
                                width: major_width,
                                height: 2.0,
                            },
                            snap: true,
                            ..renderer::Quad::default()
                        },
                        color!(0xffffff),
                    );
                    // TODO: Render tick label
                }
            }
        }

        // Draw axis line
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x + bounds.width - 1.0,
                    y: bounds.y,
                    width: 1.0,
                    height: bounds.height,
                },
                snap: true,
                ..renderer::Quad::default()
            },
            color!(0xffffff),
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Axis> for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
    Message: 'a,
{
    fn from(axis: Axis) -> Element<'a, Message, Theme, Renderer> {
        Element::new(axis)
    }
}

fn distribute_linear_ticks(
    size: f32,
    major_tick_spacing: f32,
    minor_tick_spacing: f32,
    range: &RangeInclusive<f64>,
    extra_ticks: &[f64],
) -> Vec<Tick> {
    if range.is_empty() {
        return Vec::new();
    }

    // Find nice major ticks
    let major_ticks = (size / major_tick_spacing).floor() as f64;
    let ideal_tick = (range.end() - range.start()) / major_ticks;
    let nice_tick = next_nice_number(ideal_tick);
    let top = (range.end() / nice_tick).ceil() * nice_tick;
    let bottom = (range.start() / nice_tick).floor() * nice_tick;

    // How to convert between physical values and pixel coordinates
    let to_pixels = |value: f64| value_to_pixels(value, range, size);

    // Distributing the major ticks
    let mut ticks = Vec::new();
    let mut value = bottom + nice_tick;
    while value < top {
        let tick = Tick {
            value,
            position: to_pixels(value),
            significance: Significance::Major,
        };
        ticks.push(tick);
        value += nice_tick;
    }

    // Distribute the minor ticks
    let actual_major_tick_spacing =  ((value - range.start()) / (range.end() - range.start())) as f32 * size;
    let minor_tick = nice_minor_ticks(nice_tick, actual_major_tick_spacing, minor_tick_spacing);
    let mut minor_ticks = Vec::new();
    let mut previous_major_value = bottom;
    for major_tick in ticks.iter() {
        value = previous_major_value + minor_tick;
        while value < major_tick.value {
            let position = to_pixels(value);
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
            position: to_pixels(*value),
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

fn distribute_logarithmic_ticks(
    _size: f32,
    _major_tick_spacing: f32,
    _minor_tick_spacing: f32,
    _range: &RangeInclusive<f64>,
    _extra_ticks: &[f64],
) -> Vec<Tick> {
    Vec::new()
}

fn next_power_of_ten(number: f64) -> f64 {
    if number > 0.0 {
        10f64.powf(number.log10().ceil())
    } else if number < 0.0 {
        -1.0 * 10f64.powf((-number).log10().ceil())
    } else {
        0.0
    }
}

fn next_nice_number(number: f64) -> f64 {
    let nice_tick = next_power_of_ten(number);
    if nice_tick > 4.0 * number {
        nice_tick / 4.0
    } else if nice_tick > 2.0 * number {
        nice_tick / 2.0
    } else {
        nice_tick
    }
}

fn nice_minor_ticks(major_tick: f64, major_tick_spacing: f32, minor_tick_spacing: f32) -> f64 {
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

fn value_to_pixels(value: f64, range: &RangeInclusive<f64>, size: f32) -> f32 {
    let fraction = (value - range.start()) / (range.end() - range.start());
    size - fraction as f32 * size
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
    use assert_float_eq::assert_f64_near;

    use super::{distribute_linear_ticks, next_nice_number, next_power_of_ten, nice_minor_ticks};

    #[test]
    fn distribute_linear_ticks_simple_example() {
        let ticks = distribute_linear_ticks(300.0, 64.0, 16.0, &(-5.0..=105.0), &[42.0]);
        assert_eq!(ticks.len(), 19);
        // Two major ticks
        assert_eq!(ticks[0].value, 0.0);
        assert_eq!(ticks[1].value, 100.0);
        // One extra tick at the end
        assert_eq!(ticks[18].value, 42.0);
    }

    #[test]
    fn distribute_linear_ticks_default_line_chart() {
        let ticks = distribute_linear_ticks(150.0, 64.0, 16.0, &(-5.0..=105.0), &[]);
        assert_eq!(ticks.len(), 11);
        // Two major ticks
        assert_eq!(ticks[0].value, 0.0);
        assert_eq!(ticks[1].value, 100.0);
        // Many minor ticks
    }

    #[test]
    fn next_power_of_ten_positive_numbers() {
        assert_f64_near!(next_power_of_ten(987.0), 1000.0);
        assert_f64_near!(next_power_of_ten(99_999.0), 100_000.0);
        assert_f64_near!(next_power_of_ten(999_999_999.0), 1_000_000_000.0);
    }

    #[test]
    fn next_power_of_ten_negative_numbers() {
        assert_f64_near!(next_power_of_ten(-987.0), -1000.0);
        assert_f64_near!(next_power_of_ten(-99_999.0), -100_000.0);
        assert_f64_near!(next_power_of_ten(-999_999_999.0), -1_000_000_000.0);
    }

    #[test]
    fn next_power_of_ten_zero() {
        assert_f64_near!(next_power_of_ten(0.0), 0.0);
    }

    #[test]
    fn nice_tick_small() {
        assert_f64_near!(next_nice_number(9.0), 10.0);
        assert_f64_near!(next_nice_number(4.0), 5.0);
        assert_f64_near!(next_nice_number(1.8), 2.5);
        assert_f64_near!(next_nice_number(0.9), 1.0);
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
