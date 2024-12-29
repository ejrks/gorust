use mvecops::get_inflexions_from_vector;
use mvecops::get_curve_no_reductions;
use mvecops::naudr::closed_curves::GlobalCurveData;
use mvecops::get_bloat_data;

use godot::global::godot_print;

use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct CurveGeneration {
    #[var]
    pub curve_definitions: Array<i64>,
    #[var]
    pub curve_point_order: Array<i64>,

    base: Base<Node3D>
}

#[godot_api]
impl INode3D for CurveGeneration {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            curve_definitions: Array::new(),
            curve_point_order: Array::new(),

            base,
        }
    }
}

#[godot_api]
impl CurveGeneration {
    #[func]
    fn fill_curve_data(&mut self, input_data: Array<i64>, sample_size: i64, dominants_recurrency: i64) {
        let mut format_input_data: Vec<u32> = vec![];
        let format_sample_size: usize = sample_size as usize;
        let format_dominants_recurrency: usize = dominants_recurrency as usize;

        for entry in input_data.iter_shared() {
            format_input_data.push(entry.clone() as u32);
        }

        let result = get_inflexions_from_vector(format_input_data, format_sample_size, format_dominants_recurrency);

        let curve_definitions_u32 = result.curves_global_output.data;
        let curve_point_order_u32 = result.curves_global_orderd.data;

        let mut output_definitions_format: Array<i64> = Array::new();
        for entry in curve_definitions_u32 {
            output_definitions_format.push(entry.clone() as i64);
        }
        self.curve_definitions = output_definitions_format;

        let mut output_point_order_format: Array<i64> = Array::new();
        for entry in curve_point_order_u32 {
            output_point_order_format.push(entry.clone() as i64);
        }
        self.curve_point_order = output_point_order_format;
    }

    #[func]
    fn fill_curves_no_reduction(&mut self, input_data: Array<i64>, sample_size: i64, dominants_recurrency: i64) {
        let mut format_input_data: Vec<u32> = vec![];
        let format_sample_size: usize = sample_size as usize;
        let format_dominants_recurrency: usize = dominants_recurrency as usize;

        for entry in input_data.iter_shared() {
            format_input_data.push(entry.clone() as u32);
        }

        let result = get_curve_no_reductions(format_input_data, format_sample_size, format_dominants_recurrency);

        let curve_definitions_u32 = result.curves_global_output.data;
        let curve_point_order_u32 = result.curves_global_orderd.data;

        let mut output_definitions_format: Array<i64> = Array::new();
        for entry in curve_definitions_u32 {
            output_definitions_format.push(entry.clone() as i64);
        }
        self.curve_definitions = output_definitions_format;

        let mut output_point_order_format: Array<i64> = Array::new();
        for entry in curve_point_order_u32 {
            output_point_order_format.push(entry.clone() as i64);
        }
        self.curve_point_order = output_point_order_format;
    }

    #[func]
    fn fill_bloat_data(&mut self, input_data: Array<i64>, sample_size: i64) {
        let mut format_input_data: Vec<u32> = vec![];
        let format_sample_size: usize = sample_size as usize;

        for entry in input_data.iter_shared() {
            format_input_data.push(entry.clone() as u32);
        }

        let result = get_bloat_data(format_input_data, format_sample_size);

        let curve_definitions_u32 = result.curves_global_output.data;
        let curve_point_order_u32 = result.curves_global_orderd.data;

        let mut output_definitions_format: Array<i64> = Array::new();
        for entry in curve_definitions_u32 {
            output_definitions_format.push(entry.clone() as i64);
        }
        self.curve_definitions = output_definitions_format;

        let mut output_point_order_format: Array<i64> = Array::new();
        for entry in curve_point_order_u32 {
            output_point_order_format.push(entry.clone() as i64);
        }
        self.curve_point_order = output_point_order_format;
    }
}