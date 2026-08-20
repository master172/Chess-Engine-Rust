use macroquad::color::Color;

pub const SELECTED_PIECE: Color = Color::from_rgba(246, 211, 101, 204);
pub const LEGAL_SQUARE: Color = Color::from_rgba(124, 179, 66, 167);

pub fn _get_overlay_color(base: &Color, target: &Color) -> Color {
    let r: f32 = (target.r - (1.0 - target.a) * base.r) / target.a;
    let g: f32 = (target.g - (1.0 - target.a) * base.g) / target.a;
    let b: f32 = (target.b - (1.0 - target.a) * base.b) / target.a;
    let a: f32 = (target.a - base.a) / (1.0 - base.a);
    Color { r, g, b, a }
}
