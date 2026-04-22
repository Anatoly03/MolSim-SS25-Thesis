//! This module contains the [NewtonForce] struct, which implements the
//! [Force] trait according to Newton's law of universal gravitation.

use crate::{Particle, Vec3, forces::Force};

/// A struct representing a Newton (or Coloumb-like) force, which implements
/// the [Force] trait.
#[derive(Default)]
pub struct NewtonForce;

impl Force for NewtonForce {
    fn system_name(&self) -> &str {
        "newton"
    }

    fn force(&self, particle: &Particle, other: &Particle) -> Vec3 {
        let diff = Particle::position_difference(particle, other);
        let distance = diff.length();

        if distance == 0.0 {
            Vec3::zero()
        } else {
            let mul_mass = Particle::mass_product(particle, other);
            diff * (mul_mass / distance.powi(3))
        }
    }
}
