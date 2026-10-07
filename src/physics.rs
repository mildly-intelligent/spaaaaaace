use std::f64;

use macroquad::{color::RED, math::{DVec2 as Vec2, dvec2 as vec2}, window::clear_background};

use crate::physics::ImpactType::{HighSpeedHeadOn, HighSpeedSheer, LowSpeedHeadOn, LowSpeedSheer};

/// The gravitational constant
const G:f64 = 6.67e-11;

/// Calculates the force between two gravitational bodies
/// Takes the masses of the two bodies and the distance between them
pub fn old_white_guy(m1: &f64, m2: &f64, distance_squared: &f64) -> f64 {
    (G * m1 * m2) / distance_squared
}

/// Gravitational body
#[derive(Debug)]
pub struct Body {
    /// Radius of the body in pixels
    pub radius: f64,
    /// Mass of the body in ¯\_(ツ)_/¯
    pub mass: f64,
    /// Velocity of the body in pixels per second
    pub vel: Vec2,
    /// Position of the body in pixels
    pub pos: Vec2,
    /// Acceleration, used for drawing arrows
    pub acc: Vec2,
}

impl Body {
    /// Converts the body's mass from ¯\_(ツ)_/¯ to kg
    pub fn get_mass_kilograms(&self) -> f64 {
        self.mass / 1e+15
    }

    /// Applies a net force to a body
    /// Takes the force in Newtons and a normalized vector direction
    fn apply_force(&mut self, force: Vec2) {
        self.acc = force / self.get_mass_kilograms();
    }

    /// Handles all the logic for what happens when the body updates
    /// As of now, only updates the position based on the velocity
    pub fn tick(&mut self, dt: f64, bodies: &mut Vec<&mut Body>) {
        self.pos += self.vel * dt;
        for body in bodies.iter_mut() {
            // !WARNING! RANDOM BULLSHIT AHEAD !WARNING //
            // I HAVE NO IDEA WHY THIS WORKS!! NO TOUCH
            // Thank you https://splashkit.io/guides/physics/5-collisions-and-gravity/
            if self.pos.distance(body.pos) < (self.radius + body.radius) {
                let collision_normal = (self.pos - body.pos).normalize();
                let offset = (self.pos.distance(body.pos) - self.radius - body.radius) * collision_normal;
                self.pos -= offset;

                let velocity_dot_normal = self.vel.dot(collision_normal);
                let velocity_normal = collision_normal * velocity_dot_normal;
                let velocity_tangent = self.vel - velocity_normal;
                self.vel = velocity_tangent - 0.9*velocity_normal;
            }
        }
        self.vel += self.acc * dt;
    }
}

/// Calculates the net force acting on `b1`
fn calc_net_forces(b1: &mut Body, others: &mut Vec<&mut Body>) -> Vec2 {
    let mut net_force = Vec2::ZERO;
    for b2 in others.iter_mut() {
        let force = old_white_guy(&b1.mass, &b2.mass, &b1.pos.distance_squared(b2.pos));
        let direction = (b2.pos - b1.pos).normalize();
        net_force += force * direction;
    }
    net_force
}

/// Calculates and applies forces acting on `b1`
pub fn calc_and_apply_forces(b1: &mut Body, others: &mut Vec<&mut Body>) {
    let force = calc_net_forces(b1, others);
    b1.apply_force(force);
}
