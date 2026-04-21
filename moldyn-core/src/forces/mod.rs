mod ljp;
mod newton;

use crate::{Particle, Vec3};
pub use ljp::LennardJonesForce;
pub use newton::NewtonForce;
use serde::{Deserialize, de::Visitor};

pub trait Force {
    /// Returns the name of the force system, which is used for serialization
    /// and deserialization. The characters are expected to be in lowercase.
    fn system_name(&self) -> &str;

    /// Calculates the force between two particles, not changing their state.
    fn force(&self, particle: &Particle, other: &Particle) -> Vec3;

    /// Applies the force to a particle pair. The force is added (or subtracted)
    /// to the `force` field of each particle forming an attraction or repulsion.
    fn apply_force(&self, particle: &mut Particle, other: &mut Particle) {
        let force = self.force(particle, other);
        particle.apply_force(force);
        other.apply_force(-force);
    }
}

struct ForceVisitor;

impl<'de> Visitor<'de> for ForceVisitor {
    type Value = Box<dyn Force>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a force")
    }

    /// If the force is represented as a string, we can parse it as a known force type,
    /// assuming default parameters. Strings are case-insensitive.
    ///
    /// # Example
    ///
    /// ```yaml
    /// # Particle definition input file example
    ///
    /// name: halleys-comet
    /// force: gravitational
    /// ```
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.to_ascii_lowercase().as_str() {
            "lennardjones" | "lj" => Ok(Box::new(LennardJonesForce::default())),
            "newton" | "gravitational" => Ok(Box::new(NewtonForce::default())),
            _ => Err(E::custom(format!("Unknown force type: {}", value))),
        }
    }

    // TODO: implement deserialization with parameters.
    // idea: force: { type: ..., params... }
}

impl<'de> Deserialize<'de> for Box<dyn Force> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ForceVisitor)
    }
}

impl Default for Box<dyn Force> {
    /// The default force calculation system for this project is the lennard
    /// jones potential. If not specified, the system will be initialized with
    /// default parameters.
    fn default() -> Self {
        Box::new(LennardJonesForce::default())
    }
}
