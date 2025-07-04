use std::collections::HashMap;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::mesh::Mesh;
use crate::quaternion::Quaternion;
use crate::scene::Scene;
use crate::vector::Vector;

impl Scene {
    /// Modifies [Scene] to have deduplicated [Mesh]es.
    fn deduplicate_meshes(&mut self, tolerance: f64) {
        self.duplicate_meshes();

        let deduplication_result = Mesh::deduplicate_with_id_info(self.meshes.clone(), tolerance);

        let number_of_meshes = deduplication_result.len();
        let mut new_meshes: Vec<Mesh> = Vec::with_capacity(number_of_meshes);

        for i in 0..number_of_meshes {
            let current = &deduplication_result[i].0;
            let clone = Mesh::new_with_id(current.id, current.coordinates.clone(), current.indices.clone());
            new_meshes.push(clone);
        }
        
        self.meshes = new_meshes;

        let number_of_elements = self.elements.len();

        for i in 0..number_of_elements {
            let current = &self.elements[i];
            let old_mesh_id = current.mesh_id;

            let new_id_and_local_coordinate_system = Self::get_new_mesh_id_and_local_coordinate_system_from_deduplication_result(old_mesh_id, &deduplication_result);
            let new_local_coordinate_system = new_id_and_local_coordinate_system.1;

            let new_mesh_id = new_id_and_local_coordinate_system.0;
            let new_quaternion = LocalCoordinateSystem::global().get_rotation_to(&new_local_coordinate_system);
            let new_vector = new_local_coordinate_system.origin.to_vector();

            self.elements[i].mesh_id = new_mesh_id;
            self.elements[i].vector = new_vector;
            self.elements[i].rotation = new_quaternion;
        }
    }

    /// Modifies [Scene] to have duplicated [Mesh]es, so each [Element] will have 1 corresponding
    /// Mesh, which should not be reused by any other Element.
    ///
    /// Each Element after this process should have 0,0,0 `vector` (no moving) & 0,0,0,1 `rotation`
    /// (no rotating).
    ///
    /// This process should also remove unused Meshes as a side effect.
    fn duplicate_meshes(&mut self) {
        let number_of_elements = self.elements.len();

        let mut duplicated_meshes: Vec<Mesh> = Vec::with_capacity(number_of_elements);

        for i in 0..number_of_elements {
            let mut mesh = self.get_transformed_mesh_for_element(&self.elements[i]);
            mesh.id = Some(i);
            duplicated_meshes.push(mesh);

            self.elements[i].vector = Vector::zero();
            self.elements[i].rotation = Quaternion::identity();
            self.elements[i].mesh_id = i;
        }

        self.meshes = duplicated_meshes;
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
}

/*#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::{from_value, to_string};
    use super::*;

    #[test]
    pub fn test_deduplicate_meshes_pyramid() {
        let path = "created_files/Pyramid.bim";
        let read_file = fs::File::open(path).expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");
        let mut scene: Scene = from_value(json).unwrap();
        scene.deduplicate_meshes(0.001);
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/PyramidDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");
    }

    #[test]
    pub fn test_deduplicate_meshes_cubes() {
        let path = "created_files/Cubes.bim";
        let read_file = fs::File::open(path).expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");
        let mut scene: Scene = from_value(json).unwrap();
        scene.deduplicate_meshes(0.001);
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/CubesDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");
    }

    #[test]
    pub fn test_deduplicate_meshes_structure() {
        let path = "models/TestStructure.bim";
        let read_file = fs::File::open(path).expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");
        let mut scene: Scene = from_value(json).unwrap();
        println!("{0}", scene.meshes.len());
        scene.deduplicate_meshes(0.001);
        println!("{0}", scene.meshes.len());
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/TestStructureDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");
    }

    #[test]
    pub fn test_deduplicate_meshes_beambridge() {
        let path = "models/BeamBridgeExample.bim";
        let read_file = fs::File::open(path).expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");
        let mut scene: Scene = from_value(json).unwrap();
        println!("{0}", scene.meshes.len());
        scene.deduplicate_meshes(0.001);
        println!("{0}", scene.meshes.len());
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/BeamBridgeExampleDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");
    }

    #[test]
    pub fn test_deduplicate_meshes_samplehouse() {
        let path = "models/SampleHouse.bim";
        let read_file = fs::File::open(path).expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");
        let mut scene: Scene = from_value(json).unwrap();
        println!("{0}", scene.meshes.len());
        scene.deduplicate_meshes(0.001);
        println!("{0}", scene.meshes.len());
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/SampleHouseDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");
    }
}*/