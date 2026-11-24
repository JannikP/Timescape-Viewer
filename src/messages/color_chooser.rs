use crate::state::color_chooser::Authority;

#[derive(Debug, Clone)]
pub enum ColorChooserMessage {
    ChangeAuthority(Authority),
    ChangeColor{ hue: f32, saturation: f32 },
}
