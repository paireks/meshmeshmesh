use crate::mesh::Mesh;
use std::collections::{HashMap, HashSet};
use crate::face_neighbours::FaceNeighbours;
use crate::graph::Graph;
use crate::point::Point;

impl Mesh {

    /// Creates a new [Mesh], but with selected faces flipped
    ///
    /// Orientation of Mesh's faces is regulated by right hand thumb rule.
    ///
    /// This method allows to flip selected faces, so the normals will be reversed.
    /// 
    /// Selecting is controlled with `indices_of_faces_to_remove` parameter.
    ///
    /// # Example
    ///
    /// Here is an example of flipping selected 3 faces (with ids: 0, 3, and 5) for a pyramid Mesh
    ///
    /// ```
    /// use std::collections::HashSet;
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
    ///     0,1,2, // to be flipped (0)
    ///     0,2,3,
    ///
    ///     // Side faces
    ///     0,1,4,
    ///     1,2,4, // to be flipped (3)
    ///     2,3,4,
    ///     3,0,4 // to be flipped (5)
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
    ///     2,1,0, // flipped (0)
    ///     0,2,3,
    ///
    ///     // Side faces flipped
    ///     0,1,4,
    ///     4,2,1, // flipped (3)
    ///     2,3,4,
    ///     4,0,3 // flipped (5)
    /// ]);
    /// let actual = input.get_with_faces_flipped(HashSet::from([0, 3, 5]));
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_faces_flipped(&self, indices_of_faces_to_remove:HashSet<usize>) -> Mesh {
        let mut new_indices: Vec<usize> = Vec::<usize>::new();
        let start_indices = self.get_start_indices();
        let middle_indices = self.get_middle_indices();
        let end_indices = self.get_end_indices();
        let number_of_faces = self.get_number_of_faces();

        for i in 0..number_of_faces {
            if indices_of_faces_to_remove.contains(&i) {
                new_indices.push(end_indices[i]);
                new_indices.push(middle_indices[i]);
                new_indices.push(start_indices[i]);
            }
            else {
                new_indices.push(start_indices[i]);
                new_indices.push(middle_indices[i]);
                new_indices.push(end_indices[i]);
            }
        }

        Mesh::new(self.coordinates.clone(), new_indices)
    }

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
    
    /// Tries to flip the faces of the [Mesh] using offset. 
    /// 
    /// It offsets origin of normal for every face and checks if it's inside the Mesh.
    /// If yes -> then it flips.
    /// 
    /// # Example
    ///
    /// ```
    /// use std::collections::HashSet;
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
    ///     // Base faces flipped
    ///     0,1,2, // flipped
    ///     3,2,0,
    ///
    ///     // Side faces flipped
    ///     0,1,4,
    ///     4,2,1, // flipped
    ///     2,3,4,
    ///     4,0,3 // flipped
    /// ]);
    /// let actual = input.get_with_faces_flipped_outside_using_offset(0.001);
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
    ///     2,1,0, // fixed
    ///     3,2,0,
    ///
    ///     // Side faces flipped
    ///     0,1,4,
    ///     1,2,4, // fixed
    ///     2,3,4,
    ///     3,0,4 // fixed
    /// ]);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_faces_flipped_outside_using_offset(&self, offset:f64) -> Mesh {
        let faces_to_flip = self.get_ids_of_faces_flipped_inside_using_offset(offset);
        self.get_with_faces_flipped(faces_to_flip)
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

    /// Welds vertices of the given [Mesh].
    ///
    /// In other words, it searches for duplicate (with tolerance) vertices and removes these duplicates.
    ///
    /// It updates both coordinates and indices and creates a new Mesh with it.
    ///
    /// It can be useful for several purposes, including making Meshes more compact.
    ///
    /// # Example
    ///
    /// Here is the example with pyramid that has a lot of duplicate vertices, in fact each
    /// triangle has its own vertices, which is convenient, but not compact. You can see there that
    /// after a welding it detects a duplicate coordinates and remove them.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input= Mesh::new(
    /// vec![
    ///     0.0, 0.0, 0.0, // 0
    ///     10.0, 0.0, 0.0, // 1
    ///     10.0,10.0,0.0, // 2
    ///
    ///     0.0, 0.0, 0.0, // duplicate of 0 -> should be removed
    ///     10.0,10.0,0.0, // duplicate of 2 -> should be removed
    ///     0.0,10.0,0.0, // new 3
    ///
    ///     0.0, 0.0, 0.0, // duplicate of 0 -> should be removed
    ///     10.0, 0.0, 0.0, // duplicate of 1 -> should be removed
    ///     5.0,5.0,4.0, // new 4
    ///
    ///     10.0, 0.0, 0.0, // duplicate of 1 -> should be removed
    ///     10.0,10.0,0.0, // duplicate of 2 -> should be removed
    ///     5.0,5.0,4.0, // duplicate of 8 -> should be removed
    ///
    ///     10.0,10.0,0.0, // duplicate of 2 -> should be removed
    ///     0.0,10.0,0.0, // duplicate of 5 -> should be removed
    ///     5.0,5.0,4.0, // duplicate of 8 -> should be removed
    ///
    ///     0.0,10.0,0.0, // duplicate of 5 -> should be removed
    ///     0.0,0.0,0.0, // duplicate of 0 -> should be removed
    ///     5.0,5.0,4.0, // duplicate of 8 -> should be removed
    /// ],
    /// vec![
    ///     // Base faces
    ///     0,1,2,
    ///     3,4,5,
    ///
    ///     // Side faces
    ///     6,7,8,
    ///     9,10,11,
    ///     12,13,14,
    ///     15,16,17
    /// ]);
    ///
    /// let actual = input.get_with_welded_vertices(0.001);
    /// let expected = Mesh::new(
    /// vec![
    ///     0.0, 0.0, 0.0, // 0
    ///     10.0, 0.0, 0.0, // 1
    ///     10.0,10.0,0.0, // 2
    ///     0.0,10.0,0.0, // new 3
    ///
    ///     5.0,5.0,4.0, // new 4
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
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_welded_vertices(&self, tolerance: f64) -> Mesh {
        let vertices = self.to_points();
        let duplicate_vertices_info = Point::scan_for_duplicates_with_tolerance_info(&vertices, tolerance);
        let info_length = duplicate_vertices_info.len();

        let mut duplicates_above_count: Vec<usize> = Vec::<usize>::new(); // First step is to create a Vec of duplicates above these vertices. It is necessary to apply proper offset later.
        let mut current_duplicates_count = 0;
        for i in 0..info_length {
            duplicates_above_count.push(current_duplicates_count);
            if duplicate_vertices_info[i].1 {
                current_duplicates_count += 1;
            }
        }

        if current_duplicates_count > 0 { // It means there actually were some duplicates at all, so it makes sense to weld
            let mut indices_replacement_instructions: HashMap<usize, usize> = HashMap::<usize, usize>::new();
            let mut vertices_replacement_instructions: HashSet<usize> = HashSet::<usize>::new();

            for i in 0..info_length {
                let offset = duplicates_above_count[duplicate_vertices_info[i].0];
                if duplicate_vertices_info[i].1 { // If true = it is a duplicate
                    let new_index = duplicate_vertices_info[i].0 - offset;
                    indices_replacement_instructions.insert(i, new_index); // Specifies how to switch index
                    vertices_replacement_instructions.insert(i); // Specifies which vertex should be removed
                }
                else { // If false = it's not a duplicate, but still we have to update the index, because of removal of vertices above
                    let new_index = i - offset;
                    indices_replacement_instructions.insert(i, new_index); // Specifies how to switch index
                }
            }

            let mesh_with_replaced_indices = self.get_with_replaced_indices(indices_replacement_instructions);
            let mesh_with_replaced_indices_and_removed_vertices = mesh_with_replaced_indices.get_with_removed_vertices_without_indices_update(vertices_replacement_instructions);

            mesh_with_replaced_indices_and_removed_vertices
        }
        else { // No duplicates - no welding
            Mesh::new(self.coordinates.clone(), self.indices.clone())
        }
    }

    /// Creates a new [Mesh] which has all vertices unwelded.
    ///
    /// It duplicates vertices for every face.
    ///
    /// It might be useful for some cases where each face should have its own vertices.
    ///
    /// However, it makes Meshes much bigger.
    ///
    /// # Example
    ///
    /// Here is an example with pyramid, which has all vertices welded, but after this operation
    /// it's all unwelded.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input = Mesh::new(
    /// vec![
    ///     0.0, 0.0, 0.0,
    ///     10.0, 0.0, 0.0,
    ///     10.0,10.0,0.0,
    ///     0.0,10.0,0.0,
    ///
    ///     5.0,5.0,4.0,
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
    /// let actual = input.get_with_unwelded_vertices();
    ///
    /// let expected = Mesh::new(
    /// vec![
    ///     0.0, 0.0, 0.0, // first face
    ///     10.0, 0.0, 0.0,
    ///     10.0,10.0,0.0,
    ///
    ///     0.0, 0.0, 0.0, // second face
    ///     10.0,10.0,0.0,
    ///     0.0,10.0,0.0,
    ///
    ///     0.0, 0.0, 0.0, // third face
    ///     10.0, 0.0, 0.0,
    ///     5.0,5.0,4.0,
    ///
    ///     10.0, 0.0, 0.0, // fourth face
    ///     10.0,10.0,0.0,
    ///     5.0,5.0,4.0,
    ///
    ///     10.0,10.0,0.0, // fifth face
    ///     0.0,10.0,0.0,
    ///     5.0,5.0,4.0,
    ///
    ///     0.0,10.0,0.0, // sixth face
    ///     0.0,0.0,0.0,
    ///     5.0,5.0,4.0,
    /// ],
    /// vec![
    ///     // Base faces
    ///     0,1,2,
    ///     3,4,5,
    ///
    ///     // Side faces
    ///     6,7,8,
    ///     9,10,11,
    ///     12,13,14,
    ///     15,16,17
    /// ]);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_unwelded_vertices(&self) -> Mesh {
        Mesh::from_triangles(self.to_triangles())
    }

    /// Creates a new [Mesh], but for all indices it offsets them by given number.
    ///
    /// So if offset will be 3, and indices were e.g. (1, 5, 6), then it will be (4, 8, 9).
    ///
    /// It could be useful for some operations, such as e.g. joining of [Mesh]es.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input = Mesh::new(
    /// vec![
    ///     0.0, 0.0, 0.0,
    ///     10.0, 0.0, 0.0,
    ///     10.0,10.0,0.0,
    ///     0.0,10.0,0.0,
    ///
    ///     5.0,5.0,4.0,
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
    /// let offset = 10;
    /// let actual = input.get_with_index_offset(offset);
    ///
    /// let expected = Mesh::new(
    /// vec![
    ///     0.0, 0.0, 0.0,
    ///     10.0, 0.0, 0.0,
    ///     10.0,10.0,0.0,
    ///     0.0,10.0,0.0,
    ///
    ///     5.0,5.0,4.0,
    /// ],
    /// vec![
    ///     // Base faces
    ///     10,11,12,
    ///     10,12,13,
    ///
    ///     // Side faces
    ///     10,11,14,
    ///     11,12,14,
    ///     12,13,14,
    ///     13,10,14
    /// ]);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_with_index_offset(&self, index_offset: usize) -> Mesh {
        let mut new_indices = Vec::from_iter(self.indices.clone());
        new_indices.iter_mut().for_each(|x| *x += index_offset);
        Mesh::new(self.coordinates.clone(), new_indices)
    }

    /// Creates a new [Mesh] which is a result of joining it with another one.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let a = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    ///
    /// let b = Mesh::new(vec![20.0, 20.0, 20.0,
    ///                        30.0, 20.0, 20.0,
    ///                        30.0, 5.0, 20.0],
    /// vec![0, 1, 2]);
    ///
    /// let actual = a.get_by_joining_with(&b);
    /// let expected = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                               10.0, 0.0, 0.0,
    ///                               10.0, -15.0, 0.0,
    ///                               20.0, 20.0, 20.0,
    ///                               30.0, 20.0, 20.0,
    ///                               30.0, 5.0, 20.0],
    /// vec![0, 1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_by_joining_with(&self, another_mesh: &Mesh) -> Mesh {
        let max_index = self.indices.iter().max_by(|a, b| a.cmp(b)).unwrap();
        let offset = max_index + 1;
        let another_mesh_with_indices_offset = another_mesh.get_with_index_offset(offset);
        let mut coordinates: Vec<f64> = self.coordinates.clone();
        coordinates.extend(another_mesh_with_indices_offset.coordinates.clone());
        let mut indices: Vec<usize> = self.indices.clone();
        indices.extend(another_mesh_with_indices_offset.indices.clone());

        Mesh::new(coordinates, indices)
    }

    /// Splits given disconnected [Mesh] into separate connected parts.
    ///
    /// Disconnected parts here means their faces are separated.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Mesh::new(
    /// vec![0.0, 0.0, 0.0, // 0
    ///      2.5, 5.0, 0.0, // 1
    ///      5.0, 0.0, 0.0, // 2
    ///      7.5, 5.0, 0.0, // 3
    ///      10.0, 0.0, 0.0, // 4
    ///      5.0, 10.0, 0.0, // 5
    ///      5.0, 5.0, 3.0, // 6
    ///      2.5, 5.0, 3.0, // 7
    ///      0.0, 0.0, 3.0, // 8
    ///      10.0, 0.0, 3.0, // 9
    ///      5.0, 5.0, 5.0, // 10
    ///      2.5, 5.0, 5.0, // 11
    ///      0.0, 0.0, 5.0, // 12
    /// ],
    /// vec![0, 2, 1, // big_group
    ///      10, 11, 12, // isolated_triangle
    ///      1, 2, 3, // big_group
    ///      2, 4, 3, // big_group
    ///      1, 3, 5, // big_group
    ///      7, 8, 6, // small_group
    ///      7, 8, 9, // small_group
    ///      6, 7, 12, // small_group
    /// ]
    /// );
    ///
    /// let big_group = Mesh::new(
    /// vec![0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 2.5, 5.0, 0.0, 2.5, 5.0, 0.0, 5.0, 0.0, 0.0, 7.5, 5.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 7.5, 5.0, 0.0, 2.5, 5.0, 0.0, 7.5, 5.0, 0.0, 5.0, 10.0, 0.0],
    /// vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    /// );
    ///
    /// let isolated_triangle = Mesh::new(
    /// vec![5.0, 5.0, 5.0, 2.5, 5.0, 5.0, 0.0, 0.0, 5.0],
    /// vec![0, 1, 2]
    /// );
    ///
    /// let small_group = Mesh::new(
    /// vec![2.5, 5.0, 3.0, 0.0, 0.0, 3.0, 5.0, 5.0, 3.0, 2.5, 5.0, 3.0, 0.0, 0.0, 3.0, 10.0, 0.0, 3.0, 5.0, 5.0, 3.0, 2.5, 5.0, 3.0, 0.0, 0.0, 5.0],
    /// vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
    /// );
    ///
    /// let expected = vec![small_group, big_group, isolated_triangle];
    ///
    /// let actual = input.split_by_face_disconnected();
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert!(expected[i].eq(&actual[i]));
    /// }
    ///
    /// ```
    pub fn split_by_face_disconnected(&self) -> Vec<Mesh> {

        let face_neighbours = FaceNeighbours::from_mesh(self);
        let graph = Graph::from_face_neighbours(&face_neighbours);

        let isolated_groups = graph.split_disconnected_vertices();
        let mut isolated_meshes:Vec<Mesh> = Vec::new();
        for isolated_group in isolated_groups {
            isolated_meshes.push(self.get_part_by_face_ids(&isolated_group))
        }

        isolated_meshes
    }

    /// Gets only specific part of the [Mesh] using specified face ids.
    ///
    /// The result Mesh is unwelded.
    ///
    /// # Example
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
    ///     0,2,3, // Specified (1)
    ///
    ///     // Side faces
    ///     0,1,4,
    ///     1,2,4, // Specified (3)
    ///     2,3,4, // Specified (4)
    ///     3,0,4
    /// ]);
    ///
    /// let face_ids_specified = vec![1, 3, 4];
    ///
    /// let actual = input.get_part_by_face_ids(&face_ids_specified);
    ///
    /// let expected = Mesh::new(
    /// vec![
    ///     -2.0,1.0,0.0, // 0
    ///     8.0,11.0,0.0, // 2
    ///     -2.0,11.0,0.0, // 3
    ///
    ///     8.0,1.0,0.0, // 1
    ///     8.0,11.0,0.0, // 2
    ///     3.0,6.0,4.0, //4
    ///
    ///     8.0,11.0,0.0, // 2
    ///     -2.0,11.0,0.0, // 3
    ///     3.0,6.0,4.0, // 4
    /// ],
    /// vec![
    ///     0,1,2, // Specified (1)
    ///     3,4,5, // Specified (3)
    ///     6,7,8, // Specified (4)
    /// ]);
    ///
    /// assert!(expected.eq(&actual));
    ///
    /// ```
    pub fn get_part_by_face_ids(&self, face_ids: &Vec<usize>) -> Mesh {
        let mut coordinates: Vec<f64> = Vec::new();
        let mut indices: Vec<usize> = Vec::new();

        let mut current_max: usize = 0;
        for face_id in face_ids {
            for i in 0..3 {
                let coordinate_id = self.indices[face_id * 3 + i];
                for j in 0..3 {
                    coordinates.push(self.coordinates[coordinate_id * 3 + j]);
                }
                indices.push(current_max + i);
            }
            current_max += 3;
        }

        Mesh::new(coordinates, indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_with_faces_flipped() {
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
            0,1,2, // to be flipped (0)
            0,2,3,
        
            // Side faces
            0,1,4,
            1,2,4, // to be flipped (3)
            2,3,4,
            3,0,4 // to be flipped (5)
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
            2,1,0, // flipped (0)
            0,2,3,
        
            // Side faces flipped
            0,1,4,
            4,2,1, // flipped (3)
            2,3,4,
            4,0,3 // flipped (5)
        ]);
        let actual = input.get_with_faces_flipped(HashSet::from([0, 3, 5]));
        
        assert_eq!(expected.eq(&actual), true);
    }
    
    #[test]
    fn test_get_with_all_faces_flipped() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);
        let expected = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces flipped
                2, 1, 0,
                3, 2, 0,

                // Side faces flipped
                4, 1, 0,
                4, 2, 1,
                4, 3, 2,
                4, 0, 3
            ]);
        let actual = input.get_with_all_faces_flipped();

        assert_eq!(expected.eq(&actual), true);
    }
    
    #[test]
    fn test_get_with_faces_flipped_outside_using_offset() {
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
            // Base faces flipped
            0,1,2, // flipped
            3,2,0,
        
            // Side faces flipped
            0,1,4,
            4,2,1, // flipped
            2,3,4,
            4,0,3 // flipped
        ]);
        let actual = input.get_with_faces_flipped_outside_using_offset(0.001);
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
            2,1,0, // fixed
            3,2,0,
        
            // Side faces flipped
            0,1,4,
            1,2,4, // fixed
            2,3,4,
            3,0,4 // fixed
        ]);
        
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_removed_vertices_without_indices_update_first_removed() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        let replacement_instructions = HashSet::from([0]);

        let expected = Mesh::new(
            vec![
                // Base
                //-2.0,1.0,0.0, <- removed
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);
        let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_removed_vertices_without_indices_update_last_removed() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        let replacement_instructions = HashSet::from([4]);

        let expected = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                //3.0,6.0,4.0 <- removed
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);
        let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_removed_vertices_without_indices_update_last_two_in_the_middle_removed() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        let replacement_instructions = HashSet::from([1, 3]);

        let expected = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                //8.0,1.0,0.0, <- removed
                8.0, 11.0, 0.0,
                //-2.0,11.0,0.0, <- removed

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);
        let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_removed_vertices_without_indices_update_all_removed() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        let replacement_instructions = HashSet::from([2, 3, 1, 0, 4]);

        let expected = Mesh::new(
            vec![
                // All removed
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);
        let actual = input.get_with_removed_vertices_without_indices_update(replacement_instructions);
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_replaced_indices() {
        let input = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        let mut replacement_instructions = HashMap::new();
        replacement_instructions.insert(0, 3);
        replacement_instructions.insert(4, 1);

        let expected = Mesh::new(
            vec![
                // Base
                -2.0, 1.0, 0.0,
                8.0, 1.0, 0.0,
                8.0, 11.0, 0.0,
                -2.0, 11.0, 0.0,

                // Top
                3.0, 6.0, 4.0
            ],
            vec![
                // Base faces
                3, 1, 2, // 0 -> 3
                3, 2, 3, // 0 -> 3

                // Side faces
                3, 1, 1, // 0 -> 3 & 4 -> 1
                1, 2, 1, // 4 -> 1
                2, 3, 1, // 4 -> 1
                3, 3, 1, // 0 -> 3 & 4 -> 1
            ]);
        let actual = input.get_with_replaced_indices(replacement_instructions);
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_welded_vertices_correct_pyramid() {
        let input = Mesh::new(
            vec![
                0.0, 0.0, 0.0, // 0
                10.0, 0.0, 0.0, // 1
                10.0, 10.0, 0.0, // 2
                0.0, 10.0, 0.0, // 3

                5.0, 5.0, 4.0, // 4
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        let actual = input.get_with_welded_vertices(0.001);
        let expected = Mesh::new(
            vec![
                0.0, 0.0, 0.0, // 0
                10.0, 0.0, 0.0, // 1
                10.0, 10.0, 0.0, // 2
                0.0, 10.0, 0.0, // new 3

                5.0, 5.0, 4.0, // new 4
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_welded_vertices_pyramid() {
        let input = Mesh::new(
            vec![
                0.0, 0.0, 0.0, // 0
                10.0, 0.0, 0.0, // 1
                10.0, 10.0, 0.0, // 2

                0.0, 0.0, 0.0, // duplicate of 0 -> should be removed
                10.0, 10.0, 0.0, // duplicate of 2 -> should be removed
                0.0, 10.0, 0.0, // new 3

                0.0, 0.0, 0.0, // duplicate of 0 -> should be removed
                10.0, 0.0, 0.0, // duplicate of 1 -> should be removed
                5.0, 5.0, 4.0, // new 4

                10.0, 0.0, 0.0, // duplicate of 1 -> should be removed
                10.0, 10.0, 0.0, // duplicate of 2 -> should be removed
                5.0, 5.0, 4.0, // duplicate of new 4 -> should be removed

                10.0, 10.0, 0.0, // duplicate of 2 -> should be removed
                0.0, 10.0, 0.0, // duplicate of new 3 -> should be removed
                5.0, 5.0, 4.0, // duplicate of new 4 -> should be removed

                0.0, 10.0, 0.0, // duplicate of new 3 -> should be removed
                0.0, 0.0, 0.0, // duplicate of 0 -> should be removed
                5.0, 5.0, 4.0, // duplicate of new 4 -> should be removed
            ],
            vec![
                // Base faces
                0, 1, 2,
                3, 4, 5,

                // Side faces
                6, 7, 8,
                9, 10, 11,
                12, 13, 14,
                15, 16, 17
            ]);

        let actual = input.get_with_welded_vertices(0.001);
        let expected = Mesh::new(
            vec![
                0.0, 0.0, 0.0, // 0
                10.0, 0.0, 0.0, // 1
                10.0, 10.0, 0.0, // 2
                0.0, 10.0, 0.0, // new 3

                5.0, 5.0, 4.0, // new 4
            ],
            vec![
                // Base faces
                0, 1, 2,
                0, 2, 3,

                // Side faces
                0, 1, 4,
                1, 2, 4,
                2, 3, 4,
                3, 0, 4
            ]);

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_with_welded_vertices_pyramid_different_order() {
        let input = Mesh::new(
            vec![
                10.0, 10.0, 0.0, // 0
                10.0, 0.0, 0.0, // 1
                0.0, 0.0, 0.0, // 2

                0.0, 10.0, 0.0, // 3
                10.0, 10.0, 0.0, // duplicate of 0 -> should be removed
                0.0, 0.0, 0.0 - 0.00099, // duplicate of 2 -> should be removed

                0.0, 0.0, 0.0, // duplicate of 2 -> should be removed
                0.0, 10.0, 0.0, // duplicate of 3 -> should be removed
                5.0, 5.0, 4.0, // new 4

                0.0, 10.0, 0.0, // duplicate of 3 -> should be removed
                10.0, 10.0, 0.0, // duplicate of 0 -> should be removed
                5.0, 5.0, 4.0, // duplicate of new 4 -> should be removed

                10.0, 10.0, 0.0, // duplicate of 0 -> should be removed
                10.0, 0.0, 0.0, // duplicate of 1 -> should be removed
                5.0, 5.0, 4.0, // duplicate of new 4 -> should be removed

                10.0, 0.0, 0.0, // duplicate of 1 -> should be removed
                0.0, 0.0, 0.0, // duplicate of 2 -> should be removed
                5.0, 5.0, 4.0, // duplicate of new 4 -> should be removed
            ],
            vec![
                15, 16, 17,
                12, 13, 14,
                0, 1, 2,
                3, 4, 5,
                9, 10, 11,
                6, 7, 8,
            ]);

        let actual = input.get_with_welded_vertices(0.001);
        let expected = Mesh::new(
            vec![
                10.0, 10.0, 0.0, // 0
                10.0, 0.0, 0.0, // 1
                0.0, 0.0, 0.0, // 2
                0.0,10.0,0.0, // 3
                5.0,5.0,4.0, // 4
            ],
            vec![
                1,2,4,
                0,1,4,
                0,1,2,
                3,0,2,
                3,0,4,
                2,3,4]
        );

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    pub fn test_get_with_unwelded_vertices() {
        let input = Mesh::new(vec![
            0.0, 0.0, 0.0,
            10.0, 0.0, 0.0,
            10.0,10.0,0.0,
            0.0,10.0,0.0,
            5.0,5.0,4.0,
        ], vec![
            // Base faces
            0,1,2,
            0,2,3,
            // Side faces
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]);

        let actual = input.get_with_unwelded_vertices();
        let expected = Mesh::new(vec![
            0.0, 0.0, 0.0, // first face
            10.0, 0.0, 0.0,
            10.0,10.0,0.0,
            0.0, 0.0, 0.0, // second face
            10.0,10.0,0.0,
            0.0,10.0,0.0,
            0.0, 0.0, 0.0, // third face
            10.0, 0.0, 0.0,
            5.0,5.0,4.0,
            10.0, 0.0, 0.0, // fourth face
            10.0,10.0,0.0,
            5.0,5.0,4.0,
            10.0,10.0,0.0, // fifth face
            0.0,10.0,0.0,
            5.0,5.0,4.0,
            0.0,10.0,0.0, // sixth face
            0.0,0.0,0.0,
            5.0,5.0,4.0,
        ], vec![
            // Base faces
            0,1,2,
            3,4,5,
            // Side faces
            6,7,8,
            9,10,11,
            12,13,14,
            15,16,17
        ]);

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    pub fn test_get_with_index_offset() {
        let input = Mesh::new(vec![
            0.0, 0.0, 0.0,
            10.0, 0.0, 0.0,
            10.0,10.0,0.0,
            0.0,10.0,0.0,
            5.0,5.0,4.0,
        ], vec![
            // Base faces
            0,1,2,
            0,2,3,
            // Side faces
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]);
        let offset = 10;
        let actual = input.get_with_index_offset(offset);
        let expected = Mesh::new(vec![
            0.0, 0.0, 0.0,
            10.0, 0.0, 0.0,
            10.0,10.0,0.0,
            0.0,10.0,0.0,
            5.0,5.0,4.0,
        ], vec![
            // Base faces
            10,11,12,
            10,12,13,
            // Side faces
            10,11,14,
            11,12,14,
            12,13,14,
            13,10,14
        ]);
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    pub fn test_get_by_joining_with_another(){
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
        vec![0, 1, 2]);

        let b = Mesh::new(vec![20.0, 20.0, 20.0,
                               30.0, 20.0, 20.0,
                               30.0, 5.0, 20.0],
        vec![0, 1, 2]);

        let actual = a.get_by_joining_with(&b);
        let expected = Mesh::new(vec![0.0, 0.0, 0.0,
                                      10.0, 0.0, 0.0,
                                      10.0, -15.0, 0.0,
                                      20.0, 20.0, 20.0,
                                      30.0, 20.0, 20.0,
                                      30.0, 5.0, 20.0],
        vec![0, 1, 2, 3, 4, 5]);

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    pub fn test_get_by_joining_with_another_with_welded(){
        let a = Mesh::new(vec![
            0.0, 0.0, 0.0,
            10.0, 0.0, 0.0,
            10.0,10.0,0.0,
            0.0,10.0,0.0,
            5.0,5.0,4.0,
        ], vec![
            // Base faces
            0,1,2,
            0,2,3,
            // Side faces
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]);

        let b = Mesh::new(vec![20.0, 20.0, 20.0,
                               30.0, 20.0, 20.0,
                               30.0, 5.0, 20.0],
                          vec![0, 1, 2]);

        let actual = a.get_by_joining_with(&b);
        let expected = Mesh::new(vec![0.0, 0.0, 0.0,
                                                       10.0, 0.0, 0.0,
                                                       10.0,10.0,0.0,
                                                       0.0,10.0,0.0,
                                                       5.0,5.0,4.0,
                                                       20.0, 20.0, 20.0,
                                                       30.0, 20.0, 20.0,
                                                       30.0, 5.0, 20.0],
                                           vec![
                                                        // Base faces
                                                        0,1,2,
                                                        0,2,3,
                                                        // Side faces
                                                        0,1,4,
                                                        1,2,4,
                                                        2,3,4,
                                                        3,0,4,
                                                        // Joined
                                                        5,6,7]);

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    pub fn test_get_part_by_face_ids() {
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
            0,2,3, // Specified (1)

            // Side faces
            0,1,4,
            1,2,4, // Specified (3)
            2,3,4, // Specified (4)
            3,0,4
        ]);

        let face_ids_specified = vec![1, 3, 4];

        let actual = input.get_part_by_face_ids(&face_ids_specified);

        let expected = Mesh::new(
        vec![
            -2.0,1.0,0.0, // 0
            8.0,11.0,0.0, // 2
            -2.0,11.0,0.0, // 3

            8.0,1.0,0.0, // 1
            8.0,11.0,0.0, // 2
            3.0,6.0,4.0, // 4

            8.0,11.0,0.0, // 2
            -2.0,11.0,0.0, // 3
            3.0,6.0,4.0, // 4
        ],
        vec![
            0,1,2, // Specified (1)
            3,4,5, // Specified (3)
            6,7,8, // Specified (4)
        ]);

        println!("Actual: {:?}", actual);

        assert!(expected.eq(&actual));
    }
    
    #[test]
    pub fn test_split_face_disconnected() {
        let input = Mesh::new(
            vec![0.0, 0.0, 0.0, // 0
                 2.5, 5.0, 0.0, // 1
                 5.0, 0.0, 0.0, // 2
                 7.5, 5.0, 0.0, // 3
                 10.0, 0.0, 0.0, // 4
                 5.0, 10.0, 0.0, // 5
                 5.0, 5.0, 3.0, // 6
                 2.5, 5.0, 3.0, // 7
                 0.0, 0.0, 3.0, // 8
                 10.0, 0.0, 3.0, // 9
                 5.0, 5.0, 5.0, // 10
                 2.5, 5.0, 5.0, // 11
                 0.0, 0.0, 5.0, // 12
                 ],
            vec![0, 2, 1, // big_group
                 10, 11, 12, // isolated_triangle
                 1, 2, 3, // big_group
                 2, 4, 3, // big_group
                 1, 3, 5, // big_group
                 7, 8, 6, // small_group
                 7, 8, 9, // small_group
                 6, 7, 12, // small_group
                 ]
        );
        
        let big_group = Mesh::new(
            vec![0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 2.5, 5.0, 0.0, 2.5, 5.0, 0.0, 5.0, 0.0, 0.0, 7.5, 5.0, 0.0, 5.0, 0.0, 0.0, 10.0, 0.0, 0.0, 7.5, 5.0, 0.0, 2.5, 5.0, 0.0, 7.5, 5.0, 0.0, 5.0, 10.0, 0.0],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        );
        
        let isolated_triangle = Mesh::new(
            vec![5.0, 5.0, 5.0, 2.5, 5.0, 5.0, 0.0, 0.0, 5.0],
            vec![0, 1, 2]
        );
        
        let small_group = Mesh::new(
            vec![2.5, 5.0, 3.0, 0.0, 0.0, 3.0, 5.0, 5.0, 3.0, 2.5, 5.0, 3.0, 0.0, 0.0, 3.0, 10.0, 0.0, 3.0, 5.0, 5.0, 3.0, 2.5, 5.0, 3.0, 0.0, 0.0, 5.0],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
        
        let expected = vec![small_group, big_group, isolated_triangle];
        
        let actual = input.split_by_face_disconnected();

        for act in &actual {
            println!("{:?}", act);
        }
        
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].eq(&actual[i]));
        }
    }

    #[test]
    pub fn test_split_face_disconnected_whole_connected() {
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

        let expected_group = Mesh::new(vec![-2.0, 1.0, 0.0, 8.0, 1.0, 0.0, 8.0, 11.0, 0.0, -2.0, 1.0, 0.0, 8.0, 11.0, 0.0, -2.0, 11.0, 0.0, -2.0, 1.0, 0.0, 8.0, 1.0, 0.0, 3.0, 6.0, 4.0, 8.0, 1.0, 0.0, 8.0, 11.0, 0.0, 3.0, 6.0, 4.0, 8.0, 11.0, 0.0, -2.0, 11.0, 0.0, 3.0, 6.0, 4.0, -2.0, 11.0, 0.0, -2.0, 1.0, 0.0, 3.0, 6.0, 4.0], vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17
        ]);

        let expected = vec![expected_group];

        let actual = input.split_by_face_disconnected();

        for act in &actual {
            println!("{:?}", act);
        }

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].eq(&actual[i]));
        }
    }
}