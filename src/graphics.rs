use macroquad::prelude::*;

use crate::physics::Body;

/// Coordinates of the center of the screen (duh dumbass)
const SCREEN_CENTER: DVec2 = dvec2(965., 540.);

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
        let pos = self.pos + SCREEN_CENTER;

        draw_circle(
            pos.x as f32,
            pos.y as f32,
            self.radius as f32,
            PLANET_COLOR
        );

        if DISPLAY_ARROWS {
            draw_line(
                pos.x as f32, pos.y as f32,
                pos.x as f32 + self.vel.x as f32, pos.y as f32 + self.vel.y as f32,
                5., VELOCITY_ARROW_COLOR
            );
            draw_line(
                pos.x as f32, pos.y as f32,
                pos.x as f32 + self.acc.x as f32 / 5., pos.y as f32 + self.acc.y as f32 / 5.,
                5., FORCE_ARROW_COLOR
            );
        }
    }
}
