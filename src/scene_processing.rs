/*use std::collections::HashMap;
use crate::element::Element;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::mesh::Mesh;
use crate::scene::Scene;

impl Scene {
    /// Modifies [Scene] to have deduplicated [Mesh]es.
    fn deduplicate_meshes(mut self, tolerance: f64) {
        let deduplication_result = Mesh::deduplicate_with_id_info(self.meshes, tolerance);

        let number_of_meshes = deduplication_result.len();
        let mut new_meshes: Vec<Mesh> = Vec::with_capacity(number_of_meshes);

        for i in 0..number_of_meshes {
            let current = &deduplication_result[i].0;
            let clone = Mesh::new_with_id(current.id, current.coordinates.clone(), current.indices.clone());
            new_meshes.push(clone);
        }
        
        self.meshes = new_meshes;

        let number_of_elements = self.elements.len();
        let mut new_elements: Vec<Element> = Vec::with_capacity(number_of_elements);

        for i in 0..number_of_elements {
            let current = &self.elements[i];
            let old_mesh_id = current.mesh_id;
            let old_vector = current.vector;
            let old_quaternion = current.rotation;

            let new_id_and_local_coordinate_system = Self::get_new_mesh_id_and_local_coordinate_system_from_deduplication_result(old_mesh_id, &deduplication_result);
            let new_local_coordinate_system = new_id_and_local_coordinate_system.1;

            let new_mesh_id = new_id_and_local_coordinate_system.0;
            
        }
        
        self.elements = new_elements;
    }

    fn get_new_mesh_id_and_local_coordinate_system_from_deduplication_result(old_mesh_id: usize, deduplication_result: &Vec<(Mesh, HashMap<usize, LocalCoordinateSystem>)>) -> (usize, LocalCoordinateSystem) {
        for deduplicated_mesh_result in deduplication_result {
            let mesh = &deduplicated_mesh_result.0;
            let map = &deduplicated_mesh_result.1;
            if map.contains_key(&old_mesh_id) {
                let new_coordinate_system = map[&old_mesh_id];
                let new_mesh_id = mesh.id.unwrap();

                return (new_mesh_id, new_coordinate_system);
            }
        }

        panic!("Couldn't find new mesh_id for deduplication.")
    }
}*/