//! TODO document

use serde::{Deserialize, Serialize};

/// The struct representing the global simulation settings, which includes the
/// time range and simulation delta time.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SimulationArgs {
    /// TODO document
    #[serde(default)]
    pub time_start: Option<f64>,

    /// TODO document
    #[serde(default)]
    pub time_end: Option<f64>,

    /// TODO document
    #[serde(default)]
    pub time_step: Option<f64>,
}
