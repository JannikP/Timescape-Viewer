use iced::widget::scrollable::{Catalog, Status, Style};

use super::MakoTheme;

#[derive(Default)]
pub enum ScrollableClass {
    #[default]
    Standard,
}

impl ScrollableClass {
    fn active(
        &self,
        style: &MakoTheme,
        is_horizontal_scrollbar_disabled: bool,
        is_vertical_scrollbar_disabled: bool,
    ) -> Style {
        let colors = style.colors();
        Style {
            container: todo!(),
            vertical_rail: todo!(),
            horizontal_rail: todo!(),
            gap: None,
            auto_scroll: todo!(),
        }
    }

    fn hovered(
        &self,
        style: &MakoTheme,
        is_horizontal_scrollbar_hovered: bool,
        is_vertical_scrollbar_hovered: bool,
        is_horizontal_scrollbar_disabled: bool,
        is_vertical_scrollbar_disabled: bool,
    ) -> Style {
        let colors = style.colors();
        let mut style = self.active(
            style,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        );
        style
    }

    fn dragged(
        &self,
        style: &MakoTheme,
        is_horizontal_scrollbar_dragged: bool,
        is_vertical_scrollbar_dragged: bool,
        is_horizontal_scrollbar_disabled: bool,
        is_vertical_scrollbar_disabled: bool,
    ) -> Style {
        let colors = style.colors();
        let mut style = self.active(
            style,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        );
        style
    }
}

impl Catalog for MakoTheme {
    type Class<'a> = ScrollableClass;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active {
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            } => class.active(
                self,
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            ),
            Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            } => class.hovered(
                self,
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            ),
            Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            } => class.dragged(
                self,
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                is_horizontal_scrollbar_disabled,
                is_vertical_scrollbar_disabled,
            ),
        }
    }
}
