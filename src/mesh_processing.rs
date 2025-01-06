use crate::mesh::Mesh;
use std::collections::HashMap;

impl Mesh {

    /// Creates a new [Mesh], but with all faces flipped
    ///
    /// Orientation of Mesh's faces is regulated by right hand thumb rule.
    ///
    /// This method allows to flip all the faces, so all normals will be reversed.
    ///
    /// # Example
    ///
    /// Here is an example of flipping all faces for a pyramid Mesh
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
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
    /// ]);
    /// let expected = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
    /// ],
    /// vec![
    ///     // Base faces flipped
    ///     2,1,0,
    ///     3,2,0,
    ///
    ///     // Side faces flipped
    ///     4,1,0,
    ///     4,2,1,
    ///     4,3,2,
    ///     4,0,3
    /// ]);
    /// let actual = input.get_with_all_faces_flipped();
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_all_faces_flipped(&self) -> Mesh {
        let mut new_indices: Vec<usize> = Vec::<usize>::new();
        let start_indices = self.get_start_indices();
        let middle_indices = self.get_middle_indices();
        let end_indices = self.get_end_indices();
        let number_of_faces = self.get_number_of_faces();

        for i in 0..number_of_faces {
            new_indices.push(end_indices[i]);
            new_indices.push(middle_indices[i]);
            new_indices.push(start_indices[i]);
        }

        Mesh::new(self.coordinates.clone(), new_indices)
    }

/*    pub fn get_with_welded_vertices(&self) -> Mesh {
        // This field saves information about which indices should be switched and to which new value
        let mut translate_indices_instruction: HashMap<usize, usize> = HashMap::new();;

    }*/

    /// Allows to replace specific indices with new ones
    ///
    /// Creates the new [Mesh], but with replaced indices
    ///
    /// # Arguments
    ///
    /// * `replacement_instruction` - this argument determines which indices should be switched and what should be the new value.
    /// For instance (<1, 21>, <5, 16>) -> it means it should replace 1 into 21 and 5 into 16.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
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
    /// ]);
    ///
    /// let mut replacement_instructions = HashMap::new();
    /// replacement_instructions.insert(0, 3);
    /// replacement_instructions.insert(4, 1);
    ///
    /// let expected = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
    /// ],
    /// vec![
    ///     // Base faces
    ///     3,1,2, // 0 -> 3
    ///     3,2,3, // 0 -> 3
    ///
    ///     // Side faces
    ///     3,1,1, // 0 -> 3 & 4 -> 1
    ///     1,2,1, // 4 -> 1
    ///     2,3,1, // 4 -> 1
    ///     3,3,1, // 0 -> 3 & 4 -> 1
    /// ]);
    /// let actual = input.get_with_replaced_indices(replacement_instructions);
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_replaced_indices(&self, replacement_instruction:HashMap<usize, usize>) -> Mesh {
        let mut new_indices: Vec<usize> = Vec::<usize>::new();
        for i in &self.indices {
            if replacement_instruction.contains_key(&i) {
                let replacement = replacement_instruction.get(i).unwrap().clone();
                new_indices.push(replacement);
            }
            else {
                new_indices.push(i.clone());
            }
        }

        Mesh::new(self.coordinates.clone(), new_indices)
    }
}

#[test]
fn test_get_with_all_faces_flipped() {
    let input = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,

            // Top
            3.0,6.0,4.0
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
        ]);
    let expected = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,

            // Top
            3.0,6.0,4.0
        ],
        vec![
            // Base faces flipped
            2,1,0,
            3,2,0,

            // Side faces flipped
            4,1,0,
            4,2,1,
            4,3,2,
            4,0,3
        ]);
    let actual = input.get_with_all_faces_flipped();

    assert_eq!(expected.eq(&actual), true);
}

/*#[test]
fn test_get_with_welded_vertices() {}*/

#[test]
fn test_get_with_replaced_indices() {
    let input = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,

            // Top
            3.0,6.0,4.0
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
        ]);

    let mut replacement_instructions = HashMap::new();
    replacement_instructions.insert(0, 3);
    replacement_instructions.insert(4, 1);

    let expected = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,

            // Top
            3.0,6.0,4.0
        ],
        vec![
            // Base faces
            3,1,2, // 0 -> 3
            3,2,3, // 0 -> 3

            // Side faces
            3,1,1, // 0 -> 3 & 4 -> 1
            1,2,1, // 4 -> 1
            2,3,1, // 4 -> 1
            3,3,1, // 0 -> 3 & 4 -> 1
        ]);
    let actual = input.get_with_replaced_indices(replacement_instructions);
    assert_eq!(expected.eq(&actual), true);
}