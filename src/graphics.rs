use macroquad::prelude::*;

use crate::physics::Body;

/// Do you want arrows for velocity and acceleration
const DISPLAY_ARROWS:bool = true;

const VELOCITY_ARROW_COLOR:Color = RED;
const FORCE_ARROW_COLOR:Color = GREEN;
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

        if DISPLAY_ARROWS {
            draw_line(
                self.pos.x as f32, self.pos.y as f32,
                self.pos.x as f32 + self.vel.x as f32, self.pos.y as f32 + self.vel.y as f32,
                5., VELOCITY_ARROW_COLOR
            );
            draw_line(
                self.pos.x as f32, self.pos.y as f32,
                self.pos.x as f32 + self.acc.x as f32 / 5., self.pos.y as f32 + self.acc.y as f32 / 5.,
                5., FORCE_ARROW_COLOR
            );
        }
    }
}