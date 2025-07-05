use std::collections::HashMap;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::mesh::Mesh;
use crate::quaternion::Quaternion;
use crate::scene::Scene;
use crate::vector::Vector;

impl Scene {
    /// Modifies [Scene] to have deduplicated [Mesh]es.
    ///
    /// The correctness after deduplication should be checked also manually.
    ///
    /// This method can more or less deform the scene, because of the transformations and other
    /// operations made on the geometries (Meshes). That's one of the reasons to check model
    /// manually to see if these deformations look acceptable.
    pub (crate) fn deduplicate_meshes(&mut self, tolerance: f64) {
        self.duplicate_meshes();

        let duplicated_meshes_transformed = self.meshes.clone(); // Used later for correctness check. These Meshes order is aligned with Elements order.

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

        // From this moment there is only a correctness check.
        for i in 0..number_of_elements {
            let new_mesh = self.get_transformed_mesh_for_element(&self.elements[i]);
            let old_mesh = &duplicated_meshes_transformed[i];
            assert_eq!(Some(i), old_mesh.id); // Makes sure old meshes duplicated were sorted and were properly cloned
            assert!(new_mesh.eq_with_tolerance_without_id(old_mesh, tolerance));
        }
    }

    /// Modifies [Scene] to have duplicated [Mesh]es, so each [Element] will have 1 corresponding
    /// Mesh, which should not be reused by any other Element.
    ///
    /// Each Element after this process should have 0,0,0 `vector` (no moving) & 0,0,0,1 `rotation`
    /// (no rotating).
    ///
    /// This process should also remove unused Meshes as a side effect.
    pub (crate) fn duplicate_meshes(&mut self) {
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

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::{from_value, to_string};
    use crate::color::Color;
    use crate::element::Element;
    use super::*;

    #[test]
    pub fn test_deduplicate_meshes_pyramid() {
/*        let path = "created_files/Pyramid.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let mut scene: Scene = from_value(json).unwrap();
        scene.deduplicate_meshes(0.001);
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/PyramidDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");*/

        let mesh = Mesh::new_with_id(
            Some(0),
            vec![
                // Base
                0.0,0.0,0.0,
                10.0,0.0,0.0,
                10.0,10.0,0.0,
                0.0,10.0,0.0,

                // Top
                5.0,5.0,4.0
            ],
            vec![
                // Base faces
                0,1,2,
                0,2,3,

                // Side faces
                0,1,4,
                1,2,4,
                2,3,4,
                3,0,4
            ]
        );

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Pyramid"));

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("John Doe"));
        file_info.insert(String::from("Date"), String::from("28.09.1999"));

        let element = Element::new(
            0,
            Vector::new(0.,0.,0.),
            Quaternion::new(0., 0., 0., 1.),
            String::from("76e051c1-1bd7-44fc-8e2e-db2b64055068"),
            String::from("Structure"),
            Color::new(255,255,0,255),
            None,
            info,
        );

        let mut actual: Scene = Scene::new(String::from("1.0.0"),
                                     vec![mesh],
                                     vec![element],
                                     file_info);

        actual.deduplicate_meshes(0.001);

        let path = "models/expected/PyramidDeduplication.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let expected: Scene = from_value(json).unwrap();

        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }

    #[test]
    pub fn test_deduplicate_meshes_cubes() {
/*        let path = "created_files/Cubes.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let mut scene: Scene = from_value(json).unwrap();
        scene.deduplicate_meshes(0.001);
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/CubesDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");*/

        let mesh = Mesh::new_with_id(
            Some(0),
            vec![
                0.0, 0.0, 0.0,
                10.0, 0.0, 0.0,
                10.0, 0.0, 20.0,
                0.0, 0.0, 20.0,
                0.0, 30.0, 0.0,
                10.0, 30.0, 0.0,
                10.0, 30.0, 20.0,
                0.0, 30.0, 20.0
            ],
            vec![
                // Front side
                0, 1, 2,
                0, 2, 3,

                // Bottom side
                0, 1, 4,
                1, 4, 5,

                // Left side
                0, 4, 3,
                4, 3, 7,

                // Right side
                1, 2, 5,
                2, 5, 6,

                // Top side
                2, 3, 7,
                2, 6, 7,

                // Back side
                4, 5, 7,
                5, 6, 7
            ]
        );

        let mut info1: HashMap<String, String> = HashMap::new();
        info1.insert(String::from("Name"), String::from("Red Cube"));

        let mut info2: HashMap<String, String> = HashMap::new();
        info2.insert(String::from("Name"), String::from("Green Cube"));

        let mut info3: HashMap<String, String> = HashMap::new();
        info3.insert(String::from("Name"), String::from("Blue Cube"));

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("John Doe"));

        let element1 = Element::new(
            0,
            Vector::new(-100.,-100.,-100.),
            Quaternion::new(0., 0., 0., 1.),
            String::from("9f61b565-06a2-4bef-8b72-f37091ab54d6"),
            String::from("Brick"),
            Color::new(255,0,0,255),
            None,
            info1,
        );

        let element2 = Element::new(
            0,
            Vector::new(0.,0.,0.),
            Quaternion::new(0., 0., 0., 1.),
            String::from("4d00c967-791a-42a6-a5e8-cf05831bc11d"),
            String::from("Brick"),
            Color::new(0,255,0,126),
            None,
            info2,
        );

        let element3 = Element::new(
            0,
            Vector::new(100.,100.,100.),
            Quaternion::new(0., 0., 0., 1.),
            String::from("8501a5e3-4709-47d8-bd5d-33d745a435d5"),
            String::from("Brick"),
            Color::new(0,0,255,10),
            None,
            info3,
        );

        let mut actual: Scene = Scene::new(String::from("1.0.0"),
                                     vec![mesh],
                                     vec![element1, element2, element3],
                                     file_info);

        actual.deduplicate_meshes(0.001);

        let path = "models/expected/CubesDeduplication.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let expected: Scene = from_value(json).unwrap();

        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }

    #[test]
    pub fn test_deduplicate_meshes_multiple_meshes() {
/*        let path = "models/expected/MultipleMeshesDuplication.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let mut scene: Scene = from_value(json).unwrap();
        scene.deduplicate_meshes(0.001);
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/MultipleMeshesDeduplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");*/

        let path = "models/input/MultipleMeshesDuplication.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let mut actual: Scene = from_value(json).unwrap();
        actual.deduplicate_meshes(0.001);

        let path = "models/expected/MultipleMeshesDeduplication.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let expected: Scene = from_value(json).unwrap();

        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }

    #[test]
    pub fn test_duplicate_meshes_multiple_meshes() {
/*        let path = "models/MultipleMeshes.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let mut scene: Scene = from_value(json).unwrap();
        scene.duplicate_meshes();
        let file_serialized = to_string(&scene);
        let file_serialized_string = file_serialized.ok().unwrap();
        let path_after = "created_files/MultipleMeshesDuplication.bim";
        fs::write(path_after, file_serialized_string).expect("Unable to write the file");*/

        let path = "models/input/MultipleMeshes.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let mut actual: Scene = from_value(json).unwrap();
        actual.duplicate_meshes();

        let path = "models/expected/MultipleMeshesDuplication.bim";
        let read_file = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
        let expected: Scene = from_value(json).unwrap();

        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }
}