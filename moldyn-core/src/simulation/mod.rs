//! TODO document

mod sum;

use crate::Particle;
pub use sum::DirectSum;

// to self: tried to keep dyn-compatibility. following approaches failed:
// - fn ...(... impl Fn) is technically generic
// - type PairIter = ... is also generic/ typed
// - returning `Iter` and `IterMut` works for `particles` (`particles_mut`)
//   but `particle_pairs` had implementation problems returning slide::IntoIter

/// An interface-level abstraction of a molecular dynamics simulation. A
/// `simulation` is a method of organizing the particles and forces in a way
/// that allows for efficient computation.
pub trait Simulation {
    /// Invokes a lambda callback for each particle in the simulation.
    fn for_each_particles<'a>(&'a self, f: &mut dyn FnMut(&Particle));

    /// Invokes a lambda callback for each particle (mutable) in the simulation.
    fn for_each_particles_mut<'a>(&'a mut self, f: &mut dyn FnMut(&mut Particle));

    /// The core method of the trait. Different implementations of [Simulation] vary
    /// in performance as this is the heaviest part of the simulation. Invokes a lambda
    /// callback for pair of particles in the simulation, with the following limitations:
    ///
    /// - An iterator over distinct pairs of particles, accounting for symmetry.
    /// - If you receive a pair `(a, b)` it is guaranteed that you will not receive `(b, a)`.
    /// - There is no guarantee you will receive all pairs.
    fn for_each_particle_pairs_mut<'a>(
        &'a mut self,
        f: &mut dyn FnMut(&mut Particle, &mut Particle),
    );

    /// The number of particles in the simulation.
    fn particle_count(&self) -> usize;

    /// TODO document
    fn step(&mut self) {}
}
