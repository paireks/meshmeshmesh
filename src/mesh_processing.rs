use crate::mesh::Mesh;
use std::collections::{HashMap, HashSet};

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

    /// Removes specified vertices of a [Mesh], but it doesn't update its indices.
    ///
    /// It means that it will only affect coordinates list. In other words: faces will be specified
    /// same way as before.
    ///
    /// This method can be useful to e.g. remove unused vertices (vertices that are not used by
    /// any face).
    ///
    /// # Example
    ///
    /// Here in this example 2 different vertices (id. 1 and id. 3) are being removed. You can
    /// see there that indices are not updated, and because of that they point into different
    /// vertices if you'll leave it like that.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use meshmeshmesh::mesh::Mesh;
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
    /// let replacement_instructions = HashSet::from([1, 3]);
    ///
    /// let expected = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     //8.0,1.0,0.0, <- removed (id. 1)
    ///     8.0,11.0,0.0,
    ///     //-2.0,11.0,0.0, <- removed (id. 3)
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
    /// let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_removed_vertices_without_indices_update(&self, indices_of_vertices_to_remove:HashSet<usize>) -> Mesh {

        let mut to_remove_vector = Vec::from_iter(indices_of_vertices_to_remove.iter());
        to_remove_vector.sort_unstable();
        to_remove_vector.reverse(); // This reversing allows to remove coordinates going from bottom to top,
        // which is easier, as it doesn't require keeping updates of indices to remove

        let max_vertex_id = self.get_number_of_vertices() - 1;
        if to_remove_vector[0] > &max_vertex_id {
            panic!("Set of indices of vertices to be removed is > than the number of vertices.");
        }

        let mut new_coordinates: Vec<f64> = self.coordinates.clone();
        for index_to_remove in to_remove_vector {
            let offset = index_to_remove*3;
            new_coordinates.remove(offset); // x
            new_coordinates.remove(offset); // y
            new_coordinates.remove(offset); // z. Three times same offset, because after each removal position shifts to another coordinate to be removed (x => y => z)
        }

        Mesh::new(new_coordinates, self.indices.clone())
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

#[test]
fn test_get_with_removed_vertices_without_indices_update_first_removed() {
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

    let replacement_instructions = HashSet::from([0]);

    let expected = Mesh::new(
        vec![
            // Base
            //-2.0,1.0,0.0, <- removed
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
    let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
    assert_eq!(expected.eq(&actual), true);
}

#[test]
fn test_get_with_removed_vertices_without_indices_update_last_removed() {
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

    let replacement_instructions = HashSet::from([4]);

    let expected = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,

            // Top
            //3.0,6.0,4.0 <- removed
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
    let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
    assert_eq!(expected.eq(&actual), true);
}

#[test]
fn test_get_with_removed_vertices_without_indices_update_last_two_in_the_middle_removed() {
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

    let replacement_instructions = HashSet::from([1, 3]);

    let expected = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            //8.0,1.0,0.0, <- removed
            8.0,11.0,0.0,
            //-2.0,11.0,0.0, <- removed

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
    let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
    assert_eq!(expected.eq(&actual), true);
}

#[test]
fn test_get_with_removed_vertices_without_indices_update_all_removed() {
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

    let replacement_instructions = HashSet::from([2,3,1,0,4]);

    let expected = Mesh::new(
        vec![
            // All removed
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
    let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
    assert_eq!(expected.eq(&actual), true);
}

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