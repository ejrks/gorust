use mvecops::beorc::def::DefinitionUnit;
use mvecops::beorc::def::TrainingUnit;
use mvecops::beorc::database::LivingDataUnit;
use mvecops::beorc::medium::Medium;
use mvecops::beorc::def::Trace;

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

    #[func]
    fn training_and_return(&mut self) -> Gd<GDefinitionUnit> {
        let result = self.training_unit.train_w_report();

        let new_object = GDefinitionUnit {
            definition_unit: result,
        };

        let new_definition: Gd<GDefinitionUnit> = Gd::from_object(new_object);
        return new_definition;
    }

    
    #[func]
    fn q_base_name(&self) -> GString {
        return GString::from(self.training_unit.base.id.clone());
    }
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GLivingDataUnit {
    pub data: LivingDataUnit,
}

#[godot_api]
impl INode3D for GLivingDataUnit {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            data: LivingDataUnit::empty(),
        }
    }
}

#[godot_api]
impl GLivingDataUnit {
    #[func]
    fn load_from_file(&mut self, quick_target: GString, heavy_target: GString, resolution: i64) {
        self.data.load_from_file(quick_target.to_string(), heavy_target.to_string(), resolution);
    }
    #[func]
    fn dump_to_file(&mut self, append_name: GString) {
        self.data.dump_to_file(append_name.to_string());
    }

    #[func]
    fn new_or_update_definition(&mut self, input_data: Gd<GDefinitionUnit>) {
        let unpacked_definition: &DefinitionUnit = &input_data.bind().definition_unit;
        let definition_name = unpacked_definition.id.to_string();
        let mut found_old_entry: bool = false;
        for entry in &mut self.data.definitions {
            if (entry.id.to_string() == definition_name) {
                *entry = unpacked_definition.clone();
                found_old_entry = true;
                break;
            }
        }
        if !found_old_entry {
            self.data.definitions.push(unpacked_definition.clone());
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GMedium {
    pub medium_instance: Medium,
}

#[godot_api]
impl INode3D for GMedium {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            medium_instance: Medium::empty(),
        }
    }
}


#[godot_api]
impl GMedium {
    #[func]
    fn setup(&mut self, data_unit: Gd<GLivingDataUnit>) {
        self.medium_instance = Medium::new(data_unit.bind().data.clone());
    }
    #[func]
    fn reset_search(&mut self) {
        self.medium_instance.reset_search();
    }
    #[func]
    fn feed_trace(&mut self, indexes: Array<i64>, time_stamp: i64, resolution: i64) {
        let mut formatted_data_array: Vec<i64> = Vec::new();
        for entry in indexes.iter_shared() {
            formatted_data_array.push(entry);
        }
        let new_trace: Trace = Trace::new(time_stamp, formatted_data_array ,resolution);
        self.medium_instance.feed_trace(new_trace);
    }
    #[func]
    fn get_predictions(&self) -> Array<GString> {
        let result = self.medium_instance.get_list_of_predictions().0;
        let mut new_array: Array<GString> = Array::new();
        for entry in result {
            new_array.push(&GString::from(entry));
        }
        return new_array;
    }
}

