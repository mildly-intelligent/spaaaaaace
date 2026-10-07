use macroquad::math::{DVec2 as Vec2, dvec2 as vec2};

/// The gravitational constant
const G:f64 = 6.67e-11;

/// Calculates the force between two gravitational bodies
/// Takes the masses of the two bodies and the distance between them
pub fn old_white_guy(m1: &f64, m2: &f64, d: &f64) -> f64 {
    (G * m1 * m2) / (d * d)
}

/// Gravitational body
#[derive(Debug)]
pub struct Body {
    /// Radius of the body in pixels
    /// R<sub>🜨</sub> = 25px
    pub radius: f64,
    /// Mass of the body in UNIT-TBD
    pub mass: f64,
    /// Velocity of the body in pixels per second
    pub vel: Vec2,
    /// Position of the body in pixels
    /// R<sub>🜨</sub> = 25px
    pub pos: Vec2,
}

impl Body {
    /// Applies a sum force to a body
    /// Takes the force in Newtons and a normalized vector direction
    fn apply_force(&mut self, force: f64, direction: Vec2) {
        let acceleration = force / self.mass;
        self.vel += acceleration * direction;
    }
}