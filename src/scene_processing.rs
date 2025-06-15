use std::collections::HashSet;
use crate::scene::Scene;

impl Scene {
    /// Uses `get_planar_simplify` method for [Mesh]es only with the given `id`.
    ///
    /// This way you can try to simplify only specific Meshes.
    ///
    /// If any [Mesh] couldn't be simplified for any reason, then it tries to skip this one and continue.
    ///
    /// If any [Element] uses such [Mesh] & it has `face_colors` - such Mesh shouldn't be simplified.
    /// That's when it also should skip such Mesh. Such Meshes cannot be simplified, because then
    /// the `face_colors` won't match.
    ///
    /// For such operation please check manually the output results.
    fn planar_simplify_for_ids(&mut self, ids: HashSet<usize>, tolerance: f64, angle_tolerance: f64)  {
        let ids_to_ignore = self.get_mesh_ids_with_face_colors_applied();

        for id in ids {
            if !ids_to_ignore.contains(&id) {
                let mesh = self.meshes.iter_mut().find(|x| x.id == Some(id)).unwrap();
                let remeshed_result = mesh.get_planar_simplify(tolerance, angle_tolerance);
                if remeshed_result.is_ok() {
                    let mut remeshed = remeshed_result.unwrap();
                    std::mem::swap(mesh, &mut remeshed);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use serde_json::{from_value, to_string};
    use super::*;

    #[test]
    fn test_planar_simplify_for_ids_multiple_meshes() {
        let read_file = fs::File::open("models/MultipleMeshes.bim").expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");
        let mut scene_unpacked: Scene = from_value(json).unwrap();

        let mut ids = HashSet::new();
        ids.insert(0);
        ids.insert(2);

        scene_unpacked.planar_simplify_for_ids(ids, 0.0001, 0.0001);

        let scene_serialized = to_string(&scene_unpacked);
        let scene_serialized_string = scene_serialized.unwrap();

        fs::write("created_files/MultipleMeshesMeshId0And2PlanarSimplify.bim",
                  scene_serialized_string).expect("Unable to write the file");

/*        let expected_file = fs::File::open("models/MultipleMeshesMeshId0And2PlanarSimplify.bim").expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(expected_file).expect("File has to be a proper JSON file");
        let expected_scene_unpacked: Scene = from_value(json).unwrap();

        assert!(expected_scene_unpacked.eq(&scene_unpacked));*/
    }
}