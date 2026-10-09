use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::math::dvec2 as vec2;

use crate::graphics::Drawable;
use crate::physics::{Body, Bodies};

mod physics;
mod graphics;

/// Constant factor to scale all time in the simulation by
const TIMESCALE: f64 = 2.;

#[macroquad::main("Spaaaaaaaaaaaaaaaaaaaace")]
async fn main() {
    // Make sure I can see the window, the one time Hyprland forsakes me :orphanage:
    set_window_size(1920, 1080);
    set_fullscreen(true);

    let earth = Body::new(
        60.,
        120.,
        vec2(0., 0.),
        vec2(0., 0.),
    );

    let moon1 = Body::new(
        15.,
        1.25,
        vec2(300., 0.),
        vec2(0., 150.),
    );

    let moon2 = Body::new(
        12.,
        1.15,
        vec2(500., 0.),
        vec2(0., 125.),
    );

    let mut bodies = Bodies::new([Box::new(earth), Box::new(moon1), Box::new(moon2)]);

    // Scaled deltaTime
    let mut dt = 1./60. * TIMESCALE;

    // Main drawing loop
    loop {
        clear_background(BLACK);

        bodies.calc_and_apply_forces();
        // earth.calc_and_apply_forces(&mut vec![&mut moon1, &mut moon2]);
        // moon1.calc_and_apply_forces(&mut vec![&mut earth, &mut moon2]);
        // moon2.calc_and_apply_forces(&mut vec![&mut earth, &mut moon1]);

        bodies.tick(dt);
        // earth.tick(dt, &mut vec![&mut moon1, &mut moon2]);
        // moon1.tick(dt, &mut vec![&mut earth, &mut moon2]);
        // moon2.tick(dt, &mut vec![&mut earth, &mut moon1]);

        // earth.pos -= earth.pos;
        // moon1.pos -= earth.pos;
        // moon2.pos -= earth.pos;

        bodies.draw();
        // earth.draw();
        // moon1.draw();
        // moon2.draw();

        next_frame().await;
        dt = get_frame_time() as f64 * TIMESCALE;
    }
}
