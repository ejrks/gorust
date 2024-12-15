use std::vec;

use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ObjectWithData {
    data: Vec<i32>,

    base: Base<Node3D>
}

#[godot_api]
impl INode3D for ObjectWithData {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            data: Vec::with_capacity(10),

            base,
        }
    }
}

#[godot_api]
impl ObjectWithData {
    #[func]
    fn get_debug_data(&mut self) -> Vec<i32> {
        let mut debug_data = Vec::new();
        debug_data.push(3);
        debug_data.push(1);
        debug_data.push(3);
        debug_data.push(5);
        debug_data
    }
}