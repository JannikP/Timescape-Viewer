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
