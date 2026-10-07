use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::math::{DVec2 as Vec2, dvec2 as vec2};

use crate::graphics::Drawable;
use crate::physics::{Body, calc_and_apply_forces};

mod physics;
mod graphics;

/// Constant factor to scale all time in the simulation by
const TIMESCALE: f64 = 0.5;

#[macroquad::main("Spaaaaaaaaaaaaaaaaaaaace")]
async fn main() {
    // Make sure I can see the window, the one time Hyprland forsakes me :orphanage:
    set_window_size(1920, 1080);
    set_fullscreen(true);

    let mut earth = Body {
        radius: 50.,
        mass: 100.,
        vel: vec2(0., 0.),
        pos: vec2(0., 0.),
        acc: Vec2::ZERO,
    };

    let mut moon1 = Body {
        radius: 15.,
        mass: 1.25,
        vel: vec2(300., 0.),
        pos: vec2(-200., 45.),
        acc: Vec2::ZERO,
    };

    let mut _moon2 = Body {
        radius: 12.,
        mass: 1.15,
        vel: vec2(0., 100.),
        pos: vec2(400., 0.),
        acc: Vec2::ZERO,
    };

    // Scaled deltaTime
    let mut dt = 1./60. * TIMESCALE;

    // Main drawing loop
    loop {
        clear_background(BLACK);

        calc_and_apply_forces(&mut earth, &mut vec![&mut moon1]);
        calc_and_apply_forces(&mut moon1, &mut vec![&mut earth]);

        earth.tick(dt, &mut vec![&mut moon1]);
        moon1.tick(dt, &mut vec![&mut earth]);
        // moon2.tick(dt);

        // earth.pos -= earth.pos;
        // moon1.pos -= earth.pos;
        // moon2.pos -= earth.pos;

        earth.draw();
        moon1.draw();
        // moon2.draw();

        next_frame().await;
        dt = get_frame_time() as f64 * TIMESCALE;
    }
}
