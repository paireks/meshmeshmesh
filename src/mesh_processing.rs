use crate::mesh::Mesh;
use std::collections::{HashMap, HashSet};
use crate::face_neighbours::FaceNeighbours;
use crate::face_neighbours_angle::FaceNeighboursAngle;
use crate::graph::Graph;
use crate::point::Point;
use crate::polygon2d::Polygon2D;
use crate::polygon::Polygon;

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
    /// let c = Mesh::new(vec![0.0, 0.0, 5.0,
    ///                        10.0, 0.0, 5.0,
    ///                        10.0, 5.0, 5.0,
    ///                        10.0, -15.0, 5.0],
    /// vec![0, 1, 2, 1, 2, 3]);
    ///
    /// let actual = Mesh::join(&vec![a, b, c]);
    /// let expected = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                               10.0, 0.0, 0.0,
    ///                               10.0, -15.0, 0.0,
    ///                               20.0, 20.0, 20.0,
    ///                               30.0, 20.0, 20.0,
    ///                               30.0, 5.0, 20.0,
    ///                               0.0, 0.0, 5.0,
    ///                               10.0, 0.0, 5.0,
    ///                               10.0, 5.0, 5.0,
    ///                               10.0, -15.0, 5.0],
    /// vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 7, 8, 9]);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn join(meshes: &Vec<Mesh>) -> Mesh {
        let mut joined = Mesh::new(meshes[0].coordinates.clone(), meshes[0].indices.clone());
        let number_of_meshes = meshes.len();
        for i in 1..number_of_meshes {
            joined = joined.get_by_joining_with(&meshes[i]);
        }

        joined
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

    /// Tries to remesh the planar parts of the [Mesh].
    ///
    /// It tries to detect and group planar parts of given [Mesh] and remesh it from scratch.
    /// 
    /// It doesn't make sense to use this method for Meshes which are already optimized, or
    /// for Meshes which don't have clear planar polygons, which could be remeshed.
    ///
    /// Returns ([`Ok`]) or failure ([`Err`]) depending on the remesh result, if it succeeds or not.
    ///
    /// Even if it succeeds - it's good to additionally check if it looks correct.
    pub fn get_planar_remesh(&self, max_angle: f64, tolerance: f64) -> Result<Mesh, String> {
        let original_aabb = self.get_bounding_box();
        let original_area = self.get_area();

        let welded_original = self.get_with_welded_vertices(tolerance);

        let planar_meshes: Vec<Mesh> = welded_original.split_by_face_angle(max_angle, Some(tolerance));
        let mut planar_meshes_remeshed: Vec<Mesh> = Vec::with_capacity(planar_meshes.len());

        for planar_mesh in planar_meshes {
            let remeshed_result = planar_mesh.get_remesh_for_planar_mesh(tolerance);
            if remeshed_result.is_ok() {
                planar_meshes_remeshed.push(remeshed_result?)
            }
            else {
                return Err(remeshed_result.err().unwrap());
            }
        }

        let remeshed = Mesh::join(&planar_meshes_remeshed);
        let remeshed_welded = remeshed.get_with_welded_vertices(tolerance);

        let remeshed_aabb = remeshed_welded.get_bounding_box();
        if !remeshed_aabb.eq_with_tolerance(&original_aabb, tolerance) {
            return Err("The bounding box of the remeshed Mesh seems to be different from the original one".to_string())
        }

        let remeshed_area = remeshed_welded.get_area();
        if (remeshed_area - original_area).abs() > tolerance {
            return Err("The area of the remeshed Mesh seems to be different from the original one".to_string())
        }

        Ok(remeshed_welded)
    }

    /// Splits given disconnected [Mesh] into separate connected parts.
    ///
    /// Disconnected parts here means their faces are separated.
    ///
    /// # Example
    ///
    /// Below there is the example of splitting 1 [Mesh] that has 3 isolated parts, so in result
    /// of splitting it returns 3 separate Meshes. Because `weld_vertices_tolerance` is set to `None`
    /// then there is no welding used for output. If you'd like to weld resulting Meshes, just
    /// set this parameter.
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
    /// let actual = input.split_by_face_disconnected(None);
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert!(expected[i].eq(&actual[i]));
    /// }
    ///
    /// ```
    pub fn split_by_face_disconnected(&self, weld_vertices_tolerance: Option<f64>) -> Vec<Mesh> {

        let face_neighbours = FaceNeighbours::from_mesh(self);
        let graph = Graph::from_face_neighbours(&face_neighbours);

        let isolated_groups = graph.split_disconnected_vertices();
        let mut isolated_meshes:Vec<Mesh> = Vec::new();
        for isolated_group in isolated_groups {
            isolated_meshes.push(self.get_part_by_face_ids(&isolated_group))
        }

        if weld_vertices_tolerance.is_some() {
            let tolerance = weld_vertices_tolerance.unwrap();
            for i in 0..isolated_meshes.len() {
                isolated_meshes[i] = isolated_meshes[i].get_with_welded_vertices(tolerance);
            }
        }

        isolated_meshes
    }

    /// Splits given [Mesh] where the value of angle between faces' normals is higher than given one
    /// in the `max_angle` parameter. This angle should be given in radians.
    ///
    /// Optionally you can use `weld_vertices_tolerance` to weld resulting [Mesh]es.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let mesh = Mesh::new(
    ///     vec![0.0, 0.0, -0.5,
    ///          2.5, 5.0, 0.5,
    ///          5.0, 0.0, 0.3,
    ///          7.5, 5.0, -0.4,
    ///          10.0, 0.0, 0.1,
    ///          5.0, 10.0, 0.9,
    ///          ],
    ///     vec![0, 2, 1, // first face
    ///          1, 2, 3, // second face
    ///          2, 4, 3, // third face
    ///          1, 3, 5, // fourth face
    ///          ]
    /// );
    ///
    /// let actual = mesh.split_by_face_angle(0.3, Some(0.001));
    ///
    /// let big_group = Mesh::new(
    ///     vec![2.5, 5.0, 0.5, 5.0, 0.0, 0.3, 7.5, 5.0, -0.4, 10.0, 0.0, 0.1, 5.0, 10.0, 0.9],
    ///     vec![0, 1, 2, 1, 3, 2, 0, 2, 4]
    /// );
    ///
    /// let small_group = Mesh::new(
    ///     vec![0.0, 0.0, -0.5, 5.0, 0.0, 0.3, 2.5, 5.0, 0.5],
    ///     vec![0, 1, 2]
    /// );
    ///
    /// let expected = vec![big_group, small_group];
    ///
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn split_by_face_angle(&self, max_angle: f64, weld_vertices_tolerance: Option<f64>) -> Vec<Mesh> {

        let face_neighbours = FaceNeighbours::from_mesh(self);
        let triangles = self.to_triangles();
        let face_neighbours_angles = FaceNeighboursAngle::from_face_neighbours_and_triangles(&face_neighbours, &triangles);

        let graph = Graph::from_face_neighbours_with_max_angle(&face_neighbours, &face_neighbours_angles, max_angle);

        let isolated_groups = graph.split_disconnected_vertices();
        let mut isolated_meshes:Vec<Mesh> = Vec::new();
        for isolated_group in isolated_groups {
            isolated_meshes.push(self.get_part_by_face_ids(&isolated_group))
        }

        if weld_vertices_tolerance.is_some() {
            let tolerance = weld_vertices_tolerance.unwrap();
            for i in 0..isolated_meshes.len() {
                isolated_meshes[i] = isolated_meshes[i].get_with_welded_vertices(tolerance);
            }
        }

        isolated_meshes
    }

    /// Gets Polygons for planar [Mesh].
    fn get_polygons_for_planar_mesh(&self) -> Vec<Polygon> {
        let edges = self.get_edges_with_missing_neighbour(); // Getting Graph and calculating grouped edges
        let graph = Graph::from_edges_into_undirected(self.get_number_of_vertices(), &edges);
        let disconnected_parts = graph.split_disconnected_vertices();

        let mut polygons = Vec::with_capacity(disconnected_parts.len()); // Converting grouped edges to Polygons
        for disconnected_part in disconnected_parts {
            if disconnected_part.len() > 2 {
                let mut polygon_vertices = Vec::new();
                for i in disconnected_part {
                    let vertex = Point::new(self.coordinates[i*3], self.coordinates[i*3+1], self.coordinates[i*3+2]);
                    polygon_vertices.push(vertex)
                }
                polygons.push(Polygon::new(polygon_vertices))
            }
        }

        polygons
    }

    /// Remeshes the [Mesh] which is already a planar one.
    fn get_remesh_for_planar_mesh(&self, tolerance: f64) -> Result<Mesh, String> {
        let original_normal = self.get_face_normal_vectors_unitized()[0];
        let original_aabb = self.get_bounding_box();
        let original_area = self.get_area();

        let welded_original = self.get_with_welded_vertices(tolerance);

        let polygons = welded_original.get_polygons_for_planar_mesh();
        
        let local_coordinate_system_of_polygons = polygons[0].get_local_coordinate_system(); // Converting Polygons to 2D Polygons
        let mut polygon2ds = Vec::with_capacity(polygons.len());
        for polygon in polygons {
            polygon2ds.push(polygon.to_polygon2d(&local_coordinate_system_of_polygons));
        }

        let mut cleaned_polygon2ds = Vec::with_capacity(polygon2ds.len()); // Cleaning up 2D Polygons
        for polygon2d in polygon2ds {
            let polygon2d_with_removed_duplicates = polygon2d.get_with_removed_neighbour_duplicates_with_tolerance(tolerance);
            let polygon2d_with_removed_parallel = polygon2d_with_removed_duplicates.get_with_removed_neighbour_parallel_segments_with_tolerance(tolerance);
            if polygon2d_with_removed_parallel.vertices.len() > 2 {
                cleaned_polygon2ds.push(polygon2d_with_removed_parallel);
            }
        }
        
        let remeshed_xy: Mesh;
        
        if cleaned_polygon2ds.len() > 1 {
            let mut biggest_bounding_area = 0.0; // Trying to find out which Polygon2D is the main one and should be triangulated
            let mut biggest_bounding_area_polygon_id= 0;
            for i in 0..cleaned_polygon2ds.len() {
                let current_bounding_area = cleaned_polygon2ds[i].get_bounding_area().get_area();
                if current_bounding_area > biggest_bounding_area {
                    biggest_bounding_area = current_bounding_area;
                    biggest_bounding_area_polygon_id = i;
                }
            }
            
            let mut holes = Vec::with_capacity(cleaned_polygon2ds.len() - 1); // All others should be holes
            for i in 0..cleaned_polygon2ds.len() {
                if i != biggest_bounding_area_polygon_id {
                    holes.push(Polygon2D::new(cleaned_polygon2ds[i].vertices.clone()))
                }
            }
            
            remeshed_xy = cleaned_polygon2ds[biggest_bounding_area_polygon_id].triangulate_raw_with_holes(&holes);
        }
        else {
            remeshed_xy = cleaned_polygon2ds[0].triangulate_raw(); // If there is one Polygon then no need to search for holes
        }

        let mut remeshed = remeshed_xy.get_in_local_coordinate_system(&local_coordinate_system_of_polygons); // Placing back to the original location

        let remeshed_normal = remeshed.get_face_normal_vectors_unitized()[0];
        if !remeshed_normal.eq_with_tolerance(&original_normal, tolerance) {
            let reversed_remeshed_normal = remeshed_normal.get_reversed();
            if reversed_remeshed_normal.eq_with_tolerance(&original_normal, tolerance) {
                remeshed = remeshed.get_with_all_faces_flipped();
            }
            else {
                return Err("The face normal of the remeshed planar Mesh seems to be different from the original one".to_string())
            }
        }

        let remeshed_aabb = remeshed.get_bounding_box();
        if !remeshed_aabb.eq_with_tolerance(&original_aabb, tolerance) {
            return Err("The bounding box of the remeshed planar Mesh seems to be different from the original one".to_string())
        }

        let remeshed_area = remeshed.get_area();
        if (remeshed_area - original_area).abs() > tolerance {
            return Err("The area of the remeshed planar Mesh seems to be different from the original one".to_string())
        }

        Ok(remeshed)
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
    pub fn test_join() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
        vec![0, 1, 2]);

        let b = Mesh::new(vec![20.0, 20.0, 20.0,
                               30.0, 20.0, 20.0,
                               30.0, 5.0, 20.0],
        vec![0, 1, 2]);

        let c = Mesh::new(vec![0.0, 0.0, 5.0,
                               10.0, 0.0, 5.0,
                               10.0, 5.0, 5.0,
                               10.0, -15.0, 5.0],
        vec![0, 1, 2, 1, 2, 3]);

        let actual = Mesh::join(&vec![a, b, c]);
        let expected = Mesh::new(vec![0.0, 0.0, 0.0,
                                      10.0, 0.0, 0.0,
                                      10.0, -15.0, 0.0,
                                      20.0, 20.0, 20.0,
                                      30.0, 20.0, 20.0,
                                      30.0, 5.0, 20.0,
                                      0.0, 0.0, 5.0,
                                      10.0, 0.0, 5.0,
                                      10.0, 5.0, 5.0,
                                      10.0, -15.0, 5.0],
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 7, 8, 9]);

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
        
        let actual = input.split_by_face_disconnected(None);

        for act in &actual {
            println!("{:?}", act);
        }
        
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].eq(&actual[i]));
        }
    }

    #[test]
    pub fn test_split_face_disconnected_with_vertices_welding() {
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
            vec![0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 2.5, 5.0, 0.0, 7.5, 5.0, 0.0, 10.0, 0.0, 0.0, 5.0, 10.0, 0.0],
            vec![0, 1, 2, 2, 1, 3, 1, 4, 3, 2, 3, 5]
        );

        let isolated_triangle = Mesh::new(
            vec![5.0, 5.0, 5.0, 2.5, 5.0, 5.0, 0.0, 0.0, 5.0],
            vec![0, 1, 2]
        );

        let small_group = Mesh::new(
            vec![2.5, 5.0, 3.0, 0.0, 0.0, 3.0, 5.0, 5.0, 3.0, 10.0, 0.0, 3.0, 0.0, 0.0, 5.0],
            vec![0, 1, 2, 0, 1, 3, 2, 0, 4]
        );

        let expected = vec![small_group, big_group, isolated_triangle];

        let actual = input.split_by_face_disconnected(Some(0.001));

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

        let actual = input.split_by_face_disconnected(None);

        for act in &actual {
            println!("{:?}", act);
        }

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].eq(&actual[i]));
        }
    }
    
    #[test]
    pub fn test_split_by_face_angle() {
        let mesh = Mesh::new(
            vec![0.0, 0.0, -0.5,
                 2.5, 5.0, 0.5,
                 5.0, 0.0, 0.3,
                 7.5, 5.0, -0.4,
                 10.0, 0.0, 0.1,
                 5.0, 10.0, 0.9,
                 ],
            vec![0, 2, 1, // first face
                 1, 2, 3, // second face
                 2, 4, 3, // third face
                 1, 3, 5, // fourth face
                 ]
        );

        let big_group = Mesh::new(
            vec![2.5, 5.0, 0.5, 5.0, 0.0, 0.3, 7.5, 5.0, -0.4, 10.0, 0.0, 0.1, 5.0, 10.0, 0.9],
            vec![0, 1, 2, 1, 3, 2, 0, 2, 4]
        );
        
        let small_group = Mesh::new(
            vec![0.0, 0.0, -0.5, 5.0, 0.0, 0.3, 2.5, 5.0, 0.5],
            vec![0, 1, 2]
        );

        let expected = vec![big_group, small_group];
        
        let actual = mesh.split_by_face_angle(0.3, Some(0.001));

        for act in &actual {
            println!("{:?}", act);
        }

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].eq(&actual[i]));
        }
    }

    #[test]
    fn test_get_polygons_for_planar_mesh() {
        let mesh = Mesh::new(
            vec![
                53.727875, 19.499361, -0.982379,
                52.333458, 20.601471, 2.715811,
                57.607727, 21.541887, -4.680569,
                56.21331, 22.643997, -0.982379,
                54.818893, 23.746105, 2.715811,
                53.424473, 24.848215, 6.414001,
                60.093159, 24.686518, -4.680569,
                58.698742, 25.788628, -0.982379,
                57.304321, 26.890738, 2.715811,
                55.909904, 27.992847, 6.414001,
                61.184177, 28.933264, -0.982379,
                59.789757, 30.035374, 2.715811,
                69.534203, 36.01841, -5.471667,
                67.603172, 34.689728, -4.033733,
                65.672142, 33.361042, -2.5958,
                63.741116, 32.032356, -1.157868,
                61.810089, 30.703671, 0.280065,
                62.935524, 32.966969, 1.363022,
                64.060959, 35.230267, 2.44598,
                65.186394, 37.493568, 3.528937,
                66.311829, 39.756866, 4.611895,
                64.615532, 37.592625, 4.588615,
                62.919231, 35.428379, 4.565335,
                61.222935, 33.264133, 4.542056,
                59.526634, 31.09989, 4.518776,
                59.505108, 32.76738, 6.705313,
                59.483585, 34.434868, 8.89185,
                59.462059, 36.102356, 11.078387,
                59.440533, 37.769844, 13.264924,
                58.626892, 35.596706, 11.789324,
                57.813251, 33.423573, 10.313724,
                56.999607, 31.250437, 8.838123,
                56.18597, 29.077303, 7.362524,
                55.025703, 29.512056, 9.817457,
                53.86544, 29.94681, 12.272391,
                52.705173, 30.381563, 14.727325,
                51.54491, 30.816315, 17.182259,
                51.924709, 29.464354, 14.817967,
                52.304512, 28.112391, 12.453676,
                52.684311, 26.760426, 10.089385,
                53.06411, 25.408464, 7.725094,
                51.20829, 24.444422, 9.510723,
                49.352467, 23.480379, 11.296352,
                47.496643, 22.516336, 13.081983,
                45.640823, 21.552294, 14.867612,
                47.068993, 21.537907, 12.517708,
                48.497166, 21.523523, 10.167805,
                49.925335, 21.509136, 7.817901,
                51.353504, 21.494749, 5.467997,
                49.510986, 19.500145, 5.902272,
                47.668468, 17.505537, 6.336547,
                45.825951, 15.510929, 6.770823,
                43.983429, 13.516321, 7.205098,
                45.914459, 14.845006, 5.767165,
                47.845486, 16.173691, 4.329232,
                49.776516, 17.502377, 2.8913,
                51.707546, 18.831066, 1.453367,
                50.582108, 16.567766, 0.37041,
                49.456673, 14.304465, -0.712548,
                48.331238, 12.041166, -1.795505,
                47.205803, 9.777866, -2.878463,
                48.902103, 11.942111, -2.855183,
                50.5984, 14.106356, -2.831903,
                52.294697, 16.270597, -2.808623,
                53.990997, 18.434843, -2.785344,
                54.012524, 16.767357, -4.971881,
                54.03405, 15.099867, -7.158419,
                54.055576, 13.432381, -9.344954,
                54.077099, 11.764891, -11.531492,
                54.890739, 13.938025, -10.055892,
                55.704384, 16.111162, -8.580292,
                56.518021, 18.284296, -7.104691,
                57.331665, 20.457434, -5.629091,
                58.491928, 20.022678, -8.084024,
                59.652195, 19.587927, -10.538959,
                60.812458, 19.153172, -12.993895,
                61.972725, 18.71842, -15.448829,
                61.592922, 20.070381, -13.084537,
                61.21312, 21.422342, -10.720244,
                60.833324, 22.774307, -8.355954,
                60.453522, 24.126268, -5.991662,
                62.309345, 25.090311, -7.777292,
                64.165169, 26.054356, -9.562921,
                66.020988, 27.018396, -11.348551,
                67.876808, 27.982441, -13.134179,
                66.448639, 27.996826, -10.784276,
                65.02047, 28.011211, -8.434373,
                63.5923, 28.025599, -6.084468,
                62.164127, 28.039984, -3.734565,
                64.006645, 30.034592, -4.16884,
                65.849167, 32.029198, -4.603116,
                67.691681, 34.023804, -5.037391,
                59.002144, 20.439777, -8.378758,
                54.515488, 29.094957, 10.11219,
            ],
            vec![
                4,1,0,
                7,3,2,
                8,4,3,
                9,5,4,
                11,8,7,
                69,67,68,
                67,70,66,
                65,66,70,
                69,70,67,
                59,60,61,
                61,58,59,
                61,62,58,
                62,63,58,
                65,70,71,
                92,73,74,
                74,78,92,
                72,73,92,
                77,75,76,
                74,75,77,
                80,2,92,
                3,0,64,
                65,71,64,
                71,72,64,
                63,64,0,
                64,2,3,
                92,2,72,
                72,2,64,
                63,0,57,
                54,51,53,
                56,57,0,
                1,49,55,
                58,63,57,
                55,49,54,
                53,51,52,
                50,51,54,
                43,44,45,
                50,54,49,
                56,1,55,
                4,48,1,
                5,47,48,
                39,40,93,
                40,5,9,
                41,47,5,
                48,4,5,
                1,56,0,
                40,41,5,
                43,46,42,
                42,46,47,
                45,46,43,
                39,93,38,
                42,47,41,
                48,49,1,
                78,74,77,
                80,92,79,
                81,86,87,
                81,82,86,
                6,80,81,
                88,7,6,
                6,2,80,
                89,10,88,
                6,87,88,
                7,88,10,
                6,81,87,
                85,82,83,
                82,85,86,
                83,84,85,
                13,91,12,
                14,90,91,
                16,10,15,
                15,10,89,
                15,89,90,
                14,15,90,
                14,91,13,
                16,11,10,
                17,11,16,
                32,9,24,
                8,24,9,
                93,9,32,
                9,93,40,
                31,32,24,
                11,24,8,
                23,24,11,
                25,31,24,
                36,37,35,
                35,37,38,
                34,93,33,
                32,33,93,
                34,35,38,
                38,93,34,
                22,23,18,
                17,23,11,
                19,22,18,
                20,21,19,
                22,19,21,
                26,30,25,
                31,25,30,
                27,30,26,
                28,29,27,
                30,27,29,
                17,18,23,
                92,78,79,
                4,0,3,
                7,2,6,
                8,3,7,
                9,4,8,
                11,7,10,
            ]
        );

        let actual = mesh.get_polygons_for_planar_mesh();

        for act in &actual {
            println!("New Polygon");
            for vertex in &act.vertices {
                println!("{0}, {1}, {2}", vertex.x, vertex.y, vertex.z);
            }
        }

        let expected = vec![
            Polygon::new( vec![
                Point::new(69.534203, 36.01841, -5.471667),
                Point::new(67.603172, 34.689728, -4.033733),
                Point::new(65.672142, 33.361042, -2.5958),
                Point::new(63.741116, 32.032356, -1.157868),
                Point::new(61.810089, 30.703671, 0.280065),
                Point::new(62.935524, 32.966969, 1.363022),
                Point::new(64.060959, 35.230267, 2.44598),
                Point::new(65.186394, 37.493568, 3.528937),
                Point::new(66.311829, 39.756866, 4.611895),
                Point::new(64.615532, 37.592625, 4.588615),
                Point::new(62.919231, 35.428379, 4.565335),
                Point::new(61.222935, 33.264133, 4.542056),
                Point::new(59.526634, 31.09989, 4.518776),
                Point::new(59.505108, 32.76738, 6.705313),
                Point::new(59.483585, 34.434868, 8.89185),
                Point::new(59.462059, 36.102356, 11.078387),
                Point::new(59.440533, 37.769844, 13.264924),
                Point::new(58.626892, 35.596706, 11.789324),
                Point::new(57.813251, 33.423573, 10.313724),
                Point::new(56.999607, 31.250437, 8.838123),
                Point::new(56.18597, 29.077303, 7.362524),
                Point::new(55.025703, 29.512056, 9.817457),
                Point::new(53.86544, 29.94681, 12.272391),
                Point::new(52.705173, 30.381563, 14.727325),
                Point::new(51.54491, 30.816315, 17.182259),
                Point::new(51.924709, 29.464354, 14.817967),
                Point::new(52.304512, 28.112391, 12.453676),
                Point::new(52.684311, 26.760426, 10.089385),
                Point::new(53.06411, 25.408464, 7.725094),
                Point::new(51.20829, 24.444422, 9.510723),
                Point::new(49.352467, 23.480379, 11.296352),
                Point::new(47.496643, 22.516336, 13.081983),
                Point::new(45.640823, 21.552294, 14.867612),
                Point::new(47.068993, 21.537907, 12.517708),
                Point::new(48.497166, 21.523523, 10.167805),
                Point::new(49.925335, 21.509136, 7.817901),
                Point::new(51.353504, 21.494749, 5.467997),
                Point::new(49.510986, 19.500145, 5.902272),
                Point::new(47.668468, 17.505537, 6.336547),
                Point::new(45.825951, 15.510929, 6.770823),
                Point::new(43.983429, 13.516321, 7.205098),
                Point::new(45.914459, 14.845006, 5.767165),
                Point::new(47.845486, 16.173691, 4.329232),
                Point::new(49.776516, 17.502377, 2.8913),
                Point::new(51.707546, 18.831066, 1.453367),
                Point::new(50.582108, 16.567766, 0.37041),
                Point::new(49.456673, 14.304465, -0.712548),
                Point::new(48.331238, 12.041166, -1.795505),
                Point::new(47.205803, 9.777866, -2.878463),
                Point::new(48.902103, 11.942111, -2.855183),
                Point::new(50.5984, 14.106356, -2.831903),
                Point::new(52.294697, 16.270597, -2.808623),
                Point::new(53.990997, 18.434843, -2.785344),
                Point::new(54.012524, 16.767357, -4.971881),
                Point::new(54.03405, 15.099867, -7.158419),
                Point::new(54.055576, 13.432381, -9.344954),
                Point::new(54.077099, 11.764891, -11.531492),
                Point::new(54.890739, 13.938025, -10.055892),
                Point::new(55.704384, 16.111162, -8.580292),
                Point::new(56.518021, 18.284296, -7.104691),
                Point::new(57.331665, 20.457434, -5.629091),
                Point::new(58.491928, 20.022678, -8.084024),
                Point::new(59.652195, 19.587927, -10.538959),
                Point::new(60.812458, 19.153172, -12.993895),
                Point::new(61.972725, 18.71842, -15.448829),
                Point::new(61.592922, 20.070381, -13.084537),
                Point::new(61.21312, 21.422342, -10.720244),
                Point::new(60.833324, 22.774307, -8.355954),
                Point::new(60.453522, 24.126268, -5.991662),
                Point::new(62.309345, 25.090311, -7.777292),
                Point::new(64.165169, 26.054356, -9.562921),
                Point::new(66.020988, 27.018396, -11.348551),
                Point::new(67.876808, 27.982441, -13.134179),
                Point::new(66.448639, 27.996826, -10.784276),
                Point::new(65.02047, 28.011211, -8.434373),
                Point::new(63.5923, 28.025599, -6.084468),
                Point::new(62.164127, 28.039984, -3.734565),
                Point::new(64.006645, 30.034592, -4.16884),
                Point::new(65.849167, 32.029198, -4.603116),
                Point::new(67.691681, 34.023804, -5.037391),
            ]),
        ];

        assert_eq!(actual.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn test_get_remesh_for_planar_mesh_star() {
        let mesh = Mesh::new(
            vec![
                53.727875, 19.499361, -0.982379,
                52.333458, 20.601471, 2.715811,
                57.607727, 21.541887, -4.680569,
                56.21331, 22.643997, -0.982379,
                54.818893, 23.746105, 2.715811,
                53.424473, 24.848215, 6.414001,
                60.093159, 24.686518, -4.680569,
                58.698742, 25.788628, -0.982379,
                57.304321, 26.890738, 2.715811,
                55.909904, 27.992847, 6.414001,
                61.184177, 28.933264, -0.982379,
                59.789757, 30.035374, 2.715811,
                69.534203, 36.01841, -5.471667,
                67.603172, 34.689728, -4.033733,
                65.672142, 33.361042, -2.5958,
                63.741116, 32.032356, -1.157868,
                61.810089, 30.703671, 0.280065,
                62.935524, 32.966969, 1.363022,
                64.060959, 35.230267, 2.44598,
                65.186394, 37.493568, 3.528937,
                66.311829, 39.756866, 4.611895,
                64.615532, 37.592625, 4.588615,
                62.919231, 35.428379, 4.565335,
                61.222935, 33.264133, 4.542056,
                59.526634, 31.09989, 4.518776,
                59.505108, 32.76738, 6.705313,
                59.483585, 34.434868, 8.89185,
                59.462059, 36.102356, 11.078387,
                59.440533, 37.769844, 13.264924,
                58.626892, 35.596706, 11.789324,
                57.813251, 33.423573, 10.313724,
                56.999607, 31.250437, 8.838123,
                56.18597, 29.077303, 7.362524,
                55.025703, 29.512056, 9.817457,
                53.86544, 29.94681, 12.272391,
                52.705173, 30.381563, 14.727325,
                51.54491, 30.816315, 17.182259,
                51.924709, 29.464354, 14.817967,
                52.304512, 28.112391, 12.453676,
                52.684311, 26.760426, 10.089385,
                53.06411, 25.408464, 7.725094,
                51.20829, 24.444422, 9.510723,
                49.352467, 23.480379, 11.296352,
                47.496643, 22.516336, 13.081983,
                45.640823, 21.552294, 14.867612,
                47.068993, 21.537907, 12.517708,
                48.497166, 21.523523, 10.167805,
                49.925335, 21.509136, 7.817901,
                51.353504, 21.494749, 5.467997,
                49.510986, 19.500145, 5.902272,
                47.668468, 17.505537, 6.336547,
                45.825951, 15.510929, 6.770823,
                43.983429, 13.516321, 7.205098,
                45.914459, 14.845006, 5.767165,
                47.845486, 16.173691, 4.329232,
                49.776516, 17.502377, 2.8913,
                51.707546, 18.831066, 1.453367,
                50.582108, 16.567766, 0.37041,
                49.456673, 14.304465, -0.712548,
                48.331238, 12.041166, -1.795505,
                47.205803, 9.777866, -2.878463,
                48.902103, 11.942111, -2.855183,
                50.5984, 14.106356, -2.831903,
                52.294697, 16.270597, -2.808623,
                53.990997, 18.434843, -2.785344,
                54.012524, 16.767357, -4.971881,
                54.03405, 15.099867, -7.158419,
                54.055576, 13.432381, -9.344954,
                54.077099, 11.764891, -11.531492,
                54.890739, 13.938025, -10.055892,
                55.704384, 16.111162, -8.580292,
                56.518021, 18.284296, -7.104691,
                57.331665, 20.457434, -5.629091,
                58.491928, 20.022678, -8.084024,
                59.652195, 19.587927, -10.538959,
                60.812458, 19.153172, -12.993895,
                61.972725, 18.71842, -15.448829,
                61.592922, 20.070381, -13.084537,
                61.21312, 21.422342, -10.720244,
                60.833324, 22.774307, -8.355954,
                60.453522, 24.126268, -5.991662,
                62.309345, 25.090311, -7.777292,
                64.165169, 26.054356, -9.562921,
                66.020988, 27.018396, -11.348551,
                67.876808, 27.982441, -13.134179,
                66.448639, 27.996826, -10.784276,
                65.02047, 28.011211, -8.434373,
                63.5923, 28.025599, -6.084468,
                62.164127, 28.039984, -3.734565,
                64.006645, 30.034592, -4.16884,
                65.849167, 32.029198, -4.603116,
                67.691681, 34.023804, -5.037391,
                59.002144, 20.439777, -8.378758,
                54.515488, 29.094957, 10.11219,
            ],
            vec![
                4,1,0,
                7,3,2,
                8,4,3,
                9,5,4,
                11,8,7,
                69,67,68,
                67,70,66,
                65,66,70,
                69,70,67,
                59,60,61,
                61,58,59,
                61,62,58,
                62,63,58,
                65,70,71,
                92,73,74,
                74,78,92,
                72,73,92,
                77,75,76,
                74,75,77,
                80,2,92,
                3,0,64,
                65,71,64,
                71,72,64,
                63,64,0,
                64,2,3,
                92,2,72,
                72,2,64,
                63,0,57,
                54,51,53,
                56,57,0,
                1,49,55,
                58,63,57,
                55,49,54,
                53,51,52,
                50,51,54,
                43,44,45,
                50,54,49,
                56,1,55,
                4,48,1,
                5,47,48,
                39,40,93,
                40,5,9,
                41,47,5,
                48,4,5,
                1,56,0,
                40,41,5,
                43,46,42,
                42,46,47,
                45,46,43,
                39,93,38,
                42,47,41,
                48,49,1,
                78,74,77,
                80,92,79,
                81,86,87,
                81,82,86,
                6,80,81,
                88,7,6,
                6,2,80,
                89,10,88,
                6,87,88,
                7,88,10,
                6,81,87,
                85,82,83,
                82,85,86,
                83,84,85,
                13,91,12,
                14,90,91,
                16,10,15,
                15,10,89,
                15,89,90,
                14,15,90,
                14,91,13,
                16,11,10,
                17,11,16,
                32,9,24,
                8,24,9,
                93,9,32,
                9,93,40,
                31,32,24,
                11,24,8,
                23,24,11,
                25,31,24,
                36,37,35,
                35,37,38,
                34,93,33,
                32,33,93,
                34,35,38,
                38,93,34,
                22,23,18,
                17,23,11,
                19,22,18,
                20,21,19,
                22,19,21,
                26,30,25,
                31,25,30,
                27,30,26,
                28,29,27,
                30,27,29,
                17,18,23,
                92,78,79,
                4,0,3,
                7,2,6,
                8,3,7,
                9,4,8,
                11,7,10,
            ]
        );

        let actual = mesh.get_remesh_for_planar_mesh(0.001);

        let expected = Mesh::new(
            vec![
                69.53420300760052, 36.018409998055155, -5.47166701491627, 67.87681079462823, 27.982438792885212, -13.134177288049447, 66.31182394530234, 39.756870011805105, 4.6118919047238816, 61.97272083338372, 18.718423277046632, -15.448831553327818, 62.16412083025457, 28.039988884940925, -3.7345687951122906, 61.810080869644295, 30.70367741476083, 0.280060032120148, 60.45351365291641, 24.12627458169805, -5.991667101063174, 59.52662230964126, 31.099899234882436, 4.518768851629442, 57.331653792924904, 20.45744286748573, -5.629097865514115, 59.44051851756776, 37.769855438463594, 13.264915127207477, 54.07708667486, 11.764900740122526, -11.531499540796736, 56.18595339209981, 29.077316116099023, 7.3625138182377174, 53.99098159217984, 18.43485517324377, -2.7853534426919175, 53.06409034596102, 25.408479525528797, 7.725081963345637, 51.707526611826864, 18.83108133158251, 1.4533551305968997, 51.35348309249431, 21.494765537585025, 5.467984188466039, 51.544886476039856, 30.816333604432465, 17.182244590842558, 47.20578007697476, 9.777884129906358, -2.8784770361467658, 45.64079359414275, 21.55231723717868, 14.867593994720519, 43.98340091695297, 13.516343193981879, 7.205080817744908,
            ],
            vec![
                5, 4, 0, 6, 1, 4, 6, 4, 5, 7, 5, 2, 7, 6, 5, 8, 3, 6, 8, 6, 7, 9, 8, 7, 10, 8, 9, 11, 10, 9, 12, 10, 11, 13, 12, 11, 14, 12, 13, 15, 14, 13, 16, 13, 11, 17, 12, 14, 18, 15, 13, 19, 14, 15,
            ]
        );

        println!("{:?}", actual);

        assert!(actual.is_ok());
        assert!(expected.eq(&actual.unwrap()));
    }
}