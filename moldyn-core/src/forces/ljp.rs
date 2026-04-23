//! This module contains the [LennardJonesForce] struct, which implements
//! the [Force] trait according to the Lennard-Jones potential.

use crate::{Particle, Vec3, forces::Force};

/// A struct representing a Lennard-Jones force, which implements the
/// [Force] trait.
pub struct LennardJonesForce {
    // TODO document
    cutoff_radius: f64,

    // TODO document
    epsilon: f64,

    // TODO document
    sigma: f64,
}

impl Force for LennardJonesForce {
    fn system_name(&self) -> &str {
        "lennard-jones"
    }

    fn force(&self, _particle: &Particle, _other: &Particle) -> Vec3 {
        let _ = self.cutoff_radius; // to avoid 'unused variable' warning
        let _ = self.epsilon; // to avoid 'unused variable' warning
        let _ = self.sigma; // to avoid 'unused variable' warning
        todo!("lennard jones force not implemented")
    }
}

impl Default for LennardJonesForce {
    /// The default instance of [LennardJonesForce]. The parameters are set
    /// to the following.
    ///
    /// | Parameter | Value |
    /// | --- | --- |
    /// | `cutoff_radius` | `3.0` |
    /// | `epsilon` | `5.0` |
    /// | `sigma` | `1.0` |`
    fn default() -> Self {
        Self {
            // values are taken from 'assignment 3 task 2'
            cutoff_radius: 3.0,
            epsilon: 5.0,
            sigma: 1.0,
        }
    }
}
