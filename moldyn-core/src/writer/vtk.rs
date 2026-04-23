//! Defines a simple text writer for the simulation data. This is a basic
//! implementation used only to verify the capacity of the program to write
//! output files and is deprecated for future use.

use crate::{OutputWriter, Simulation};
use std::io::BufWriter;
use vtkio::model::*;

/// A simple text writer for the simulation data.
///
/// # C++ Source Code
///
/// The [VtkWriter] attempts to rewrite the functionality of the C++ code from
/// the [original project](https://github.com/TUM-I5/MolSim/blob/master/src/outputWriter/VTKWriter.cpp)
/// in Rust utilizing the [vtkio] crate.
///
/// ```cpp
/// void VTKWriter::plotParticles(std::list<Particle> particles, const std::string &filename, int iteration)
/// {
///     // Initialize points
///     auto points = vtkSmartPointer<vtkPoints>::New();
///
///     // Create and configure data arrays
///     vtkNew<vtkFloatArray> massArray;
///     massArray->SetName("mass");
///     massArray->SetNumberOfComponents(1);
///
///     vtkNew<vtkFloatArray> velocityArray;
///     velocityArray->SetName("velocity");
///     velocityArray->SetNumberOfComponents(3);
///
///     vtkNew<vtkFloatArray> forceArray;
///     forceArray->SetName("force");
///     forceArray->SetNumberOfComponents(3);
///
///     vtkNew<vtkIntArray> typeArray;
///     typeArray->SetName("type");
///     typeArray->SetNumberOfComponents(1);
///
///     for (auto &p : particles)
///     {
///         points->InsertNextPoint(p.getX().data());
///         massArray->InsertNextValue(static_cast<float>(p.getM()));
///         velocityArray->InsertNextTuple(p.getV().data());
///         forceArray->InsertNextTuple(p.getF().data());
///         typeArray->InsertNextValue(p.getType());
///     }
///
///     // Set up the grid
///     auto grid = vtkSmartPointer<vtkUnstructuredGrid>::New();
///     grid->SetPoints(points);
///
///     // Add arrays to the grid
///     grid->GetPointData()->AddArray(massArray);
///     grid->GetPointData()->AddArray(velocityArray);
///     grid->GetPointData()->AddArray(forceArray);
///     grid->GetPointData()->AddArray(typeArray);
///
///     // Create filename with iteration number
///     std::stringstream strstr;
///     strstr << filename << "_" << std::setfill('0') << std::setw(4) << iteration << ".vtu";
///
///     // Create writer and set data
///     vtkNew<vtkXMLUnstructuredGridWriter> writer;
///     writer->SetFileName(strstr.str().c_str());
///     writer->SetInputData(grid);
///     writer->SetDataModeToAscii();
///
///     // Write the file
///     writer->Write();
/// }
/// ```
#[derive(Default)]
pub struct VtkWriter {
    pub frame_number: usize,
}

impl OutputWriter for VtkWriter {
    fn next_frame_number(&mut self) -> usize {
        self.frame_number += 1;
        self.frame_number
    }

    fn write_frame(
        &mut self,
        writer: &mut BufWriter<std::fs::File>,
        state: &Box<dyn Simulation>,
    ) -> std::io::Result<()> {
        // equivalent cpp: auto points = vtkSmartPointer<vtkPoints>::New();
        let mut points = Vec::new();

        state.for_each_particles(&mut |p| {
            // equivalent cpp: points->InsertNextPoint(p.getX().data());
            points.push(p.get_position().x);
            points.push(p.get_position().y);
            points.push(p.get_position().z);
        });

        let attributes = Attributes {
            point: vec![],
            cell: vec![],
        };

        // equivalent cpp: vtkNew<vtkXMLUnstructuredGridWriter> writer;
        let grid = UnstructuredGridPiece {
            points: IOBuffer::F64(points),
            cells: Cells {
                cell_verts: VertexNumbers::XML {
                    connectivity: vec![],
                    offsets: vec![],
                },
                types: vec![],
            },
            data: attributes,
        };

        let vtk = Vtk {
            version: Version::new((1, 0)),
            // equivalent cpp: writer->SetFileName(strstr.str().c_str());
            title: String::default(),
            file_path: None,
            // equivalent cpp: writer->SetDataModeToAscii();
            // equivalent cpp: writer->SetInputData(grid);
            byte_order: ByteOrder::BigEndian,
            data: DataSet::inline(grid),
        };

        vtk.write_xml(writer)?;

        Ok(())
    }

    fn extension(&self) -> &str {
        "vtu"
    }
}
