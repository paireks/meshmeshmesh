use crate::mesh::Mesh;

impl Mesh {

/*    /// Gets a new Mesh, but with all faces flipped
    pub fn get_with_all_faces_flipped(&self) -> Mesh {
        let mut new_indices: Vec<usize> = Vec::<usize>::new();
        let start_indices = self.get_start_indices();
        let middle_indices = self.get_middle_indices();
        let end_indices = self.get_end_indices();

        for index in self.indices {
            new_indices.push(index);
        }

        Mesh::new(self.coordinates.clone(), new_indices)
    }*/
}