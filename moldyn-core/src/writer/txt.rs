//! Defines a simple text writer for the simulation data. This is a basic
//! implementation used only to verify the capacity of the program to write
//! output files and is deprecated for future use.

use crate::{OutputWriter, Simulation};
use std::io::{BufWriter, Write};

/// A simple text writer for the simulation data.
#[derive(Default)]
pub struct TxtWriter {
    pub frame_number: usize,
}

impl OutputWriter for TxtWriter {
    fn next_frame_number(&mut self) -> usize {
        self.frame_number += 1;
        self.frame_number
    }

    fn write_frame(
        &mut self,
        writer: &mut BufWriter<std::fs::File>,
        state: &dyn Simulation,
    ) -> std::io::Result<()> {
        state.for_each_particles(&mut move |p| {
            writeln!(writer, "{p:?}").expect("Error writing to output file");
        });
        Ok(())
    }

    fn extension(&self) -> &str {
        "txt"
    }
}
