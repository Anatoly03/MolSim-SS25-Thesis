//! TODO document

use crate::{Particle, simulation::Simulation};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

/// The [DirectSum] simulation method is the most intuitive way to process
/// a molecular dynamics simulation. It bases the computation on the
/// [Direct Sum](https://en.wikipedia.org/wiki/Direct_sum) method.
///
/// **Newton Pair Optimization**: The only optimization [DirectSum] performs
/// is avoiding computing the same pair of particles twice.
pub struct DirectSum {
    particles: Vec<Particle>,
}

impl Simulation for DirectSum {
    fn for_each_particles<'a>(&'a self, f: &mut dyn FnMut(&Particle)) {
        for part in &self.particles {
            f(part);
        }
    }

    fn for_each_particles_mut<'a>(&'a mut self, f: &mut dyn FnMut(&mut Particle)) {
        for part in &mut self.particles {
            f(part);
        }
    }

    // index-based approach because two mutable iterators were problematic
    // TODO again we run into (&mut, &mut) problems, rethink design choice
    fn for_each_particle_pairs_mut<'a>(&'a mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        let count = self.particle_count();

        for i in 0..count {
            // newtons third law: skip same pairs
            for j in (i + 1)..count {
                let particle_i = &mut self.particles[i];
                let particle_j = &mut self.particles[j];

                f(particle_i, particle_j);
            }
        }
    }

    fn particle_count(&self) -> usize {
        self.particles.len()
    }
}
