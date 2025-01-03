use mvecops::beorc::def::DefinitionUnit;
use mvecops::beorc::def::TrainingUnit;

use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GDefinitionUnit {
    pub definition_unit: DefinitionUnit,
}

#[godot_api]
impl INode3D for GDefinitionUnit {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            definition_unit: DefinitionUnit::new(0),
        }
    }
}

#[godot_api]
impl GDefinitionUnit {
    #[func]
    fn setup(&mut self, id: GString, resolution: i64) {
        self.definition_unit = DefinitionUnit::new(resolution);
        self.definition_unit.id = id.to_string();
    }

    #[func]
    fn feed(&mut self, time_stamp: i64, indexes: Array<i64>) {
        let mut format_indexes: Vec<i64> = Vec::new();
        for entry in indexes.iter_shared() {
            format_indexes.push(entry.clone());
        }

        self.definition_unit.feed(time_stamp, format_indexes);
    }

    #[func]
    fn q_trace_number(&self) -> i64 {
        return self.definition_unit.traces.len() as i64;
    }
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GTrainingUnit {
    pub training_unit: TrainingUnit,
}

#[godot_api]
impl INode3D for GTrainingUnit {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            training_unit: TrainingUnit::empty(0.5),
        }
    }
}

#[godot_api]
impl GTrainingUnit {
    #[func]
    fn setup(&mut self, new_base: Gd<GDefinitionUnit>, error_margin: f64) {
        self.training_unit = TrainingUnit::new(&new_base.bind().definition_unit, error_margin);
    }

    #[func]
    fn feed(&mut self, new_instance: Gd<GDefinitionUnit>) {
        self.training_unit.feed(new_instance.bind().definition_unit.clone());
    }

    #[func]
    fn start_training_w_report(&mut self) {
        self.training_unit.train_w_report();
    }
}




