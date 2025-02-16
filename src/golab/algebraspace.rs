use godot::prelude::*;
use godot::classes::Node3D;

use godot::classes::INode3D;

use uotech::aspace::basis::Basis;

/// Godot interface for [`uotech::aspace::basis::Basis`].
///
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct GAlgebraBasis {
    pub basis: Basis,
    pub points: Vec<Vec<f64>>,
}

#[godot_api]
impl INode3D for GAlgebraBasis {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            basis: Basis::empty(),
            points: Vec::new(),
        }
    }
}

#[godot_api]
impl GAlgebraBasis {
    #[func]
    fn add_vector_to_basis(&mut self, input: Array<f64>) {
        let mut new_vector: Vec<f64> = Vec::new();
        for element in input.iter_shared() {
            new_vector.push(element);
        }
        self.basis.vectors.push(new_vector);
    }

    #[func]
    fn generate_points(&mut self, range: i64) -> i64 {
        self.points = self.basis.generate_points(range);
        return self.points.len() as i64;
    }

    #[func]
    fn get_point_at(&self, index: i64) -> Array<f64> {
        let fetched_point = &self.points[index as usize];
        let mut result_array: Array<f64> = Array::new();
        for entry in fetched_point {
            result_array.push(entry.clone());
        }
        return result_array;
    }

    #[func]
    fn reset_basis(&mut self) {
        self.basis = Basis::empty();
    }
}
