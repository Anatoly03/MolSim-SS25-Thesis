mod vtk;
mod xyz;

use crate::particle::Particle;
use std::io::Result;
pub use vtk::VTKWriter;
pub use xyz::XYZWriter;

// this trait is dyn-compatible
// see https://doc.rust-lang.org/std/keyword.dyn.html
pub trait OutputWriter {
    fn plot_particles(
        &self,
        particles: &mut Vec<Particle>,
        filename: &str,
        iteration: usize,
    ) -> Result<()>;
}
