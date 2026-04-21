//! TODO document

use crate::Particle;
use std::vec::IntoIter;

/// An interface-level abstraction of a molecular dynamics simulation.
/// TODO document
pub trait Simulation {
    /// TODO document
    fn particles(&self) -> IntoIter<&mut Particle>;

    /// TODO document
    fn particle_count(&self) -> usize {
        self.particles().len()
    }

    /// TODO document
    /// TODO do we need this method? it breaks dyn compatibility (generics)
    /// TODO retink
    fn for_each_particle(&mut self, mut f: impl FnMut(&mut Particle)) {
        for particle in self.particles() {
            f(particle);
        }
    }

    /// TODO document
    fn step(&mut self);
}
