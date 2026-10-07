use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::math::dvec2 as vec2;

use crate::graphics::Drawable;
use crate::physics::{Body, calc_and_apply_forces};

mod physics;
mod graphics;

/// Constant factor to scale all time in the simulation by
const TIMESCALE: f64 = 2.;
/// Coordinates of the center of the screen (duh dumbass)
const SCREEN_CENTER: DVec2 = vec2(965., 540.);

#[macroquad::main("Spaaaaaaaaaaaaaaaaaaaace")]
async fn main() {
    // Make sure I can see the window, the one time Hyprland forsakes me :orphanage:
    set_window_size(1920, 1080);
    set_fullscreen(true);

    let mut earth = Body {
        radius: 50.,
        mass: 100.,
        vel: vec2(0., 0.),
        pos: vec2(0., 0.) + SCREEN_CENTER,
        acc: vec2(0., 0.),
    };

    let mut moon = Body {
        radius: 13.635,
        mass: 1.23,
        vel: vec2(0., 150.),
        pos: vec2(200., 0.) + SCREEN_CENTER,
        acc: vec2(0., 0.),
    };

    // Scaled deltaTime
    let mut dt = 1./60. * TIMESCALE;

    // Main drawing loop
    loop {
        clear_background(BLACK);

        earth.draw();
        moon.draw();
        
        calc_and_apply_forces(&mut earth, &mut moon, dt);

        earth.tick(dt);
        moon.tick(dt);

        next_frame().await;
        dt = get_frame_time() as f64 * TIMESCALE;
    }
}
