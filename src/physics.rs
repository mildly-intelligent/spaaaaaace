use macroquad::math::DVec2 as Vec2;

/// The gravitational constant
const G:f64 = 6.67e-11;

/// Calculates the force between two gravitational bodies
/// Takes the masses of the two bodies and the distance between them
pub fn old_white_guy(m1: &f64, m2: &f64, distance_squared: &f64) -> f64 {
    (G * m1 * m2) / distance_squared
}

/// Gravitational body
#[derive(Debug, Clone, Copy)]
pub struct Body {
    /// Radius of the body in pixels
    pub radius: f64,
    /// Mass of the body in ¯\_(ツ)_/¯
    pub mass: f64,
    /// Position of the body in pixels
    pub pos: Vec2,
    /// Velocity of the body in pixels per second
    pub vel: Vec2,
    /// Acceleration, used for drawing arrows
    pub acc: Vec2,
}

impl Body {
    pub fn new(radius: f64, mass: f64, pos: Vec2, vel: Vec2) -> Self {
        Self {
            radius,
            mass,
            pos,
            vel,
            ..Default::default()
        }
    }

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
    pub fn tick(&mut self, dt: f64, bodies: &mut Vec<Box<Body>>) {
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
                self.vel = velocity_tangent - 0.6*velocity_normal;
            }
        }
        self.vel += self.acc * dt;
    }

    /// Calculates the net force acting on `b1`
    fn calc_net_forces(&mut self, others: &mut Vec<Box<Body>>) -> Vec2 {
        let mut net_force = Vec2::ZERO;
        for b2 in others.iter_mut() {
            let force = old_white_guy(&self.mass, &b2.mass, &self.pos.distance_squared(b2.pos));
            let direction = (b2.pos - self.pos).normalize();
            net_force += force * direction;
        }
        net_force
    }

    /// Calculates and applies forces acting on `b1`
    pub fn calc_and_apply_forces(&mut self, others: &mut Vec<Box<Body>>) {
        let force = self.calc_net_forces(others);
        self.apply_force(force);
    }
}

impl Default for Body {
    fn default() -> Self {
        Self {
            radius: 0.,
            mass: 0.,
            pos: crate::graphics::SCREEN_CENTER / 2.,
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
        }
    }
}

pub struct Bodies(Vec<Box<Body>>);
impl IntoIterator for Bodies {
    type Item = Box<Body>;

    type IntoIter = <Vec<Box<Body>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl Bodies {
    pub fn new<const N: usize>(items: [Box<Body>; N]) -> Self {
        Self(Vec::from(items))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<Body>> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Separates one item from the list out
    // pub fn split(&mut self, index: usize) -> (&mut Body, Vec<&mut Body>) {
    //     // HOLY SHIT I DID IT
    //     let mut copy: Vec<&mut Body> = self.0.iter()
    //         .map(|a| &**a)
    //         .copied()
    //         .collect();
    //     copy.remove(index);
    //     (self.0[index], copy)
    // }

    pub fn fadslk(&self, index: usize) -> Vec<usize> {
        (0..self.len()).filter(|a| a != &index).collect()
    }

    pub fn calc_and_apply_forces(&mut self) {
        for i in 0..self.len() {
            let others = self.fadslk(i);
            let mut others = others
                .iter()
                .map(|a| self.0.get(*a).unwrap().clone())
                .collect();
            (*self.0.get_mut(i).unwrap()).calc_and_apply_forces(&mut others);
        }
    }

    pub fn tick(&mut self, dt: f64) {
        for i in 0..self.len() {
            let others = self.fadslk(i);
            let mut others = others
                .iter()
                .map(|a| self.0.get(*a).unwrap().clone())
                .collect();
            (*self.0.get_mut(i).unwrap()).tick(dt, &mut others);
        }
    }
}
