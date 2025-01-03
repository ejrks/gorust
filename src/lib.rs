use godot::prelude::*;

struct ELAU;

#[gdextension]
unsafe impl ExtensionLibrary for ELAU {}

use crate::godottest::object_with_data;
pub mod godottest;


pub mod dataparser;
pub mod tracetrainer;