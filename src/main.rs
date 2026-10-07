use macroquad::prelude::*;
use macroquad::math::dvec2 as vec2;

use crate::graphics::Drawable;
use crate::physics::Body;

mod physics;
mod graphics;

#[macroquad::main("Spaaaaaaaaaaaaaaaaaaaace")]
async fn main() {
    let earth = Body {
        radius: 25.,
        mass: 5.,
        vel: vec2(0., 0.),
        pos: vec2((screen_width() as f64)/2., (screen_height() as f64)/2.),
    };

    // Main drawing loop
    loop {
        clear_background(BLACK);

        earth.draw();
        dbg!(&earth);

        next_frame().await
    }
}
