use crate::mesh::Mesh;
use std::fs;
use std::path::Path;
use serde_json::{from_value};
use dotbim_rust::file::File;
use dotbim_rust::mesh;

impl Mesh {

    /// Gets all [Mesh]es from dotbim file.
    ///
    /// It takes all Meshes from the "meshes" list in file.
    /// It doesn't move & rotate them according to "element" translations.
    ///
    /// # Example
    ///
    /// Let's read a Pyramid.bim file and take all (in this example 1) meshes from it.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let path = "models/dotbim/Pyramid.bim";
    /// let actual = Mesh::get_meshes_from_dotbim(path);
    /// let expected_mesh = Mesh::new(
    /// vec![
    ///     // Base
    ///     0.0,0.0,0.0,
    ///     10.0,0.0,0.0,
    ///     10.0,10.0,0.0,
    ///     0.0,10.0,0.0,
    ///
    ///     // Top
    ///     5.0,5.0,4.0
    /// ],
    /// vec![
    ///     // Base faces
    ///     0,1,2,
    ///     0,2,3,
    ///
    ///     // Side faces
    ///     0,1,4,
    ///     1,2,4,
    ///     2,3,4,
    ///     3,0,4
    /// ]
    /// );
    ///
    /// assert_eq!(actual.len(), 1);
    /// assert_eq!(actual[0].eq(&expected_mesh), true);
    /// ```
    pub fn get_meshes_from_dotbim<P: AsRef<Path>>(path: P) -> Vec<Mesh> {
        let read_file = fs::File::open(path).expect("Cannot read the file");
        let json: serde_json::Value = serde_json::from_reader(read_file).expect("File has to be a proper JSON file");
        let read_file_unpacked: File = from_value(json).unwrap();

        let mut meshes = Vec::<Mesh>::new();
        for dotbim_mesh in read_file_unpacked.meshes {
            meshes.push(Self::dotbim_mesh_to_native_mesh(dotbim_mesh));
        }

        meshes
    }

    /// Converts dotbim mesh into native [Mesh].
    fn dotbim_mesh_to_native_mesh(dotbim_mesh: mesh::Mesh) -> Mesh {
        let mut indices_converted = Vec::<usize>::new();
        for index in dotbim_mesh.indices {
            let index_converted = usize::try_from(index).unwrap();
            indices_converted.push(index_converted);
        }
        Mesh::new(dotbim_mesh.coordinates.clone(), indices_converted)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::to_string;
    use super::*;

    #[test]
    fn test_get_meshes_from_dotbim() {
        let path = "models/dotbim/Pyramid.bim";
        let actual = Mesh::get_meshes_from_dotbim(path);
        let expected_mesh = Mesh::new(
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

        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].eq(&expected_mesh), true);
    }

/*    #[test]
    fn test_get_mesh_from_dotbim_and_weld() {
        let path = "models/dotbim/Teapot.bim";
        let actual = Mesh::get_meshes_from_dotbim(path);
        let teapot_welded = actual[0].get_with_welded_vertices(0.0001);
        let teapot_welded_serialized = to_string(&teapot_welded).ok().unwrap();
    }*/
}