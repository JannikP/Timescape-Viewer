use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::{self, Widget};
use iced::alignment::Vertical;
use iced::{Color, Element, Event, Length, Point, Rectangle, Size, mouse};

use crate::core::{Domain, Significance};
use crate::messages::{Message, navigation};
use crate::utilities::scroll_to_zoom;

pub struct Axis<'a, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    chart: usize,
    domain: Domain,
    width: Length,
    height: Length,
    shaping: text::Shaping,
    font: Option<Renderer::Font>,
    class: Theme::Class<'a>,
}

impl<'a, Theme, Renderer> Axis<'a, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
{
    #[must_use]
    pub fn new(chart: usize, domain: Domain) -> Self {
        Self {
            chart,
            domain,
            width: 70.into(),
            height: Length::Fill,
            shaping: Default::default(),
            font: None,
            class: Theme::default(),
        }
    }

    /// Sets the width of the [`Axis`].
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Axis`].
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Font`] of the tick labels of this [`Axis`].
    ///
    /// [`Font`]: text::Renderer::Font
    #[must_use]
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the [`text::Shaping`] strategy of the [`Axis`]'s labels.
    #[must_use]
    pub fn shaping(mut self, shaping: text::Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    /// Sets the style of the [`Axis`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }
}

impl<'a, Theme, Renderer> Widget<Message, Theme, Renderer> for Axis<'a, Theme, Renderer>
where
    Renderer: text::Renderer,
    Theme: Catalog,
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

    fn update(
        &mut self,
        _tree: &mut widget::Tree,
        event: &iced::Event,
        _layout: Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let pivot = if let Some(position) = cursor.position_in(*viewport) {
                    self.domain
                        .map_pixel_to_physical(position.y, viewport.height)
                } else {
                    self.domain.middle()
                };
                let zoom = scroll_to_zoom(delta);
                let message = navigation::Message::zoom_chart(self.chart, self.domain, pivot, zoom);
                shell.publish(Message::Navigation(message));
                shell.capture_event();
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);
        let ticks = self.domain.ticks(bounds.height, 64.0, 16.0, &[]);
        let to_pixels = |value: f32| self.domain.map_physical_to_pixel(value, bounds.height);
        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let label_size = 12.0;

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
                            snap: false,
                            ..renderer::Quad::default()
                        },
                        style.lines,
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
                        style.lines,
                    );
                    renderer.fill_text(
                        text::Text {
                            content: format!("{:}", tick.value),
                            font,
                            size: label_size.into(),
                            line_height: Default::default(),
                            bounds: Size {
                                width: bounds.width - major_width - 2.0,
                                height: label_size * 1.5,
                            },
                            align_x: text::Alignment::Right,
                            align_y: Vertical::Center,
                            shaping: self.shaping,
                            wrapping: text::Wrapping::None,
                        },
                        Point {
                            x: bounds.x + bounds.width - major_width - 2.0,
                            y: bounds.y + tick.position,
                        },
                        style.labels,
                        *viewport,
                    );
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
            style.lines,
        );
    }
}

impl<'a, Theme, Renderer> From<Axis<'a, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: text::Renderer + 'a,
    Theme: Catalog + 'a,
{
    fn from(axis: Axis<'a, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(axis)
    }
}

/// The style of an axis.
///
/// If not specified with [`Axis::style`]
/// the theme will provide the style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The line [`iced::Color`] of the axis. Used for tick marks and the vertical line.
    pub lines: Color,
    /// The text [`iced::Color`] of the axis. Used for tick labels.
    pub labels: Color,
}

/// A styling function for a [`Axis`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

/// The theme catalog of an [`Axis`].
///
/// All themes that can be used with [`Axis`]
/// must implement this trait.
///
/// Although, in order to use [`Axis::style`]
/// with `MyTheme`, [`Catalog::Class`] must implement
/// `From<StyleFn<'_, MyTheme>>`.
pub trait Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>) -> Style;
}
