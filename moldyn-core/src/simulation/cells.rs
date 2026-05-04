//! TODO document

use crate::SimulationArgs;
use crate::{Force, LennardJonesForce};
use crate::{Particle, simulation::Simulation};
use std::sync::Arc;

/// The [LinkedCells] simulation method is the a simple optimization of particle
/// management over the quadratic [DirectSum][crate::DirectSum] method.
pub struct LinkedCells {
    // TODO explain in slides why Arc works and Box does not
    force: Arc<dyn Force>,
    particles: Vec<Particle>,
    args: SimulationArgs,
}

impl Simulation for LinkedCells {
    fn system_name(&self) -> &str {
        todo!()
    }

    fn particles(&self) -> &[Particle] {
        todo!()
    }

    fn particles_mut(&mut self) -> &mut [Particle] {
        todo!()
    }

    fn for_each_particle_pairs_mut(&mut self, f: &mut dyn FnMut(&mut Particle, &mut Particle)) {
        todo!()
    }

    fn particle_count(&self) -> usize {
        todo!()
    }

    fn add_particles(&mut self, particles: Vec<Particle>) {
        todo!()
    }

    fn get_force(&self) -> Arc<dyn Force> {
        todo!()
    }

    fn set_force(&mut self, force: Arc<dyn Force>) {
        todo!()
    }

    fn args(&self) -> super::SimulationArgs {
        todo!()
    }

    fn set_args(&mut self, args: super::SimulationArgs) {
        todo!()
    }
}

impl Default for LinkedCells {
    fn default() -> Self {
        Self {
            force: Arc::new(LennardJonesForce::default()),
            particles: Vec::new(),
            args: SimulationArgs::default(),
        }
    }
}

#[cfg(all(test, nightly))]
mod benchmark {
    use crate::{
        CustomForce, LennardJonesForce, LinkedCells, NewtonForce, Particle, Simulation,
        SimulationArgs, Vec3,
    };
    use meval::Expr;
    use std::sync::Arc;
    use test::Bencher;

    #[bench]
    fn ten_bodies(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            particles.push(Particle::at(x as f64, 0.0, 0.0).with_mass(1.0));
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn ten_bodies_lennard_jones(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            particles.push(Particle::at(x as f64, 0.0, 0.0).with_mass(1.0));
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn hundred_bodies(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn hundred_bodies_lennard_jones(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn thousand_bodies(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
                }
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(NewtonForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }

    #[bench]
    fn thousand_bodies_lennard_jones(b: &mut Bencher) {
        let mut particles = vec![];

        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    particles.push(Particle::at(x as f64, y as f64, 0.0).with_mass(1.0));
                }
            }
        }

        let mut simulation = LinkedCells::default();
        simulation.set_force(Arc::new(LennardJonesForce::default()));
        simulation.add_particles(particles);

        b.iter(|| {
            simulation.step(0.01);
        });
    }
}
