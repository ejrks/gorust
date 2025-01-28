use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

use uotech::materials::bravais::UnitCell;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct BravaisProvider {
    pub unit_cell: UnitCell,
}

#[godot_api]
impl INode3D for BravaisProvider {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            unit_cell: UnitCell::new(),
        }
    }
}
