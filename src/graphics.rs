use macroquad::prelude::*;

use crate::physics::Body;

const PLANET_COLOR:Color = GRAY;

pub trait Drawable {
    fn draw(&self);
}

impl Drawable for Body {
    fn draw(&self) {
        draw_circle(
            self.pos.x as f32,
            self.pos.y as f32,
            self.radius as f32,
            PLANET_COLOR
        );
    }
}