use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

use uotech::materials::bravais;
use uotech::materials::bravais::UnitCell;

#[derive(GodotConvert, Var, Export)]
#[godot(via = i64)]
pub enum GCellType {
    BCC = 0,
    FCC = 1,
    HC = 2,
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct BravaisProvider {
    pub unit_cell: UnitCell,
    #[export]
    pub cell_type: GCellType,
}

#[godot_api]
impl INode3D for BravaisProvider {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            unit_cell: UnitCell::new(),
            cell_type: GCellType::BCC,
        }
    }
}

#[godot_api]
impl BravaisProvider {
    #[func]
    fn get_cell_positions(&mut self) -> Array<Vector3> {
        match self.cell_type {
            GCellType::BCC => self.unit_cell.cell_type = bravais::CellType::BCC,
            GCellType::FCC => self.unit_cell.cell_type = bravais::CellType::FCC,
            GCellType::HC => self.unit_cell.cell_type = bravais::CellType::HC,
        }

        let mut result: Array<Vector3> = Array::new();

        let bravais_points = self.unit_cell.get_cell_positions();
        for entry in bravais_points {
            result.push(Vector3::new(entry.x as f32, entry.y as f32, entry.z as f32));
        }

        return result;
    }
}



