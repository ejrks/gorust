use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

use uotech::aspace::basis;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GAlgebraBasis {
}

#[godot_api]
impl INode3D for GAlgebraBasis {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            
        }
    }
}