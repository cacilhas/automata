use raylib::prelude::{Color, RaylibDraw};

use crate::vec2::Vec2;

pub trait Sprite {
    fn sprite_move(&mut self);
    fn location(&self) -> &Vec2<f32>;
    fn color(&self) -> &Color;

    fn draw<T: RaylibDraw>(&self, handler: &mut T) {
        let loc = self.location();
        handler.draw_pixel(
            loc.x as i32,
            loc.y as i32,
            self.color(),
        );
    }
}
