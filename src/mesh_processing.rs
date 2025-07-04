use crate::mesh::Mesh;
use std::collections::{HashMap, HashSet};
use crate::bounding_box::BoundingBox;
use crate::face_neighbours::FaceNeighbours;
use crate::face_neighbours_angle::FaceNeighboursAngle;
use crate::graph::Graph;
use crate::local_coordinate_system::LocalCoordinateSystem;
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

        Mesh::new_with_id(self.id, self.coordinates.clone(), new_indices)
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

        Mesh::new_with_id(self.id, self.coordinates.clone(), new_indices)
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

        Mesh::new_with_id(self.id, new_coordinates, self.indices.clone())
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

        Mesh::new_with_id(self.id, self.coordinates.clone(), new_indices)
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
            Mesh::new_with_id(self.id, self.coordinates.clone(), self.indices.clone())
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
        Mesh::new_with_id(self.id, self.coordinates.clone(), new_indices)
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
    /// Joined [Mesh] has an `id`: `None`.
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
    /// The new result Mesh has no `id` (`None`).
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

    /// Tries to simplify the planar parts of the [Mesh].
    ///
    /// It tries to detect and group planar parts of given [Mesh] and remesh it from scratch.
    /// 
    /// It doesn't make sense to use this method for Meshes which are already optimized, or
    /// for Meshes which don't have clear planar polygons, which could be remeshed.
    ///
    /// Returns ([`Ok`]) or failure ([`Err`]) depending on the remesh result, if it succeeds or not.
    ///
    /// Even if it succeeds - you should additionally check manually if it looks correct.
    ///
    /// # Example
    ///
    /// Here is an example of complex 3D Mesh which in reality represents simple pyramid.
    /// Once it's simplified you can see how simple the result Mesh is.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input = Mesh::new(
    /// vec![
    ///     99.862946, 31.553921, 0.0,
    ///     56.983097, 52.169056, 17.497387,
    ///     52.536674, 45.829185, 34.994774,
    ///     48.090248, 39.489315, 52.492165,
    ///     37.766384, 65.646561, 17.497387,
    ///     33.319962, 59.30669, 34.994774,
    ///     28.873537, 52.96682, 52.492165,
    ///     24.427114, 46.626945, 69.989548,
    ///     18.549671, 79.124062, 17.497387,
    ///     14.103248, 72.784195, 34.994774,
    ///     9.656824, 66.444321, 52.492165,
    ///     5.210402, 60.10445, 69.989548,
    ///     0.763979, 53.76458, 87.486938,
    ///     -8.128867, 41.084839, 122.481712,
    ///     -0.667042, 92.60157, 17.497387,
    ///     -5.113465, 86.261696, 34.994774,
    ///     -9.559888, 79.921822, 52.492165,
    ///     -14.006311, 73.581955, 69.989548,
    ///     -19.883755, 106.079071, 17.497387,
    ///     -24.330177, 99.739197, 34.994774,
    ///     -28.7766, 93.39933, 52.492165,
    ///     -53.870758, 139.373947, 0.0,
    ///     85.887154, 41.355743, 0.0,
    ///     71.911362, 51.157562, 0.0,
    ///     57.93557, 60.959381, 0.0,
    ///     43.959782, 70.7612, 0.0,
    ///     29.98399, 80.563026, 0.0,
    ///     16.008198, 90.364845, 0.0,
    ///     2.032407, 100.166664, 0.0,
    ///     -11.943384, 109.968483, 0.0,
    ///     -25.919174, 119.770309, 0.0,
    ///     -39.894966, 129.572128, 0.0,
    ///     -49.29657, 129.545044, 12.248172,
    ///     -44.722378, 119.716125, 24.496344,
    ///     -40.14819, 109.887215, 36.744514,
    ///     -35.574001, 100.058304, 48.992687,
    ///     -30.999813, 90.229393, 61.240856,
    ///     -26.425623, 80.400482, 73.489029,
    ///     -21.851435, 70.571571, 85.737198,
    ///     -17.277245, 60.742661, 97.985374,
    ///     -12.703056, 50.91375, 110.233543,
    ///     2.670314, 40.131744, 110.233543,
    ///     13.469495, 39.178654, 97.985374,
    ///     24.268677, 38.225563, 85.737198,
    ///     35.067856, 37.272472, 73.489029,
    ///     45.867039, 36.319378, 61.240856,
    ///     56.666218, 35.366287, 48.992687,
    ///     67.465401, 34.413197, 36.744514,
    ///     78.26458, 33.460106, 24.496344,
    ///     89.063759, 32.507011, 12.248172,
    ///     -53.870758, 139.373947, 0.0,
    ///     -50.284065, 91.931343, 17.497387,
    ///     -42.570362, 91.250565, 34.994774,
    ///     -34.856663, 90.569786, 52.492165,
    ///     -52.347569, 68.55043, 17.497387,
    ///     -44.633869, 67.869652, 34.994774,
    ///     -36.920166, 67.188866, 52.492165,
    ///     -29.206467, 66.508087, 69.989548,
    ///     -54.411072, 45.169518, 17.497387,
    ///     -46.697372, 44.488735, 34.994774,
    ///     -38.983669, 43.807957, 52.492165,
    ///     -31.26997, 43.127178, 69.989548,
    ///     -23.556269, 42.446396, 87.486938,
    ///     -56.474575, 21.788603, 17.497387,
    ///     -48.760876, 21.107822, 34.994774,
    ///     -41.047173, 20.427044, 52.492165,
    ///     -33.333473, 19.746264, 69.989548,
    ///     -58.538078, -1.59231, 17.497387,
    ///     -50.824379, -2.27309, 34.994774,
    ///     -43.11068, -2.953869, 52.492165,
    ///     -70.378792, -47.673355, 0.0,
    ///     -8.128867, 41.084839, 122.481712,
    ///     -12.703056, 50.91375, 110.233543,
    ///     -17.277245, 60.742661, 97.985374,
    ///     -21.851435, 70.571571, 85.737198,
    ///     -26.425623, 80.400482, 73.489029,
    ///     -30.999813, 90.229393, 61.240856,
    ///     -35.574001, 100.058304, 48.992687,
    ///     -40.14819, 109.887215, 36.744514,
    ///     -44.722378, 119.716125, 24.496344,
    ///     -49.29657, 129.545044, 12.248172,
    ///     -55.371487, 122.369644, 0.0,
    ///     -56.872219, 105.365349, 0.0,
    ///     -58.372948, 88.361046, 0.0,
    ///     -59.873676, 71.356743, 0.0,
    ///     -61.374409, 54.352448, 0.0,
    ///     -62.875137, 37.348145, 0.0,
    ///     -64.37587, 20.343845, 0.0,
    ///     -65.876595, 3.339545, 0.0,
    ///     -67.377327, -13.664756, 0.0,
    ///     -68.878059, -30.669056, 0.0,
    ///     -64.153793, -38.797535, 12.248172,
    ///     -57.928802, -29.921717, 24.496344,
    ///     -51.703812, -21.045898, 36.744514,
    ///     -45.478821, -12.170078, 48.992687,
    ///     -39.253826, -3.294259, 61.240856,
    ///     -33.028835, 5.58156, 73.489029,
    ///     -26.803844, 14.457379, 85.737198,
    ///     -20.578852, 23.333199, 97.985374,
    ///     -14.35386, 32.209019, 110.233543,
    ///     -70.378792, -47.673355, 0.0,
    ///     -31.085634, -20.845886, 17.497387,
    ///     -34.352913, -13.825235, 34.994774,
    ///     -37.62019, -6.804585, 52.492165,
    ///     -9.805417, -10.942476, 17.497387,
    ///     -13.072695, -3.921826, 34.994774,
    ///     -16.339973, 3.098825, 52.492165,
    ///     -19.60725, 10.119475, 69.989548,
    ///     11.4748, -1.039067, 17.497387,
    ///     8.207521, 5.981584, 34.994774,
    ///     4.940244, 13.002234, 52.492165,
    ///     1.672966, 20.022884, 69.989548,
    ///     -1.594312, 27.043535, 87.486938,
    ///     -8.128867, 41.084839, 122.481712,
    ///     32.755016, 8.864343, 17.497387,
    ///     29.487738, 15.884994, 34.994774,
    ///     26.220461, 22.905643, 52.492165,
    ///     22.953182, 29.926294, 69.989548,
    ///     54.035233, 18.767752, 17.497387,
    ///     50.767956, 25.788403, 34.994774,
    ///     47.500675, 32.809055, 52.492165,
    ///     99.862946, 31.553921, 0.0,
    ///     89.063759, 32.507011, 12.248172,
    ///     78.26458, 33.460106, 24.496344,
    ///     67.465401, 34.413197, 36.744514,
    ///     56.666218, 35.366287, 48.992687,
    ///     45.867039, 36.319378, 61.240856,
    ///     35.067856, 37.272472, 73.489029,
    ///     24.268677, 38.225563, 85.737198,
    ///     13.469495, 39.178654, 97.985374,
    ///     2.670314, 40.131744, 110.233543,
    ///     -14.35386, 32.209019, 110.233543,
    ///     -20.578852, 23.333199, 97.985374,
    ///     -26.803844, 14.457379, 85.737198,
    ///     -33.028835, 5.58156, 73.489029,
    ///     -39.253826, -3.294259, 61.240856,
    ///     -45.478821, -12.170078, 48.992687,
    ///     -51.703812, -21.045898, 36.744514,
    ///     -57.928802, -29.921717, 24.496344,
    ///     -64.153793, -38.797535, 12.248172,
    ///     -54.902267, -40.470875, 0.0,
    ///     -39.425747, -33.268398, 0.0,
    ///     -23.949224, -26.065916, 0.0,
    ///     -8.472704, -18.863438, 0.0,
    ///     7.003817, -11.660957, 0.0,
    ///     22.480339, -4.458477, 0.0,
    ///     37.95686, 2.744002, 0.0,
    ///     53.43338, 9.946482, 0.0,
    ///     68.909904, 17.148962, 0.0,
    ///     84.386421, 24.35144, 0.0,
    ///     -70.378792, -47.673355, 0.0,
    ///     -53.870758, 139.373947, 0.0,
    ///     -43.11068, -2.953869, 0.0,
    ///     -41.047173, 20.427044, 0.0,
    ///     -38.983669, 43.807957, 0.0,
    ///     -36.920166, 67.188866, 0.0,
    ///     -34.856663, 90.569786, 0.0,
    ///     -19.969576, -4.996209, 0.0,
    ///     -17.906073, 18.384705, 0.0,
    ///     -15.842568, 41.765617, 0.0,
    ///     -13.779064, 65.14653, 0.0,
    ///     -11.71556, 88.527443, 0.0,
    ///     5.23503, 16.342365, 0.0,
    ///     7.298534, 39.723278, 0.0,
    ///     9.362039, 63.104191, 0.0,
    ///     28.376133, 14.300026, 0.0,
    ///     30.439636, 37.680939, 0.0,
    ///     32.503139, 61.061852, 0.0,
    ///     53.580738, 35.638599, 0.0,
    ///     99.862946, 31.553921, 0.0,
    ///     85.887154, 41.355743, 0.0,
    ///     71.911362, 51.157562, 0.0,
    ///     57.93557, 60.959381, 0.0,
    ///     43.959782, 70.7612, 0.0,
    ///     29.98399, 80.563026, 0.0,
    ///     16.008198, 90.364845, 0.0,
    ///     2.032407, 100.166664, 0.0,
    ///     -11.943384, 109.968483, 0.0,
    ///     -25.919174, 119.770309, 0.0,
    ///     -39.894966, 129.572128, 0.0,
    ///     -55.371487, 122.369644, 0.0,
    ///     -56.872219, 105.365349, 0.0,
    ///     -58.372948, 88.361046, 0.0,
    ///     -59.873676, 71.356743, 0.0,
    ///     -61.374409, 54.352448, 0.0,
    ///     -62.875137, 37.348145, 0.0,
    ///     -64.37587, 20.343845, 0.0,
    ///     -65.876595, 3.339545, 0.0,
    ///     -67.377327, -13.664756, 0.0,
    ///     -68.878059, -30.669056, 0.0,
    ///     -54.902267, -40.470875, 0.0,
    ///     -39.425747, -33.268398, 0.0,
    ///     -23.949224, -26.065916, 0.0,
    ///     -8.472704, -18.863438, 0.0,
    ///     7.003817, -11.660957, 0.0,
    ///     22.480339, -4.458477, 0.0,
    ///     37.95686, 2.744002, 0.0,
    ///     53.43338, 9.946482, 0.0,
    ///     68.909904, 17.148962, 0.0,
    ///     84.386421, 24.35144, 0.0,
    /// ],
    /// vec![
    ///     5,2,1,
    ///     6,3,5,
    ///     9,5,8,
    ///     10,6,9,
    ///     11,7,6,
    ///     7,11,12,
    ///     15,9,14,
    ///     16,10,9,
    ///     17,11,16,
    ///     12,11,17,
    ///     19,15,14,
    ///     20,16,15,
    ///     49,0,22,
    ///     48,49,23,
    ///     22,23,49,
    ///     1,48,23,
    ///     24,25,4,
    ///     25,26,4,
    ///     47,48,1,
    ///     8,26,27,
    ///     23,24,1,
    ///     46,47,2,
    ///     1,2,47,
    ///     45,46,3,
    ///     2,3,46,
    ///     6,45,3,
    ///     44,45,7,
    ///     6,7,45,
    ///     43,44,7,
    ///     42,43,12,
    ///     12,41,42,
    ///     7,12,43,
    ///     27,28,14,
    ///     18,14,29,
    ///     30,18,29,
    ///     32,30,31,
    ///     32,31,21,
    ///     19,18,34,
    ///     32,33,30,
    ///     33,34,18,
    ///     30,33,18,
    ///     36,17,16,
    ///     38,39,12,
    ///     12,13,41,
    ///     39,40,12,
    ///     38,12,17,
    ///     40,13,12,
    ///     20,19,35,
    ///     36,20,35,
    ///     17,37,38,
    ///     37,17,36,
    ///     36,16,20,
    ///     19,34,35,
    ///     55,52,51,
    ///     56,53,55,
    ///     59,55,58,
    ///     60,56,59,
    ///     61,57,56,
    ///     57,61,62,
    ///     64,59,63,
    ///     65,60,59,
    ///     66,61,65,
    ///     62,61,66,
    ///     68,64,67,
    ///     69,65,64,
    ///     80,50,81,
    ///     79,80,82,
    ///     81,82,80,
    ///     51,79,82,
    ///     83,84,54,
    ///     84,85,54,
    ///     78,79,51,
    ///     82,83,51,
    ///     52,78,51,
    ///     77,78,52,
    ///     57,75,76,
    ///     77,53,76,
    ///     53,56,76,
    ///     52,53,77,
    ///     74,75,57,
    ///     56,57,76,
    ///     62,74,57,
    ///     62,73,74,
    ///     72,73,62,
    ///     99,71,72,
    ///     63,86,87,
    ///     58,86,63,
    ///     87,88,63,
    ///     85,86,58,
    ///     91,89,90,
    ///     91,90,70,
    ///     67,89,92,
    ///     91,92,89,
    ///     88,89,67,
    ///     67,92,93,
    ///     97,66,96,
    ///     65,95,66,
    ///     62,66,97,
    ///     99,62,98,
    ///     97,98,62,
    ///     72,62,99,
    ///     94,68,93,
    ///     67,93,68,
    ///     69,68,94,
    ///     96,66,95,
    ///     95,65,69,
    ///     94,95,69,
    ///     105,102,101,
    ///     106,103,105,
    ///     109,105,104,
    ///     110,106,105,
    ///     111,107,110,
    ///     107,111,112,
    ///     115,109,108,
    ///     116,110,115,
    ///     117,111,110,
    ///     112,111,117,
    ///     119,115,114,
    ///     120,116,115,
    ///     139,100,140,
    ///     138,139,141,
    ///     140,141,139,
    ///     101,138,141,
    ///     142,143,104,
    ///     143,144,104,
    ///     137,138,101,
    ///     108,144,145,
    ///     141,142,101,
    ///     136,137,102,
    ///     101,102,137,
    ///     135,136,103,
    ///     102,103,136,
    ///     106,135,103,
    ///     134,135,107,
    ///     106,107,135,
    ///     133,134,107,
    ///     132,133,112,
    ///     112,131,132,
    ///     107,112,133,
    ///     114,145,146,
    ///     118,114,147,
    ///     148,118,147,
    ///     122,148,149,
    ///     122,149,121,
    ///     119,118,124,
    ///     122,123,148,
    ///     123,124,118,
    ///     148,123,118,
    ///     126,117,116,
    ///     128,129,112,
    ///     112,113,131,
    ///     129,130,112,
    ///     128,112,117,
    ///     130,113,112,
    ///     120,119,125,
    ///     126,120,125,
    ///     117,127,128,
    ///     127,117,126,
    ///     126,116,120,
    ///     119,124,125,
    ///     114,108,145,
    ///     158,157,153,
    ///     159,158,153,
    ///     160,159,155,
    ///     161,160,155,
    ///     162,157,158,
    ///     163,162,159,
    ///     164,163,160,
    ///     161,164,160,
    ///     166,165,162,
    ///     167,166,163,
    ///     168,165,166,
    ///     167,168,166,
    ///     190,150,189,
    ///     152,188,187,
    ///     186,153,187,
    ///     188,190,189,
    ///     152,192,191,
    ///     188,152,191,
    ///     185,153,186,
    ///     154,153,185,
    ///     191,190,188,
    ///     183,155,184,
    ///     156,182,181,
    ///     179,181,180,
    ///     182,156,155,
    ///     177,156,178,
    ///     178,156,181,
    ///     180,151,179,
    ///     179,178,181,
    ///     154,185,184,
    ///     157,192,152,
    ///     157,194,193,
    ///     157,162,194,
    ///     195,194,162,
    ///     165,195,162,
    ///     192,157,193,
    ///     197,196,165,
    ///     195,165,196,
    ///     197,165,168,
    ///     199,198,170,
    ///     198,197,168,
    ///     199,170,169,
    ///     164,161,175,
    ///     167,164,174,
    ///     174,164,175,
    ///     161,177,176,
    ///     161,176,175,
    ///     161,156,177,
    ///     168,171,198,
    ///     170,198,171,
    ///     172,171,168,
    ///     173,172,167,
    ///     168,167,172,
    ///     174,173,167,
    ///     5,1,4,
    ///     3,2,5,
    ///     5,4,8,
    ///     6,5,9,
    ///     11,6,10,
    ///     9,8,14,
    ///     16,9,15,
    ///     11,10,16,
    ///     19,14,18,
    ///     20,15,19,
    ///     24,4,1,
    ///     26,8,4,
    ///     27,14,8,
    ///     14,28,29,
    ///     55,51,54,
    ///     53,52,55,
    ///     55,54,58,
    ///     56,55,59,
    ///     61,56,60,
    ///     59,58,63,
    ///     65,59,64,
    ///     61,60,65,
    ///     64,63,67,
    ///     69,64,68,
    ///     83,54,51,
    ///     85,58,54,
    ///     88,67,63,
    ///     105,101,104,
    ///     103,102,105,
    ///     109,104,108,
    ///     110,105,109,
    ///     107,106,110,
    ///     115,108,114,
    ///     110,109,115,
    ///     117,110,116,
    ///     119,114,118,
    ///     120,115,119,
    ///     142,104,101,
    ///     144,108,104,
    ///     114,146,147,
    ///     157,152,153,
    ///     159,153,154,
    ///     159,154,155,
    ///     161,155,156,
    ///     162,158,159,
    ///     163,159,160,
    ///     166,162,163,
    ///     167,163,164,
    ///     153,152,187,
    ///     155,154,184,
    ///     182,155,183,
    /// ]
    /// );
    ///
    /// let actual_result = input.get_planar_simplify(0.001, 0.0175);
    /// let actual = actual_result.unwrap();
    ///
    /// let expected = Mesh::new(
    /// vec![
    ///     -53.870758091681715, 139.3739470069094, 0.0, 99.86294597389475, 31.553920927744112, 0.0, -70.37879198032073, -47.67335500055474, 0.0, -8.128867794119, 41.08484095051423, 122.48171103461392
    /// ],
    /// vec![
    ///     2, 0, 1, 2, 1, 3, 0, 2, 3, 1, 0, 3
    /// ]
    /// );
    ///
    /// println!("{:?}", actual);
    ///
    /// assert!(expected.eq(&actual));
    ///
    /// ```
    pub fn get_planar_simplify(&self, tolerance: f64, angle_tolerance: f64) -> Result<Mesh, String> {
        let original_aabb = self.get_bounding_box();
        let original_area = self.get_area();

        let welded_original = self.get_with_welded_vertices(tolerance);

        let unaccepted_edges = welded_original.get_edges_with_more_than_2_neighbours();
        if unaccepted_edges.len() > 0 {
            return Err("This Mesh has edges with more than 2 neighbouring faces, so it cannot be simplified using this method".to_string())
        }

        let planar_meshes: Vec<Mesh> = welded_original.split_by_face_angle(angle_tolerance, Some(tolerance));
        let mut planar_meshes_remeshed: Vec<Mesh> = Vec::with_capacity(planar_meshes.len());

        for planar_mesh in planar_meshes {
            let remeshed_result = planar_mesh.get_planar_simplify_for_planar_mesh(tolerance, angle_tolerance);
            if remeshed_result.is_ok() {
                planar_meshes_remeshed.push(remeshed_result?)
            }
            else {
                planar_meshes_remeshed.push(planar_mesh) // Couldn't remesh so returns original
            }
        }

        let remeshed = Mesh::join(&planar_meshes_remeshed);
        let mut remeshed_welded = remeshed.get_with_welded_vertices(tolerance);

        let remeshed_aabb = remeshed_welded.get_bounding_box();
        if !remeshed_aabb.eq_with_tolerance(&original_aabb, tolerance) {
            return Err("The bounding box of the remeshed Mesh seems to be different from the original one".to_string())
        }

        let remeshed_area = remeshed_welded.get_area();
        if (remeshed_area - original_area).abs() > tolerance {
            return Err("The area of the remeshed Mesh seems to be different from the original one".to_string())
        }
        
        remeshed_welded.id = self.id;

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

    /// Remeshes in the simplified way the [Mesh] which is already a planar one.
    ///
    /// It should work for planar Meshes only.
    ///
    /// # Example
    ///
    /// Here is an example of the planar Mesh which was simplified.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let mesh = Mesh::new(
    /// vec![
    ///     -17.906073, 18.384705, 0.0,
    ///     -19.969576, -4.996209, 0.0,
    ///     -41.047173, 20.427044, 0.0,
    ///     -15.842568, 41.765617, 0.0,
    ///     -13.779064, 65.14653, 0.0,
    ///     -36.920166, 67.188866, 0.0,
    ///     -11.71556, 88.527443, 0.0,
    ///     5.23503, 16.342365, 0.0,
    ///     7.298534, 39.723278, 0.0,
    ///     9.362039, 63.104191, 0.0,
    ///     30.439636, 37.680939, 0.0,
    ///     28.376133, 14.300026, 0.0,
    ///     32.503139, 61.061852, 0.0,
    ///     53.580738, 35.638599, 0.0,
    ///     -54.902267, -40.470875, 0.0,
    ///     -70.378792, -47.673355, 0.0,
    ///     -68.878059, -30.669056, 0.0,
    ///     -43.11068, -2.953869, 0.0,
    ///     -67.377327, -13.664756, 0.0,
    ///     -65.876595, 3.339545, 0.0,
    ///     -64.37587, 20.343845, 0.0,
    ///     -23.949224, -26.065916, 0.0,
    ///     -39.425747, -33.268398, 0.0,
    ///     -62.875137, 37.348145, 0.0,
    ///     -38.983669, 43.807957, 0.0,
    ///     -59.873676, 71.356743, 0.0,
    ///     -61.374409, 54.352448, 0.0,
    ///     -34.856663, 90.569786, 0.0,
    ///     -58.372948, 88.361046, 0.0,
    ///     -56.872219, 105.365349, 0.0,
    ///     -39.894966, 129.572128, 0.0,
    ///     -55.371487, 122.369644, 0.0,
    ///     -11.943384, 109.968483, 0.0,
    ///     -25.919174, 119.770309, 0.0,
    ///     -53.870758, 139.373947, 0.0,
    ///     7.003817, -11.660957, 0.0,
    ///     -8.472704, -18.863438, 0.0,
    ///     22.480339, -4.458477, 0.0,
    ///     53.43338, 9.946482, 0.0,
    ///     37.95686, 2.744002, 0.0,
    ///     84.386421, 24.35144, 0.0,
    ///     68.909904, 17.148962, 0.0,
    ///     85.887154, 41.355743, 0.0,
    ///     99.862946, 31.553921, 0.0,
    ///     16.008198, 90.364845, 0.0,
    ///     29.98399, 80.563026, 0.0,
    ///     2.032407, 100.166664, 0.0,
    ///     71.911362, 51.157562, 0.0,
    ///     57.93557, 60.959381, 0.0,
    ///     43.959782, 70.7612, 0.0,
    /// ],
    /// vec![
    ///     0,1,2,
    ///     3,0,2,
    ///     4,3,5,
    ///     6,4,5,
    ///     7,1,0,
    ///     8,7,3,
    ///     9,8,4,
    ///     6,9,4,
    ///     10,11,7,
    ///     12,10,8,
    ///     13,11,10,
    ///     12,13,10,
    ///     14,15,16,
    ///     17,18,19,
    ///     20,2,19,
    ///     18,14,16,
    ///     17,21,22,
    ///     18,17,22,
    ///     23,2,20,
    ///     24,2,23,
    ///     22,14,18,
    ///     25,5,26,
    ///     27,28,29,
    ///     30,29,31,
    ///     28,27,5,
    ///     32,27,33,
    ///     33,27,29,
    ///     31,34,30,
    ///     30,33,29,
    ///     24,23,26,
    ///     1,21,17,
    ///     1,35,36,
    ///     1,7,35,
    ///     37,35,7,
    ///     11,37,7,
    ///     21,1,36,
    ///     38,39,11,
    ///     37,11,39,
    ///     38,11,13,
    ///     40,41,42,
    ///     41,38,13,
    ///     40,42,43,
    ///     9,6,44,
    ///     12,9,45,
    ///     45,9,44,
    ///     6,32,46,
    ///     6,46,44,
    ///     6,27,32,
    ///     13,47,41,
    ///     42,41,47,
    ///     48,47,13,
    ///     49,48,12,
    ///     13,12,48,
    ///     45,49,12,
    ///     1,17,2,
    ///     3,2,24,
    ///     3,24,5,
    ///     6,5,27,
    ///     7,0,3,
    ///     8,3,4,
    ///     10,7,8,
    ///     12,8,9,
    ///     2,17,19,
    ///     5,24,26,
    ///     28,5,25,
    /// ]
    /// );
    ///
    /// let actual = mesh.get_planar_simplify_for_planar_mesh(0.001, 0.01745);
    ///
    /// let expected = Mesh::new(
    /// vec![
    ///     -53.870758091681715, 139.3739470069094, 0.0, 99.86294597389475, 31.553920927744112, 0.0, -70.37879198032073, -47.67335500055474, 0.0
    /// ],
    /// vec![
    ///     2, 0, 1
    /// ]
    /// );
    ///
    /// println!("{:?}", actual);
    ///
    /// assert!(actual.is_ok());
    /// assert!(expected.eq(&actual.unwrap()));
    /// ```
    pub fn get_planar_simplify_for_planar_mesh(&self, tolerance: f64, angle_tolerance: f64) -> Result<Mesh, String> {
        let original_normal = self.get_face_normal_vectors_unitized()[0];
        let original_aabb = self.get_bounding_box();
        let original_area = self.get_area();

        let welded_original = self.get_with_welded_vertices(tolerance);

        let polygons = welded_original.get_polygons_for_planar_mesh();
        
        let local_coordinate_system_of_polygons = self.get_local_coordinate_system_for_first_face(); // Converting Polygons to 2D Polygons
        let mut polygon2ds = Vec::with_capacity(polygons.len());
        for polygon in polygons {
            polygon2ds.push(polygon.to_polygon2d(&local_coordinate_system_of_polygons));
        }

        let mut cleaned_polygon2ds = Vec::with_capacity(polygon2ds.len()); // Cleaning up 2D Polygons
        for polygon2d in polygon2ds {
            let polygon2d_with_removed_duplicates = polygon2d.get_with_removed_neighbour_duplicates_with_tolerance(tolerance);
            let polygon2d_with_removed_parallel = polygon2d_with_removed_duplicates.get_with_removed_neighbour_parallel_segments_with_tolerance(angle_tolerance);
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
        else if cleaned_polygon2ds.len() == 1 {
            remeshed_xy = cleaned_polygon2ds[0].triangulate_raw(); // If there is one Polygon then no need to search for holes
        }
        else {
            return Err("After polygon cleaning there is 0 polygons for this planar Mesh".to_string());
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

    /// Creates a 'vec' of tuples with unique [Mesh] located in Global Coordinate System,
    /// and a `vec` of [LocalCoordinateSystem]s where these instances are located.
    ///
    /// This way you can save a [Mesh] once and then locate them in different places, rather than
    /// saving all the Meshes.
    ///
    /// `tolerance` input is used for [Mesh] comparison.
    /// 
    /// Please check manually the correctness of the deduplication output.
    ///
    /// # Example
    ///
    /// Below is an example of a simple case with 3 Meshes, that are actually all duplicates of a
    /// box. That's why after the deduplication the result is a single Mesh located in the Global
    /// Coordinate System with 3 [LocalCoordinateSystem]s.
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    /// let box_a = Mesh::new(
    /// vec![
    ///     72.761534, 33.54827, -22.526636,
    ///     66.343965, 33.868829, -41.466331,
    ///     80.852884, 15.513684, -25.573567,
    ///     74.435315, 15.834243, -44.513262,
    ///     55.634221, 24.90819, -16.869414,
    ///     49.216652, 25.228749, -35.809109,
    ///     63.725571, 6.873605, -19.916345,
    ///     57.308002, 7.194164, -38.85604,
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let box_b = Mesh::new(
    /// vec![
    ///     -25.420272, 61.131243, 18.969046,
    ///     -28.073985, 77.243424, 7.421083,
    ///     -20.361479, 50.416475, 2.856865,
    ///     -23.015192, 66.528656, -8.691098,
    ///     -44.587077, 56.07245, 16.315332,
    ///     -47.24079, 72.18463, 4.76737,
    ///     -39.528284, 45.357682, 0.203151,
    ///     -42.181997, 61.469863, -11.344811,
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let box_c = Mesh::new(
    /// vec![
    ///     67.432021, 133.273959, 35.72196,
    ///     65.33002, 139.523639, 16.840141,
    ///     74.376341, 115.712505, 29.136237,
    ///     72.27434, 121.962185, 10.254418,
    ///     48.794478, 126.02573, 35.397681,
    ///     46.692477, 132.27541, 16.515862,
    ///     55.738797, 108.464276, 28.811958,
    ///     53.636796, 114.713956, 9.930139,
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let input = vec![box_a, box_b, box_c];
    ///
    /// let actual = Mesh::deduplicate(input, 0.001);
    ///
    /// let expected_box = Mesh::new(
    /// vec![
    ///     6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let expected: Vec<(Mesh, Vec<LocalCoordinateSystem>)> = vec![
    ///     (expected_box, vec![
    ///         LocalCoordinateSystem::new(
    ///             Point::new(63.331977333333334,24.977273666666665,-40.596234),
    ///             Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
    ///             Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
    ///         ),
    ///         LocalCoordinateSystem::new(
    ///             Point::new(-32.77665566666666,71.98557,1.1657849999999996),
    ///             Vector::new(0.9583402460123103,0.2529396989475088,0.13268564944789007),
    ///             Vector::new(0.2529396670241323,-0.5357383830896744,-0.80560903031817),
    ///         ),
    ///         LocalCoordinateSystem::new(
    ///             Point::new(61.432278999999994,131.25374466666665,14.536807000000001),
    ///             Vector::new(0.931877162134053,0.36241145471899117,0.01621395021112327),
    ///             Vector::new(0.3472159773829504,-0.8780727160278832,-0.3292861527872357),
    ///         ),
    ///     ]),
    /// ];
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
    ///     assert_eq!(expected[i].1.len(), actual[i].1.len());
    ///     for j in 0..expected[i].1.len() {
    ///         assert!(expected[i].1[j].eq_with_tolerance(&actual[i].1[j], 0.0001));
    ///     }
    /// }
    /// ```
    pub fn deduplicate(meshes: Vec<Mesh>, tolerance: f64) -> Vec<(Mesh, Vec<LocalCoordinateSystem>)> {
        let mut uniques: Vec<(Mesh, Vec<LocalCoordinateSystem>)> = Vec::new();
        let mut aabbs_of_uniques: Vec<BoundingBox> = Vec::new();

        for current_mesh in meshes {
            let mut found_duplicate = false;

            let current_mesh_local_coordinate_system = current_mesh.get_local_coordinate_system_for_first_face();
            let current_mesh_in_global = current_mesh.get_in_global_coordinate_system(&current_mesh_local_coordinate_system);
            let current_aabb = current_mesh_in_global.get_bounding_box();

            for i in 0..aabbs_of_uniques.len() {
                let existing_aabb = &aabbs_of_uniques[i];
                if current_aabb.eq_with_tolerance(existing_aabb, tolerance) {
                    let existing_candidate_unique_mesh = &uniques[i].0;
                    if current_mesh_in_global.eq_with_tolerance_without_id(existing_candidate_unique_mesh, tolerance) {
                        uniques[i].1.push(current_mesh_local_coordinate_system);
                        found_duplicate = true;
                        break
                    }
                }
            }

            if !found_duplicate {
                uniques.push((current_mesh_in_global, vec![current_mesh_local_coordinate_system]));
                aabbs_of_uniques.push(current_aabb);
            }
        }

        uniques
    }

    /// Creates a `vec` of tuples with unique [Mesh] located in Global Coordinate System,
    /// and a [HashMap] with original Mesh `id` as the key and [LocalCoordinateSystem]s where these 
    /// instances are located as value. This way it's easier to track which Meshes were deduplicated 
    /// and how to set them back to their original position.
    ///
    /// This way you can save a [Mesh] once and then locate them in different places, rather than
    /// saving all the Meshes.
    ///
    /// This method requires having [Mesh] ids set up.
    ///
    /// `tolerance` input is used for [Mesh] comparison.
    ///
    /// Please check manually the correctness of the deduplication output.
    ///
    /// # Example
    ///
    /// Below is an example of a simple case with 3 Meshes, that are actually all duplicates of a
    /// box. That's why after the deduplication the result is a single Mesh located in the Global
    /// Coordinate System with 3 [LocalCoordinateSystem]s.
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let box_a = Mesh::new_with_id(
    /// Some(0),
    /// vec![
    ///     72.761534, 33.54827, -22.526636,
    ///     66.343965, 33.868829, -41.466331,
    ///     80.852884, 15.513684, -25.573567,
    ///     74.435315, 15.834243, -44.513262,
    ///     55.634221, 24.90819, -16.869414,
    ///     49.216652, 25.228749, -35.809109,
    ///     63.725571, 6.873605, -19.916345,
    ///     57.308002, 7.194164, -38.85604,
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let box_b = Mesh::new_with_id(
    /// Some(1),
    /// vec![
    ///     -25.420272, 61.131243, 18.969046,
    ///     -28.073985, 77.243424, 7.421083,
    ///     -20.361479, 50.416475, 2.856865,
    ///     -23.015192, 66.528656, -8.691098,
    ///     -44.587077, 56.07245, 16.315332,
    ///     -47.24079, 72.18463, 4.76737,
    ///     -39.528284, 45.357682, 0.203151,
    ///     -42.181997, 61.469863, -11.344811,
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let box_c = Mesh::new_with_id(
    /// Some(2),
    /// vec![
    ///     67.432021, 133.273959, 35.72196,
    ///     65.33002, 139.523639, 16.840141,
    ///     74.376341, 115.712505, 29.136237,
    ///     72.27434, 121.962185, 10.254418,
    ///     48.794478, 126.02573, 35.397681,
    ///     46.692477, 132.27541, 16.515862,
    ///     55.738797, 108.464276, 28.811958,
    ///     53.636796, 114.713956, 9.930139,
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let input = vec![box_a, box_b, box_c];
    ///
    /// let actual = Mesh::deduplicate_with_id_info(input, 0.001);
    ///
    /// let expected_box = Mesh::new_with_id(
    /// Some(0),
    /// vec![
    ///     6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8
    /// ],
    /// vec![
    ///     5,1,3,
    ///     6,2,0,
    ///     7,3,2,
    ///     3,1,0,
    ///     1,5,4,
    ///     5,7,6,
    ///     5,3,7,
    ///     6,0,4,
    ///     7,2,6,
    ///     3,0,2,
    ///     1,4,0,
    ///     5,6,4,
    /// ]);
    ///
    /// let mut expected_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
    /// expected_map.insert(0,
    ///     LocalCoordinateSystem::new(
    ///         Point::new(63.331977333333334,24.977273666666665,-40.596234),
    ///         Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
    ///         Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
    ///     )
    /// );
    /// expected_map.insert(1,
    ///     LocalCoordinateSystem::new(
    ///         Point::new(-32.77665566666666,71.98557,1.1657849999999996),
    ///         Vector::new(0.9583402460123103,0.2529396989475088,0.13268564944789007),
    ///         Vector::new(0.2529396670241323,-0.5357383830896744,-0.80560903031817),
    ///     )
    /// );
    /// expected_map.insert(2,
    ///     LocalCoordinateSystem::new(
    ///         Point::new(61.432278999999994,131.25374466666665,14.536807000000001),
    ///         Vector::new(0.931877162134053,0.36241145471899117,0.01621395021112327),
    ///         Vector::new(0.3472159773829504,-0.8780727160278832,-0.3292861527872357),
    ///     )
    /// );
    ///
    /// let expected = vec![(expected_box, expected_map)];
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
    ///     let expected_map = &expected[i].1;
    ///     let actual_map = &actual[i].1;
    ///     for expected_key_value_pair in expected_map {
    ///         assert!(actual_map.contains_key(expected_key_value_pair.0));
    ///         let expected_value = expected_key_value_pair.1;
    ///         let actual_value = actual_map[expected_key_value_pair.0];
    ///         assert!(expected_value.eq_with_tolerance(&actual_value, 0.0001));
    ///     }
    /// }
    /// ```
    pub fn deduplicate_with_id_info(meshes: Vec<Mesh>, tolerance: f64) -> Vec<(Mesh, HashMap<usize, LocalCoordinateSystem>)> {
        for mesh in &meshes {
            if mesh.id.is_none() {
                panic!("This method (deduplicate_with_id_info) requires to have all Mesh ids being set up");
            }
        }
        
        let mut uniques: Vec<(Mesh, HashMap<usize, LocalCoordinateSystem>)> = Vec::new();
        let mut aabbs_of_uniques: Vec<BoundingBox> = Vec::new();

        for current_mesh in meshes {
            let mut found_duplicate = false;

            let current_mesh_local_coordinate_system = current_mesh.get_local_coordinate_system_for_first_face();
            let current_mesh_in_global = current_mesh.get_in_global_coordinate_system(&current_mesh_local_coordinate_system);
            let current_aabb = current_mesh_in_global.get_bounding_box();

            for i in 0..aabbs_of_uniques.len() {
                let existing_aabb = &aabbs_of_uniques[i];
                if current_aabb.eq_with_tolerance(existing_aabb, tolerance) {
                    let existing_candidate_unique_mesh = &uniques[i].0;
                    if current_mesh_in_global.eq_with_tolerance_without_id(existing_candidate_unique_mesh, tolerance) {
                        uniques[i].1.insert(current_mesh.id.unwrap(), current_mesh_local_coordinate_system);
                        found_duplicate = true;
                        break
                    }
                }
            }

            if !found_duplicate {
                let mut new_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
                new_map.insert(current_mesh.id.unwrap(), current_mesh_local_coordinate_system);
                uniques.push((current_mesh_in_global, new_map));
                aabbs_of_uniques.push(current_aabb);
            }
        }

        uniques
    }


    /// Gets Polygons for planar [Mesh].
    fn get_polygons_for_planar_mesh(&self) -> Vec<Polygon> {
        let edges = self.get_edges_with_missing_neighbour(); // Getting Graph and calculating grouped edges
        let graph = Graph::new(self.get_number_of_vertices(), edges);
        let disconnected_parts = graph.split_disconnected_loops();

        let mut polygons = Vec::with_capacity(disconnected_parts.len()); // Converting grouped edges to Polygons
        for disconnected_part in disconnected_parts {
            let mut polygon_vertices = Vec::new();
            for i in disconnected_part {
                let vertex = Point::new(self.coordinates[i*3], self.coordinates[i*3+1], self.coordinates[i*3+2]);
                polygon_vertices.push(vertex)
            }
            polygons.push(Polygon::new(polygon_vertices))
        }

        polygons
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
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
    fn test_planar_simplify_pyramid() {
        let input = Mesh::new(
            vec![
                99.862946, 31.553921, 0.0,
                56.983097, 52.169056, 17.497387,
                52.536674, 45.829185, 34.994774,
                48.090248, 39.489315, 52.492165,
                37.766384, 65.646561, 17.497387,
                33.319962, 59.30669, 34.994774,
                28.873537, 52.96682, 52.492165,
                24.427114, 46.626945, 69.989548,
                18.549671, 79.124062, 17.497387,
                14.103248, 72.784195, 34.994774,
                9.656824, 66.444321, 52.492165,
                5.210402, 60.10445, 69.989548,
                0.763979, 53.76458, 87.486938,
                -8.128867, 41.084839, 122.481712,
                -0.667042, 92.60157, 17.497387,
                -5.113465, 86.261696, 34.994774,
                -9.559888, 79.921822, 52.492165,
                -14.006311, 73.581955, 69.989548,
                -19.883755, 106.079071, 17.497387,
                -24.330177, 99.739197, 34.994774,
                -28.7766, 93.39933, 52.492165,
                -53.870758, 139.373947, 0.0,
                85.887154, 41.355743, 0.0,
                71.911362, 51.157562, 0.0,
                57.93557, 60.959381, 0.0,
                43.959782, 70.7612, 0.0,
                29.98399, 80.563026, 0.0,
                16.008198, 90.364845, 0.0,
                2.032407, 100.166664, 0.0,
                -11.943384, 109.968483, 0.0,
                -25.919174, 119.770309, 0.0,
                -39.894966, 129.572128, 0.0,
                -49.29657, 129.545044, 12.248172,
                -44.722378, 119.716125, 24.496344,
                -40.14819, 109.887215, 36.744514,
                -35.574001, 100.058304, 48.992687,
                -30.999813, 90.229393, 61.240856,
                -26.425623, 80.400482, 73.489029,
                -21.851435, 70.571571, 85.737198,
                -17.277245, 60.742661, 97.985374,
                -12.703056, 50.91375, 110.233543,
                2.670314, 40.131744, 110.233543,
                13.469495, 39.178654, 97.985374,
                24.268677, 38.225563, 85.737198,
                35.067856, 37.272472, 73.489029,
                45.867039, 36.319378, 61.240856,
                56.666218, 35.366287, 48.992687,
                67.465401, 34.413197, 36.744514,
                78.26458, 33.460106, 24.496344,
                89.063759, 32.507011, 12.248172,
                -53.870758, 139.373947, 0.0,
                -50.284065, 91.931343, 17.497387,
                -42.570362, 91.250565, 34.994774,
                -34.856663, 90.569786, 52.492165,
                -52.347569, 68.55043, 17.497387,
                -44.633869, 67.869652, 34.994774,
                -36.920166, 67.188866, 52.492165,
                -29.206467, 66.508087, 69.989548,
                -54.411072, 45.169518, 17.497387,
                -46.697372, 44.488735, 34.994774,
                -38.983669, 43.807957, 52.492165,
                -31.26997, 43.127178, 69.989548,
                -23.556269, 42.446396, 87.486938,
                -56.474575, 21.788603, 17.497387,
                -48.760876, 21.107822, 34.994774,
                -41.047173, 20.427044, 52.492165,
                -33.333473, 19.746264, 69.989548,
                -58.538078, -1.59231, 17.497387,
                -50.824379, -2.27309, 34.994774,
                -43.11068, -2.953869, 52.492165,
                -70.378792, -47.673355, 0.0,
                -8.128867, 41.084839, 122.481712,
                -12.703056, 50.91375, 110.233543,
                -17.277245, 60.742661, 97.985374,
                -21.851435, 70.571571, 85.737198,
                -26.425623, 80.400482, 73.489029,
                -30.999813, 90.229393, 61.240856,
                -35.574001, 100.058304, 48.992687,
                -40.14819, 109.887215, 36.744514,
                -44.722378, 119.716125, 24.496344,
                -49.29657, 129.545044, 12.248172,
                -55.371487, 122.369644, 0.0,
                -56.872219, 105.365349, 0.0,
                -58.372948, 88.361046, 0.0,
                -59.873676, 71.356743, 0.0,
                -61.374409, 54.352448, 0.0,
                -62.875137, 37.348145, 0.0,
                -64.37587, 20.343845, 0.0,
                -65.876595, 3.339545, 0.0,
                -67.377327, -13.664756, 0.0,
                -68.878059, -30.669056, 0.0,
                -64.153793, -38.797535, 12.248172,
                -57.928802, -29.921717, 24.496344,
                -51.703812, -21.045898, 36.744514,
                -45.478821, -12.170078, 48.992687,
                -39.253826, -3.294259, 61.240856,
                -33.028835, 5.58156, 73.489029,
                -26.803844, 14.457379, 85.737198,
                -20.578852, 23.333199, 97.985374,
                -14.35386, 32.209019, 110.233543,
                -70.378792, -47.673355, 0.0,
                -31.085634, -20.845886, 17.497387,
                -34.352913, -13.825235, 34.994774,
                -37.62019, -6.804585, 52.492165,
                -9.805417, -10.942476, 17.497387,
                -13.072695, -3.921826, 34.994774,
                -16.339973, 3.098825, 52.492165,
                -19.60725, 10.119475, 69.989548,
                11.4748, -1.039067, 17.497387,
                8.207521, 5.981584, 34.994774,
                4.940244, 13.002234, 52.492165,
                1.672966, 20.022884, 69.989548,
                -1.594312, 27.043535, 87.486938,
                -8.128867, 41.084839, 122.481712,
                32.755016, 8.864343, 17.497387,
                29.487738, 15.884994, 34.994774,
                26.220461, 22.905643, 52.492165,
                22.953182, 29.926294, 69.989548,
                54.035233, 18.767752, 17.497387,
                50.767956, 25.788403, 34.994774,
                47.500675, 32.809055, 52.492165,
                99.862946, 31.553921, 0.0,
                89.063759, 32.507011, 12.248172,
                78.26458, 33.460106, 24.496344,
                67.465401, 34.413197, 36.744514,
                56.666218, 35.366287, 48.992687,
                45.867039, 36.319378, 61.240856,
                35.067856, 37.272472, 73.489029,
                24.268677, 38.225563, 85.737198,
                13.469495, 39.178654, 97.985374,
                2.670314, 40.131744, 110.233543,
                -14.35386, 32.209019, 110.233543,
                -20.578852, 23.333199, 97.985374,
                -26.803844, 14.457379, 85.737198,
                -33.028835, 5.58156, 73.489029,
                -39.253826, -3.294259, 61.240856,
                -45.478821, -12.170078, 48.992687,
                -51.703812, -21.045898, 36.744514,
                -57.928802, -29.921717, 24.496344,
                -64.153793, -38.797535, 12.248172,
                -54.902267, -40.470875, 0.0,
                -39.425747, -33.268398, 0.0,
                -23.949224, -26.065916, 0.0,
                -8.472704, -18.863438, 0.0,
                7.003817, -11.660957, 0.0,
                22.480339, -4.458477, 0.0,
                37.95686, 2.744002, 0.0,
                53.43338, 9.946482, 0.0,
                68.909904, 17.148962, 0.0,
                84.386421, 24.35144, 0.0,
                -70.378792, -47.673355, 0.0,
                -53.870758, 139.373947, 0.0,
                -43.11068, -2.953869, 0.0,
                -41.047173, 20.427044, 0.0,
                -38.983669, 43.807957, 0.0,
                -36.920166, 67.188866, 0.0,
                -34.856663, 90.569786, 0.0,
                -19.969576, -4.996209, 0.0,
                -17.906073, 18.384705, 0.0,
                -15.842568, 41.765617, 0.0,
                -13.779064, 65.14653, 0.0,
                -11.71556, 88.527443, 0.0,
                5.23503, 16.342365, 0.0,
                7.298534, 39.723278, 0.0,
                9.362039, 63.104191, 0.0,
                28.376133, 14.300026, 0.0,
                30.439636, 37.680939, 0.0,
                32.503139, 61.061852, 0.0,
                53.580738, 35.638599, 0.0,
                99.862946, 31.553921, 0.0,
                85.887154, 41.355743, 0.0,
                71.911362, 51.157562, 0.0,
                57.93557, 60.959381, 0.0,
                43.959782, 70.7612, 0.0,
                29.98399, 80.563026, 0.0,
                16.008198, 90.364845, 0.0,
                2.032407, 100.166664, 0.0,
                -11.943384, 109.968483, 0.0,
                -25.919174, 119.770309, 0.0,
                -39.894966, 129.572128, 0.0,
                -55.371487, 122.369644, 0.0,
                -56.872219, 105.365349, 0.0,
                -58.372948, 88.361046, 0.0,
                -59.873676, 71.356743, 0.0,
                -61.374409, 54.352448, 0.0,
                -62.875137, 37.348145, 0.0,
                -64.37587, 20.343845, 0.0,
                -65.876595, 3.339545, 0.0,
                -67.377327, -13.664756, 0.0,
                -68.878059, -30.669056, 0.0,
                -54.902267, -40.470875, 0.0,
                -39.425747, -33.268398, 0.0,
                -23.949224, -26.065916, 0.0,
                -8.472704, -18.863438, 0.0,
                7.003817, -11.660957, 0.0,
                22.480339, -4.458477, 0.0,
                37.95686, 2.744002, 0.0,
                53.43338, 9.946482, 0.0,
                68.909904, 17.148962, 0.0,
                84.386421, 24.35144, 0.0,
            ],
            vec![
                5,2,1,
                6,3,5,
                9,5,8,
                10,6,9,
                11,7,6,
                7,11,12,
                15,9,14,
                16,10,9,
                17,11,16,
                12,11,17,
                19,15,14,
                20,16,15,
                49,0,22,
                48,49,23,
                22,23,49,
                1,48,23,
                24,25,4,
                25,26,4,
                47,48,1,
                8,26,27,
                23,24,1,
                46,47,2,
                1,2,47,
                45,46,3,
                2,3,46,
                6,45,3,
                44,45,7,
                6,7,45,
                43,44,7,
                42,43,12,
                12,41,42,
                7,12,43,
                27,28,14,
                18,14,29,
                30,18,29,
                32,30,31,
                32,31,21,
                19,18,34,
                32,33,30,
                33,34,18,
                30,33,18,
                36,17,16,
                38,39,12,
                12,13,41,
                39,40,12,
                38,12,17,
                40,13,12,
                20,19,35,
                36,20,35,
                17,37,38,
                37,17,36,
                36,16,20,
                19,34,35,
                55,52,51,
                56,53,55,
                59,55,58,
                60,56,59,
                61,57,56,
                57,61,62,
                64,59,63,
                65,60,59,
                66,61,65,
                62,61,66,
                68,64,67,
                69,65,64,
                80,50,81,
                79,80,82,
                81,82,80,
                51,79,82,
                83,84,54,
                84,85,54,
                78,79,51,
                82,83,51,
                52,78,51,
                77,78,52,
                57,75,76,
                77,53,76,
                53,56,76,
                52,53,77,
                74,75,57,
                56,57,76,
                62,74,57,
                62,73,74,
                72,73,62,
                99,71,72,
                63,86,87,
                58,86,63,
                87,88,63,
                85,86,58,
                91,89,90,
                91,90,70,
                67,89,92,
                91,92,89,
                88,89,67,
                67,92,93,
                97,66,96,
                65,95,66,
                62,66,97,
                99,62,98,
                97,98,62,
                72,62,99,
                94,68,93,
                67,93,68,
                69,68,94,
                96,66,95,
                95,65,69,
                94,95,69,
                105,102,101,
                106,103,105,
                109,105,104,
                110,106,105,
                111,107,110,
                107,111,112,
                115,109,108,
                116,110,115,
                117,111,110,
                112,111,117,
                119,115,114,
                120,116,115,
                139,100,140,
                138,139,141,
                140,141,139,
                101,138,141,
                142,143,104,
                143,144,104,
                137,138,101,
                108,144,145,
                141,142,101,
                136,137,102,
                101,102,137,
                135,136,103,
                102,103,136,
                106,135,103,
                134,135,107,
                106,107,135,
                133,134,107,
                132,133,112,
                112,131,132,
                107,112,133,
                114,145,146,
                118,114,147,
                148,118,147,
                122,148,149,
                122,149,121,
                119,118,124,
                122,123,148,
                123,124,118,
                148,123,118,
                126,117,116,
                128,129,112,
                112,113,131,
                129,130,112,
                128,112,117,
                130,113,112,
                120,119,125,
                126,120,125,
                117,127,128,
                127,117,126,
                126,116,120,
                119,124,125,
                114,108,145,
                158,157,153,
                159,158,153,
                160,159,155,
                161,160,155,
                162,157,158,
                163,162,159,
                164,163,160,
                161,164,160,
                166,165,162,
                167,166,163,
                168,165,166,
                167,168,166,
                190,150,189,
                152,188,187,
                186,153,187,
                188,190,189,
                152,192,191,
                188,152,191,
                185,153,186,
                154,153,185,
                191,190,188,
                183,155,184,
                156,182,181,
                179,181,180,
                182,156,155,
                177,156,178,
                178,156,181,
                180,151,179,
                179,178,181,
                154,185,184,
                157,192,152,
                157,194,193,
                157,162,194,
                195,194,162,
                165,195,162,
                192,157,193,
                197,196,165,
                195,165,196,
                197,165,168,
                199,198,170,
                198,197,168,
                199,170,169,
                164,161,175,
                167,164,174,
                174,164,175,
                161,177,176,
                161,176,175,
                161,156,177,
                168,171,198,
                170,198,171,
                172,171,168,
                173,172,167,
                168,167,172,
                174,173,167,
                5,1,4,
                3,2,5,
                5,4,8,
                6,5,9,
                11,6,10,
                9,8,14,
                16,9,15,
                11,10,16,
                19,14,18,
                20,15,19,
                24,4,1,
                26,8,4,
                27,14,8,
                14,28,29,
                55,51,54,
                53,52,55,
                55,54,58,
                56,55,59,
                61,56,60,
                59,58,63,
                65,59,64,
                61,60,65,
                64,63,67,
                69,64,68,
                83,54,51,
                85,58,54,
                88,67,63,
                105,101,104,
                103,102,105,
                109,104,108,
                110,105,109,
                107,106,110,
                115,108,114,
                110,109,115,
                117,110,116,
                119,114,118,
                120,115,119,
                142,104,101,
                144,108,104,
                114,146,147,
                157,152,153,
                159,153,154,
                159,154,155,
                161,155,156,
                162,158,159,
                163,159,160,
                166,162,163,
                167,163,164,
                153,152,187,
                155,154,184,
                182,155,183,
            ]
        );

        let actual_result = input.get_planar_simplify(0.001, 0.0175);
        let actual = actual_result.unwrap();

        let expected = Mesh::new(
            vec![
                -53.870758091681715, 139.3739470069094, 0.0, 99.86294597389475, 31.553920927744112, 0.0, -70.37879198032073, -47.67335500055474, 0.0, -8.128867794119, 41.08484095051423, 122.48171103461392
            ],
            vec![
                2, 0, 1, 2, 1, 3, 0, 2, 3, 1, 0, 3
            ]
        );

        println!("{:?}", actual);

        assert!(expected.eq(&actual));
    }

    #[test]
    fn test_planar_simplify_beam() {
        let input = Mesh::new(
            vec![
                -10.364054, -1.750129, 2.305292,
                -10.364054, -1.750129, 0.305292,
                -10.364054, 35.179871, 1.305292,
                -10.364054, 182.899872, 2.305292,
                -10.364054, 182.899872, 0.305292,
                -10.364054, 330.619873, 1.305292,
                -10.364054, 367.549866, 2.305292,
                -10.364054, 367.549866, 0.305292,
                -10.364054, 341.171295, 0.305292,
                -10.364054, 314.792725, 0.305292,
                -10.364054, 288.414154, 0.305292,
                -10.364054, 262.035583, 0.305292,
                -10.364054, 235.657013, 0.305292,
                -10.364054, 209.278442, 0.305292,
                -10.364054, 156.521301, 0.305292,
                -10.364054, 130.142731, 0.305292,
                -10.364054, 103.76416, 0.305292,
                -10.364054, 77.385582, 0.305292,
                -10.364054, 51.007015, 0.305292,
                -10.364054, 24.628443, 0.305292,
                -10.364054, 35.179871, 0.805292,
                -10.364054, 16.71487, 1.305292,
                -10.364054, 16.71487, 0.805292,
                -10.364054, 35.179871, 1.805292,
                -10.364054, 16.71487, 1.805292,
                -10.364054, 349.084869, 1.305292,
                -10.364054, 330.619873, 1.805292,
                -10.364054, 349.084869, 1.805292,
                -10.364054, 330.619873, 0.805292,
                -10.364054, 349.084869, 0.805292,
                -10.364054, 24.628443, 2.305292,
                -10.364054, 51.007015, 2.305292,
                -10.364054, 77.385582, 2.305292,
                -10.364054, 103.76416, 2.305292,
                -10.364054, 130.142731, 2.305292,
                -10.364054, 156.521301, 2.305292,
                -10.364054, 209.278442, 2.305292,
                -10.364054, 235.657013, 2.305292,
                -10.364054, 262.035583, 2.305292,
                -10.364054, 288.414154, 2.305292,
                -10.364054, 314.792725, 2.305292,
                -10.364054, 341.171295, 2.305292,
                -10.364054, 109.039871, 1.305292,
                -10.364054, 145.969864, 1.305292,
                -10.364054, 182.899872, 1.305292,
                -10.364054, 219.829865, 1.305292,
                -10.364054, 256.759857, 1.305292,
                -10.364054, 293.68988, 1.305292,
                -10.364054, -1.750129, 0.305292,
                9.635946, -1.750129, 0.305292,
                -5.364054, 90.574867, 0.305292,
                -0.364054, 90.574867, 0.305292,
                4.635946, 90.574867, 0.305292,
                -10.364054, 182.899872, 0.305292,
                -5.364054, 182.899872, 0.305292,
                -0.364054, 182.899872, 0.305292,
                4.635946, 182.899872, 0.305292,
                9.635946, 182.899872, 0.305292,
                -5.364054, 275.224884, 0.305292,
                -0.364054, 275.224884, 0.305292,
                4.635946, 275.224884, 0.305292,
                -10.364054, 367.549866, 0.305292,
                9.635946, 367.549866, 0.305292,
                -10.364054, 24.628443, 0.305292,
                -10.364054, 51.007015, 0.305292,
                -10.364054, 77.385582, 0.305292,
                -10.364054, 103.76416, 0.305292,
                -10.364054, 130.142731, 0.305292,
                -10.364054, 156.521301, 0.305292,
                -10.364054, 209.278442, 0.305292,
                -10.364054, 235.657013, 0.305292,
                -10.364054, 262.035583, 0.305292,
                -10.364054, 288.414154, 0.305292,
                -10.364054, 314.792725, 0.305292,
                -10.364054, 341.171295, 0.305292,
                -6.364054, 367.549866, 0.305292,
                -2.364054, 367.549866, 0.305292,
                1.635946, 367.549866, 0.305292,
                5.635946, 367.549866, 0.305292,
                -5.364054, 321.38736, 0.305292,
                -7.864054, 275.224884, 0.305292,
                -7.864054, 321.38736, 0.305292,
                -0.364054, 321.38736, 0.305292,
                -2.864054, 275.224884, 0.305292,
                -2.864054, 321.38736, 0.305292,
                4.635946, 321.38736, 0.305292,
                2.135946, 275.224884, 0.305292,
                2.135946, 321.38736, 0.305292,
                7.135946, 275.224884, 0.305292,
                7.135946, 321.38736, 0.305292,
                5.635946, -1.750129, 0.305292,
                1.635946, -1.750129, 0.305292,
                -2.364054, -1.750129, 0.305292,
                -6.364054, -1.750129, 0.305292,
                7.135946, 90.574867, 0.305292,
                4.635946, 44.412373, 0.305292,
                7.135946, 44.412373, 0.305292,
                2.135946, 90.574867, 0.305292,
                -0.364054, 44.412373, 0.305292,
                2.135946, 44.412373, 0.305292,
                -2.864054, 90.574867, 0.305292,
                -5.364054, 44.412373, 0.305292,
                -2.864054, 44.412373, 0.305292,
                -7.864054, 90.574867, 0.305292,
                -7.864054, 44.412373, 0.305292,
                9.635946, 341.171295, 0.305292,
                9.635946, 314.792725, 0.305292,
                9.635946, 288.414154, 0.305292,
                9.635946, 262.035583, 0.305292,
                9.635946, 235.657013, 0.305292,
                9.635946, 209.278442, 0.305292,
                9.635946, 156.521301, 0.305292,
                9.635946, 130.142731, 0.305292,
                9.635946, 103.76416, 0.305292,
                9.635946, 77.385582, 0.305292,
                9.635946, 51.007015, 0.305292,
                9.635946, 24.628443, 0.305292,
                9.635946, -1.750129, 0.305292,
                9.635946, -1.750129, 2.305292,
                9.635946, 35.179871, 1.305292,
                9.635946, 182.899872, 0.305292,
                9.635946, 182.899872, 2.305292,
                9.635946, 330.619873, 1.305292,
                9.635946, 367.549866, 0.305292,
                9.635946, 367.549866, 2.305292,
                9.635946, 349.084869, 1.305292,
                9.635946, 330.619873, 0.805292,
                9.635946, 349.084869, 0.805292,
                9.635946, 330.619873, 1.805292,
                9.635946, 349.084869, 1.805292,
                9.635946, 35.179871, 1.805292,
                9.635946, 16.71487, 1.305292,
                9.635946, 16.71487, 1.805292,
                9.635946, 35.179871, 0.805292,
                9.635946, 16.71487, 0.805292,
                9.635946, 24.628443, 0.305292,
                9.635946, 51.007015, 0.305292,
                9.635946, 77.385582, 0.305292,
                9.635946, 103.76416, 0.305292,
                9.635946, 130.142731, 0.305292,
                9.635946, 156.521301, 0.305292,
                9.635946, 209.278442, 0.305292,
                9.635946, 235.657013, 0.305292,
                9.635946, 262.035583, 0.305292,
                9.635946, 288.414154, 0.305292,
                9.635946, 314.792725, 0.305292,
                9.635946, 341.171295, 0.305292,
                9.635946, 341.171295, 2.305292,
                9.635946, 314.792725, 2.305292,
                9.635946, 288.414154, 2.305292,
                9.635946, 262.035583, 2.305292,
                9.635946, 235.657013, 2.305292,
                9.635946, 209.278442, 2.305292,
                9.635946, 156.521301, 2.305292,
                9.635946, 130.142731, 2.305292,
                9.635946, 103.76416, 2.305292,
                9.635946, 77.385582, 2.305292,
                9.635946, 51.007015, 2.305292,
                9.635946, 24.628443, 2.305292,
                9.635946, 109.039871, 1.305292,
                9.635946, 145.969864, 1.305292,
                9.635946, 182.899872, 1.305292,
                9.635946, 219.829865, 1.305292,
                9.635946, 256.759857, 1.305292,
                9.635946, 293.68988, 1.305292,
                9.635946, -1.750129, 2.305292,
                0.335946, -1.750129, 2.305292,
                6.535946, 72.109871, 2.305292,
                3.435946, 72.109871, 2.305292,
                6.535946, 145.969864, 2.305292,
                3.435946, 145.969864, 2.305292,
                6.535946, 219.829865, 2.305292,
                3.435946, 219.829865, 2.305292,
                6.535946, 293.68988, 2.305292,
                3.435946, 293.68988, 2.305292,
                9.635946, 367.549866, 2.305292,
                6.535946, 367.549866, 2.305292,
                3.435946, 367.549866, 2.305292,
                0.335946, 367.549866, 2.305292,
                6.535946, 330.619873, 2.305292,
                8.085946, 293.68988, 2.305292,
                8.085946, 330.619873, 2.305292,
                1.885946, 293.68988, 2.305292,
                3.435946, 330.619873, 2.305292,
                1.885946, 330.619873, 2.305292,
                3.435946, -1.750129, 2.305292,
                6.535946, -1.750129, 2.305292,
                1.885946, 72.109871, 2.305292,
                3.435946, 35.179871, 2.305292,
                1.885946, 35.179871, 2.305292,
                8.085946, 72.109871, 2.305292,
                6.535946, 35.179871, 2.305292,
                8.085946, 35.179871, 2.305292,
                9.635946, 24.628443, 2.305292,
                9.635946, 51.007015, 2.305292,
                9.635946, 77.385582, 2.305292,
                9.635946, 103.76416, 2.305292,
                9.635946, 130.142731, 2.305292,
                9.635946, 156.521301, 2.305292,
                9.635946, 182.899872, 2.305292,
                9.635946, 209.278442, 2.305292,
                9.635946, 235.657013, 2.305292,
                9.635946, 262.035583, 2.305292,
                9.635946, 288.414154, 2.305292,
                9.635946, 314.792725, 2.305292,
                9.635946, 341.171295, 2.305292,
                0.335946, 347.033203, 2.305292,
                0.335946, 326.516541, 2.305292,
                0.335946, 305.999878, 2.305292,
                0.335946, 285.483215, 2.305292,
                0.335946, 264.966553, 2.305292,
                0.335946, 244.449875, 2.305292,
                0.335946, 223.933197, 2.305292,
                0.335946, 203.416534, 2.305292,
                0.335946, 182.899872, 2.305292,
                0.335946, 162.383209, 2.305292,
                0.335946, 141.866531, 2.305292,
                0.335946, 121.349869, 2.305292,
                0.335946, 100.833206, 2.305292,
                0.335946, 80.316536, 2.305292,
                0.335946, 59.79987, 2.305292,
                0.335946, 39.283203, 2.305292,
                0.335946, 18.766537, 2.305292,
                0.335946, -1.750129, 2.305292,
                0.335946, -1.750129, 20.305292,
                0.335946, 26.657564, 4.555292,
                0.335946, 26.657564, 6.805292,
                0.335946, 26.657564, 9.055292,
                0.335946, 26.657564, 11.305292,
                0.335946, 26.657564, 13.555292,
                0.335946, 26.657564, 15.805292,
                0.335946, 26.657564, 18.055292,
                0.335946, 55.065254, 4.555292,
                0.335946, 55.065254, 6.805292,
                0.335946, 55.065254, 9.055292,
                0.335946, 55.065254, 11.305292,
                0.335946, 55.065254, 13.555292,
                0.335946, 55.065254, 15.805292,
                0.335946, 55.065254, 18.055292,
                0.335946, 83.472946, 4.555292,
                0.335946, 83.472946, 6.805292,
                0.335946, 83.472946, 9.055292,
                0.335946, 83.472946, 11.305292,
                0.335946, 83.472946, 13.555292,
                0.335946, 83.472946, 15.805292,
                0.335946, 83.472946, 18.055292,
                0.335946, 111.880638, 4.555292,
                0.335946, 111.880638, 6.805292,
                0.335946, 111.880638, 9.055292,
                0.335946, 111.880638, 11.305292,
                0.335946, 111.880638, 13.555292,
                0.335946, 111.880638, 15.805292,
                0.335946, 111.880638, 18.055292,
                0.335946, 140.28833, 4.555292,
                0.335946, 140.28833, 6.805292,
                0.335946, 140.28833, 9.055292,
                0.335946, 140.28833, 11.305292,
                0.335946, 140.28833, 13.555292,
                0.335946, 140.28833, 15.805292,
                0.335946, 140.28833, 18.055292,
                0.335946, 168.69603, 4.555292,
                0.335946, 168.69603, 6.805292,
                0.335946, 168.69603, 9.055292,
                0.335946, 168.69603, 11.305292,
                0.335946, 168.69603, 13.555292,
                0.335946, 168.69603, 15.805292,
                0.335946, 168.69603, 18.055292,
                0.335946, 197.103714, 4.555292,
                0.335946, 197.103714, 6.805292,
                0.335946, 197.103714, 9.055292,
                0.335946, 197.103714, 11.305292,
                0.335946, 197.103714, 13.555292,
                0.335946, 197.103714, 15.805292,
                0.335946, 197.103714, 18.055292,
                0.335946, 225.511414, 4.555292,
                0.335946, 225.511414, 6.805292,
                0.335946, 225.511414, 9.055292,
                0.335946, 225.511414, 11.305292,
                0.335946, 225.511414, 13.555292,
                0.335946, 225.511414, 15.805292,
                0.335946, 225.511414, 18.055292,
                0.335946, 253.919098, 4.555292,
                0.335946, 253.919098, 6.805292,
                0.335946, 253.919098, 9.055292,
                0.335946, 253.919098, 11.305292,
                0.335946, 253.919098, 13.555292,
                0.335946, 253.919098, 15.805292,
                0.335946, 253.919098, 18.055292,
                0.335946, 282.326782, 4.555292,
                0.335946, 282.326782, 6.805292,
                0.335946, 282.326782, 9.055292,
                0.335946, 282.326782, 11.305292,
                0.335946, 282.326782, 13.555292,
                0.335946, 282.326782, 15.805292,
                0.335946, 282.326782, 18.055292,
                0.335946, 310.734497, 4.555292,
                0.335946, 310.734497, 6.805292,
                0.335946, 310.734497, 9.055292,
                0.335946, 310.734497, 11.305292,
                0.335946, 310.734497, 13.555292,
                0.335946, 310.734497, 15.805292,
                0.335946, 310.734497, 18.055292,
                0.335946, 339.142181, 4.555292,
                0.335946, 339.142181, 6.805292,
                0.335946, 339.142181, 9.055292,
                0.335946, 339.142181, 11.305292,
                0.335946, 339.142181, 13.555292,
                0.335946, 339.142181, 15.805292,
                0.335946, 339.142181, 18.055292,
                0.335946, 367.549866, 2.305292,
                0.335946, 367.549866, 20.305292,
                0.335946, 367.549866, 5.905292,
                0.335946, 367.549866, 9.505292,
                0.335946, 367.549866, 13.105291,
                0.335946, 367.549866, 16.705292,
                0.335946, -1.750129, 16.705292,
                0.335946, -1.750129, 13.105291,
                0.335946, -1.750129, 9.505292,
                0.335946, -1.750129, 5.905292,
                0.335946, 18.766537, 2.305292,
                0.335946, 39.283203, 2.305292,
                0.335946, 59.79987, 2.305292,
                0.335946, 80.316536, 2.305292,
                0.335946, 100.833206, 2.305292,
                0.335946, 121.349869, 2.305292,
                0.335946, 141.866531, 2.305292,
                0.335946, 162.383209, 2.305292,
                0.335946, 182.899872, 2.305292,
                0.335946, 203.416534, 2.305292,
                0.335946, 223.933197, 2.305292,
                0.335946, 244.449875, 2.305292,
                0.335946, 264.966553, 2.305292,
                0.335946, 285.483215, 2.305292,
                0.335946, 305.999878, 2.305292,
                0.335946, 326.516541, 2.305292,
                0.335946, 347.033203, 2.305292,
                0.335946, 347.033203, 20.305292,
                0.335946, 326.516541, 20.305292,
                0.335946, 305.999878, 20.305292,
                0.335946, 285.483215, 20.305292,
                0.335946, 264.966553, 20.305292,
                0.335946, 244.449875, 20.305292,
                0.335946, 223.933197, 20.305292,
                0.335946, 203.416534, 20.305292,
                0.335946, 182.899872, 20.305292,
                0.335946, 162.383209, 20.305292,
                0.335946, 141.866531, 20.305292,
                0.335946, 121.349869, 20.305292,
                0.335946, 100.833206, 20.305292,
                0.335946, 80.316536, 20.305292,
                0.335946, 59.79987, 20.305292,
                0.335946, 39.283203, 20.305292,
                0.335946, 18.766537, 20.305292,
                0.335946, 3.859701, 15.163343,
                0.335946, 3.759784, 15.168348,
                0.335946, 3.660867, 15.18331,
                0.335946, 3.567592, 15.206958,
                0.335946, 3.477022, 15.23946,
                0.335946, 3.388218, 15.281466,
                0.335946, 3.304151, 15.331856,
                0.335946, 3.225415, 15.390239,
                0.335946, 3.152599, 15.456224,
                0.335946, 3.08617, 15.529571,
                0.335946, 3.027716, 15.608526,
                0.335946, 2.97751, 15.692434,
                0.335946, 2.935823, 15.78064,
                0.335946, 2.902927, 15.872486,
                0.335946, 2.879093, 15.967319,
                0.335946, 2.864594, 16.064482,
                0.335946, 2.859701, 16.163319,
                0.335946, 2.864594, 16.262154,
                0.335946, 2.879093, 16.359318,
                0.335946, 2.902927, 16.454149,
                0.335946, 2.935823, 16.545996,
                0.335946, 2.97751, 16.634201,
                0.335946, 3.027716, 16.718109,
                0.335946, 3.08617, 16.797064,
                0.335946, 3.152599, 16.870411,
                0.335946, 3.225404, 16.936388,
                0.335946, 3.304136, 16.99477,
                0.335946, 3.388206, 17.045164,
                0.335946, 3.477022, 17.087175,
                0.335946, 3.56942, 17.120234,
                0.335946, 3.664613, 17.144077,
                0.335946, 3.761685, 17.158476,
                0.335946, 3.859701, 17.163292,
                0.335946, 3.959617, 17.158287,
                0.335946, 4.058534, 17.143326,
                0.335946, 4.151809, 17.119678,
                0.335946, 4.242379, 17.087175,
                0.335946, 4.331183, 17.04517,
                0.335946, 4.41525, 16.99478,
                0.335946, 4.493987, 16.936396,
                0.335946, 4.566802, 16.870411,
                0.335946, 4.633232, 16.797066,
                0.335946, 4.691685, 16.718109,
                0.335946, 4.741891, 16.634201,
                0.335946, 4.783578, 16.545996,
                0.335946, 4.816474, 16.454149,
                0.335946, 4.840308, 16.359318,
                0.335946, 4.854807, 16.262156,
                0.335946, 4.859701, 16.163319,
                0.335946, 4.854807, 16.064482,
                0.335946, 4.840308, 15.967319,
                0.335946, 4.816474, 15.872486,
                0.335946, 4.783578, 15.78064,
                0.335946, 4.741891, 15.692434,
                0.335946, 4.691685, 15.608526,
                0.335946, 4.633232, 15.529571,
                0.335946, 4.566802, 15.456224,
                0.335946, 4.493997, 15.390248,
                0.335946, 4.415265, 15.331866,
                0.335946, 4.331196, 15.281472,
                0.335946, 4.242379, 15.23946,
                0.335946, 4.149981, 15.206401,
                0.335946, 4.054788, 15.182557,
                0.335946, 3.957716, 15.168159,
                0.335946, 26.657564, 14.680292,
                0.335946, 12.453717, 15.805292,
                0.335946, 12.453717, 13.555292,
                0.335946, 12.453717, 14.680292,
                0.335946, 12.453717, 15.242792,
                0.335946, 5.351794, 15.805292,
                0.335946, 5.351794, 14.680292,
                0.335946, 5.351794, 15.242792,
                0.335946, 26.657564, 16.930292,
                0.335946, 12.453717, 18.055292,
                0.335946, 12.453717, 16.930292,
                0.335946, 5.351794, 14.961542,
                0.335946, 1.800833, 15.242792,
                0.335946, 1.800833, 14.680292,
                0.335946, 1.800833, 14.961542,
                0.335946, 12.453717, 14.117792,
                0.335946, 5.351794, 13.555292,
                0.335946, 5.351794, 14.117792,
                0.335946, 26.657564, 12.430292,
                0.335946, 12.453717, 11.305292,
                0.335946, 12.453717, 12.430292,
                0.335946, 5.351794, 15.102167,
                0.335946, 1.800833, 15.102167,
                0.335946, 3.576313, 14.961542,
                0.335946, 3.576313, 15.102167,
                0.335946, 12.453717, 14.961542,
                0.335946, 8.902756, 15.242792,
                0.335946, 8.902756, 14.680292,
                0.335946, 8.902756, 14.961542,
                0.335946, 26.657564, 15.242792,
                0.335946, 19.555639, 15.805292,
                0.335946, 19.555639, 14.680292,
                0.335946, 19.555639, 15.242792,
                0.335946, 55.065254, 14.680292,
                0.335946, 40.861408, 15.805292,
                0.335946, 40.861408, 13.555292,
                0.335946, 40.861408, 14.680292,
                0.335946, 5.351794, 15.524042,
                0.335946, 1.800833, 15.805292,
                0.335946, 1.800833, 15.524042,
                0.335946, 12.453717, 16.367792,
                0.335946, 5.351794, 16.930292,
                0.335946, 5.351794, 16.367792,
                0.335946, 5.351794, 15.17248,
                0.335946, 4.464054, 15.102167,
                0.335946, 4.464054, 15.17248,
                0.335946, 8.902756, 15.102167,
                0.335946, 7.127275, 15.242792,
                0.335946, 7.127275, 14.961542,
                0.335946, 7.127275, 15.102167,
                0.335946, 12.453717, 15.524042,
                0.335946, 8.902756, 15.805292,
                0.335946, 8.902756, 15.524042,
                0.335946, 5.351794, 15.383417,
                0.335946, 1.800833, 15.383417,
                0.335946, 2.688573, 15.242792,
                0.335946, 1.800833, 15.17248,
                0.335946, 2.688573, 15.102167,
                0.335946, 2.688573, 15.17248,
                0.335946, 0.025352, 15.242792,
                0.335946, 0.025352, 14.961542,
                0.335946, 0.025352, 15.102167,
                0.335946, 2.688573, 15.383417,
                0.335946, 1.800833, 15.313105,
                0.335946, 2.688573, 15.313105,
                0.335946, 0.025352, 15.524042,
                0.335946, 0.025352, 15.383417,
                0.335946, 2.688573, 15.524042,
                0.335946, 1.800833, 15.45373,
                0.335946, 2.688573, 15.45373,
                0.335946, 5.351794, 15.664667,
                0.335946, 1.800833, 15.664667,
                0.335946, 5.351794, 16.086542,
                0.335946, 1.800833, 16.367792,
                0.335946, 1.800833, 16.086542,
                0.335946, 2.688573, 15.664667,
                0.335946, 1.800833, 15.594355,
                0.335946, 2.688573, 15.594355,
                0.335946, 0.025352, 15.805292,
                0.335946, 0.025352, 15.664667,
                0.335946, 2.688573, 15.805292,
                0.335946, 1.800833, 15.73498,
                0.335946, 2.688573, 15.73498,
                0.335946, 5.351794, 15.945917,
                0.335946, 1.800833, 15.945917,
                0.335946, 12.453717, 16.086542,
                0.335946, 8.902756, 16.367792,
                0.335946, 8.902756, 16.086542,
                0.335946, 26.657564, 16.367792,
                0.335946, 19.555639, 16.930292,
                0.335946, 19.555639, 16.367792,
                0.335946, 55.065254, 16.930292,
                0.335946, 40.861408, 18.055292,
                0.335946, 40.861408, 16.930292,
                0.335946, 2.688573, 15.945917,
                0.335946, 1.800833, 15.875605,
                0.335946, 0.025352, 16.086542,
                0.335946, 0.025352, 15.945917,
                0.335946, 2.688573, 16.086542,
                0.335946, 1.800833, 16.01623,
                0.335946, 5.351794, 16.227167,
                0.335946, 1.800833, 16.227167,
                0.335946, 5.351794, 16.649042,
                0.335946, 1.800833, 16.930292,
                0.335946, 1.800833, 16.649042,
                0.335946, 12.453717, 17.492792,
                0.335946, 5.351794, 18.055292,
                0.335946, 5.351794, 17.492792,
                0.335946, 26.657564, 19.180292,
                0.335946, 12.453717, 19.180292,
                0.335946, 2.688573, 16.227167,
                0.335946, 1.800833, 16.156855,
                0.335946, 0.025352, 16.367792,
                0.335946, 0.025352, 16.227167,
                0.335946, 1.800833, 16.29748,
                0.335946, 2.688573, 16.29748,
                0.335946, 5.351794, 16.508417,
                0.335946, 1.800833, 16.508417,
                0.335946, 12.453717, 16.649042,
                0.335946, 8.902756, 16.930292,
                0.335946, 8.902756, 16.649042,
                0.335946, 2.688573, 16.508417,
                0.335946, 1.800833, 16.438105,
                0.335946, 0.025352, 16.649042,
                0.335946, 0.025352, 16.508417,
                0.335946, 2.688573, 16.649042,
                0.335946, 1.800833, 16.57873,
                0.335946, 2.688573, 16.57873,
                0.335946, 5.351794, 16.789667,
                0.335946, 1.800833, 16.789667,
                0.335946, 5.351794, 17.211542,
                0.335946, 1.800833, 17.492792,
                0.335946, 1.800833, 17.211542,
                0.335946, 2.688573, 16.789667,
                0.335946, 1.800833, 16.719355,
                0.335946, 2.688573, 16.719355,
                0.335946, 0.025352, 16.930292,
                0.335946, 0.025352, 16.789667,
                0.335946, 2.688573, 16.930292,
                0.335946, 1.800833, 16.85998,
                0.335946, 2.688573, 16.85998,
                0.335946, 5.351794, 17.070917,
                0.335946, 3.576313, 17.211542,
                0.335946, 1.800833, 17.070917,
                0.335946, 12.453717, 17.211542,
                0.335946, 8.902756, 17.492792,
                0.335946, 8.902756, 17.211542,
                0.335946, 26.657564, 17.492792,
                0.335946, 19.555639, 18.055292,
                0.335946, 19.555639, 17.492792,
                0.335946, 2.688573, 17.070917,
                0.335946, 1.800833, 17.000605,
                0.335946, 2.688573, 17.000605,
                0.335946, 0.025352, 17.211542,
                0.335946, 0.025352, 17.070917,
                0.335946, 2.688573, 17.211542,
                0.335946, 1.800833, 17.14123,
                0.335946, 2.688573, 17.14123,
                0.335946, 5.351794, 17.352167,
                0.335946, 3.576313, 17.492792,
                0.335946, 1.800833, 17.352167,
                0.335946, 3.576313, 17.352167,
                0.335946, 5.351794, 17.774042,
                0.335946, 1.800833, 18.055292,
                0.335946, 1.800833, 17.774042,
                0.335946, 12.453717, 18.617792,
                0.335946, 5.351794, 19.180292,
                0.335946, 5.351794, 18.617792,
                0.335946, 5.351794, 17.14123,
                0.335946, 4.464054, 17.211542,
                0.335946, 4.464054, 17.14123,
                0.335946, 8.902756, 17.070917,
                0.335946, 7.127275, 17.211542,
                0.335946, 7.127275, 16.930292,
                0.335946, 7.127275, 17.070917,
                0.335946, 5.351794, 17.000605,
                0.335946, 5.351794, 16.85998,
                0.335946, 8.902756, 16.789667,
                0.335946, 7.127275, 16.649042,
                0.335946, 7.127275, 16.789667,
                0.335946, 5.351794, 16.719355,
                0.335946, 5.351794, 16.57873,
                0.335946, 8.902756, 16.508417,
                0.335946, 7.127275, 16.367792,
                0.335946, 7.127275, 16.508417,
                0.335946, 5.351794, 16.438105,
                0.335946, 5.351794, 16.29748,
                0.335946, 8.902756, 16.227167,
                0.335946, 7.127275, 16.086542,
                0.335946, 7.127275, 16.227167,
                0.335946, 5.351794, 16.156855,
                0.335946, 5.351794, 16.01623,
                0.335946, 8.902756, 15.945917,
                0.335946, 7.127275, 15.805292,
                0.335946, 7.127275, 15.945917,
                0.335946, 5.351794, 15.875605,
                0.335946, 5.351794, 15.73498,
                0.335946, 8.902756, 15.664667,
                0.335946, 7.127275, 15.524042,
                0.335946, 7.127275, 15.664667,
                0.335946, 5.351794, 15.594355,
                0.335946, 5.351794, 15.45373,
                0.335946, 8.902756, 15.383417,
                0.335946, 7.127275, 15.383417,
                0.335946, 5.351794, 15.313105,
                0.335946, 3.859701, 8.763344,
                0.335946, 3.759784, 8.768348,
                0.335946, 3.660867, 8.78331,
                0.335946, 3.567592, 8.806957,
                0.335946, 3.477022, 8.83946,
                0.335946, 3.388218, 8.881466,
                0.335946, 3.304151, 8.931856,
                0.335946, 3.225415, 8.990239,
                0.335946, 3.152599, 9.056224,
                0.335946, 3.08617, 9.129571,
                0.335946, 3.027716, 9.208526,
                0.335946, 2.97751, 9.292434,
                0.335946, 2.935823, 9.380639,
                0.335946, 2.902927, 9.472486,
                0.335946, 2.879093, 9.567319,
                0.335946, 2.864594, 9.664481,
                0.335946, 2.859701, 9.763318,
                0.335946, 2.864594, 9.862154,
                0.335946, 2.879093, 9.959317,
                0.335946, 2.902927, 10.05415,
                0.335946, 2.935823, 10.145996,
                0.335946, 2.97751, 10.234201,
                0.335946, 3.027716, 10.31811,
                0.335946, 3.08617, 10.397065,
                0.335946, 3.152599, 10.470411,
                0.335946, 3.225404, 10.536387,
                0.335946, 3.304136, 10.594769,
                0.335946, 3.388206, 10.645164,
                0.335946, 3.477022, 10.687175,
                0.335946, 3.56942, 10.720235,
                0.335946, 3.664613, 10.744078,
                0.335946, 3.761685, 10.758477,
                0.335946, 3.859701, 10.763291,
                0.335946, 3.959617, 10.758287,
                0.335946, 4.058534, 10.743326,
                0.335946, 4.151809, 10.719678,
                0.335946, 4.242379, 10.687176,
                0.335946, 4.331183, 10.64517,
                0.335946, 4.41525, 10.59478,
                0.335946, 4.493987, 10.536396,
                0.335946, 4.566802, 10.470411,
                0.335946, 4.633232, 10.397065,
                0.335946, 4.691685, 10.31811,
                0.335946, 4.741891, 10.234201,
                0.335946, 4.783578, 10.145996,
                0.335946, 4.816474, 10.05415,
                0.335946, 4.840308, 9.959317,
                0.335946, 4.854807, 9.862155,
                0.335946, 4.859701, 9.763318,
                0.335946, 4.854807, 9.664481,
                0.335946, 4.840308, 9.567319,
                0.335946, 4.816474, 9.472486,
                0.335946, 4.783578, 9.38064,
                0.335946, 4.741891, 9.292435,
                0.335946, 4.691685, 9.208527,
                0.335946, 4.633232, 9.129571,
                0.335946, 4.566802, 9.056224,
                0.335946, 4.493997, 8.990248,
                0.335946, 4.415265, 8.931866,
                0.335946, 4.331196, 8.881472,
                0.335946, 4.242379, 8.83946,
                0.335946, 4.149981, 8.806401,
                0.335946, 4.054788, 8.782557,
                0.335946, 3.957716, 8.768159,
                0.335946, 26.657564, 7.930292,
                0.335946, 12.453717, 9.055292,
                0.335946, 12.453717, 6.805292,
                0.335946, 12.453717, 7.930292,
                0.335946, 12.453717, 8.492792,
                0.335946, 5.351794, 9.055292,
                0.335946, 5.351794, 7.930292,
                0.335946, 5.351794, 8.492792,
                0.335946, 26.657564, 10.180292,
                0.335946, 12.453717, 10.180292,
                0.335946, 5.351794, 8.774042,
                0.335946, 1.800833, 9.055292,
                0.335946, 1.800833, 8.492792,
                0.335946, 1.800833, 8.774042,
                0.335946, 12.453717, 9.617792,
                0.335946, 5.351794, 10.180292,
                0.335946, 5.351794, 9.617792,
                0.335946, 5.351794, 8.633417,
                0.335946, 1.800833, 8.633417,
                0.335946, 3.576313, 8.492792,
                0.335946, 3.576313, 8.633417,
                0.335946, 5.351794, 8.211542,
                0.335946, 1.800833, 7.930292,
                0.335946, 1.800833, 8.211542,
                0.335946, 12.453717, 7.367792,
                0.335946, 5.351794, 6.805292,
                0.335946, 5.351794, 7.367792,
                0.335946, 26.657564, 5.680292,
                0.335946, 12.453717, 4.555292,
                0.335946, 12.453717, 5.680292,
                0.335946, 12.453717, 8.774042,
                0.335946, 8.902756, 9.055292,
                0.335946, 8.902756, 8.492792,
                0.335946, 8.902756, 8.774042,
                0.335946, 26.657564, 8.492792,
                0.335946, 19.555639, 9.055292,
                0.335946, 19.555639, 7.930292,
                0.335946, 19.555639, 8.492792,
                0.335946, 55.065254, 7.930292,
                0.335946, 40.861408, 9.055292,
                0.335946, 40.861408, 6.805292,
                0.335946, 40.861408, 7.930292,
                0.335946, 5.351794, 8.70373,
                0.335946, 4.464054, 8.774042,
                0.335946, 3.576313, 8.70373,
                0.335946, 4.464054, 8.633417,
                0.335946, 4.464054, 8.70373,
                0.335946, 8.902756, 8.633417,
                0.335946, 7.127275, 8.774042,
                0.335946, 7.127275, 8.492792,
                0.335946, 7.127275, 8.633417,
                0.335946, 12.453717, 8.211542,
                0.335946, 8.902756, 7.930292,
                0.335946, 8.902756, 8.211542,
                0.335946, 5.351794, 8.914667,
                0.335946, 1.800833, 8.914667,
                0.335946, 5.351794, 9.336542,
                0.335946, 1.800833, 9.617792,
                0.335946, 1.800833, 9.336542,
                0.335946, 5.351794, 8.844355,
                0.335946, 8.902756, 8.914667,
                0.335946, 7.127275, 9.055292,
                0.335946, 7.127275, 8.914667,
                0.335946, 12.453717, 9.336542,
                0.335946, 8.902756, 9.617792,
                0.335946, 8.902756, 9.336542,
                0.335946, 26.657564, 9.617792,
                0.335946, 19.555639, 10.180292,
                0.335946, 19.555639, 9.617792,
                0.335946, 55.065254, 10.180292,
                0.335946, 40.861408, 11.305292,
                0.335946, 40.861408, 10.180292,
                0.335946, 2.688573, 8.914667,
                0.335946, 1.800833, 8.844355,
                0.335946, 2.688573, 8.774042,
                0.335946, 2.688573, 8.844355,
                0.335946, 0.025352, 9.055292,
                0.335946, 0.025352, 8.774042,
                0.335946, 0.025352, 8.914667,
                0.335946, 2.688573, 9.055292,
                0.335946, 1.800833, 8.98498,
                0.335946, 2.688573, 8.98498,
                0.335946, 5.351794, 9.195917,
                0.335946, 1.800833, 9.195917,
                0.335946, 2.688573, 9.195917,
                0.335946, 1.800833, 9.125605,
                0.335946, 2.688573, 9.125605,
                0.335946, 0.025352, 9.336542,
                0.335946, 0.025352, 9.195917,
                0.335946, 2.688573, 9.336542,
                0.335946, 1.800833, 9.26623,
                0.335946, 2.688573, 9.26623,
                0.335946, 5.351794, 9.477167,
                0.335946, 1.800833, 9.477167,
                0.335946, 5.351794, 9.899042,
                0.335946, 1.800833, 10.180292,
                0.335946, 1.800833, 9.899042,
                0.335946, 12.453717, 10.742792,
                0.335946, 5.351794, 11.305292,
                0.335946, 5.351794, 10.742792,
                0.335946, 1.800833, 9.406855,
                0.335946, 2.688573, 9.406855,
                0.335946, 0.025352, 9.617792,
                0.335946, 0.025352, 9.477167,
                0.335946, 1.800833, 9.54748,
                0.335946, 2.688573, 9.54748,
                0.335946, 5.351794, 9.758417,
                0.335946, 1.800833, 9.758417,
                0.335946, 12.453717, 9.899042,
                0.335946, 8.902756, 10.180292,
                0.335946, 8.902756, 9.899042,
                0.335946, 1.800833, 9.688105,
                0.335946, 2.688573, 9.688105,
                0.335946, 0.025352, 9.899042,
                0.335946, 0.025352, 9.758417,
                0.335946, 2.688573, 9.899042,
                0.335946, 1.800833, 9.82873,
                0.335946, 2.688573, 9.82873,
                0.335946, 5.351794, 10.039667,
                0.335946, 1.800833, 10.039667,
                0.335946, 5.351794, 10.461542,
                0.335946, 1.800833, 10.742792,
                0.335946, 1.800833, 10.461542,
                0.335946, 1.800833, 9.969355,
                0.335946, 0.025352, 10.180292,
                0.335946, 0.025352, 10.039667,
                0.335946, 2.688573, 10.180292,
                0.335946, 1.800833, 10.10998,
                0.335946, 2.688573, 10.10998,
                0.335946, 5.351794, 10.320917,
                0.335946, 1.800833, 10.320917,
                0.335946, 12.453717, 10.461542,
                0.335946, 8.902756, 10.742792,
                0.335946, 8.902756, 10.461542,
                0.335946, 26.657564, 10.742792,
                0.335946, 19.555639, 11.305292,
                0.335946, 19.555639, 10.742792,
                0.335946, 2.688573, 10.320917,
                0.335946, 1.800833, 10.250605,
                0.335946, 2.688573, 10.250605,
                0.335946, 0.025352, 10.461542,
                0.335946, 0.025352, 10.320917,
                0.335946, 2.688573, 10.461542,
                0.335946, 1.800833, 10.39123,
                0.335946, 2.688573, 10.39123,
                0.335946, 5.351794, 10.602167,
                0.335946, 1.800833, 10.602167,
                0.335946, 5.351794, 11.024042,
                0.335946, 1.800833, 11.305292,
                0.335946, 1.800833, 11.024042,
                0.335946, 12.453717, 11.867792,
                0.335946, 5.351794, 12.430292,
                0.335946, 5.351794, 11.867792,
                0.335946, 2.688573, 10.602167,
                0.335946, 1.800833, 10.531855,
                0.335946, 2.688573, 10.531855,
                0.335946, 0.025352, 10.742792,
                0.335946, 0.025352, 10.602167,
                0.335946, 2.688573, 10.742792,
                0.335946, 1.800833, 10.67248,
                0.335946, 2.688573, 10.67248,
                0.335946, 5.351794, 10.883417,
                0.335946, 3.576313, 11.024042,
                0.335946, 1.800833, 10.883417,
                0.335946, 3.576313, 10.883417,
                0.335946, 12.453717, 11.024042,
                0.335946, 8.902756, 11.305292,
                0.335946, 8.902756, 11.024042,
                0.335946, 5.351794, 10.813105,
                0.335946, 4.464054, 10.883417,
                0.335946, 3.576313, 10.813105,
                0.335946, 4.464054, 10.742792,
                0.335946, 4.464054, 10.813105,
                0.335946, 8.902756, 10.883417,
                0.335946, 7.127275, 11.024042,
                0.335946, 7.127275, 10.742792,
                0.335946, 7.127275, 10.883417,
                0.335946, 5.351794, 10.67248,
                0.335946, 8.902756, 10.602167,
                0.335946, 7.127275, 10.461542,
                0.335946, 7.127275, 10.602167,
                0.335946, 5.351794, 10.531855,
                0.335946, 5.351794, 10.39123,
                0.335946, 8.902756, 10.320917,
                0.335946, 7.127275, 10.180292,
                0.335946, 7.127275, 10.320917,
                0.335946, 5.351794, 10.250605,
                0.335946, 5.351794, 10.10998,
                0.335946, 8.902756, 10.039667,
                0.335946, 7.127275, 9.899042,
                0.335946, 7.127275, 10.039667,
                0.335946, 5.351794, 9.969355,
                0.335946, 5.351794, 9.82873,
                0.335946, 8.902756, 9.758417,
                0.335946, 7.127275, 9.617792,
                0.335946, 7.127275, 9.758417,
                0.335946, 5.351794, 9.688105,
                0.335946, 5.351794, 9.54748,
                0.335946, 8.902756, 9.477167,
                0.335946, 7.127275, 9.336542,
                0.335946, 7.127275, 9.477167,
                0.335946, 5.351794, 9.406855,
                0.335946, 5.351794, 9.26623,
                0.335946, 8.902756, 9.195917,
                0.335946, 7.127275, 9.195917,
                0.335946, 5.351794, 9.125605,
                0.335946, 5.351794, 8.98498,
                0.335946, -1.750129, 20.305292,
                3.435946, 72.109871, 20.305292,
                6.535946, 72.109871, 20.305292,
                3.435946, 145.969864, 20.305292,
                6.535946, 145.969864, 20.305292,
                3.435946, 219.829865, 20.305292,
                6.535946, 219.829865, 20.305292,
                3.435946, 293.68988, 20.305292,
                6.535946, 293.68988, 20.305292,
                0.335946, 367.549866, 20.305292,
                3.435946, 367.549866, 20.305292,
                6.535946, 367.549866, 20.305292,
                9.635946, 367.549866, 20.305292,
                3.435946, 330.619873, 20.305292,
                1.885946, 293.68988, 20.305292,
                1.885946, 330.619873, 20.305292,
                6.535946, 330.619873, 20.305292,
                4.985946, 293.68988, 20.305292,
                4.985946, 330.619873, 20.305292,
                8.085946, 293.68988, 20.305292,
                8.085946, 330.619873, 20.305292,
                0.335946, 18.766537, 20.305292,
                0.335946, 39.283203, 20.305292,
                0.335946, 59.79987, 20.305292,
                0.335946, 80.316536, 20.305292,
                0.335946, 100.833206, 20.305292,
                0.335946, 121.349869, 20.305292,
                0.335946, 141.866531, 20.305292,
                0.335946, 162.383209, 20.305292,
                0.335946, 182.899872, 20.305292,
                0.335946, 203.416534, 20.305292,
                0.335946, 223.933197, 20.305292,
                0.335946, 244.449875, 20.305292,
                0.335946, 264.966553, 20.305292,
                0.335946, 285.483215, 20.305292,
                0.335946, 305.999878, 20.305292,
                0.335946, 326.516541, 20.305292,
                0.335946, 347.033203, 20.305292,
                0.764909, 13.907396, 20.305292,
                0.764909, 11.297809, 20.305292,
                0.764909, 8.688221, 20.305292,
                0.764909, 6.078633, 20.305292,
                0.764909, 3.469046, 20.305292,
                0.764909, 0.859458, 20.305292,
                0.764909, -1.750129, 20.305292,
                1.885946, 72.109871, 20.305292,
                3.435946, 35.179871, 20.305292,
                1.885946, 35.179871, 20.305292,
                1.110946, 35.179871, 20.305292,
                1.885946, 16.71487, 20.305292,
                1.110946, 16.71487, 20.305292,
                9.635946, 13.907396, 20.305292,
                8.15744, 13.907396, 20.305292,
                6.678934, 13.907396, 20.305292,
                5.200428, 13.907396, 20.305292,
                3.721922, 13.907396, 20.305292,
                2.243416, 13.907396, 20.305292,
                8.085946, 72.109871, 20.305292,
                6.535946, 35.179871, 20.305292,
                8.085946, 35.179871, 20.305292,
                8.860947, 35.179871, 20.305292,
                8.860947, 16.71487, 20.305292,
                7.310946, 35.179871, 20.305292,
                7.310946, 16.71487, 20.305292,
                4.985946, 72.109871, 20.305292,
                4.985946, 35.179871, 20.305292,
                5.760946, 35.179871, 20.305292,
                4.985946, 16.71487, 20.305292,
                5.760946, 16.71487, 20.305292,
                4.210946, 35.179871, 20.305292,
                3.435946, 16.71487, 20.305292,
                4.210946, 16.71487, 20.305292,
                2.660946, 35.179871, 20.305292,
                2.660946, 16.71487, 20.305292,
                1.498446, 16.71487, 20.305292,
                9.635946, 338.079651, 20.305292,
                9.635946, 308.609467, 20.305292,
                9.635946, 279.139252, 20.305292,
                9.635946, 249.669052, 20.305292,
                9.635946, 220.198837, 20.305292,
                9.635946, 190.728638, 20.305292,
                9.635946, 161.258423, 20.305292,
                9.635946, 131.788223, 20.305292,
                9.635946, 102.318016, 20.305292,
                9.635946, 72.847809, 20.305292,
                9.635946, 43.377602, 20.305292,
                9.635946, 13.907396, 22.305292,
                9.635946, 53.201004, 21.305292,
                9.635946, 131.788223, 22.305292,
                9.635946, 328.256256, 21.305292,
                9.635946, 367.549866, 20.305292,
                9.635946, 367.549866, 22.305292,
                9.635946, 347.903076, 21.305292,
                9.635946, 328.256256, 20.805292,
                9.635946, 347.903076, 20.805292,
                9.635946, 328.256256, 21.805292,
                9.635946, 347.903076, 21.805292,
                9.635946, 13.907396, 21.305292,
                9.635946, 53.201004, 21.805292,
                9.635946, 33.554199, 21.305292,
                9.635946, 33.554199, 21.805292,
                9.635946, 33.554199, 22.055292,
                9.635946, 23.730799, 21.805292,
                9.635946, 23.730799, 22.055292,
                9.635946, 33.554199, 21.555292,
                9.635946, 23.730799, 21.305292,
                9.635946, 23.730799, 21.555292,
                9.635946, 53.201004, 20.805292,
                9.635946, 33.554199, 20.805292,
                9.635946, 33.554199, 20.555292,
                9.635946, 23.730799, 20.805292,
                9.635946, 23.730799, 20.555292,
                9.635946, 13.907396, 20.305292,
                9.635946, 43.377602, 20.305292,
                9.635946, 72.847809, 20.305292,
                9.635946, 102.318016, 20.305292,
                9.635946, 131.788223, 20.305292,
                9.635946, 161.258423, 20.305292,
                9.635946, 190.728638, 20.305292,
                9.635946, 220.198837, 20.305292,
                9.635946, 249.669052, 20.305292,
                9.635946, 279.139252, 20.305292,
                9.635946, 308.609467, 20.305292,
                9.635946, 338.079651, 20.305292,
                9.635946, 338.079651, 22.305292,
                9.635946, 308.609467, 22.305292,
                9.635946, 279.139252, 22.305292,
                9.635946, 249.669052, 22.305292,
                9.635946, 220.198837, 22.305292,
                9.635946, 190.728638, 22.305292,
                9.635946, 161.258423, 22.305292,
                9.635946, 102.318016, 22.305292,
                9.635946, 72.847809, 22.305292,
                9.635946, 43.377602, 22.305292,
                9.635946, 131.788223, 21.305292,
                9.635946, 171.081833, 21.305292,
                9.635946, 210.375443, 21.305292,
                9.635946, 249.669052, 21.305292,
                9.635946, 288.962646, 21.305292,
                -10.364054, -1.750129, 22.305292,
                4.635946, 90.574867, 22.305292,
                -0.364054, 90.574867, 22.305292,
                -5.364054, 90.574867, 22.305292,
                4.635946, 182.899872, 22.305292,
                -0.364054, 182.899872, 22.305292,
                -5.364054, 182.899872, 22.305292,
                -10.364054, 182.899872, 22.305292,
                4.635946, 275.224884, 22.305292,
                -0.364054, 275.224884, 22.305292,
                -5.364054, 275.224884, 22.305292,
                9.635946, 367.549866, 22.305292,
                -10.364054, 367.549866, 22.305292,
                5.635946, 367.549866, 22.305292,
                1.635946, 367.549866, 22.305292,
                -2.364054, 367.549866, 22.305292,
                -6.364054, 367.549866, 22.305292,
                4.635946, 321.38736, 22.305292,
                7.135946, 275.224884, 22.305292,
                7.135946, 321.38736, 22.305292,
                -0.364054, 321.38736, 22.305292,
                2.135946, 275.224884, 22.305292,
                2.135946, 321.38736, 22.305292,
                -5.364054, 321.38736, 22.305292,
                -2.864054, 275.224884, 22.305292,
                -2.864054, 321.38736, 22.305292,
                -7.864054, 275.224884, 22.305292,
                -7.864054, 321.38736, 22.305292,
                -6.654399, -1.750129, 22.305292,
                -2.944745, -1.750129, 22.305292,
                0.764909, -1.750129, 22.305292,
                -7.864054, 90.574867, 22.305292,
                -5.364054, 44.412373, 22.305292,
                -7.864054, 44.412373, 22.305292,
                -2.864054, 90.574867, 22.305292,
                -0.364054, 44.412373, 22.305292,
                -2.864054, 44.412373, 22.305292,
                2.135946, 90.574867, 22.305292,
                4.635946, 44.412373, 22.305292,
                2.135946, 44.412373, 22.305292,
                -10.364054, 341.171295, 22.305292,
                -10.364054, 314.792725, 22.305292,
                -10.364054, 288.414154, 22.305292,
                -10.364054, 262.035583, 22.305292,
                -10.364054, 235.657013, 22.305292,
                -10.364054, 209.278442, 22.305292,
                -10.364054, 156.521301, 22.305292,
                -10.364054, 130.142731, 22.305292,
                -10.364054, 103.76416, 22.305292,
                -10.364054, 77.385582, 22.305292,
                -10.364054, 51.007015, 22.305292,
                -10.364054, 24.628443, 22.305292,
                0.764909, 0.859458, 22.305292,
                0.764909, 3.469046, 22.305292,
                0.764909, 6.078633, 22.305292,
                0.764909, 8.688221, 22.305292,
                0.764909, 11.297809, 22.305292,
                0.764909, 13.907396, 22.305292,
                0.885946, 44.412373, 22.305292,
                -0.364054, 21.331121, 22.305292,
                2.135946, 21.331121, 22.305292,
                0.885946, 21.331121, 22.305292,
                2.243416, 13.907396, 22.305292,
                3.721922, 13.907396, 22.305292,
                5.200428, 13.907396, 22.305292,
                6.678934, 13.907396, 22.305292,
                8.15744, 13.907396, 22.305292,
                9.635946, 13.907396, 22.305292,
                0.260946, 21.331121, 22.305292,
                -0.364054, 9.790496, 22.305292,
                0.260946, 9.790496, 22.305292,
                -1.614054, 44.412373, 22.305292,
                -2.864054, 21.331121, 22.305292,
                -1.614054, 21.331121, 22.305292,
                3.385946, 44.412373, 22.305292,
                4.635946, 21.331121, 22.305292,
                3.385946, 21.331121, 22.305292,
                7.135946, 90.574867, 22.305292,
                7.135946, 44.412373, 22.305292,
                2.760946, 21.331121, 22.305292,
                4.010946, 21.331121, 22.305292,
                5.885946, 44.412373, 22.305292,
                7.135946, 21.331121, 22.305292,
                5.885946, 21.331121, 22.305292,
                5.260946, 21.331121, 22.305292,
                6.510946, 21.331121, 22.305292,
                8.385946, 44.412373, 22.305292,
                8.385946, 21.331121, 22.305292,
                7.760946, 21.331121, 22.305292,
                9.635946, 43.377602, 22.305292,
                9.635946, 72.847809, 22.305292,
                9.635946, 102.318016, 22.305292,
                9.635946, 131.788223, 22.305292,
                9.635946, 161.258423, 22.305292,
                9.635946, 190.728638, 22.305292,
                9.635946, 220.198837, 22.305292,
                9.635946, 249.669052, 22.305292,
                9.635946, 279.139252, 22.305292,
                9.635946, 308.609467, 22.305292,
                9.635946, 338.079651, 22.305292,
                -10.364054, -1.750129, 22.305292,
                -10.364054, -1.750129, 20.305292,
                -10.364054, 35.179871, 21.305292,
                -10.364054, 182.899872, 22.305292,
                -10.364054, 182.899872, 20.305292,
                -10.364054, 330.619873, 21.305292,
                -10.364054, 367.549866, 22.305292,
                -10.364054, 367.549866, 20.305292,
                -10.364054, 349.084869, 21.305292,
                -10.364054, 330.619873, 21.805292,
                -10.364054, 349.084869, 21.805292,
                -10.364054, 330.619873, 20.805292,
                -10.364054, 349.084869, 20.805292,
                -10.364054, 35.179871, 20.805292,
                -10.364054, 16.71487, 21.305292,
                -10.364054, 16.71487, 20.805292,
                -10.364054, 35.179871, 21.805292,
                -10.364054, 16.71487, 21.805292,
                -10.364054, 24.628443, 22.305292,
                -10.364054, 51.007015, 22.305292,
                -10.364054, 77.385582, 22.305292,
                -10.364054, 103.76416, 22.305292,
                -10.364054, 130.142731, 22.305292,
                -10.364054, 156.521301, 22.305292,
                -10.364054, 209.278442, 22.305292,
                -10.364054, 235.657013, 22.305292,
                -10.364054, 262.035583, 22.305292,
                -10.364054, 288.414154, 22.305292,
                -10.364054, 314.792725, 22.305292,
                -10.364054, 341.171295, 22.305292,
                -10.364054, 341.171295, 20.305292,
                -10.364054, 314.792725, 20.305292,
                -10.364054, 288.414154, 20.305292,
                -10.364054, 262.035583, 20.305292,
                -10.364054, 235.657013, 20.305292,
                -10.364054, 209.278442, 20.305292,
                -10.364054, 156.521301, 20.305292,
                -10.364054, 130.142731, 20.305292,
                -10.364054, 103.76416, 20.305292,
                -10.364054, 77.385582, 20.305292,
                -10.364054, 51.007015, 20.305292,
                -10.364054, 24.628443, 20.305292,
                -10.364054, 109.039871, 21.305292,
                -10.364054, 145.969864, 21.305292,
                -10.364054, 182.899872, 21.305292,
                -10.364054, 219.829865, 21.305292,
                -10.364054, 256.759857, 21.305292,
                -10.364054, 293.68988, 21.305292,
                -10.364054, -1.750129, 20.305292,
                -7.264054, -1.750129, 20.305292,
                -4.164054, -1.750129, 20.305292,
                -1.064054, -1.750129, 20.305292,
                -7.264054, 72.109871, 20.305292,
                -4.164054, 72.109871, 20.305292,
                -7.264054, 145.969864, 20.305292,
                -4.164054, 145.969864, 20.305292,
                -7.264054, 219.829865, 20.305292,
                -4.164054, 219.829865, 20.305292,
                -7.264054, 293.68988, 20.305292,
                -4.164054, 293.68988, 20.305292,
                -10.364054, 367.549866, 20.305292,
                -7.264054, 367.549866, 20.305292,
                -4.164054, 367.549866, 20.305292,
                -1.064054, 367.549866, 20.305292,
                -7.264054, 330.619873, 20.305292,
                -8.814054, 293.68988, 20.305292,
                -8.814054, 330.619873, 20.305292,
                -4.164054, 330.619873, 20.305292,
                -5.714054, 293.68988, 20.305292,
                -5.714054, 330.619873, 20.305292,
                -2.614054, 293.68988, 20.305292,
                -2.614054, 330.619873, 20.305292,
                -2.614054, 72.109871, 20.305292,
                -4.164054, 35.179871, 20.305292,
                -2.614054, 35.179871, 20.305292,
                -5.714054, 72.109871, 20.305292,
                -7.264054, 35.179871, 20.305292,
                -5.714054, 35.179871, 20.305292,
                -8.814054, 72.109871, 20.305292,
                -8.814054, 35.179871, 20.305292,
                -10.364054, 24.628443, 20.305292,
                -10.364054, 51.007015, 20.305292,
                -10.364054, 77.385582, 20.305292,
                -10.364054, 103.76416, 20.305292,
                -10.364054, 130.142731, 20.305292,
                -10.364054, 156.521301, 20.305292,
                -10.364054, 182.899872, 20.305292,
                -10.364054, 209.278442, 20.305292,
                -10.364054, 235.657013, 20.305292,
                -10.364054, 262.035583, 20.305292,
                -10.364054, 288.414154, 20.305292,
                -10.364054, 314.792725, 20.305292,
                -10.364054, 341.171295, 20.305292,
                -1.064054, 347.033203, 20.305292,
                -1.064054, 326.516541, 20.305292,
                -1.064054, 305.999878, 20.305292,
                -1.064054, 285.483215, 20.305292,
                -1.064054, 264.966553, 20.305292,
                -1.064054, 244.449875, 20.305292,
                -1.064054, 223.933197, 20.305292,
                -1.064054, 203.416534, 20.305292,
                -1.064054, 182.899872, 20.305292,
                -1.064054, 162.383209, 20.305292,
                -1.064054, 141.866531, 20.305292,
                -1.064054, 121.349869, 20.305292,
                -1.064054, 100.833206, 20.305292,
                -1.064054, 80.316536, 20.305292,
                -1.064054, 59.79987, 20.305292,
                -1.064054, 39.283203, 20.305292,
                -1.064054, 18.766537, 20.305292,
                -1.064054, -1.750129, 20.305292,
                -1.064054, -1.750129, 2.305292,
                -1.064054, 26.657564, 18.055292,
                -1.064054, 26.657564, 15.805292,
                -1.064054, 26.657564, 13.555292,
                -1.064054, 26.657564, 11.305292,
                -1.064054, 26.657564, 9.055292,
                -1.064054, 26.657564, 6.805292,
                -1.064054, 26.657564, 4.555292,
                -1.064054, 55.065254, 18.055292,
                -1.064054, 55.065254, 15.805292,
                -1.064054, 55.065254, 13.555292,
                -1.064054, 55.065254, 11.305292,
                -1.064054, 55.065254, 9.055292,
                -1.064054, 55.065254, 6.805292,
                -1.064054, 55.065254, 4.555292,
                -1.064054, 83.472946, 18.055292,
                -1.064054, 83.472946, 15.805292,
                -1.064054, 83.472946, 13.555292,
                -1.064054, 83.472946, 11.305292,
                -1.064054, 83.472946, 9.055292,
                -1.064054, 83.472946, 6.805292,
                -1.064054, 83.472946, 4.555292,
                -1.064054, 111.880638, 18.055292,
                -1.064054, 111.880638, 15.805292,
                -1.064054, 111.880638, 13.555292,
                -1.064054, 111.880638, 11.305292,
                -1.064054, 111.880638, 9.055292,
                -1.064054, 111.880638, 6.805292,
                -1.064054, 111.880638, 4.555292,
                -1.064054, 140.28833, 18.055292,
                -1.064054, 140.28833, 15.805292,
                -1.064054, 140.28833, 13.555292,
                -1.064054, 140.28833, 11.305292,
                -1.064054, 140.28833, 9.055292,
                -1.064054, 140.28833, 6.805292,
                -1.064054, 140.28833, 4.555292,
                -1.064054, 168.69603, 18.055292,
                -1.064054, 168.69603, 15.805292,
                -1.064054, 168.69603, 13.555292,
                -1.064054, 168.69603, 11.305292,
                -1.064054, 168.69603, 9.055292,
                -1.064054, 168.69603, 6.805292,
                -1.064054, 168.69603, 4.555292,
                -1.064054, 197.103714, 18.055292,
                -1.064054, 197.103714, 15.805292,
                -1.064054, 197.103714, 13.555292,
                -1.064054, 197.103714, 11.305292,
                -1.064054, 197.103714, 9.055292,
                -1.064054, 197.103714, 6.805292,
                -1.064054, 197.103714, 4.555292,
                -1.064054, 225.511414, 18.055292,
                -1.064054, 225.511414, 15.805292,
                -1.064054, 225.511414, 13.555292,
                -1.064054, 225.511414, 11.305292,
                -1.064054, 225.511414, 9.055292,
                -1.064054, 225.511414, 6.805292,
                -1.064054, 225.511414, 4.555292,
                -1.064054, 253.919098, 18.055292,
                -1.064054, 253.919098, 15.805292,
                -1.064054, 253.919098, 13.555292,
                -1.064054, 253.919098, 11.305292,
                -1.064054, 253.919098, 9.055292,
                -1.064054, 253.919098, 6.805292,
                -1.064054, 253.919098, 4.555292,
                -1.064054, 282.326782, 18.055292,
                -1.064054, 282.326782, 15.805292,
                -1.064054, 282.326782, 13.555292,
                -1.064054, 282.326782, 11.305292,
                -1.064054, 282.326782, 9.055292,
                -1.064054, 282.326782, 6.805292,
                -1.064054, 282.326782, 4.555292,
                -1.064054, 310.734497, 18.055292,
                -1.064054, 310.734497, 15.805292,
                -1.064054, 310.734497, 13.555292,
                -1.064054, 310.734497, 11.305292,
                -1.064054, 310.734497, 9.055292,
                -1.064054, 310.734497, 6.805292,
                -1.064054, 310.734497, 4.555292,
                -1.064054, 339.142181, 18.055292,
                -1.064054, 339.142181, 15.805292,
                -1.064054, 339.142181, 13.555292,
                -1.064054, 339.142181, 11.305292,
                -1.064054, 339.142181, 9.055292,
                -1.064054, 339.142181, 6.805292,
                -1.064054, 339.142181, 4.555292,
                -1.064054, 367.549866, 20.305292,
                -1.064054, 367.549866, 2.305292,
                -1.064054, 367.549866, 16.705292,
                -1.064054, 367.549866, 13.105291,
                -1.064054, 367.549866, 9.505292,
                -1.064054, 367.549866, 5.905292,
                -1.064054, -1.750129, 5.905292,
                -1.064054, -1.750129, 9.505292,
                -1.064054, -1.750129, 13.105291,
                -1.064054, -1.750129, 16.705292,
                -1.064054, 18.766537, 20.305292,
                -1.064054, 39.283203, 20.305292,
                -1.064054, 59.79987, 20.305292,
                -1.064054, 80.316536, 20.305292,
                -1.064054, 100.833206, 20.305292,
                -1.064054, 121.349869, 20.305292,
                -1.064054, 141.866531, 20.305292,
                -1.064054, 162.383209, 20.305292,
                -1.064054, 182.899872, 20.305292,
                -1.064054, 203.416534, 20.305292,
                -1.064054, 223.933197, 20.305292,
                -1.064054, 244.449875, 20.305292,
                -1.064054, 264.966553, 20.305292,
                -1.064054, 285.483215, 20.305292,
                -1.064054, 305.999878, 20.305292,
                -1.064054, 326.516541, 20.305292,
                -1.064054, 347.033203, 20.305292,
                -1.064054, 347.033203, 2.305292,
                -1.064054, 326.516541, 2.305292,
                -1.064054, 305.999878, 2.305292,
                -1.064054, 285.483215, 2.305292,
                -1.064054, 264.966553, 2.305292,
                -1.064054, 244.449875, 2.305292,
                -1.064054, 223.933197, 2.305292,
                -1.064054, 203.416534, 2.305292,
                -1.064054, 182.899872, 2.305292,
                -1.064054, 162.383209, 2.305292,
                -1.064054, 141.866531, 2.305292,
                -1.064054, 121.349869, 2.305292,
                -1.064054, 100.833206, 2.305292,
                -1.064054, 80.316536, 2.305292,
                -1.064054, 59.79987, 2.305292,
                -1.064054, 39.283203, 2.305292,
                -1.064054, 18.766537, 2.305292,
                -1.064054, 3.859701, 15.163343,
                -1.064054, 3.957716, 15.168159,
                -1.064054, 4.054788, 15.182557,
                -1.064054, 4.149981, 15.206401,
                -1.064054, 4.242379, 15.23946,
                -1.064054, 4.331196, 15.281472,
                -1.064054, 4.415265, 15.331866,
                -1.064054, 4.493997, 15.390248,
                -1.064054, 4.566802, 15.456224,
                -1.064054, 4.633232, 15.529571,
                -1.064054, 4.691685, 15.608526,
                -1.064054, 4.741891, 15.692434,
                -1.064054, 4.783578, 15.78064,
                -1.064054, 4.816474, 15.872486,
                -1.064054, 4.840308, 15.967319,
                -1.064054, 4.854807, 16.064482,
                -1.064054, 4.859701, 16.163319,
                -1.064054, 4.854807, 16.262156,
                -1.064054, 4.840308, 16.359318,
                -1.064054, 4.816474, 16.454149,
                -1.064054, 4.783578, 16.545996,
                -1.064054, 4.741891, 16.634201,
                -1.064054, 4.691685, 16.718109,
                -1.064054, 4.633232, 16.797066,
                -1.064054, 4.566802, 16.870411,
                -1.064054, 4.493987, 16.936396,
                -1.064054, 4.41525, 16.99478,
                -1.064054, 4.331183, 17.04517,
                -1.064054, 4.242379, 17.087175,
                -1.064054, 4.151809, 17.119678,
                -1.064054, 4.058534, 17.143326,
                -1.064054, 3.959617, 17.158287,
                -1.064054, 3.859701, 17.163292,
                -1.064054, 3.761685, 17.158476,
                -1.064054, 3.664613, 17.144077,
                -1.064054, 3.56942, 17.120234,
                -1.064054, 3.477022, 17.087175,
                -1.064054, 3.388206, 17.045164,
                -1.064054, 3.304136, 16.99477,
                -1.064054, 3.225404, 16.936388,
                -1.064054, 3.152599, 16.870411,
                -1.064054, 3.08617, 16.797064,
                -1.064054, 3.027716, 16.718109,
                -1.064054, 2.97751, 16.634201,
                -1.064054, 2.935823, 16.545996,
                -1.064054, 2.902927, 16.454149,
                -1.064054, 2.879093, 16.359318,
                -1.064054, 2.864594, 16.262154,
                -1.064054, 2.859701, 16.163319,
                -1.064054, 2.864594, 16.064482,
                -1.064054, 2.879093, 15.967319,
                -1.064054, 2.902927, 15.872486,
                -1.064054, 2.935823, 15.78064,
                -1.064054, 2.97751, 15.692434,
                -1.064054, 3.027716, 15.608526,
                -1.064054, 3.08617, 15.529571,
                -1.064054, 3.152599, 15.456224,
                -1.064054, 3.225415, 15.390239,
                -1.064054, 3.304151, 15.331856,
                -1.064054, 3.388218, 15.281466,
                -1.064054, 3.477022, 15.23946,
                -1.064054, 3.567592, 15.206958,
                -1.064054, 3.660867, 15.18331,
                -1.064054, 3.759784, 15.168348,
                -1.064054, 26.657564, 14.680292,
                -1.064054, 12.453717, 13.555292,
                -1.064054, 12.453717, 15.805292,
                -1.064054, 12.453717, 14.680292,
                -1.064054, 12.453717, 15.242792,
                -1.064054, 5.351794, 14.680292,
                -1.064054, 5.351794, 15.805292,
                -1.064054, 5.351794, 15.242792,
                -1.064054, 26.657564, 16.930292,
                -1.064054, 12.453717, 18.055292,
                -1.064054, 12.453717, 16.930292,
                -1.064054, 5.351794, 14.961542,
                -1.064054, 1.800833, 14.680292,
                -1.064054, 1.800833, 15.242792,
                -1.064054, 1.800833, 14.961542,
                -1.064054, 12.453717, 14.117792,
                -1.064054, 5.351794, 13.555292,
                -1.064054, 5.351794, 14.117792,
                -1.064054, 26.657564, 12.430292,
                -1.064054, 12.453717, 11.305292,
                -1.064054, 12.453717, 12.430292,
                -1.064054, 5.351794, 15.102167,
                -1.064054, 3.576313, 14.961542,
                -1.064054, 1.800833, 15.102167,
                -1.064054, 3.576313, 15.102167,
                -1.064054, 5.351794, 15.524042,
                -1.064054, 1.800833, 15.805292,
                -1.064054, 1.800833, 15.524042,
                -1.064054, 12.453717, 16.367792,
                -1.064054, 5.351794, 16.930292,
                -1.064054, 5.351794, 16.367792,
                -1.064054, 12.453717, 14.961542,
                -1.064054, 8.902756, 14.680292,
                -1.064054, 8.902756, 15.242792,
                -1.064054, 8.902756, 14.961542,
                -1.064054, 26.657564, 15.242792,
                -1.064054, 19.555639, 14.680292,
                -1.064054, 19.555639, 15.805292,
                -1.064054, 19.555639, 15.242792,
                -1.064054, 55.065254, 14.680292,
                -1.064054, 40.861408, 13.555292,
                -1.064054, 40.861408, 15.805292,
                -1.064054, 40.861408, 14.680292,
                -1.064054, 5.351794, 15.17248,
                -1.064054, 4.464054, 15.102167,
                -1.064054, 4.464054, 15.17248,
                -1.064054, 5.351794, 15.383417,
                -1.064054, 1.800833, 15.383417,
                -1.064054, 12.453717, 15.524042,
                -1.064054, 8.902756, 15.805292,
                -1.064054, 8.902756, 15.524042,
                -1.064054, 8.902756, 15.102167,
                -1.064054, 7.127275, 14.961542,
                -1.064054, 7.127275, 15.242792,
                -1.064054, 7.127275, 15.102167,
                -1.064054, 5.351794, 15.313105,
                -1.064054, 8.902756, 15.383417,
                -1.064054, 7.127275, 15.524042,
                -1.064054, 7.127275, 15.383417,
                -1.064054, 5.351794, 15.45373,
                -1.064054, 5.351794, 15.664667,
                -1.064054, 1.800833, 15.664667,
                -1.064054, 5.351794, 16.086542,
                -1.064054, 1.800833, 16.367792,
                -1.064054, 1.800833, 16.086542,
                -1.064054, 5.351794, 15.594355,
                -1.064054, 8.902756, 15.664667,
                -1.064054, 7.127275, 15.805292,
                -1.064054, 7.127275, 15.664667,
                -1.064054, 12.453717, 16.086542,
                -1.064054, 8.902756, 16.367792,
                -1.064054, 8.902756, 16.086542,
                -1.064054, 26.657564, 16.367792,
                -1.064054, 19.555639, 16.930292,
                -1.064054, 19.555639, 16.367792,
                -1.064054, 55.065254, 16.930292,
                -1.064054, 40.861408, 18.055292,
                -1.064054, 40.861408, 16.930292,
                -1.064054, 5.351794, 15.73498,
                -1.064054, 5.351794, 15.945917,
                -1.064054, 1.800833, 15.945917,
                -1.064054, 5.351794, 15.875605,
                -1.064054, 8.902756, 15.945917,
                -1.064054, 7.127275, 16.086542,
                -1.064054, 7.127275, 15.945917,
                -1.064054, 5.351794, 16.01623,
                -1.064054, 5.351794, 16.227167,
                -1.064054, 1.800833, 16.227167,
                -1.064054, 5.351794, 16.649042,
                -1.064054, 1.800833, 16.930292,
                -1.064054, 1.800833, 16.649042,
                -1.064054, 12.453717, 17.492792,
                -1.064054, 5.351794, 18.055292,
                -1.064054, 5.351794, 17.492792,
                -1.064054, 26.657564, 19.180292,
                -1.064054, 12.453717, 19.180292,
                -1.064054, 5.351794, 16.156855,
                -1.064054, 8.902756, 16.227167,
                -1.064054, 7.127275, 16.367792,
                -1.064054, 7.127275, 16.227167,
                -1.064054, 12.453717, 16.649042,
                -1.064054, 8.902756, 16.930292,
                -1.064054, 8.902756, 16.649042,
                -1.064054, 5.351794, 16.29748,
                -1.064054, 5.351794, 16.508417,
                -1.064054, 1.800833, 16.508417,
                -1.064054, 5.351794, 16.438105,
                -1.064054, 8.902756, 16.508417,
                -1.064054, 7.127275, 16.649042,
                -1.064054, 7.127275, 16.508417,
                -1.064054, 5.351794, 16.57873,
                -1.064054, 5.351794, 16.789667,
                -1.064054, 1.800833, 16.789667,
                -1.064054, 5.351794, 17.211542,
                -1.064054, 1.800833, 17.492792,
                -1.064054, 1.800833, 17.211542,
                -1.064054, 5.351794, 16.719355,
                -1.064054, 8.902756, 16.789667,
                -1.064054, 7.127275, 16.930292,
                -1.064054, 7.127275, 16.789667,
                -1.064054, 12.453717, 17.211542,
                -1.064054, 8.902756, 17.492792,
                -1.064054, 8.902756, 17.211542,
                -1.064054, 26.657564, 17.492792,
                -1.064054, 19.555639, 18.055292,
                -1.064054, 19.555639, 17.492792,
                -1.064054, 5.351794, 16.85998,
                -1.064054, 5.351794, 17.070917,
                -1.064054, 1.800833, 17.070917,
                -1.064054, 3.576313, 17.211542,
                -1.064054, 5.351794, 17.000605,
                -1.064054, 8.902756, 17.070917,
                -1.064054, 7.127275, 17.211542,
                -1.064054, 7.127275, 17.070917,
                -1.064054, 5.351794, 17.14123,
                -1.064054, 4.464054, 17.211542,
                -1.064054, 4.464054, 17.14123,
                -1.064054, 5.351794, 17.352167,
                -1.064054, 1.800833, 17.352167,
                -1.064054, 3.576313, 17.492792,
                -1.064054, 3.576313, 17.352167,
                -1.064054, 5.351794, 17.774042,
                -1.064054, 1.800833, 18.055292,
                -1.064054, 1.800833, 17.774042,
                -1.064054, 12.453717, 18.617792,
                -1.064054, 5.351794, 19.180292,
                -1.064054, 5.351794, 18.617792,
                -1.064054, 2.688573, 17.070917,
                -1.064054, 1.800833, 17.14123,
                -1.064054, 2.688573, 17.211542,
                -1.064054, 2.688573, 17.14123,
                -1.064054, 0.025352, 16.930292,
                -1.064054, 0.025352, 17.211542,
                -1.064054, 0.025352, 17.070917,
                -1.064054, 2.688573, 16.930292,
                -1.064054, 1.800833, 17.000605,
                -1.064054, 2.688573, 17.000605,
                -1.064054, 2.688573, 16.789667,
                -1.064054, 1.800833, 16.85998,
                -1.064054, 2.688573, 16.85998,
                -1.064054, 0.025352, 16.649042,
                -1.064054, 0.025352, 16.789667,
                -1.064054, 2.688573, 16.649042,
                -1.064054, 1.800833, 16.719355,
                -1.064054, 2.688573, 16.719355,
                -1.064054, 2.688573, 16.508417,
                -1.064054, 1.800833, 16.57873,
                -1.064054, 2.688573, 16.57873,
                -1.064054, 0.025352, 16.367792,
                -1.064054, 0.025352, 16.508417,
                -1.064054, 1.800833, 16.438105,
                -1.064054, 2.688573, 16.227167,
                -1.064054, 1.800833, 16.29748,
                -1.064054, 2.688573, 16.29748,
                -1.064054, 0.025352, 16.086542,
                -1.064054, 0.025352, 16.227167,
                -1.064054, 2.688573, 16.086542,
                -1.064054, 1.800833, 16.156855,
                -1.064054, 2.688573, 15.945917,
                -1.064054, 1.800833, 16.01623,
                -1.064054, 0.025352, 15.805292,
                -1.064054, 0.025352, 15.945917,
                -1.064054, 2.688573, 15.805292,
                -1.064054, 1.800833, 15.875605,
                -1.064054, 2.688573, 15.664667,
                -1.064054, 1.800833, 15.73498,
                -1.064054, 2.688573, 15.73498,
                -1.064054, 0.025352, 15.524042,
                -1.064054, 0.025352, 15.664667,
                -1.064054, 2.688573, 15.524042,
                -1.064054, 1.800833, 15.594355,
                -1.064054, 2.688573, 15.594355,
                -1.064054, 2.688573, 15.383417,
                -1.064054, 1.800833, 15.45373,
                -1.064054, 2.688573, 15.45373,
                -1.064054, 0.025352, 15.242792,
                -1.064054, 0.025352, 15.383417,
                -1.064054, 2.688573, 15.242792,
                -1.064054, 1.800833, 15.313105,
                -1.064054, 2.688573, 15.313105,
                -1.064054, 2.688573, 15.102167,
                -1.064054, 1.800833, 15.17248,
                -1.064054, 2.688573, 15.17248,
                -1.064054, 0.025352, 14.961542,
                -1.064054, 0.025352, 15.102167,
                -1.064054, 3.859701, 8.763344,
                -1.064054, 3.957716, 8.768159,
                -1.064054, 4.054788, 8.782557,
                -1.064054, 4.149981, 8.806401,
                -1.064054, 4.242379, 8.83946,
                -1.064054, 4.331196, 8.881472,
                -1.064054, 4.415265, 8.931866,
                -1.064054, 4.493997, 8.990248,
                -1.064054, 4.566802, 9.056224,
                -1.064054, 4.633232, 9.129571,
                -1.064054, 4.691685, 9.208527,
                -1.064054, 4.741891, 9.292435,
                -1.064054, 4.783578, 9.38064,
                -1.064054, 4.816474, 9.472486,
                -1.064054, 4.840308, 9.567319,
                -1.064054, 4.854807, 9.664481,
                -1.064054, 4.859701, 9.763318,
                -1.064054, 4.854807, 9.862155,
                -1.064054, 4.840308, 9.959317,
                -1.064054, 4.816474, 10.05415,
                -1.064054, 4.783578, 10.145996,
                -1.064054, 4.741891, 10.234201,
                -1.064054, 4.691685, 10.31811,
                -1.064054, 4.633232, 10.397065,
                -1.064054, 4.566802, 10.470411,
                -1.064054, 4.493987, 10.536396,
                -1.064054, 4.41525, 10.59478,
                -1.064054, 4.331183, 10.64517,
                -1.064054, 4.242379, 10.687176,
                -1.064054, 4.151809, 10.719678,
                -1.064054, 4.058534, 10.743326,
                -1.064054, 3.959617, 10.758287,
                -1.064054, 3.859701, 10.763291,
                -1.064054, 3.761685, 10.758477,
                -1.064054, 3.664613, 10.744078,
                -1.064054, 3.56942, 10.720235,
                -1.064054, 3.477022, 10.687175,
                -1.064054, 3.388206, 10.645164,
                -1.064054, 3.304136, 10.594769,
                -1.064054, 3.225404, 10.536387,
                -1.064054, 3.152599, 10.470411,
                -1.064054, 3.08617, 10.397065,
                -1.064054, 3.027716, 10.31811,
                -1.064054, 2.97751, 10.234201,
                -1.064054, 2.935823, 10.145996,
                -1.064054, 2.902927, 10.05415,
                -1.064054, 2.879093, 9.959317,
                -1.064054, 2.864594, 9.862154,
                -1.064054, 2.859701, 9.763318,
                -1.064054, 2.864594, 9.664481,
                -1.064054, 2.879093, 9.567319,
                -1.064054, 2.902927, 9.472486,
                -1.064054, 2.935823, 9.380639,
                -1.064054, 2.97751, 9.292434,
                -1.064054, 3.027716, 9.208526,
                -1.064054, 3.08617, 9.129571,
                -1.064054, 3.152599, 9.056224,
                -1.064054, 3.225415, 8.990239,
                -1.064054, 3.304151, 8.931856,
                -1.064054, 3.388218, 8.881466,
                -1.064054, 3.477022, 8.83946,
                -1.064054, 3.567592, 8.806957,
                -1.064054, 3.660867, 8.78331,
                -1.064054, 3.759784, 8.768348,
                -1.064054, 26.657564, 7.930292,
                -1.064054, 12.453717, 6.805292,
                -1.064054, 12.453717, 9.055292,
                -1.064054, 12.453717, 7.930292,
                -1.064054, 12.453717, 8.492792,
                -1.064054, 5.351794, 7.930292,
                -1.064054, 5.351794, 9.055292,
                -1.064054, 5.351794, 8.492792,
                -1.064054, 26.657564, 10.180292,
                -1.064054, 12.453717, 10.180292,
                -1.064054, 5.351794, 8.774042,
                -1.064054, 1.800833, 8.492792,
                -1.064054, 1.800833, 9.055292,
                -1.064054, 1.800833, 8.774042,
                -1.064054, 12.453717, 9.617792,
                -1.064054, 5.351794, 10.180292,
                -1.064054, 5.351794, 9.617792,
                -1.064054, 5.351794, 8.633417,
                -1.064054, 3.576313, 8.492792,
                -1.064054, 1.800833, 8.633417,
                -1.064054, 3.576313, 8.633417,
                -1.064054, 12.453717, 8.774042,
                -1.064054, 8.902756, 8.492792,
                -1.064054, 8.902756, 9.055292,
                -1.064054, 8.902756, 8.774042,
                -1.064054, 26.657564, 8.492792,
                -1.064054, 19.555639, 7.930292,
                -1.064054, 19.555639, 9.055292,
                -1.064054, 19.555639, 8.492792,
                -1.064054, 55.065254, 7.930292,
                -1.064054, 40.861408, 6.805292,
                -1.064054, 40.861408, 9.055292,
                -1.064054, 40.861408, 7.930292,
                -1.064054, 5.351794, 8.211542,
                -1.064054, 1.800833, 7.930292,
                -1.064054, 1.800833, 8.211542,
                -1.064054, 12.453717, 7.367792,
                -1.064054, 5.351794, 6.805292,
                -1.064054, 5.351794, 7.367792,
                -1.064054, 26.657564, 5.680292,
                -1.064054, 12.453717, 4.555292,
                -1.064054, 12.453717, 5.680292,
                -1.064054, 5.351794, 8.70373,
                -1.064054, 4.464054, 8.633417,
                -1.064054, 3.576313, 8.70373,
                -1.064054, 4.464054, 8.774042,
                -1.064054, 4.464054, 8.70373,
                -1.064054, 5.351794, 8.914667,
                -1.064054, 1.800833, 8.914667,
                -1.064054, 5.351794, 9.336542,
                -1.064054, 1.800833, 9.617792,
                -1.064054, 1.800833, 9.336542,
                -1.064054, 8.902756, 8.633417,
                -1.064054, 7.127275, 8.492792,
                -1.064054, 7.127275, 8.774042,
                -1.064054, 7.127275, 8.633417,
                -1.064054, 12.453717, 8.211542,
                -1.064054, 8.902756, 7.930292,
                -1.064054, 8.902756, 8.211542,
                -1.064054, 5.351794, 8.844355,
                -1.064054, 8.902756, 8.914667,
                -1.064054, 7.127275, 9.055292,
                -1.064054, 7.127275, 8.914667,
                -1.064054, 12.453717, 9.336542,
                -1.064054, 8.902756, 9.617792,
                -1.064054, 8.902756, 9.336542,
                -1.064054, 26.657564, 9.617792,
                -1.064054, 19.555639, 10.180292,
                -1.064054, 19.555639, 9.617792,
                -1.064054, 55.065254, 10.180292,
                -1.064054, 40.861408, 11.305292,
                -1.064054, 40.861408, 10.180292,
                -1.064054, 5.351794, 8.98498,
                -1.064054, 5.351794, 9.195917,
                -1.064054, 1.800833, 9.195917,
                -1.064054, 5.351794, 9.125605,
                -1.064054, 8.902756, 9.195917,
                -1.064054, 7.127275, 9.336542,
                -1.064054, 7.127275, 9.195917,
                -1.064054, 5.351794, 9.26623,
                -1.064054, 5.351794, 9.477167,
                -1.064054, 1.800833, 9.477167,
                -1.064054, 5.351794, 9.899042,
                -1.064054, 1.800833, 10.180292,
                -1.064054, 1.800833, 9.899042,
                -1.064054, 12.453717, 10.742792,
                -1.064054, 5.351794, 11.305292,
                -1.064054, 5.351794, 10.742792,
                -1.064054, 5.351794, 9.406855,
                -1.064054, 8.902756, 9.477167,
                -1.064054, 7.127275, 9.617792,
                -1.064054, 7.127275, 9.477167,
                -1.064054, 12.453717, 9.899042,
                -1.064054, 8.902756, 10.180292,
                -1.064054, 8.902756, 9.899042,
                -1.064054, 5.351794, 9.54748,
                -1.064054, 5.351794, 9.758417,
                -1.064054, 1.800833, 9.758417,
                -1.064054, 5.351794, 9.688105,
                -1.064054, 8.902756, 9.758417,
                -1.064054, 7.127275, 9.899042,
                -1.064054, 7.127275, 9.758417,
                -1.064054, 5.351794, 9.82873,
                -1.064054, 5.351794, 10.039667,
                -1.064054, 1.800833, 10.039667,
                -1.064054, 5.351794, 10.461542,
                -1.064054, 1.800833, 10.742792,
                -1.064054, 1.800833, 10.461542,
                -1.064054, 5.351794, 9.969355,
                -1.064054, 8.902756, 10.039667,
                -1.064054, 7.127275, 10.180292,
                -1.064054, 7.127275, 10.039667,
                -1.064054, 12.453717, 10.461542,
                -1.064054, 8.902756, 10.742792,
                -1.064054, 8.902756, 10.461542,
                -1.064054, 26.657564, 10.742792,
                -1.064054, 19.555639, 11.305292,
                -1.064054, 19.555639, 10.742792,
                -1.064054, 5.351794, 10.10998,
                -1.064054, 5.351794, 10.320917,
                -1.064054, 1.800833, 10.320917,
                -1.064054, 5.351794, 10.250605,
                -1.064054, 8.902756, 10.320917,
                -1.064054, 7.127275, 10.461542,
                -1.064054, 7.127275, 10.320917,
                -1.064054, 5.351794, 10.39123,
                -1.064054, 5.351794, 10.602167,
                -1.064054, 1.800833, 10.602167,
                -1.064054, 5.351794, 11.024042,
                -1.064054, 1.800833, 11.305292,
                -1.064054, 1.800833, 11.024042,
                -1.064054, 12.453717, 11.867792,
                -1.064054, 5.351794, 12.430292,
                -1.064054, 5.351794, 11.867792,
                -1.064054, 5.351794, 10.531855,
                -1.064054, 8.902756, 10.602167,
                -1.064054, 7.127275, 10.742792,
                -1.064054, 7.127275, 10.602167,
                -1.064054, 12.453717, 11.024042,
                -1.064054, 8.902756, 11.305292,
                -1.064054, 8.902756, 11.024042,
                -1.064054, 5.351794, 10.67248,
                -1.064054, 4.464054, 10.742792,
                -1.064054, 5.351794, 10.883417,
                -1.064054, 1.800833, 10.883417,
                -1.064054, 3.576313, 11.024042,
                -1.064054, 3.576313, 10.883417,
                -1.064054, 5.351794, 10.813105,
                -1.064054, 3.576313, 10.813105,
                -1.064054, 4.464054, 10.883417,
                -1.064054, 4.464054, 10.813105,
                -1.064054, 8.902756, 10.883417,
                -1.064054, 7.127275, 11.024042,
                -1.064054, 7.127275, 10.883417,
                -1.064054, 2.688573, 10.602167,
                -1.064054, 1.800833, 10.67248,
                -1.064054, 2.688573, 10.742792,
                -1.064054, 2.688573, 10.67248,
                -1.064054, 0.025352, 10.461542,
                -1.064054, 0.025352, 10.742792,
                -1.064054, 0.025352, 10.602167,
                -1.064054, 2.688573, 10.461542,
                -1.064054, 1.800833, 10.531855,
                -1.064054, 2.688573, 10.531855,
                -1.064054, 2.688573, 10.320917,
                -1.064054, 1.800833, 10.39123,
                -1.064054, 2.688573, 10.39123,
                -1.064054, 0.025352, 10.180292,
                -1.064054, 0.025352, 10.320917,
                -1.064054, 2.688573, 10.180292,
                -1.064054, 1.800833, 10.250605,
                -1.064054, 2.688573, 10.250605,
                -1.064054, 1.800833, 10.10998,
                -1.064054, 2.688573, 10.10998,
                -1.064054, 0.025352, 9.899042,
                -1.064054, 0.025352, 10.039667,
                -1.064054, 2.688573, 9.899042,
                -1.064054, 1.800833, 9.969355,
                -1.064054, 1.800833, 9.82873,
                -1.064054, 2.688573, 9.82873,
                -1.064054, 0.025352, 9.617792,
                -1.064054, 0.025352, 9.758417,
                -1.064054, 1.800833, 9.688105,
                -1.064054, 2.688573, 9.688105,
                -1.064054, 1.800833, 9.54748,
                -1.064054, 2.688573, 9.54748,
                -1.064054, 0.025352, 9.336542,
                -1.064054, 0.025352, 9.477167,
                -1.064054, 2.688573, 9.336542,
                -1.064054, 1.800833, 9.406855,
                -1.064054, 2.688573, 9.406855,
                -1.064054, 2.688573, 9.195917,
                -1.064054, 1.800833, 9.26623,
                -1.064054, 2.688573, 9.26623,
                -1.064054, 0.025352, 9.055292,
                -1.064054, 0.025352, 9.195917,
                -1.064054, 2.688573, 9.055292,
                -1.064054, 1.800833, 9.125605,
                -1.064054, 2.688573, 9.125605,
                -1.064054, 2.688573, 8.914667,
                -1.064054, 1.800833, 8.98498,
                -1.064054, 2.688573, 8.98498,
                -1.064054, 0.025352, 8.774042,
                -1.064054, 0.025352, 8.914667,
                -1.064054, 2.688573, 8.774042,
                -1.064054, 1.800833, 8.844355,
                -1.064054, 2.688573, 8.844355,
                -1.064054, -1.750129, 2.305292,
                -7.264054, -1.750129, 2.305292,
                -10.364054, -1.750129, 2.305292,
                -4.164054, 72.109871, 2.305292,
                -7.264054, 72.109871, 2.305292,
                -4.164054, 145.969864, 2.305292,
                -7.264054, 145.969864, 2.305292,
                -4.164054, 219.829865, 2.305292,
                -7.264054, 219.829865, 2.305292,
                -4.164054, 293.68988, 2.305292,
                -7.264054, 293.68988, 2.305292,
                -1.064054, 367.549866, 2.305292,
                -4.164054, 367.549866, 2.305292,
                -7.264054, 367.549866, 2.305292,
                -10.364054, 367.549866, 2.305292,
                -4.164054, 330.619873, 2.305292,
                -2.614054, 293.68988, 2.305292,
                -2.614054, 330.619873, 2.305292,
                -7.264054, 330.619873, 2.305292,
                -5.714054, 293.68988, 2.305292,
                -5.714054, 330.619873, 2.305292,
                -8.814054, 293.68988, 2.305292,
                -8.814054, 330.619873, 2.305292,
                -4.164054, -1.750129, 2.305292,
                -8.814054, 72.109871, 2.305292,
                -7.264054, 35.179871, 2.305292,
                -8.814054, 35.179871, 2.305292,
                -2.614054, 72.109871, 2.305292,
                -4.164054, 35.179871, 2.305292,
                -2.614054, 35.179871, 2.305292,
                -1.064054, 18.766537, 2.305292,
                -1.064054, 39.283203, 2.305292,
                -1.064054, 59.79987, 2.305292,
                -1.064054, 80.316536, 2.305292,
                -1.064054, 100.833206, 2.305292,
                -1.064054, 121.349869, 2.305292,
                -1.064054, 141.866531, 2.305292,
                -1.064054, 162.383209, 2.305292,
                -1.064054, 182.899872, 2.305292,
                -1.064054, 203.416534, 2.305292,
                -1.064054, 223.933197, 2.305292,
                -1.064054, 244.449875, 2.305292,
                -1.064054, 264.966553, 2.305292,
                -1.064054, 285.483215, 2.305292,
                -1.064054, 305.999878, 2.305292,
                -1.064054, 326.516541, 2.305292,
                -1.064054, 347.033203, 2.305292,
                -10.364054, 341.171295, 2.305292,
                -10.364054, 314.792725, 2.305292,
                -10.364054, 288.414154, 2.305292,
                -10.364054, 262.035583, 2.305292,
                -10.364054, 235.657013, 2.305292,
                -10.364054, 209.278442, 2.305292,
                -10.364054, 182.899872, 2.305292,
                -10.364054, 156.521301, 2.305292,
                -10.364054, 130.142731, 2.305292,
                -10.364054, 103.76416, 2.305292,
                -10.364054, 77.385582, 2.305292,
                -10.364054, 51.007015, 2.305292,
                -10.364054, 24.628443, 2.305292,
                -10.364054, 367.549866, 0.305292,
                -6.364054, 367.549866, 0.305292,
                -2.364054, 367.549866, 0.305292,
                1.635946, 367.549866, 0.305292,
                5.635946, 367.549866, 0.305292,
                9.635946, 367.549866, 0.305292,
                -10.364054, 367.549866, 2.305292,
                -1.064054, 367.549866, 2.305292,
                -4.164054, 367.549866, 2.305292,
                -7.264054, 367.549866, 2.305292,
                -1.064054, 367.549866, 20.305292,
                -1.064054, 367.549866, 16.705292,
                -1.064054, 367.549866, 13.105291,
                -1.064054, 367.549866, 9.505292,
                -1.064054, 367.549866, 5.905292,
                -10.364054, 367.549866, 20.305292,
                -7.264054, 367.549866, 20.305292,
                -4.164054, 367.549866, 20.305292,
                -10.364054, 367.549866, 22.305292,
                9.635946, 367.549866, 22.305292,
                5.635946, 367.549866, 22.305292,
                1.635946, 367.549866, 22.305292,
                -2.364054, 367.549866, 22.305292,
                -6.364054, 367.549866, 22.305292,
                9.635946, 367.549866, 20.305292,
                0.335946, 367.549866, 20.305292,
                3.435946, 367.549866, 20.305292,
                6.535946, 367.549866, 20.305292,
                0.335946, 367.549866, 2.305292,
                0.335946, 367.549866, 5.905292,
                0.335946, 367.549866, 9.505292,
                0.335946, 367.549866, 13.105291,
                0.335946, 367.549866, 16.705292,
                9.635946, 367.549866, 2.305292,
                6.535946, 367.549866, 2.305292,
                3.435946, 367.549866, 2.305292,
                -10.364054, -1.750129, 0.305292,
                -10.364054, -1.750129, 2.305292,
                9.635946, -1.750129, 0.305292,
                5.635946, -1.750129, 0.305292,
                1.635946, -1.750129, 0.305292,
                -2.364054, -1.750129, 0.305292,
                -6.364054, -1.750129, 0.305292,
                9.635946, -1.750129, 2.305292,
                0.335946, -1.750129, 2.305292,
                3.435946, -1.750129, 2.305292,
                6.535946, -1.750129, 2.305292,
                0.335946, -1.750129, 20.305292,
                0.335946, -1.750129, 16.705292,
                0.335946, -1.750129, 13.105291,
                0.335946, -1.750129, 9.505292,
                0.335946, -1.750129, 5.905292,
                -10.364054, -1.750129, 22.305292,
                -6.654399, -1.750129, 22.305292,
                -2.944745, -1.750129, 22.305292,
                0.764909, -1.750129, 22.305292,
                -10.364054, -1.750129, 20.305292,
                -1.064054, -1.750129, 20.305292,
                -4.164054, -1.750129, 20.305292,
                -7.264054, -1.750129, 20.305292,
                -1.064054, -1.750129, 2.305292,
                -1.064054, -1.750129, 5.905292,
                -1.064054, -1.750129, 9.505292,
                -1.064054, -1.750129, 13.105291,
                -1.064054, -1.750129, 16.705292,
                -7.264054, -1.750129, 2.305292,
                -4.164054, -1.750129, 2.305292,
                0.764909, -1.750129, 20.305292,
                0.335946, 3.859701, 15.163343,
                0.335946, 3.759784, 15.168348,
                0.335946, 3.660867, 15.18331,
                0.335946, 3.567592, 15.206958,
                0.335946, 3.477022, 15.23946,
                0.335946, 3.388218, 15.281466,
                0.335946, 3.304151, 15.331856,
                0.335946, 3.225415, 15.390239,
                0.335946, 3.152599, 15.456224,
                0.335946, 3.08617, 15.529571,
                0.335946, 3.027716, 15.608526,
                0.335946, 2.97751, 15.692434,
                0.335946, 2.935823, 15.78064,
                0.335946, 2.902927, 15.872486,
                0.335946, 2.879093, 15.967319,
                0.335946, 2.864594, 16.064482,
                0.335946, 2.859701, 16.163319,
                0.335946, 2.864594, 16.262154,
                0.335946, 2.879093, 16.359318,
                0.335946, 2.902927, 16.454149,
                0.335946, 2.935823, 16.545996,
                0.335946, 2.97751, 16.634201,
                0.335946, 3.027716, 16.718109,
                0.335946, 3.08617, 16.797064,
                0.335946, 3.152599, 16.870411,
                0.335946, 3.225404, 16.936388,
                0.335946, 3.304136, 16.99477,
                0.335946, 3.388206, 17.045164,
                0.335946, 3.477022, 17.087175,
                0.335946, 3.56942, 17.120234,
                0.335946, 3.664613, 17.144077,
                0.335946, 3.761685, 17.158476,
                0.335946, 3.859701, 17.163292,
                0.335946, 3.959617, 17.158287,
                0.335946, 4.058534, 17.143326,
                0.335946, 4.151809, 17.119678,
                0.335946, 4.242379, 17.087175,
                0.335946, 4.331183, 17.04517,
                0.335946, 4.41525, 16.99478,
                0.335946, 4.493987, 16.936396,
                0.335946, 4.566802, 16.870411,
                0.335946, 4.633232, 16.797066,
                0.335946, 4.691685, 16.718109,
                0.335946, 4.741891, 16.634201,
                0.335946, 4.783578, 16.545996,
                0.335946, 4.816474, 16.454149,
                0.335946, 4.840308, 16.359318,
                0.335946, 4.854807, 16.262156,
                0.335946, 4.859701, 16.163319,
                0.335946, 4.854807, 16.064482,
                0.335946, 4.840308, 15.967319,
                0.335946, 4.816474, 15.872486,
                0.335946, 4.783578, 15.78064,
                0.335946, 4.741891, 15.692434,
                0.335946, 4.691685, 15.608526,
                0.335946, 4.633232, 15.529571,
                0.335946, 4.566802, 15.456224,
                0.335946, 4.493997, 15.390248,
                0.335946, 4.415265, 15.331866,
                0.335946, 4.331196, 15.281472,
                0.335946, 4.242379, 15.23946,
                0.335946, 4.149981, 15.206401,
                0.335946, 4.054788, 15.182557,
                0.335946, 3.957716, 15.168159,
                0.335946, 3.859701, 15.163343,
                -0.371266, 3.859701, 15.163343,
                -0.371266, 3.859701, 15.163343,
                -1.064054, 3.859701, 15.163343,
                -1.064054, 3.957716, 15.168159,
                -1.064054, 4.054788, 15.182557,
                -1.064054, 4.149981, 15.206401,
                -1.064054, 4.242379, 15.23946,
                -1.064054, 4.331196, 15.281472,
                -1.064054, 4.415265, 15.331866,
                -1.064054, 4.493997, 15.390248,
                -1.064054, 4.566802, 15.456224,
                -1.064054, 4.633232, 15.529571,
                -1.064054, 4.691685, 15.608526,
                -1.064054, 4.741891, 15.692434,
                -1.064054, 4.783578, 15.78064,
                -1.064054, 4.816474, 15.872486,
                -1.064054, 4.840308, 15.967319,
                -1.064054, 4.854807, 16.064482,
                -1.064054, 4.859701, 16.163319,
                -1.064054, 4.854807, 16.262156,
                -1.064054, 4.840308, 16.359318,
                -1.064054, 4.816474, 16.454149,
                -1.064054, 4.783578, 16.545996,
                -1.064054, 4.741891, 16.634201,
                -1.064054, 4.691685, 16.718109,
                -1.064054, 4.633232, 16.797066,
                -1.064054, 4.566802, 16.870411,
                -1.064054, 4.493987, 16.936396,
                -1.064054, 4.41525, 16.99478,
                -1.064054, 4.331183, 17.04517,
                -1.064054, 4.242379, 17.087175,
                -1.064054, 4.151809, 17.119678,
                -1.064054, 4.058534, 17.143326,
                -1.064054, 3.959617, 17.158287,
                -1.064054, 3.859701, 17.163292,
                -1.064054, 3.761685, 17.158476,
                -1.064054, 3.664613, 17.144077,
                -1.064054, 3.56942, 17.120234,
                -1.064054, 3.477022, 17.087175,
                -1.064054, 3.388206, 17.045164,
                -1.064054, 3.304136, 16.99477,
                -1.064054, 3.225404, 16.936388,
                -1.064054, 3.152599, 16.870411,
                -1.064054, 3.08617, 16.797064,
                -1.064054, 3.027716, 16.718109,
                -1.064054, 2.97751, 16.634201,
                -1.064054, 2.935823, 16.545996,
                -1.064054, 2.902927, 16.454149,
                -1.064054, 2.879093, 16.359318,
                -1.064054, 2.864594, 16.262154,
                -1.064054, 2.859701, 16.163319,
                -1.064054, 2.864594, 16.064482,
                -1.064054, 2.879093, 15.967319,
                -1.064054, 2.902927, 15.872486,
                -1.064054, 2.935823, 15.78064,
                -1.064054, 2.97751, 15.692434,
                -1.064054, 3.027716, 15.608526,
                -1.064054, 3.08617, 15.529571,
                -1.064054, 3.152599, 15.456224,
                -1.064054, 3.225415, 15.390239,
                -1.064054, 3.304151, 15.331856,
                -1.064054, 3.388218, 15.281466,
                -1.064054, 3.477022, 15.23946,
                -1.064054, 3.567592, 15.206958,
                -1.064054, 3.660867, 15.18331,
                -1.064054, 3.759784, 15.168348,
                -1.064054, 3.859701, 15.163343,
                0.764909, 13.907396, 20.305292,
                0.764909, 13.907396, 22.305292,
                0.764909, 9.993014, 20.805292,
                0.764909, 9.993014, 21.305292,
                0.764909, 9.993014, 21.805292,
                0.764909, 6.078633, 20.805292,
                0.764909, 6.078633, 21.305292,
                0.764909, 6.078633, 21.805292,
                0.764909, 2.164252, 20.805292,
                0.764909, 2.164252, 21.305292,
                0.764909, 2.164252, 21.805292,
                0.764909, 11.297809, 20.305292,
                0.764909, 8.688221, 20.305292,
                0.764909, 6.078633, 20.305292,
                0.764909, 3.469046, 20.305292,
                0.764909, 0.859458, 20.305292,
                0.764909, -1.750129, 22.305292,
                0.764909, 0.859458, 22.305292,
                0.764909, 3.469046, 22.305292,
                0.764909, 6.078633, 22.305292,
                0.764909, 8.688221, 22.305292,
                0.764909, 11.297809, 22.305292,
                0.764909, -1.750129, 20.305292,
                0.764909, 13.907396, 21.305292,
                7.418187, 13.907396, 20.805292,
                7.418187, 13.907396, 21.305292,
                7.418187, 13.907396, 21.805292,
                5.200428, 13.907396, 20.805292,
                5.200428, 13.907396, 21.305292,
                5.200428, 13.907396, 21.805292,
                2.982669, 13.907396, 20.805292,
                2.982669, 13.907396, 21.305292,
                2.982669, 13.907396, 21.805292,
                0.764909, 13.907396, 20.305292,
                0.764909, 13.907396, 22.305292,
                9.635946, 13.907396, 20.305292,
                8.15744, 13.907396, 20.305292,
                6.678934, 13.907396, 20.305292,
                5.200428, 13.907396, 20.305292,
                3.721922, 13.907396, 20.305292,
                2.243416, 13.907396, 20.305292,
                9.635946, 13.907396, 22.305292,
                9.635946, 13.907396, 21.305292,
                2.243416, 13.907396, 22.305292,
                3.721922, 13.907396, 22.305292,
                5.200428, 13.907396, 22.305292,
                6.678934, 13.907396, 22.305292,
                8.15744, 13.907396, 22.305292,
                0.764909, 13.907396, 21.305292,
                0.335946, 3.859701, 8.763344,
                0.335946, 3.759784, 8.768348,
                0.335946, 3.660867, 8.78331,
                0.335946, 3.567592, 8.806957,
                0.335946, 3.477022, 8.83946,
                0.335946, 3.388218, 8.881466,
                0.335946, 3.304151, 8.931856,
                0.335946, 3.225415, 8.990239,
                0.335946, 3.152599, 9.056224,
                0.335946, 3.08617, 9.129571,
                0.335946, 3.027716, 9.208526,
                0.335946, 2.97751, 9.292434,
                0.335946, 2.935823, 9.380639,
                0.335946, 2.902927, 9.472486,
                0.335946, 2.879093, 9.567319,
                0.335946, 2.864594, 9.664481,
                0.335946, 2.859701, 9.763318,
                0.335946, 2.864594, 9.862154,
                0.335946, 2.879093, 9.959317,
                0.335946, 2.902927, 10.05415,
                0.335946, 2.935823, 10.145996,
                0.335946, 2.97751, 10.234201,
                0.335946, 3.027716, 10.31811,
                0.335946, 3.08617, 10.397065,
                0.335946, 3.152599, 10.470411,
                0.335946, 3.225404, 10.536387,
                0.335946, 3.304136, 10.594769,
                0.335946, 3.388206, 10.645164,
                0.335946, 3.477022, 10.687175,
                0.335946, 3.56942, 10.720235,
                0.335946, 3.664613, 10.744078,
                0.335946, 3.761685, 10.758477,
                0.335946, 3.859701, 10.763291,
                0.335946, 3.959617, 10.758287,
                0.335946, 4.058534, 10.743326,
                0.335946, 4.151809, 10.719678,
                0.335946, 4.242379, 10.687176,
                0.335946, 4.331183, 10.64517,
                0.335946, 4.41525, 10.59478,
                0.335946, 4.493987, 10.536396,
                0.335946, 4.566802, 10.470411,
                0.335946, 4.633232, 10.397065,
                0.335946, 4.691685, 10.31811,
                0.335946, 4.741891, 10.234201,
                0.335946, 4.783578, 10.145996,
                0.335946, 4.816474, 10.05415,
                0.335946, 4.840308, 9.959317,
                0.335946, 4.854807, 9.862155,
                0.335946, 4.859701, 9.763318,
                0.335946, 4.854807, 9.664481,
                0.335946, 4.840308, 9.567319,
                0.335946, 4.816474, 9.472486,
                0.335946, 4.783578, 9.38064,
                0.335946, 4.741891, 9.292435,
                0.335946, 4.691685, 9.208527,
                0.335946, 4.633232, 9.129571,
                0.335946, 4.566802, 9.056224,
                0.335946, 4.493997, 8.990248,
                0.335946, 4.415265, 8.931866,
                0.335946, 4.331196, 8.881472,
                0.335946, 4.242379, 8.83946,
                0.335946, 4.149981, 8.806401,
                0.335946, 4.054788, 8.782557,
                0.335946, 3.957716, 8.768159,
                0.335946, 3.859701, 8.763344,
                -0.371266, 3.859701, 8.763344,
                -0.371266, 3.859701, 8.763344,
                -1.064054, 3.859701, 8.763344,
                -1.064054, 3.957716, 8.768159,
                -1.064054, 4.054788, 8.782557,
                -1.064054, 4.149981, 8.806401,
                -1.064054, 4.242379, 8.83946,
                -1.064054, 4.331196, 8.881472,
                -1.064054, 4.415265, 8.931866,
                -1.064054, 4.493997, 8.990248,
                -1.064054, 4.566802, 9.056224,
                -1.064054, 4.633232, 9.129571,
                -1.064054, 4.691685, 9.208527,
                -1.064054, 4.741891, 9.292435,
                -1.064054, 4.783578, 9.38064,
                -1.064054, 4.816474, 9.472486,
                -1.064054, 4.840308, 9.567319,
                -1.064054, 4.854807, 9.664481,
                -1.064054, 4.859701, 9.763318,
                -1.064054, 4.854807, 9.862155,
                -1.064054, 4.840308, 9.959317,
                -1.064054, 4.816474, 10.05415,
                -1.064054, 4.783578, 10.145996,
                -1.064054, 4.741891, 10.234201,
                -1.064054, 4.691685, 10.31811,
                -1.064054, 4.633232, 10.397065,
                -1.064054, 4.566802, 10.470411,
                -1.064054, 4.493987, 10.536396,
                -1.064054, 4.41525, 10.59478,
                -1.064054, 4.331183, 10.64517,
                -1.064054, 4.242379, 10.687176,
                -1.064054, 4.151809, 10.719678,
                -1.064054, 4.058534, 10.743326,
                -1.064054, 3.959617, 10.758287,
                -1.064054, 3.859701, 10.763291,
                -1.064054, 3.761685, 10.758477,
                -1.064054, 3.664613, 10.744078,
                -1.064054, 3.56942, 10.720235,
                -1.064054, 3.477022, 10.687175,
                -1.064054, 3.388206, 10.645164,
                -1.064054, 3.304136, 10.594769,
                -1.064054, 3.225404, 10.536387,
                -1.064054, 3.152599, 10.470411,
                -1.064054, 3.08617, 10.397065,
                -1.064054, 3.027716, 10.31811,
                -1.064054, 2.97751, 10.234201,
                -1.064054, 2.935823, 10.145996,
                -1.064054, 2.902927, 10.05415,
                -1.064054, 2.879093, 9.959317,
                -1.064054, 2.864594, 9.862154,
                -1.064054, 2.859701, 9.763318,
                -1.064054, 2.864594, 9.664481,
                -1.064054, 2.879093, 9.567319,
                -1.064054, 2.902927, 9.472486,
                -1.064054, 2.935823, 9.380639,
                -1.064054, 2.97751, 9.292434,
                -1.064054, 3.027716, 9.208526,
                -1.064054, 3.08617, 9.129571,
                -1.064054, 3.152599, 9.056224,
                -1.064054, 3.225415, 8.990239,
                -1.064054, 3.304151, 8.931856,
                -1.064054, 3.388218, 8.881466,
                -1.064054, 3.477022, 8.83946,
                -1.064054, 3.567592, 8.806957,
                -1.064054, 3.660867, 8.78331,
                -1.064054, 3.759784, 8.768348,
                -1.064054, 3.859701, 8.763344,
            ],
            vec![
                2,21,24,
                20,22,21,
                25,5,26,
                29,28,5,
                24,0,30,
                23,24,30,
                21,0,24,
                32,2,23,
                23,30,31,
                1,0,21,
                23,31,32,
                42,2,32,
                34,42,33,
                35,43,34,
                33,42,32,
                35,3,44,
                22,19,1,
                21,22,1,
                20,19,22,
                20,18,19,
                20,17,18,
                20,2,17,
                42,43,15,
                34,43,42,
                43,44,14,
                43,14,15,
                17,42,16,
                2,42,17,
                3,36,44,
                45,36,37,
                46,37,38,
                45,37,46,
                38,39,47,
                45,46,12,
                44,45,13,
                26,40,41,
                27,41,6,
                26,5,47,
                40,26,47,
                41,27,26,
                27,6,25,
                39,40,47,
                46,47,10,
                12,13,45,
                9,10,47,
                28,9,47,
                7,8,29,
                28,29,8,
                25,7,29,
                7,25,6,
                9,28,8,
                47,5,28,
                50,101,104,
                51,98,102,
                100,102,101,
                52,95,99,
                97,99,98,
                94,96,95,
                100,50,54,
                55,51,100,
                54,55,100,
                97,51,55,
                56,52,97,
                55,56,97,
                83,59,55,
                54,58,83,
                55,54,83,
                86,60,56,
                55,59,86,
                56,55,86,
                79,58,80,
                82,59,83,
                84,83,58,
                85,60,86,
                87,86,59,
                89,88,60,
                104,48,63,
                104,63,64,
                101,93,104,
                48,104,93,
                92,93,101,
                64,65,104,
                103,65,66,
                103,67,68,
                103,66,67,
                54,50,103,
                68,54,103,
                69,54,53,
                54,68,53,
                65,103,104,
                98,91,92,
                91,98,99,
                98,92,102,
                99,95,91,
                96,116,49,
                95,96,90,
                96,49,90,
                116,96,115,
                114,115,96,
                56,111,94,
                94,113,114,
                94,111,112,
                112,113,94,
                56,94,52,
                80,69,70,
                80,70,71,
                80,58,54,
                81,80,72,
                72,80,71,
                61,81,74,
                73,74,81,
                81,61,75,
                79,81,75,
                84,79,76,
                76,77,82,
                84,76,82,
                111,56,57,
                56,110,57,
                56,60,88,
                88,89,107,
                88,110,56,
                88,109,110,
                88,107,108,
                108,109,88,
                85,87,77,
                89,85,78,
                105,106,89,
                89,62,105,
                62,89,78,
                80,54,69,
                119,131,134,
                130,132,131,
                125,122,126,
                129,128,122,
                134,117,135,
                133,134,135,
                131,117,134,
                137,119,133,
                133,135,136,
                118,117,131,
                133,136,137,
                159,119,137,
                139,159,138,
                140,160,139,
                138,159,137,
                140,120,161,
                132,158,118,
                131,132,118,
                130,158,132,
                130,157,158,
                130,156,157,
                130,119,156,
                159,160,154,
                139,160,159,
                160,161,153,
                160,153,154,
                156,159,155,
                119,159,156,
                120,141,161,
                162,141,142,
                163,142,143,
                162,142,163,
                143,144,164,
                162,163,151,
                161,162,152,
                126,145,146,
                127,146,123,
                126,122,164,
                145,126,164,
                146,127,126,
                127,123,125,
                144,145,164,
                163,164,149,
                151,152,162,
                148,149,164,
                128,148,164,
                124,147,129,
                128,129,147,
                125,124,129,
                124,125,123,
                148,128,147,
                164,122,128,
                167,191,192,
                187,189,188,
                170,168,167,
                172,170,169,
                174,172,171,
                179,173,180,
                184,182,174,
                192,165,193,
                192,193,194,
                191,186,192,
                165,192,186,
                190,192,194,
                191,188,185,
                196,169,190,
                169,196,197,
                167,190,169,
                169,198,199,
                169,197,198,
                195,196,190,
                167,168,188,
                189,185,188,
                189,187,220,
                166,185,189,
                189,222,166,
                221,222,189,
                170,187,168,
                170,217,187,
                219,187,218,
                220,187,219,
                187,217,218,
                170,215,216,
                216,217,170,
                214,215,170,
                171,169,199,
                171,200,201,
                171,201,202,
                202,180,171,
                204,180,203,
                180,202,203,
                180,173,171,
                170,172,214,
                181,204,205,
                181,205,175,
                179,183,174,
                181,176,179,
                176,181,175,
                204,181,180,
                176,177,183,
                174,182,172,
                213,172,212,
                211,212,172,
                210,182,209,
                182,210,211,
                182,211,172,
                178,184,177,
                183,177,184,
                182,184,208,
                184,178,206,
                206,207,184,
                209,182,208,
                226,688,715,
                713,715,714,
                227,721,723,
                716,690,723,
                721,687,716,
                723,721,716,
                737,689,722,
                723,690,737,
                722,723,737,
                720,723,722,
                746,719,716,
                687,717,746,
                716,687,746,
                717,747,748,
                892,740,748,
                747,691,892,
                748,747,892,
                745,696,734,
                748,740,745,
                734,748,745,
                746,748,734,
                719,734,736,
                728,703,736,
                734,696,728,
                736,734,728,
                736,703,693,
                733,736,735,
                733,718,690,
                716,719,733,
                690,716,733,
                765,697,766,
                767,766,741,
                758,741,759,
                761,759,699,
                729,696,745,
                766,697,762,
                764,741,766,
                762,764,766,
                759,741,764,
                763,699,759,
                764,763,759,
                696,729,732,
                732,730,706,
                728,732,731,
                706,704,698,
                731,706,705,
                693,703,731,
                705,693,731,
                705,698,709,
                707,693,705,
                709,707,705,
                707,709,708,
                690,718,739,
                735,693,707,
                739,718,735,
                707,739,735,
                739,707,692,
                737,739,738,
                738,692,712,
                710,689,738,
                712,710,738,
                710,712,711,
                722,689,710,
                710,688,226,
                226,686,722,
                722,710,226,
                228,821,822,
                851,783,822,
                821,436,851,
                822,821,851,
                817,695,753,
                822,783,817,
                753,822,817,
                820,822,753,
                436,852,853,
                860,853,852,
                784,833,860,
                852,784,860,
                853,860,862,
                860,833,847,
                854,785,861,
                862,847,854,
                861,862,854,
                859,862,861,
                859,818,783,
                851,853,859,
                783,851,859,
                848,833,784,
                834,835,848,
                784,834,848,
                855,847,833,
                848,850,855,
                833,848,855,
                848,835,849,
                847,855,858,
                855,850,856,
                854,858,857,
                863,785,857,
                844,807,845,
                846,845,832,
                839,832,840,
                841,840,808,
                845,807,842,
                843,832,845,
                842,843,845,
                840,832,843,
                826,808,840,
                843,826,840,
                829,808,826,
                827,816,829,
                826,827,829,
                824,816,827,
                810,781,824,
                827,810,824,
                828,808,829,
                830,829,816,
                823,816,824,
                825,824,781,
                864,819,817,
                783,818,864,
                817,783,864,
                818,861,866,
                863,831,866,
                861,785,863,
                866,861,863,
                867,806,865,
                866,831,867,
                865,866,867,
                864,866,865,
                819,865,871,
                868,815,871,
                865,806,868,
                871,865,868,
                872,701,870,
                871,815,872,
                870,871,872,
                869,871,870,
                869,795,695,
                817,819,869,
                695,817,869,
                874,796,794,
                695,795,874,
                794,695,874,
                795,870,876,
                873,804,876,
                870,701,873,
                876,870,873,
                877,780,875,
                876,804,877,
                875,876,877,
                874,876,875,
                796,875,881,
                878,792,881,
                875,780,878,
                881,875,878,
                882,702,880,
                881,792,882,
                880,881,882,
                879,881,880,
                879,750,700,
                794,796,879,
                700,794,879,
                812,781,813,
                814,813,805,
                809,782,801,
                813,781,810,
                811,805,813,
                810,811,813,
                809,805,811,
                799,782,809,
                811,799,809,
                802,782,799,
                800,793,802,
                799,800,802,
                797,793,800,
                788,743,797,
                800,788,797,
                803,802,793,
                793,797,798,
                791,790,779,
                779,786,787,
                787,786,744,
                790,743,788,
                789,779,790,
                788,789,790,
                786,779,789,
                773,744,786,
                789,773,786,
                776,744,773,
                774,769,776,
                773,774,776,
                771,769,774,
                762,697,771,
                774,762,771,
                775,744,776,
                777,776,769,
                770,769,771,
                772,771,697,
                884,751,749,
                700,750,884,
                749,700,884,
                750,880,886,
                883,778,886,
                880,702,883,
                886,880,883,
                887,742,885,
                886,778,887,
                885,886,887,
                884,886,885,
                751,885,890,
                888,768,890,
                885,742,888,
                890,885,888,
                891,691,747,
                890,768,891,
                747,890,891,
                889,890,747,
                889,717,687,
                749,751,889,
                687,749,889,
                694,753,754,
                794,700,754,
                753,695,794,
                754,753,794,
                749,687,721,
                754,700,749,
                721,754,749,
                752,754,721,
                229,419,437,
                437,837,838,
                852,436,836,
                838,784,852,
                836,838,852,
                836,436,821,
                821,228,435,
                435,437,836,
                836,821,435,
                230,447,449,
                467,421,449,
                447,418,467,
                449,447,467,
                442,420,448,
                449,421,442,
                448,449,442,
                446,449,448,
                614,469,467,
                418,468,614,
                467,418,614,
                468,610,616,
                613,487,616,
                610,422,613,
                616,610,613,
                617,454,615,
                616,487,617,
                615,616,617,
                614,616,615,
                469,615,620,
                618,470,620,
                615,454,618,
                620,615,618,
                621,424,464,
                620,470,621,
                464,620,621,
                619,620,464,
                619,443,421,
                467,469,619,
                421,467,619,
                497,455,498,
                499,498,488,
                492,488,493,
                494,493,456,
                498,455,495,
                496,488,498,
                495,496,498,
                493,488,496,
                482,456,493,
                496,482,493,
                485,456,482,
                483,471,485,
                482,483,485,
                480,471,483,
                476,429,480,
                483,476,480,
                484,456,485,
                486,485,471,
                479,471,480,
                481,480,429,
                462,460,424,
                460,462,461,
                472,429,473,
                475,473,439,
                475,474,441,
                474,439,431,
                440,441,474,
                431,440,474,
                461,441,440,
                428,438,461,
                440,428,461,
                473,429,476,
                478,439,473,
                476,478,473,
                439,478,477,
                440,431,430,
                423,428,440,
                430,423,440,
                463,445,442,
                421,443,463,
                442,421,463,
                443,464,466,
                460,438,466,
                464,424,460,
                466,464,460,
                466,438,428,
                463,466,465,
                465,428,423,
                444,445,465,
                423,444,465,
                442,445,444,
                444,423,434,
                432,420,444,
                434,432,444,
                432,434,433,
                448,420,432,
                432,419,229,
                229,417,448,
                448,432,229,
                231,565,566,
                565,426,522,
                561,427,506,
                566,522,561,
                506,566,561,
                564,566,506,
                579,524,562,
                562,522,426,
                426,523,579,
                579,562,426,
                523,580,581,
                576,524,579,
                581,548,576,
                579,581,576,
                524,576,578,
                576,548,577,
                572,559,578,
                577,549,572,
                578,577,572,
                586,547,575,
                578,559,586,
                575,578,586,
                573,549,570,
                571,560,573,
                570,571,573,
                568,560,571,
                553,520,568,
                571,553,568,
                547,586,587,
                558,585,587,
                572,549,573,
                574,573,560,
                567,560,568,
                569,568,520,
                522,562,563,
                575,547,589,
                589,563,562,
                562,524,575,
                575,589,562,
                563,589,591,
                585,558,591,
                589,547,585,
                591,589,585,
                592,458,590,
                591,558,592,
                590,591,592,
                588,591,590,
                588,536,427,
                561,563,588,
                427,561,588,
                594,537,535,
                427,536,594,
                535,427,594,
                536,590,596,
                593,545,596,
                590,458,593,
                596,590,593,
                597,519,595,
                596,545,597,
                595,596,597,
                594,596,595,
                537,595,601,
                598,533,601,
                595,519,598,
                601,595,598,
                602,459,600,
                601,533,602,
                600,601,602,
                599,601,600,
                599,503,457,
                535,537,599,
                457,535,599,
                555,520,556,
                557,556,546,
                550,546,551,
                552,551,521,
                556,520,553,
                554,546,556,
                553,554,556,
                551,546,554,
                540,521,551,
                554,540,551,
                543,521,540,
                541,534,543,
                540,541,543,
                539,534,541,
                529,490,539,
                541,529,539,
                542,521,543,
                544,543,534,
                538,534,539,
                490,531,532,
                527,518,528,
                528,491,515,
                531,490,529,
                530,518,531,
                529,530,531,
                528,518,530,
                513,491,528,
                530,513,528,
                516,491,513,
                514,501,516,
                513,514,516,
                512,501,514,
                495,455,512,
                514,495,512,
                511,501,512,
                512,455,497,
                604,504,502,
                457,503,604,
                502,457,604,
                503,600,606,
                603,517,606,
                600,459,603,
                606,600,603,
                607,489,605,
                606,517,607,
                605,606,607,
                604,606,605,
                504,605,611,
                608,500,611,
                605,489,608,
                611,605,608,
                612,422,610,
                611,500,612,
                610,611,612,
                609,611,610,
                609,468,418,
                502,504,609,
                418,502,609,
                425,506,507,
                535,457,507,
                506,427,535,
                507,506,535,
                502,418,447,
                507,457,502,
                447,507,502,
                505,507,447,
                526,583,584,
                582,584,523,
                582,426,565,
                565,231,525,
                525,526,582,
                582,565,525,
                726,226,713,
                713,225,232,
                232,233,726,
                726,713,232,
                234,725,727,
                720,686,727,
                725,227,720,
                727,725,720,
                727,686,226,
                724,727,726,
                235,756,757,
                820,694,757,
                756,228,820,
                757,756,820,
                752,227,725,
                757,694,752,
                725,757,752,
                755,757,725,
                756,452,435,
                452,229,435,
                435,228,756,
                756,235,236,
                237,451,453,
                446,417,453,
                451,230,446,
                453,451,446,
                453,417,229,
                450,453,452,
                238,509,510,
                564,425,510,
                509,231,564,
                510,509,564,
                505,230,451,
                510,425,505,
                451,510,505,
                508,510,451,
                240,233,232,
                724,233,240,
                241,234,724,
                240,241,724,
                755,234,241,
                242,235,755,
                241,242,755,
                243,236,235,
                450,236,243,
                244,237,450,
                243,244,450,
                508,237,244,
                245,238,508,
                244,245,508,
                247,240,239,
                248,241,240,
                249,242,241,
                250,243,242,
                251,244,243,
                252,245,244,
                254,247,246,
                255,248,247,
                256,249,248,
                257,250,249,
                258,251,250,
                259,252,251,
                261,254,253,
                262,255,254,
                263,256,255,
                264,257,256,
                265,258,257,
                266,259,258,
                268,261,260,
                269,262,261,
                270,263,262,
                271,264,263,
                272,265,264,
                273,266,265,
                275,268,267,
                276,269,268,
                277,270,269,
                278,271,270,
                279,272,271,
                280,273,272,
                282,275,274,
                283,276,275,
                284,277,276,
                285,278,277,
                286,279,278,
                287,280,279,
                289,282,281,
                290,283,282,
                291,284,283,
                292,285,284,
                293,286,285,
                294,287,286,
                296,289,288,
                297,290,289,
                298,291,290,
                299,292,291,
                300,293,292,
                301,294,293,
                303,296,295,
                304,297,296,
                305,298,297,
                306,299,298,
                307,300,299,
                308,301,300,
                763,318,708,
                318,711,708,
                317,318,763,
                763,708,709,
                763,709,698,
                762,317,764,
                763,764,317,
                774,317,762,
                317,774,773,
                317,773,789,
                789,788,317,
                698,704,763,
                699,704,760,
                704,730,760,
                763,704,699,
                627,758,761,
                628,767,758,
                761,760,625,
                630,772,765,
                632,777,770,
                634,787,775,
                775,777,633,
                770,772,631,
                635,791,779,
                765,767,629,
                317,788,800,
                317,800,799,
                791,743,790,
                798,797,743,
                317,811,810,
                317,810,827,
                317,826,843,
                317,843,842,
                317,827,826,
                317,799,811,
                802,639,782,
                791,636,743,
                798,638,793,
                803,793,638,
                637,798,743,
                639,801,782,
                809,640,805,
                849,807,844,
                809,801,640,
                842,849,835,
                805,641,814,
                842,807,849,
                803,639,802,
                834,842,835,
                635,779,787,
                316,317,842,
                787,634,635,
                775,633,634,
                770,631,632,
                772,630,631,
                632,633,777,
                638,639,803,
                639,640,801,
                641,642,814,
                638,798,637,
                640,641,805,
                636,637,743,
                623,730,622,
                622,730,732,
                760,623,624,
                730,623,760,
                622,732,685,
                685,729,684,
                683,684,729,
                628,629,767,
                627,628,758,
                625,626,761,
                624,625,760,
                627,761,626,
                765,629,630,
                730,704,706,
                643,825,812,
                644,823,825,
                825,643,644,
                812,642,643,
                812,814,642,
                830,823,644,
                645,828,830,
                828,646,841,
                649,846,839,
                839,841,647,
                846,650,844,
                646,828,645,
                645,830,644,
                648,649,839,
                647,648,839,
                651,652,844,
                650,651,844,
                650,846,649,
                653,844,652,
                653,856,844,
                850,849,856,
                654,655,858,
                856,653,654,
                856,654,858,
                646,647,841,
                844,856,849,
                656,857,655,
                635,636,791,
                223,714,318,
                477,315,316,
                430,477,316,
                477,478,315,
                430,316,434,
                477,430,431,
                834,316,842,
                476,315,478,
                483,315,476,
                496,315,482,
                495,315,496,
                482,315,483,
                514,315,495,
                356,472,475,
                358,481,472,
                360,486,479,
                361,484,486,
                479,481,359,
                364,499,492,
                511,512,366,
                497,366,512,
                497,499,365,
                492,494,363,
                516,501,511,
                494,484,362,
                315,513,530,
                315,530,529,
                515,491,516,
                315,529,541,
                570,224,315,
                541,540,315,
                540,554,315,
                315,554,553,
                315,571,570,
                315,553,571,
                570,580,224,
                514,513,315,
                518,370,531,
                527,528,369,
                515,369,528,
                518,527,370,
                532,531,370,
                490,371,539,
                570,577,548,
                570,581,580,
                570,548,581,
                570,549,577,
                490,532,371,
                368,515,516,
                580,583,224,
                367,516,511,
                367,368,516,
                497,365,366,
                492,363,364,
                494,362,363,
                364,365,499,
                527,369,370,
                369,515,368,
                532,370,371,
                539,372,538,
                371,372,539,
                373,538,372,
                366,367,511,
                475,354,355,
                353,354,441,
                475,355,356,
                353,461,416,
                416,462,415,
                353,441,461,
                486,360,361,
                359,360,479,
                357,358,472,
                356,357,472,
                359,481,358,
                316,834,838,
                475,441,354,
                361,362,484,
                374,542,544,
                375,552,542,
                542,374,375,
                544,373,374,
                373,544,538,
                550,552,375,
                550,376,557,
                557,377,555,
                569,380,567,
                574,559,572,
                381,574,567,
                378,569,555,
                550,375,376,
                555,377,378,
                557,376,377,
                381,382,574,
                380,381,567,
                379,380,569,
                574,383,559,
                384,559,383,
                383,574,382,
                385,386,586,
                559,384,385,
                559,385,586,
                378,379,569,
                414,415,462,
                587,386,387,
                682,745,681,
                681,740,680,
                678,679,691,
                745,682,729,
                679,680,892,
                683,729,682,
                685,732,729,
                678,891,677,
                677,768,676,
                742,674,675,
                887,673,674,
                888,675,676,
                692,708,712,
                681,745,740,
                680,740,892,
                678,691,891,
                677,891,768,
                675,888,742,
                673,887,778,
                674,742,887,
                676,768,888,
                679,892,691,
                712,708,711,
                883,673,778,
                662,867,661,
                661,831,660,
                666,701,665,
                663,664,868,
                806,662,663,
                863,857,658,
                857,657,658,
                857,858,655,
                659,660,831,
                658,659,863,
                665,872,664,
                702,671,672,
                673,883,672,
                883,702,672,
                671,702,882,
                670,671,882,
                792,670,882,
                780,668,669,
                668,804,667,
                669,878,780,
                792,878,670,
                780,877,668,
                878,669,670,
                873,666,667,
                877,804,668,
                225,319,320,
                232,320,321,
                239,322,323,
                246,323,324,
                321,322,239,
                319,225,714,
                320,232,225,
                323,246,239,
                253,246,324,
                714,715,318,
                260,253,326,
                260,326,327,
                328,267,327,
                328,329,274,
                267,260,327,
                330,331,281,
                329,330,274,
                331,332,288,
                332,333,288,
                335,302,334,
                334,295,333,
                309,302,335,
                667,804,873,
                666,873,701,
                664,872,815,
                664,815,868,
                665,701,872,
                662,806,867,
                661,867,831,
                838,837,316,
                784,838,834,
                659,831,863,
                663,868,806,
                334,302,295,
                288,281,331,
                304,303,312,
                312,313,305,
                304,312,305,
                303,302,311,
                311,302,309,
                711,715,688,
                715,711,318,
                714,223,319,
                316,837,433,
                413,424,412,
                412,621,411,
                409,410,618,
                424,413,462,
                410,411,470,
                414,462,413,
                416,461,462,
                409,454,408,
                408,617,407,
                613,405,406,
                422,404,405,
                487,406,407,
                423,430,434,
                412,424,621,
                411,621,470,
                409,618,454,
                408,454,617,
                406,487,613,
                404,422,612,
                405,613,422,
                407,617,487,
                410,470,618,
                316,433,434,
                500,404,612,
                593,393,394,
                392,393,458,
                398,533,397,
                397,598,396,
                394,395,545,
                558,587,389,
                587,388,389,
                391,592,390,
                586,386,587,
                391,392,592,
                389,390,558,
                396,519,395,
                608,402,403,
                404,500,403,
                500,608,403,
                402,608,489,
                401,402,489,
                607,401,489,
                517,400,401,
                603,399,400,
                400,517,603,
                399,459,602,
                399,603,459,
                517,401,607,
                398,399,602,
                602,533,398,
                397,533,598,
                396,598,519,
                437,419,433,
                395,519,597,
                394,545,593,
                393,593,458,
                390,592,558,
                392,458,592,
                395,597,545,
                525,231,509,
                313,314,307,
                238,245,350,
                245,252,348,
                259,266,345,
                273,280,343,
                266,273,344,
                252,259,347,
                308,307,314,
                509,238,351,
                352,583,526,
                584,583,580,
                351,352,525,
                526,525,352,
                351,525,509,
                252,347,348,
                224,583,352,
                266,344,345,
                341,342,280,
                287,294,340,
                294,301,338,
                301,308,337,
                287,340,341,
                308,310,336,
                336,337,308,
                310,308,314,
                523,584,580,
                388,587,387,
                657,857,656,
                965,940,938,
                894,939,965,
                938,894,965,
                939,963,966,
                967,943,941,
                940,942,967,
                941,940,967,
                959,958,957,
                895,951,959,
                957,895,959,
                961,959,951,
                958,960,964,
                962,939,894,
                957,958,962,
                894,957,962,
                952,953,954,
                956,955,952,
                951,955,956,
                955,951,895,
                950,952,955,
                895,950,955,
                957,894,896,
                897,895,957,
                896,897,957,
                899,897,896,
                910,901,899,
                898,900,910,
                899,898,910,
                906,900,907,
                909,901,910,
                911,910,900,
                913,912,901,
                936,937,893,
                893,935,936,
                893,934,935,
                934,893,933,
                933,914,932,
                967,931,943,
                941,943,931,
                967,949,931,
                914,931,932,
                893,914,933,
                914,941,931,
                916,941,915,
                914,915,941,
                938,941,916,
                942,940,949,
                949,940,965,
                941,938,940,
                949,965,966,
                938,916,917,
                967,942,949,
                966,948,949,
                964,947,948,
                964,948,962,
                963,939,948,
                947,964,960,
                963,948,966,
                959,961,947,
                945,954,944,
                951,956,946,
                947,961,946,
                961,951,946,
                945,946,956,
                956,952,945,
                958,947,960,
                958,959,947,
                954,945,952,
                978,953,977,
                953,952,950,
                978,944,953,
                939,962,948,
                894,938,896,
                918,919,938,
                896,919,920,
                896,920,921,
                938,919,896,
                898,896,922,
                921,922,896,
                898,923,924,
                925,898,924,
                907,925,926,
                907,927,928,
                908,928,929,
                907,926,927,
                908,907,928,
                907,900,898,
                903,906,908,
                930,902,908,
                902,903,908,
                908,929,930,
                925,907,898,
                953,950,977,
                976,977,950,
                897,975,976,
                950,897,976,
                897,899,973,
                950,895,897,
                897,974,975,
                899,972,973,
                899,971,972,
                971,899,912,
                909,911,904,
                903,904,911,
                913,909,904,
                969,970,912,
                912,970,971,
                912,913,969,
                968,913,905,
                904,905,913,
                901,912,899,
                906,903,911,
                918,938,917,
                980,992,1001,
                1001,1003,1004,
                994,996,995,
                993,995,999,
                997,999,998,
                997,992,980,
                991,993,997,
                980,991,997,
                985,982,986,
                989,988,982,
                1004,1005,1002,
                1006,1002,1005,
                1003,1005,1004,
                998,1003,1001,
                990,1005,1003,
                1001,1002,1000,
                1000,1006,1007,
                980,1007,1008,
                1009,1027,1008,
                1007,980,1000,
                1027,980,1008,
                1006,1000,1002,
                995,990,999,
                999,990,998,
                979,990,995,
                996,994,979,
                991,1026,994,
                993,991,994,
                1025,980,1024,
                991,1025,1026,
                1027,981,1024,
                1025,991,980,
                1023,981,1027,
                979,994,1026,
                980,1027,1024,
                1009,1010,1027,
                1028,1010,1011,
                1029,1011,1012,
                1028,1029,1022,
                1011,1029,1028,
                1030,1029,1012,
                1029,1030,1021,
                1031,1030,1014,
                1015,1031,1014,
                987,986,1016,
                1015,1016,986,
                985,987,983,
                1016,983,987,
                1015,986,1031,
                1030,1031,1019,
                1022,1029,1021,
                1019,1031,1018,
                1031,988,1018,
                1017,1018,988,
                1017,989,984,
                985,984,989,
                1017,988,989,
                984,985,983,
                982,988,1031,
                982,1031,986,
                1113,1110,1109,
                1033,1070,1113,
                1109,1033,1113,
                1116,1115,1113,
                1070,1107,1116,
                1113,1070,1116,
                1117,1114,1110,
                1113,1115,1117,
                1110,1113,1117,
                1120,1119,1118,
                1110,1114,1120,
                1118,1110,1120,
                1090,1071,1069,
                1034,1067,1090,
                1069,1034,1090,
                1100,1093,1090,
                1067,1091,1100,
                1090,1067,1100,
                1091,1101,1102,
                1090,1093,1092,
                1111,1108,1106,
                1071,1092,1111,
                1106,1071,1111,
                1112,1107,1070,
                1106,1108,1112,
                1070,1106,1112,
                1106,1070,1033,
                1069,1071,1106,
                1033,1069,1106,
                1035,1064,1068,
                1068,1104,1105,
                1103,1105,1091,
                1103,1067,1034,
                1066,1068,1103,
                1034,1066,1103,
                1063,1065,1064,
                1069,1033,1036,
                1037,1034,1069,
                1036,1037,1069,
                1066,1034,1037,
                1038,1035,1066,
                1037,1038,1066,
                1053,1041,1037,
                1036,1040,1053,
                1037,1036,1053,
                1056,1042,1038,
                1037,1041,1056,
                1038,1037,1056,
                1049,1040,1050,
                1052,1041,1053,
                1054,1053,1040,
                1055,1042,1056,
                1057,1056,1041,
                1059,1058,1042,
                1119,1098,1099,
                1098,1119,1120,
                1098,1120,1114,
                1099,1121,1119,
                1097,1098,1114,
                1096,1097,1115,
                1094,1095,1111,
                1095,1096,1107,
                1117,1097,1114,
                1109,1110,1118,
                1121,1122,1118,
                1096,1116,1107,
                1094,1111,1092,
                1095,1112,1108,
                1096,1115,1116,
                1092,1093,1094,
                1102,1062,1084,
                1102,1085,1086,
                1102,1086,1087,
                1102,1084,1085,
                1061,1064,1060,
                1060,1065,1032,
                1102,1101,1062,
                1061,1101,1105,
                1062,1101,1061,
                1102,1087,1088,
                1102,1088,1089,
                1100,1089,1093,
                1091,1105,1101,
                1061,1105,1104,
                1083,1032,1065,
                1068,1064,1104,
                1061,1104,1064,
                1081,1082,1065,
                1083,1065,1082,
                1064,1065,1060,
                1089,1100,1102,
                1118,1122,1109,
                1109,1123,1124,
                1036,1125,1126,
                1036,1126,1127,
                1036,1124,1125,
                1036,1033,1109,
                1036,1127,1050,
                1124,1036,1109,
                1129,1050,1128,
                1040,1036,1050,
                1127,1128,1050,
                1122,1123,1109,
                1051,1050,1130,
                1043,1051,1131,
                1130,1131,1051,
                1045,1051,1043,
                1054,1049,1046,
                1052,1054,1046,
                1047,1052,1046,
                1049,1051,1045,
                1079,1080,1063,
                1035,1038,1063,
                1038,1078,1063,
                1081,1063,1080,
                1063,1078,1079,
                1038,1039,1078,
                1077,1058,1076,
                1076,1058,1075,
                1077,1039,1038,
                1038,1058,1077,
                1047,1048,1055,
                1048,1059,1055,
                1073,1074,1059,
                1075,1058,1074,
                1072,1059,1044,
                1048,1044,1059,
                1073,1059,1072,
                1038,1042,1058,
                1057,1052,1047,
                1134,1146,1149,
                1145,1147,1146,
                1140,1137,1141,
                1144,1143,1137,
                1149,1132,1150,
                1148,1149,1150,
                1146,1132,1149,
                1152,1134,1148,
                1148,1150,1151,
                1133,1132,1146,
                1148,1151,1152,
                1174,1134,1152,
                1154,1174,1153,
                1155,1175,1154,
                1153,1174,1152,
                1155,1135,1176,
                1147,1173,1133,
                1146,1147,1133,
                1145,1173,1147,
                1145,1172,1173,
                1145,1171,1172,
                1145,1134,1171,
                1174,1175,1169,
                1154,1175,1174,
                1175,1176,1168,
                1175,1168,1169,
                1171,1174,1170,
                1134,1174,1171,
                1135,1156,1176,
                1177,1156,1157,
                1178,1157,1158,
                1177,1157,1178,
                1158,1159,1179,
                1177,1178,1166,
                1176,1177,1167,
                1141,1160,1161,
                1142,1161,1138,
                1141,1137,1179,
                1160,1141,1179,
                1161,1142,1141,
                1142,1138,1140,
                1159,1160,1179,
                1178,1179,1164,
                1166,1167,1177,
                1163,1164,1179,
                1143,1163,1179,
                1139,1162,1144,
                1143,1144,1162,
                1140,1139,1144,
                1139,1140,1138,
                1163,1143,1162,
                1179,1137,1143,
                1184,1208,1211,
                1185,1205,1209,
                1207,1209,1208,
                1204,1206,1205,
                1207,1184,1186,
                1187,1185,1207,
                1186,1187,1207,
                1189,1187,1186,
                1200,1191,1189,
                1188,1190,1200,
                1189,1188,1200,
                1196,1190,1197,
                1199,1191,1200,
                1201,1200,1190,
                1203,1202,1191,
                1211,1180,1212,
                1211,1212,1213,
                1208,1181,1211,
                1180,1211,1181,
                1181,1208,1209,
                1182,1181,1209,
                1210,1211,1213,
                1210,1214,1215,
                1186,1216,1217,
                1186,1215,1216,
                1186,1184,1210,
                1215,1186,1210,
                1186,1217,1218,
                1206,1182,1205,
                1206,1204,1239,
                1183,1182,1206,
                1206,1241,1183,
                1240,1241,1206,
                1187,1204,1185,
                1187,1236,1204,
                1238,1204,1237,
                1239,1204,1238,
                1204,1236,1237,
                1187,1234,1235,
                1235,1236,1187,
                1233,1234,1187,
                1188,1186,1218,
                1188,1219,1220,
                1188,1220,1221,
                1221,1197,1188,
                1223,1197,1222,
                1197,1221,1222,
                1197,1190,1188,
                1187,1189,1233,
                1224,1198,1223,
                1198,1224,1192,
                1193,1198,1192,
                1193,1194,1201,
                1196,1193,1201,
                1193,1196,1198,
                1223,1198,1197,
                1191,1202,1189,
                1232,1189,1231,
                1230,1231,1189,
                1229,1202,1228,
                1202,1229,1230,
                1202,1230,1189,
                1195,1203,1194,
                1199,1194,1203,
                1202,1203,1227,
                1203,1195,1225,
                1225,1226,1203,
                1228,1202,1227,
                1560,1445,1580,
                1580,1531,1530,
                1530,1244,1560,
                1560,1580,1530,
                1445,1528,1582,
                1580,1582,1581,
                1245,1473,1510,
                1505,1464,1510,
                1473,1438,1505,
                1510,1473,1505,
                1536,1446,1509,
                1510,1464,1536,
                1509,1510,1536,
                1508,1510,1509,
                1518,1507,1505,
                1438,1485,1518,
                1505,1438,1518,
                1485,1503,1520,
                1517,1515,1520,
                1503,1442,1517,
                1520,1503,1517,
                1521,1498,1519,
                1520,1515,1521,
                1519,1520,1521,
                1518,1520,1519,
                1507,1519,1535,
                1532,1522,1535,
                1519,1498,1532,
                1535,1519,1532,
                1539,1466,1534,
                1535,1522,1539,
                1534,1535,1539,
                1533,1535,1534,
                1533,1506,1464,
                1505,1507,1533,
                1464,1505,1533,
                1618,1462,1619,
                1619,1516,1614,
                1619,1462,1616,
                1617,1516,1619,
                1616,1617,1619,
                1615,1516,1617,
                1610,1500,1615,
                1617,1610,1615,
                1613,1500,1610,
                1611,1523,1613,
                1610,1611,1613,
                1608,1523,1611,
                1604,1499,1608,
                1611,1604,1608,
                1612,1500,1613,
                1613,1523,1607,
                1609,1608,1499,
                1606,1541,1601,
                1601,1541,1602,
                1603,1602,1526,
                1606,1499,1604,
                1605,1541,1606,
                1604,1605,1606,
                1602,1541,1605,
                1596,1526,1602,
                1605,1596,1602,
                1599,1526,1596,
                1597,1548,1599,
                1596,1597,1599,
                1594,1548,1597,
                1587,1525,1594,
                1597,1587,1594,
                1598,1526,1599,
                1600,1599,1548,
                1593,1548,1594,
                1595,1594,1525,
                1543,1538,1536,
                1464,1506,1543,
                1536,1464,1543,
                1506,1534,1545,
                1542,1540,1545,
                1534,1466,1542,
                1545,1534,1542,
                1546,1524,1544,
                1545,1540,1546,
                1544,1545,1546,
                1543,1545,1544,
                1538,1544,1555,
                1552,1547,1555,
                1544,1524,1552,
                1555,1544,1552,
                1562,1465,1554,
                1555,1547,1562,
                1554,1555,1562,
                1553,1555,1554,
                1553,1537,1446,
                1536,1538,1553,
                1446,1536,1553,
                1567,1558,1556,
                1446,1537,1567,
                1556,1446,1567,
                1537,1554,1569,
                1566,1563,1569,
                1554,1465,1566,
                1569,1554,1566,
                1570,1549,1568,
                1569,1563,1570,
                1568,1569,1570,
                1567,1569,1568,
                1568,1549,1573,
                1573,1529,1557,
                1557,1558,1568,
                1568,1573,1557,
                1556,1558,1557,
                1590,1525,1591,
                1592,1591,1564,
                1583,1564,1584,
                1586,1584,1551,
                1572,1570,1563,
                1570,1572,1571,
                1591,1525,1587,
                1589,1564,1591,
                1587,1589,1591,
                1584,1564,1589,
                1588,1551,1584,
                1589,1588,1584,
                1571,1565,1576,
                1573,1549,1571,
                1576,1573,1571,
                1585,1551,1574,
                1576,1565,1585,
                1574,1576,1585,
                1576,1574,1550,
                1573,1576,1575,
                1575,1550,1579,
                1577,1529,1575,
                1579,1577,1575,
                1577,1579,1578,
                1557,1529,1577,
                1577,1528,1445,
                1445,1527,1557,
                1557,1577,1445,
                1444,1509,1561,
                1556,1527,1561,
                1509,1446,1556,
                1561,1509,1556,
                1561,1527,1445,
                1559,1561,1560,
                1451,1439,1472,
                1472,1436,1246,
                1246,1437,1451,
                1451,1472,1246,
                1437,1452,1453,
                1468,1439,1451,
                1453,1441,1468,
                1451,1453,1468,
                1439,1468,1470,
                1488,1470,1468,
                1441,1447,1488,
                1468,1441,1488,
                1470,1488,1490,
                1488,1447,1457,
                1479,1443,1489,
                1490,1457,1479,
                1489,1490,1479,
                1487,1490,1489,
                1487,1469,1440,
                1467,1470,1487,
                1440,1467,1487,
                1458,1447,1441,
                1448,1450,1458,
                1441,1448,1458,
                1450,1639,1640,
                1637,1459,1640,
                1631,1449,1637,
                1640,1631,1637,
                1480,1457,1447,
                1458,1460,1480,
                1447,1458,1480,
                1636,1460,1458,
                1450,1459,1636,
                1458,1450,1636,
                1460,1636,1638,
                1636,1459,1637,
                1638,1637,1449,
                1457,1480,1481,
                1443,1479,1481,
                1633,1449,1634,
                1635,1634,1483,
                1628,1483,1629,
                1630,1629,1463,
                1634,1449,1631,
                1632,1483,1634,
                1631,1632,1634,
                1629,1483,1632,
                1623,1463,1629,
                1632,1623,1629,
                1626,1463,1623,
                1624,1497,1626,
                1623,1624,1626,
                1621,1497,1624,
                1616,1462,1621,
                1624,1616,1621,
                1625,1463,1626,
                1627,1626,1497,
                1620,1497,1621,
                1622,1621,1462,
                1492,1486,1484,
                1440,1469,1492,
                1484,1440,1492,
                1469,1489,1494,
                1491,1482,1494,
                1489,1443,1491,
                1494,1489,1491,
                1495,1461,1493,
                1494,1482,1495,
                1493,1494,1495,
                1492,1494,1493,
                1486,1493,1504,
                1501,1496,1504,
                1493,1461,1501,
                1504,1493,1501,
                1514,1442,1503,
                1504,1496,1514,
                1503,1504,1514,
                1502,1504,1503,
                1502,1485,1438,
                1484,1486,1502,
                1438,1484,1502,
                1436,1472,1474,
                1467,1440,1474,
                1472,1439,1467,
                1474,1472,1467,
                1484,1438,1473,
                1474,1440,1484,
                1473,1474,1484,
                1471,1474,1473,
                1821,1455,1836,
                1836,1456,1454,
                1454,1247,1821,
                1821,1836,1454,
                1844,1791,1838,
                1836,1455,1844,
                1838,1836,1844,
                1836,1838,1837,
                1454,1456,1437,
                1248,1732,1773,
                1768,1719,1773,
                1732,1707,1768,
                1773,1732,1768,
                1797,1714,1772,
                1773,1719,1797,
                1772,1773,1797,
                1771,1773,1772,
                1781,1770,1768,
                1707,1728,1781,
                1768,1707,1781,
                1728,1766,1783,
                1780,1778,1783,
                1766,1711,1780,
                1783,1766,1780,
                1784,1754,1782,
                1783,1778,1784,
                1782,1783,1784,
                1781,1783,1782,
                1770,1782,1796,
                1793,1785,1796,
                1782,1754,1793,
                1796,1782,1793,
                1800,1721,1795,
                1796,1785,1800,
                1795,1796,1800,
                1794,1796,1795,
                1794,1769,1719,
                1768,1770,1794,
                1719,1768,1794,
                1901,1717,1902,
                1903,1902,1779,
                1896,1779,1897,
                1898,1897,1756,
                1902,1717,1899,
                1900,1779,1902,
                1899,1900,1902,
                1897,1779,1900,
                1891,1756,1897,
                1900,1891,1897,
                1894,1756,1891,
                1892,1786,1894,
                1891,1892,1894,
                1889,1786,1892,
                1885,1755,1889,
                1892,1885,1889,
                1893,1756,1894,
                1895,1894,1786,
                1786,1889,1890,
                1888,1887,1802,
                1802,1883,1884,
                1887,1755,1885,
                1886,1802,1887,
                1885,1886,1887,
                1883,1802,1886,
                1879,1789,1883,
                1886,1879,1883,
                1882,1789,1879,
                1880,1809,1882,
                1879,1880,1882,
                1877,1809,1880,
                1872,1788,1877,
                1880,1872,1877,
                1881,1789,1882,
                1809,1877,1878,
                1878,1877,1788,
                1804,1799,1797,
                1719,1769,1804,
                1797,1719,1804,
                1769,1795,1806,
                1803,1801,1806,
                1795,1721,1803,
                1806,1795,1803,
                1807,1787,1805,
                1806,1801,1807,
                1805,1806,1807,
                1804,1806,1805,
                1799,1805,1816,
                1813,1808,1816,
                1805,1787,1813,
                1816,1805,1813,
                1823,1720,1815,
                1816,1808,1823,
                1815,1816,1823,
                1814,1816,1815,
                1814,1798,1714,
                1797,1799,1814,
                1714,1797,1814,
                1827,1819,1817,
                1714,1798,1827,
                1817,1714,1827,
                1798,1815,1829,
                1826,1824,1829,
                1815,1720,1826,
                1829,1815,1826,
                1830,1810,1828,
                1829,1824,1830,
                1828,1829,1830,
                1827,1829,1828,
                1819,1828,1842,
                1839,1831,1842,
                1828,1810,1839,
                1842,1828,1839,
                1846,1792,1841,
                1842,1831,1846,
                1841,1842,1846,
                1840,1842,1841,
                1840,1818,1790,
                1817,1819,1840,
                1790,1817,1840,
                1874,1788,1875,
                1876,1875,1825,
                1869,1825,1870,
                1871,1870,1812,
                1875,1788,1872,
                1873,1825,1875,
                1872,1873,1875,
                1870,1825,1873,
                1863,1812,1870,
                1873,1863,1870,
                1867,1812,1863,
                1865,1832,1867,
                1863,1865,1867,
                1860,1832,1865,
                1864,1811,1860,
                1865,1864,1860,
                1866,1812,1867,
                1868,1867,1832,
                1859,1832,1860,
                1862,1860,1811,
                1847,1792,1846,
                1792,1847,1855,
                1855,1853,1851,
                1852,1855,1854,
                1851,1849,1835,
                1854,1851,1850,
                1833,1848,1854,
                1850,1833,1854,
                1850,1835,1834,
                1791,1833,1850,
                1834,1791,1850,
                1856,1845,1843,
                1790,1818,1856,
                1843,1790,1856,
                1818,1841,1858,
                1852,1848,1858,
                1841,1792,1852,
                1858,1841,1852,
                1858,1848,1833,
                1856,1858,1857,
                1857,1833,1791,
                1844,1845,1857,
                1791,1844,1857,
                1843,1845,1844,
                1713,1772,1822,
                1817,1790,1822,
                1772,1714,1817,
                1822,1772,1817,
                1843,1455,1821,
                1822,1790,1843,
                1821,1822,1843,
                1820,1822,1821,
                1741,1708,1731,
                1731,1705,1249,
                1249,1706,1741,
                1741,1731,1249,
                1706,1742,1743,
                1762,1708,1741,
                1743,1710,1762,
                1741,1743,1762,
                1708,1762,1763,
                1762,1710,1738,
                1758,1727,1763,
                1738,1712,1758,
                1763,1738,1758,
                1761,1763,1727,
                1710,1739,1740,
                1723,1712,1738,
                1740,1716,1723,
                1738,1740,1723,
                1748,1722,1712,
                1723,1725,1748,
                1712,1723,1748,
                1723,1716,1724,
                1722,1748,1751,
                1748,1725,1749,
                1747,1751,1750,
                1910,1718,1907,
                1908,1753,1910,
                1907,1908,1910,
                1905,1753,1908,
                1899,1717,1905,
                1908,1899,1905,
                1764,1715,1750,
                1909,1718,1910,
                1911,1910,1753,
                1904,1753,1905,
                1906,1905,1717,
                1757,1729,1726,
                1709,1727,1757,
                1726,1709,1757,
                1727,1758,1760,
                1758,1712,1722,
                1747,1715,1759,
                1760,1722,1747,
                1759,1760,1747,
                1757,1760,1759,
                1729,1759,1767,
                1764,1752,1767,
                1759,1715,1764,
                1767,1759,1764,
                1777,1711,1766,
                1767,1752,1777,
                1766,1767,1777,
                1765,1767,1766,
                1765,1728,1707,
                1726,1729,1765,
                1707,1726,1765,
                1705,1731,1733,
                1761,1709,1733,
                1731,1708,1761,
                1733,1731,1761,
                1726,1707,1732,
                1733,1709,1726,
                1732,1733,1726,
                1730,1733,1732,
                1250,1745,1746,
                1744,1746,1706,
                1252,1477,1513,
                1508,1444,1513,
                1477,1245,1508,
                1513,1477,1508,
                1559,1244,1512,
                1513,1444,1559,
                1512,1513,1559,
                1511,1513,1512,
                1253,1476,1478,
                1476,1246,1436,
                1471,1245,1477,
                1478,1436,1471,
                1477,1478,1471,
                1475,1478,1477,
                1476,1775,1454,
                1775,1247,1454,
                1454,1246,1476,
                1476,1253,1254,
                1255,1736,1776,
                1771,1713,1776,
                1736,1248,1771,
                1776,1736,1771,
                1820,1247,1775,
                1776,1713,1820,
                1775,1776,1820,
                1774,1776,1775,
                1256,1735,1737,
                1735,1249,1705,
                1730,1248,1736,
                1737,1705,1730,
                1736,1737,1730,
                1734,1737,1736,
                1744,1249,1735,
                1735,1256,1257,
                1257,1250,1744,
                1744,1735,1257,
                1511,1251,1258,
                1259,1252,1511,
                1258,1259,1511,
                1475,1252,1259,
                1260,1253,1475,
                1259,1260,1475,
                1261,1254,1253,
                1774,1254,1261,
                1262,1255,1774,
                1261,1262,1774,
                1734,1255,1262,
                1263,1256,1734,
                1262,1263,1734,
                1264,1257,1256,
                1266,1259,1258,
                1267,1260,1259,
                1268,1261,1260,
                1269,1262,1261,
                1270,1263,1262,
                1271,1264,1263,
                1273,1266,1265,
                1274,1267,1266,
                1275,1268,1267,
                1276,1269,1268,
                1277,1270,1269,
                1278,1271,1270,
                1280,1273,1272,
                1281,1274,1273,
                1282,1275,1274,
                1283,1276,1275,
                1284,1277,1276,
                1285,1278,1277,
                1287,1280,1279,
                1288,1281,1280,
                1289,1282,1281,
                1290,1283,1282,
                1291,1284,1283,
                1292,1285,1284,
                1294,1287,1286,
                1295,1288,1287,
                1296,1289,1288,
                1297,1290,1289,
                1298,1291,1290,
                1299,1292,1291,
                1301,1294,1293,
                1302,1295,1294,
                1303,1296,1295,
                1304,1297,1296,
                1305,1298,1297,
                1306,1299,1298,
                1308,1301,1300,
                1309,1302,1301,
                1310,1303,1302,
                1311,1304,1303,
                1312,1305,1304,
                1313,1306,1305,
                1315,1308,1307,
                1316,1309,1308,
                1317,1310,1309,
                1318,1311,1310,
                1319,1312,1311,
                1320,1313,1312,
                1322,1315,1314,
                1323,1316,1315,
                1324,1317,1316,
                1325,1318,1317,
                1326,1319,1318,
                1327,1320,1319,
                1588,1242,1578,
                1579,1588,1578,
                1337,1242,1588,
                1588,1579,1550,
                1588,1550,1574,
                1587,1337,1589,
                1588,1589,1337,
                1597,1337,1587,
                1337,1597,1596,
                1337,1596,1605,
                1604,1337,1605,
                1574,1551,1588,
                1565,1586,1585,
                1409,1592,1583,
                1411,1590,1592,
                1583,1586,1408,
                1413,1593,1595,
                1414,1598,1600,
                1416,1601,1603,
                1603,1598,1415,
                1600,1593,1414,
                1606,1418,1499,
                1595,1590,1412,
                1611,1610,1337,
                1610,1617,1337,
                1616,1624,1337,
                1624,1623,1337,
                1617,1616,1337,
                1337,1632,1631,
                1632,1337,1623,
                1337,1631,1640,
                1640,1639,1337,
                1337,1639,1336,
                1448,1336,1639,
                1604,1611,1337,
                1608,1419,1523,
                1523,1419,1607,
                1608,1609,1419,
                1420,1612,1613,
                1418,1609,1499,
                1612,1615,1500,
                1615,1614,1516,
                1639,1450,1448,
                1421,1615,1612,
                1420,1613,1607,
                1336,1838,1834,
                1417,1606,1601,
                1419,1420,1607,
                1418,1419,1609,
                1603,1415,1416,
                1598,1414,1415,
                1418,1606,1417,
                1618,1619,1423,
                1421,1612,1420,
                1422,1615,1421,
                1614,1423,1619,
                1614,1422,1423,
                1615,1422,1614,
                1424,1618,1423,
                1416,1417,1601,
                1406,1565,1405,
                1404,1405,1565,
                1586,1406,1407,
                1404,1571,1403,
                1403,1572,1402,
                1565,1571,1404,
                1595,1412,1413,
                1590,1411,1412,
                1407,1408,1586,
                1409,1410,1592,
                1408,1409,1583,
                1411,1592,1410,
                1406,1586,1565,
                1413,1414,1593,
                1425,1620,1622,
                1426,1627,1620,
                1620,1425,1426,
                1622,1424,1425,
                1424,1622,1618,
                1427,1627,1426,
                1625,1428,1630,
                1630,1429,1628,
                1431,1633,1635,
                1635,1628,1430,
                1633,1433,1638,
                1427,1428,1625,
                1427,1625,1627,
                1430,1431,1635,
                1630,1428,1429,
                1433,1434,1638,
                1432,1433,1633,
                1431,1432,1633,
                1460,1638,1435,
                1435,1638,1434,
                1372,1460,1435,
                1336,1448,1453,
                1480,1460,1372,
                1480,1372,1373,
                1429,1430,1628,
                1581,1578,1242,
                1373,1374,1481,
                1336,1834,1864,
                1834,1835,1864,
                1335,1864,1865,
                1864,1835,1849,
                1853,1861,1849,
                1861,1811,1849,
                1864,1849,1811,
                1335,1863,1873,
                1335,1873,1872,
                1335,1880,1879,
                1335,1872,1880,
                1335,1865,1863,
                1677,1862,1861,
                1678,1859,1862,
                1681,1866,1868,
                1682,1871,1866,
                1868,1859,1680,
                1684,1874,1876,
                1881,1882,1687,
                1809,1878,1686,
                1878,1874,1685,
                1882,1809,1687,
                1876,1869,1683,
                1789,1688,1883,
                1869,1871,1683,
                1885,1335,1886,
                1879,1886,1335,
                1892,1335,1885,
                1891,1335,1892,
                1907,1739,1334,
                1891,1900,1335,
                1899,1335,1900,
                1899,1908,1335,
                1335,1907,1334,
                1908,1907,1335,
                1334,1745,1243,
                1888,1755,1887,
                1890,1889,1755,
                1884,1689,1802,
                1888,1690,1755,
                1890,1755,1691,
                1689,1888,1802,
                1718,1909,1724,
                1907,1718,1724,
                1907,1740,1739,
                1907,1716,1740,
                1692,1895,1786,
                1907,1724,1716,
                1692,1786,1890,
                1688,1884,1883,
                1334,1739,1742,
                1688,1789,1881,
                1336,1864,1335,
                1687,1688,1881,
                1878,1685,1686,
                1876,1683,1684,
                1871,1682,1683,
                1684,1685,1874,
                1689,1690,1888,
                1690,1691,1755,
                1895,1693,1893,
                1692,1693,1895,
                1691,1692,1890,
                1688,1689,1884,
                1861,1674,1675,
                1673,1853,1855,
                1861,1675,1676,
                1853,1674,1861,
                1853,1673,1674,
                1673,1855,1672,
                1671,1672,1847,
                1680,1681,1868,
                1679,1680,1859,
                1677,1678,1862,
                1676,1677,1861,
                1679,1859,1678,
                1851,1853,1849,
                1681,1682,1866,
                1694,1898,1893,
                1695,1896,1898,
                1896,1695,1696,
                1898,1694,1695,
                1694,1893,1693,
                1903,1896,1696,
                1697,1901,1903,
                1901,1698,1906,
                1700,1911,1904,
                1904,1906,1699,
                1911,1702,1909,
                1698,1901,1697,
                1697,1903,1696,
                1701,1911,1700,
                1699,1700,1904,
                1702,1703,1909,
                1701,1702,1911,
                1704,1909,1703,
                1704,1749,1909,
                1725,1724,1749,
                1751,1749,1641,
                1642,1751,1641,
                1749,1704,1641,
                1642,1643,1750,
                1698,1699,1906,
                1909,1749,1724,
                1686,1687,1809,
                1750,1643,1644,
                1374,1375,1481,
                1572,1400,1401,
                1400,1563,1399,
                1566,1398,1399,
                1572,1401,1402,
                1400,1572,1563,
                1566,1397,1398,
                1403,1571,1572,
                1397,1465,1396,
                1396,1562,1395,
                1394,1524,1393,
                1546,1392,1393,
                1547,1394,1395,
                1566,1465,1397,
                1465,1562,1396,
                1563,1566,1399,
                1394,1547,1552,
                1547,1395,1562,
                1394,1552,1524,
                1524,1546,1393,
                1392,1540,1391,
                1546,1540,1392,
                1542,1391,1540,
                1528,1578,1582,
                1582,1578,1581,
                1461,1380,1381,
                1379,1380,1495,
                1385,1442,1384,
                1384,1514,1383,
                1381,1382,1501,
                1377,1443,1376,
                1443,1481,1376,
                1379,1482,1378,
                1491,1377,1378,
                1481,1480,1373,
                1383,1496,1382,
                1539,1389,1390,
                1391,1542,1390,
                1542,1466,1390,
                1390,1466,1539,
                1389,1539,1522,
                1388,1389,1522,
                1498,1387,1388,
                1521,1386,1387,
                1388,1532,1498,
                1386,1521,1515,
                1387,1498,1521,
                1532,1388,1522,
                1385,1386,1515,
                1515,1517,1385,
                1376,1481,1375,
                1338,1530,1531,
                1581,1338,1531,
                1338,1339,1530,
                1339,1340,1251,
                1340,1341,1258,
                1342,1258,1341,
                1244,1530,1512,
                1512,1530,1339,
                1258,1342,1265,
                1265,1342,1343,
                1272,1265,1343,
                1346,1279,1345,
                1347,1286,1346,
                1344,1345,1272,
                1348,1349,1293,
                1350,1300,1349,
                1351,1352,1307,
                1354,1321,1353,
                1353,1314,1352,
                1351,1307,1350,
                1347,1348,1293,
                1328,1321,1354,
                1385,1517,1442,
                1384,1442,1514,
                1382,1496,1501,
                1381,1501,1461,
                1383,1514,1496,
                1379,1495,1482,
                1378,1482,1491,
                1453,1452,1336,
                1441,1453,1448,
                1377,1491,1443,
                1380,1461,1495,
                1346,1286,1279,
                1350,1307,1300,
                1353,1321,1314,
                1330,1331,1322,
                1321,1330,1322,
                1330,1321,1328,
                1437,1456,1837,
                1323,1331,1324,
                1338,1581,1242,
                1452,1837,1336,
                1847,1669,1670,
                1669,1846,1668,
                1831,1667,1668,
                1847,1670,1671,
                1669,1847,1846,
                1831,1666,1667,
                1672,1855,1847,
                1666,1839,1665,
                1665,1810,1664,
                1663,1826,1662,
                1720,1661,1662,
                1830,1663,1664,
                1831,1839,1666,
                1839,1810,1665,
                1846,1831,1668,
                1663,1830,1824,
                1830,1664,1810,
                1663,1824,1826,
                1826,1720,1662,
                1661,1823,1660,
                1720,1823,1661,
                1808,1660,1823,
                1791,1834,1838,
                1336,1837,1838,
                1650,1651,1778,
                1780,1649,1650,
                1800,1654,1655,
                1654,1793,1653,
                1653,1754,1652,
                1646,1764,1645,
                1764,1750,1645,
                1648,1777,1647,
                1750,1751,1642,
                1711,1648,1649,
                1752,1646,1647,
                1652,1784,1651,
                1787,1658,1659,
                1660,1808,1659,
                1808,1813,1659,
                1659,1813,1787,
                1658,1787,1807,
                1657,1658,1807,
                1657,1803,1656,
                1807,1801,1657,
                1801,1803,1657,
                1656,1803,1721,
                1800,1785,1654,
                1721,1800,1655,
                1721,1655,1656,
                1785,1793,1654,
                1653,1793,1754,
                1652,1754,1784,
                1650,1778,1780,
                1649,1780,1711,
                1651,1784,1778,
                1647,1777,1752,
                1646,1752,1764,
                1710,1743,1739,
                1743,1742,1739,
                1648,1711,1777,
                1326,1325,1332,
                1324,1332,1325,
                1332,1324,1331,
                1371,1745,1250,
                1746,1745,1334,
                1264,1271,1367,
                1327,1326,1333,
                1257,1264,1369,
                1250,1257,1370,
                1742,1706,1746,
                1243,1745,1371,
                1370,1371,1250,
                1278,1285,1364,
                1271,1278,1366,
                1285,1292,1363,
                1366,1367,1271,
                1363,1364,1285,
                1292,1299,1362,
                1360,1361,1299,
                1306,1313,1359,
                1313,1320,1357,
                1320,1327,1356,
                1306,1359,1360,
                1327,1329,1355,
                1355,1356,1327,
                1329,1327,1333,
                1746,1334,1742,
                1644,1645,1750,
                1915,1940,1941,
                1936,1938,1937,
                1918,1916,1915,
                1920,1918,1917,
                1931,1922,1920,
                1919,1921,1931,
                1920,1919,1931,
                1927,1921,1928,
                1930,1922,1931,
                1932,1931,1921,
                1934,1933,1922,
                1941,1912,1942,
                1941,1942,1943,
                1940,1935,1941,
                1941,1935,1912,
                1939,1941,1944,
                1913,1935,1940,
                1939,1945,1946,
                1939,1946,1947,
                1947,1948,1917,
                1939,1947,1917,
                1949,1917,1948,
                1944,1945,1939,
                1949,1950,1917,
                1937,1940,1915,
                1938,1913,1937,
                1938,1971,1914,
                1971,1938,1970,
                1938,1936,1970,
                1938,1914,1913,
                1918,1936,1916,
                1968,1936,1918,
                1936,1968,1969,
                1918,1966,1967,
                1967,1968,1918,
                1966,1918,1965,
                1917,1915,1939,
                1919,1917,1950,
                1919,1951,1952,
                1919,1952,1953,
                1919,1953,1928,
                1954,1928,1953,
                1954,1955,1928,
                1956,1928,1955,
                1921,1919,1928,
                1929,1956,1957,
                1929,1958,1923,
                1929,1957,1958,
                1932,1927,1924,
                1929,1924,1927,
                1924,1929,1923,
                1956,1929,1928,
                1920,1964,1965,
                1920,1965,1918,
                1920,1933,1962,
                1920,1963,1964,
                1962,1933,1961,
                1962,1963,1920,
                1922,1933,1920,
                1932,1925,1930,
                1925,1932,1924,
                1925,1934,1930,
                1959,1960,1934,
                1933,1934,1960,
                1934,1926,1959,
                1926,1934,1925,
                1972,1978,1981,
                1981,1980,1973,
                1979,1974,1980,
                1973,1980,1974,
                1986,2001,2000,
                1985,2002,2001,
                1974,1979,2000,
                1987,1990,1988,
                1995,1989,1988,
                1983,2004,2003,
                1994,1982,1989,
                1993,1997,1982,
                1995,1994,1989,
                1982,1997,2004,
                1984,2003,2002,
                1975,2000,2007,
                1976,2006,1977,
                1976,2007,2006,
                1975,2007,1976,
                1997,1993,1998,
                1992,1999,1998,
                1991,1996,1999,
                1993,1992,1998,
                2024,2028,2031,
                2030,2026,2025,
                2029,2026,2030,
                2036,2020,2019,
                2029,2027,2026,
                2009,2008,2037,
                2014,2038,2037,
                2034,2022,2021,
                2014,2013,2038,
                2038,2013,2032,
                2035,2021,2020,
                2033,2023,2022,
                2027,2019,2039,
                2027,2029,2019,
                2023,2033,2032,
                2012,2016,2032,
                2011,2018,2017,
                2016,2012,2017,
                2013,2012,2032,
                2018,2011,2010,
                2102,2101,2109,
                2110,2101,2100,
                2108,2103,2102,
                2100,2099,2111,
                2099,2098,2112,
                2113,2098,2097,
                2114,2097,2096,
                2096,2095,2115,
                2095,2094,2116,
                2093,2117,2094,
                2118,2093,2092,
                2091,2090,2120,
                2091,2119,2092,
                2121,2090,2089,
                2103,2106,2104,
                2087,2086,2124,
                2085,2125,2086,
                2087,2123,2088,
                2127,2084,2083,
                2084,2126,2085,
                2083,2082,2128,
                2081,2129,2082,
                2089,2088,2122,
                2081,2080,2130,
                2079,2078,2132,
                2131,2080,2079,
                2077,2076,2134,
                2075,2074,2136,
                2074,2073,2137,
                2135,2076,2075,
                2133,2078,2077,
                2072,2138,2073,
                2050,2049,2162,
                2049,2048,2163,
                2051,2050,2161,
                2054,2053,2158,
                2053,2052,2159,
                2051,2160,2052,
                2055,2054,2157,
                2042,2169,2043,
                2170,2042,2041,
                2044,2043,2168,
                2046,2165,2047,
                2166,2046,2045,
                2167,2045,2044,
                2048,2047,2164,
                2156,2056,2055,
                2068,2067,2144,
                2066,2065,2146,
                2145,2067,2066,
                2072,2071,2140,
                2070,2069,2142,
                2141,2071,2070,
                2069,2068,2143,
                2058,2153,2059,
                2154,2058,2057,
                2059,2152,2060,
                2148,2064,2063,
                2150,2062,2061,
                2151,2061,2060,
                2062,2149,2063,
                2155,2057,2056,
                2147,2065,2064,
                2041,2040,2105,
                2103,2108,2106,
                2108,2102,2109,
                2110,2100,2111,
                2109,2101,2110,
                2112,2098,2113,
                2114,2096,2115,
                2113,2097,2114,
                2111,2099,2112,
                2094,2117,2116,
                2092,2119,2118,
                2093,2118,2117,
                2090,2121,2120,
                2089,2122,2121,
                2123,2122,2088,
                2091,2120,2119,
                2115,2095,2116,
                2125,2124,2086,
                2127,2126,2084,
                2126,2125,2085,
                2129,2128,2082,
                2081,2130,2129,
                2083,2128,2127,
                2080,2131,2130,
                2133,2132,2078,
                2077,2134,2133,
                2134,2076,2135,
                2074,2137,2136,
                2072,2139,2138,
                2137,2073,2138,
                2135,2075,2136,
                2131,2079,2132,
                2123,2087,2124,
                2139,2072,2140,
                2162,2049,2163,
                2164,2047,2165,
                2163,2048,2164,
                2158,2053,2159,
                2160,2051,2161,
                2159,2052,2160,
                2162,2161,2050,
                2041,2105,2170,
                2170,2105,2171,
                2170,2169,2042,
                2166,2045,2167,
                2168,2043,2169,
                2167,2044,2168,
                2046,2166,2165,
                2158,2157,2054,
                2145,2066,2146,
                2146,2065,2147,
                2147,2064,2148,
                2068,2144,2143,
                2145,2144,2067,
                2142,2069,2143,
                2149,2148,2063,
                2057,2155,2154,
                2056,2156,2155,
                2157,2156,2055,
                2151,2150,2061,
                2153,2152,2059,
                2151,2060,2152,
                2154,2153,2058,
                2149,2062,2150,
                2141,2070,2142,
                2071,2141,2140,
                2107,2106,2108,
                2178,2177,2174,
                2179,2178,2175,
                2181,2180,2177,
                2182,2181,2178,
                2174,2183,2172,
                2177,2185,2184,
                2183,2174,2184,
                2195,2174,2172,
                2193,2176,2173,
                2173,2176,2195,
                2176,2192,2179,
                2193,2192,2176,
                2175,2174,2195,
                2180,2186,2177,
                2187,2186,2180,
                2181,2194,2180,
                2187,2180,2194,
                2194,2181,2188,
                2190,2182,2179,
                2189,2188,2182,
                2190,2189,2182,
                2200,2199,2196,
                2201,2200,2197,
                2203,2202,2199,
                2204,2203,2200,
                2196,2208,2207,
                2209,2208,2196,
                2214,2197,2196,
                2199,2210,2209,
                2210,2199,2211,
                2196,2199,2209,
                2219,2198,2213,
                2214,2213,2198,
                2198,2218,2201,
                2219,2218,2198,
                2214,2198,2197,
                2212,2211,2202,
                2220,2202,2203,
                2202,2220,2205,
                2212,2202,2205,
                2204,2220,2203,
                2216,2204,2201,
                2215,2206,2204,
                2220,2204,2206,
                2216,2215,2204,
                2283,2282,2290,
                2291,2282,2281,
                2289,2284,2283,
                2281,2280,2292,
                2280,2279,2293,
                2294,2279,2278,
                2295,2278,2277,
                2277,2276,2296,
                2276,2275,2297,
                2274,2298,2275,
                2299,2274,2273,
                2272,2271,2301,
                2272,2300,2273,
                2302,2271,2270,
                2284,2287,2285,
                2268,2267,2305,
                2266,2306,2267,
                2268,2304,2269,
                2308,2265,2264,
                2265,2307,2266,
                2264,2263,2309,
                2262,2310,2263,
                2270,2269,2303,
                2262,2261,2311,
                2260,2259,2313,
                2312,2261,2260,
                2258,2257,2315,
                2256,2255,2317,
                2255,2254,2318,
                2316,2257,2256,
                2314,2259,2258,
                2253,2319,2254,
                2231,2230,2343,
                2230,2229,2344,
                2232,2231,2342,
                2235,2234,2339,
                2234,2233,2340,
                2232,2341,2233,
                2236,2235,2338,
                2350,2349,2224,
                2351,2350,2223,
                2225,2224,2349,
                2227,2346,2228,
                2347,2227,2226,
                2348,2226,2225,
                2229,2228,2345,
                2337,2237,2236,
                2249,2248,2325,
                2247,2246,2327,
                2326,2248,2247,
                2253,2252,2321,
                2251,2250,2323,
                2322,2252,2251,
                2250,2249,2324,
                2239,2334,2240,
                2335,2239,2238,
                2240,2333,2241,
                2329,2245,2244,
                2331,2243,2242,
                2332,2242,2241,
                2243,2330,2244,
                2336,2238,2237,
                2328,2246,2245,
                2222,2221,2286,
                2284,2289,2287,
                2289,2283,2290,
                2291,2281,2292,
                2290,2282,2291,
                2293,2279,2294,
                2295,2277,2296,
                2294,2278,2295,
                2292,2280,2293,
                2275,2298,2297,
                2273,2300,2299,
                2274,2299,2298,
                2271,2302,2301,
                2270,2303,2302,
                2304,2303,2269,
                2272,2301,2300,
                2296,2276,2297,
                2306,2305,2267,
                2308,2307,2265,
                2307,2306,2266,
                2310,2309,2263,
                2262,2311,2310,
                2264,2309,2308,
                2261,2312,2311,
                2314,2313,2259,
                2258,2315,2314,
                2315,2257,2316,
                2255,2318,2317,
                2253,2320,2319,
                2318,2254,2319,
                2316,2256,2317,
                2312,2260,2313,
                2304,2268,2305,
                2320,2253,2321,
                2343,2230,2344,
                2345,2228,2346,
                2344,2229,2345,
                2339,2234,2340,
                2341,2232,2342,
                2340,2233,2341,
                2343,2342,2231,
                2222,2286,2351,
                2351,2286,2352,
                2347,2226,2348,
                2348,2225,2349,
                2227,2347,2346,
                2339,2338,2235,
                2326,2247,2327,
                2327,2246,2328,
                2328,2245,2329,
                2249,2325,2324,
                2326,2325,2248,
                2323,2250,2324,
                2330,2329,2244,
                2238,2336,2335,
                2237,2337,2336,
                2338,2337,2236,
                2332,2331,2242,
                2334,2333,2240,
                2332,2241,2333,
                2335,2334,2239,
                2330,2243,2331,
                2322,2251,2323,
                2252,2322,2321,
                2288,2287,2289,
                2,24,23,
                20,21,2,
                25,26,27,
                29,5,25,
                35,44,43,
                42,15,16,
                44,4,14,
                36,45,44,
                38,47,46,
                46,11,12,
                44,13,4,
                46,10,11,
                50,104,103,
                51,102,100,
                100,101,50,
                52,99,97,
                97,98,51,
                94,95,52,
                79,80,81,
                82,83,84,
                84,58,79,
                85,86,87,
                87,59,82,
                89,60,85,
                92,101,102,
                95,90,91,
                114,96,94,
                81,72,73,
                79,75,76,
                77,87,82,
                89,106,107,
                85,77,78,
                119,134,133,
                130,131,119,
                125,126,127,
                129,122,125,
                140,161,160,
                159,154,155,
                161,121,153,
                141,162,161,
                143,164,163,
                163,150,151,
                161,152,121,
                163,149,150,
                167,192,190,
                187,188,168,
                170,167,169,
                172,169,171,
                174,171,173,
                179,180,181,
                184,174,183,
                190,194,195,
                191,185,186,
                167,188,191,
                189,220,221,
                171,199,200,
                172,213,214,
                179,174,173,
                176,183,179,
                184,207,208,
                226,715,713,
                713,714,225,
                227,723,720,
                720,722,686,
                717,748,746,
                746,734,719,
                719,736,733,
                736,693,735,
                733,735,718,
                765,766,767,
                767,741,758,
                758,759,761,
                761,699,760,
                696,732,728,
                732,706,731,
                728,731,703,
                706,698,705,
                707,708,692,
                690,739,737,
                739,692,738,
                737,738,689,
                710,711,688,
                228,822,820,
                820,753,694,
                436,853,851,
                853,862,859,
                860,847,862,
                859,861,818,
                848,849,850,
                847,858,854,
                855,856,858,
                854,857,785,
                844,845,846,
                846,832,839,
                839,840,841,
                841,808,828,
                828,829,830,
                830,816,823,
                823,824,825,
                825,781,812,
                818,866,864,
                864,865,819,
                819,871,869,
                869,870,795,
                795,876,874,
                874,875,796,
                796,881,879,
                879,880,750,
                812,813,814,
                787,744,775,
                775,776,777,
                777,769,770,
                770,771,772,
                772,697,765,
                750,886,884,
                884,885,751,
                751,890,889,
                889,747,717,
                694,754,752,
                752,721,227,
                229,437,435,
                437,838,836,
                230,449,446,
                446,448,417,
                468,616,614,
                614,615,469,
                469,620,619,
                619,464,443,
                497,498,499,
                499,488,492,
                492,493,494,
                494,456,484,
                484,485,486,
                486,471,479,
                479,480,481,
                481,429,472,
                460,461,438,
                472,473,475,
                475,439,474,
                439,477,431,
                443,466,463,
                466,428,465,
                463,465,445,
                442,444,420,
                432,433,419,
                231,566,564,
                565,522,566,
                564,506,425,
                523,581,579,
                524,578,575,
                576,577,578,
                547,587,585,
                572,573,574,
                574,560,567,
                567,568,569,
                569,520,555,
                522,563,561,
                563,591,588,
                588,590,536,
                536,596,594,
                594,595,537,
                537,601,599,
                599,600,503,
                555,556,557,
                557,546,550,
                550,551,552,
                552,521,542,
                542,543,544,
                544,534,538,
                503,606,604,
                604,605,504,
                504,611,609,
                609,610,468,
                425,507,505,
                505,447,230,
                526,584,582,
                582,523,426,
                234,727,724,
                727,226,726,
                724,726,233,
                235,757,755,
                755,725,234,
                756,236,452,
                237,453,450,
                453,229,452,
                450,452,236,
                238,510,508,
                508,451,237,
                240,232,239,
                243,235,242,
                247,239,246,
                248,240,247,
                249,241,248,
                250,242,249,
                251,243,250,
                252,244,251,
                254,246,253,
                255,247,254,
                256,248,255,
                257,249,256,
                258,250,257,
                259,251,258,
                261,253,260,
                262,254,261,
                263,255,262,
                264,256,263,
                265,257,264,
                266,258,265,
                268,260,267,
                269,261,268,
                270,262,269,
                271,263,270,
                272,264,271,
                273,265,272,
                275,267,274,
                276,268,275,
                277,269,276,
                278,270,277,
                279,271,278,
                280,272,279,
                282,274,281,
                283,275,282,
                284,276,283,
                285,277,284,
                286,278,285,
                287,279,286,
                289,281,288,
                290,282,289,
                291,283,290,
                292,284,291,
                293,285,292,
                294,286,293,
                296,288,295,
                297,289,296,
                298,290,297,
                299,291,298,
                300,292,299,
                301,293,300,
                303,295,302,
                304,296,303,
                305,297,304,
                306,298,305,
                307,299,306,
                308,300,307,
                321,239,232,
                253,324,325,
                253,325,326,
                328,274,267,
                330,281,274,
                333,295,288,
                303,311,312,
                313,306,305,
                437,433,837,
                313,307,306,
                245,349,350,
                245,348,349,
                259,345,346,
                280,342,343,
                273,343,344,
                259,346,347,
                238,350,351,
                341,280,287,
                294,339,340,
                294,338,339,
                301,337,338,
                939,966,965,
                958,964,962,
                899,896,898,
                906,907,908,
                909,910,911,
                911,900,906,
                913,901,909,
                944,954,953,
                898,922,923,
                897,973,974,
                913,968,969,
                980,1001,1000,
                1001,1004,1002,
                994,995,993,
                993,999,997,
                997,998,992,
                985,986,987,
                989,982,985,
                998,1001,992,
                990,1003,998,
                979,995,996,
                1023,1027,1028,
                1010,1028,1027,
                1028,1022,1023,
                1030,1012,1013,
                1030,1020,1021,
                1030,1013,1014,
                1030,1019,1020,
                1091,1102,1100,
                1090,1092,1071,
                1035,1068,1066,
                1068,1105,1103,
                1103,1091,1067,
                1063,1064,1035,
                1049,1050,1051,
                1052,1053,1054,
                1054,1040,1049,
                1055,1056,1057,
                1057,1041,1052,
                1059,1042,1055,
                1121,1118,1119,
                1097,1117,1115,
                1095,1108,1111,
                1095,1107,1112,
                1093,1089,1094,
                1081,1065,1063,
                1050,1129,1130,
                1049,1045,1046,
                1047,1055,1057,
                1074,1058,1059,
                1134,1149,1148,
                1145,1146,1134,
                1140,1141,1142,
                1144,1137,1140,
                1155,1176,1175,
                1174,1169,1170,
                1176,1136,1168,
                1156,1177,1176,
                1158,1179,1178,
                1178,1165,1166,
                1176,1167,1136,
                1178,1164,1165,
                1184,1211,1210,
                1185,1209,1207,
                1207,1208,1184,
                1204,1205,1185,
                1189,1186,1188,
                1196,1197,1198,
                1199,1200,1201,
                1201,1190,1196,
                1203,1191,1199,
                1182,1209,1205,
                1210,1213,1214,
                1206,1239,1240,
                1188,1218,1219,
                1189,1232,1233,
                1194,1199,1201,
                1203,1226,1227,
                1445,1582,1580,
                1580,1581,1531,
                1245,1510,1508,
                1508,1509,1444,
                1485,1520,1518,
                1518,1519,1507,
                1507,1535,1533,
                1533,1534,1506,
                1601,1602,1603,
                1603,1526,1598,
                1598,1599,1600,
                1600,1548,1593,
                1593,1594,1595,
                1595,1525,1590,
                1506,1545,1543,
                1543,1544,1538,
                1538,1555,1553,
                1553,1554,1537,
                1537,1569,1567,
                1567,1568,1558,
                1556,1557,1527,
                1590,1591,1592,
                1592,1564,1583,
                1583,1584,1586,
                1586,1551,1585,
                1570,1571,1549,
                1576,1550,1575,
                1573,1575,1529,
                1577,1578,1528,
                1444,1561,1559,
                1561,1445,1560,
                1559,1560,1244,
                1437,1453,1451,
                1439,1470,1467,
                1470,1490,1487,
                1488,1457,1490,
                1487,1489,1469,
                1450,1640,1459,
                1636,1637,1638,
                1638,1449,1633,
                1457,1481,1479,
                1633,1634,1635,
                1635,1483,1628,
                1628,1629,1630,
                1630,1463,1625,
                1625,1626,1627,
                1627,1497,1620,
                1620,1621,1622,
                1622,1462,1618,
                1469,1494,1492,
                1492,1493,1486,
                1486,1504,1502,
                1502,1503,1485,
                1436,1474,1471,
                1471,1473,1245,
                1836,1837,1456,
                1454,1437,1246,
                1248,1773,1771,
                1771,1772,1713,
                1728,1783,1781,
                1781,1782,1770,
                1770,1796,1794,
                1794,1795,1769,
                1901,1902,1903,
                1903,1779,1896,
                1896,1897,1898,
                1898,1756,1893,
                1893,1894,1895,
                1878,1788,1874,
                1769,1806,1804,
                1804,1805,1799,
                1799,1816,1814,
                1814,1815,1798,
                1798,1829,1827,
                1827,1828,1819,
                1819,1842,1840,
                1840,1841,1818,
                1874,1875,1876,
                1876,1825,1869,
                1869,1870,1871,
                1871,1812,1866,
                1866,1867,1868,
                1868,1832,1859,
                1859,1860,1862,
                1862,1811,1861,
                1792,1855,1852,
                1855,1851,1854,
                1852,1854,1848,
                1851,1835,1850,
                1818,1858,1856,
                1858,1833,1857,
                1856,1857,1845,
                1843,1844,1455,
                1713,1822,1820,
                1820,1821,1247,
                1706,1743,1741,
                1708,1763,1761,
                1762,1738,1763,
                1761,1727,1709,
                1710,1740,1738,
                1723,1724,1725,
                1722,1751,1747,
                1748,1749,1751,
                1747,1750,1715,
                1909,1910,1911,
                1911,1753,1904,
                1904,1905,1906,
                1906,1717,1901,
                1727,1760,1757,
                1758,1722,1760,
                1757,1759,1729,
                1729,1767,1765,
                1765,1766,1728,
                1705,1733,1730,
                1730,1732,1248,
                1250,1746,1744,
                1744,1706,1249,
                1252,1513,1511,
                1511,1512,1251,
                1253,1478,1475,
                1476,1436,1478,
                1475,1477,1252,
                1476,1254,1775,
                1255,1776,1774,
                1774,1775,1254,
                1256,1737,1734,
                1735,1705,1737,
                1734,1736,1255,
                1261,1253,1260,
                1264,1256,1263,
                1266,1258,1265,
                1267,1259,1266,
                1268,1260,1267,
                1269,1261,1268,
                1270,1262,1269,
                1271,1263,1270,
                1273,1265,1272,
                1274,1266,1273,
                1275,1267,1274,
                1276,1268,1275,
                1277,1269,1276,
                1278,1270,1277,
                1280,1272,1279,
                1281,1273,1280,
                1282,1274,1281,
                1283,1275,1282,
                1284,1276,1283,
                1285,1277,1284,
                1287,1279,1286,
                1288,1280,1287,
                1289,1281,1288,
                1290,1282,1289,
                1291,1283,1290,
                1292,1284,1291,
                1294,1286,1293,
                1295,1287,1294,
                1296,1288,1295,
                1297,1289,1296,
                1298,1290,1297,
                1299,1291,1298,
                1301,1293,1300,
                1302,1294,1301,
                1303,1295,1302,
                1304,1296,1303,
                1305,1297,1304,
                1306,1298,1305,
                1308,1300,1307,
                1309,1301,1308,
                1310,1302,1309,
                1311,1303,1310,
                1312,1304,1311,
                1313,1305,1312,
                1315,1307,1314,
                1316,1308,1315,
                1317,1309,1316,
                1318,1310,1317,
                1319,1311,1318,
                1320,1312,1319,
                1322,1314,1321,
                1323,1315,1322,
                1324,1316,1323,
                1325,1317,1324,
                1326,1318,1325,
                1327,1319,1326,
                1339,1251,1512,
                1340,1258,1251,
                1272,1343,1344,
                1345,1279,1272,
                1349,1300,1293,
                1352,1314,1307,
                1347,1293,1286,
                1331,1323,1322,
                1437,1837,1452,
                1326,1332,1333,
                1264,1367,1368,
                1264,1368,1369,
                1257,1369,1370,
                1278,1364,1365,
                1278,1365,1366,
                1292,1362,1363,
                1299,1361,1362,
                1360,1299,1306,
                1313,1358,1359,
                1313,1357,1358,
                1320,1356,1357,
                1915,1941,1939,
                1936,1937,1916,
                1918,1915,1917,
                1920,1917,1919,
                1927,1928,1929,
                1930,1931,1932,
                1932,1921,1927,
                1934,1922,1930,
                1941,1943,1944,
                1913,1940,1937,
                1937,1915,1916,
                1936,1969,1970,
                1919,1950,1951,
                1933,1960,1961,
                1972,1981,1973,
                1986,2000,1979,
                1985,2001,1986,
                1974,2000,1975,
                1990,1995,1988,
                1983,2003,1984,
                1993,1982,1994,
                1982,2004,1983,
                1984,2002,1985,
                2006,2005,1977,
                1991,1999,1992,
                2024,2031,2025,
                2030,2025,2031,
                2036,2019,2029,
                2008,2014,2037,
                2034,2021,2035,
                2035,2020,2036,
                2033,2022,2034,
                2023,2032,2016,
                2011,2017,2012,
                2018,2010,2015,
                2178,2174,2175,
                2179,2175,2176,
                2181,2177,2178,
                2182,2178,2179,
                2177,2184,2174,
                2176,2175,2195,
                2192,2191,2179,
                2186,2185,2177,
                2181,2182,2188,
                2190,2179,2191,
                2200,2196,2197,
                2201,2197,2198,
                2203,2199,2200,
                2204,2200,2201,
                2214,2196,2207,
                2199,2202,2211,
                2218,2217,2201,
                2216,2201,2217,
                2350,2224,2223,
                2351,2223,2222,
            ]
        );

        let actual_result = input.get_planar_simplify(0.001, 0.001);
        let actual = actual_result.unwrap();

        let expected = Mesh::new(
            vec![
                -1.0640540001862646, 3.759783999845046, 8.768348000023437, 0.3359460001862644, 3.759783999845046, 8.768348000023437, -1.0640540001862646, 3.6608670001549535, 8.783309999976561, 0.3359460001862644, 3.6608670001549535, 8.783309999976561, -1.0640540001862646, 3.5675919996409373, 8.80695700009103, 0.3359460001862644, 3.5675919996409373, 8.80695700009103, 0.764908996810914, 13.907396, 22.305292, 9.635946003189087, 13.907396, 22.305292, 0.764908996810914, 13.907396, 20.305292, 9.635946003189087, 13.907396, 20.305292, 0.764909, -1.7501290014781956, 22.305292, 0.764909, -1.7501290014781956, 20.305292, -10.364054, -1.750129, 22.305292, -10.364054, -1.750129, 20.305292, -1.0640540029802317, -1.750129, 20.305292, 0.3359460029802328, -1.750129, 20.305292, -10.364054, -1.750129, 2.3052920000000015, -1.0640540029802317, -1.750129, 2.305292000000001, 0.3359460029802328, -1.750129, 2.305292000000001, 9.635946000000002, -1.750129, 2.305292, -10.364054, -1.750129, 0.3052920000000015, 9.635946000000002, -1.750129, 0.3052920000000003, -10.364054, 367.549866, 0.3052919999999996, 9.635946000000002, 367.549866, 0.30529200000000084, -10.364054, 367.549866, 2.3052919999999997, -1.0640540029802317, 367.549866, 2.305292, 0.3359460029802328, 367.549866, 2.305292, 9.635946000000002, 367.549866, 2.305292000000001, -10.364054, 367.549866, 20.305291999999998, -1.0640540029802317, 367.549866, 20.305291999999998, 0.3359460029802328, 367.549866, 20.305291999999998, 9.635946000000002, 367.549866, 20.305291999999998, -10.364054, 367.549866, 22.305291999999998, 9.635946000000002, 367.549866, 22.305291999999998, -1.064054, 4.859701168304446, 9.763317932312003, -1.064054, 4.859701168304445, 16.16331898135375, -1.064054, 4.854806911712649, 9.664481033508292, -1.064054, 4.854806911712649, 9.862154831115713, -1.064054, 4.854806911712648, 16.06448208255004, -1.064054, 4.854806911712648, 16.26215588015746, -1.064054, 4.840308201080325, 9.56731878680419, -1.064054, 4.840308201080325, 9.959317077819815, -1.064054, 4.840308201080324, 15.967318882171622, -1.064054, 4.840308201080324, 16.359318126861563, -1.064054, 4.8164739725647, 9.47248588961791, -1.064054, 4.8164739725647, 10.054149975006094, -1.064054, 4.816473972564699, 15.872485984985342, -1.064054, 4.816473972564699, 16.45414911669921, -1.064054, 4.783577930694583, 9.380639900390616, -1.064054, 4.783577930694583, 10.14599596423339, -1.064054, 4.783577930694582, 15.780639995758047, -1.064054, 4.783577930694582, 16.54599605960082, -1.064054, 4.741890918975833, 9.29243503970336, -1.064054, 4.741890918975833, 10.234200824920645, -1.064054, 4.741890918975832, 15.692434181396475, -1.064054, 4.741890918975832, 16.634200920288077, -1.064054, 4.691685211425784, 9.208526958648672, -1.064054, 4.691685211425784, 10.318109859649649, -1.064054, 4.691685211425783, 15.608526100341788, -1.064054, 4.691685211425783, 16.718109001342764, -1.064054, 4.633232128387454, 9.129570831481924, -1.064054, 4.633232128387454, 10.39706503314208, -1.064054, 4.633232128387453, 15.529570926849356, -1.064054, 4.633232128387453, 16.79706608218383, -1.064054, 4.566802036529544, 9.056224216644278, -1.064054, 4.566802036529544, 10.470411171142569, -1.064054, 4.566802036529543, 15.456223835174551, -1.064054, 4.566802036529543, 16.870410789672842, -1.064054, 4.493997108703616, 8.990248073760977, -1.064054, 4.493997108703615, 15.390248169128409, -1.064054, 4.493987095123294, 10.536395897094717, -1.064054, 4.493987095123293, 16.93639599246215, -1.064054, 4.4152650950012235, 8.93186603945922, -1.064054, 4.415265095001223, 15.331866134826651, -1.064054, 4.415249836212161, 10.594779838745108, -1.064054, 4.41524983621216, 16.99477993411254, -1.064054, 4.331195842987063, 8.88147198123168, -1.064054, 4.3311958429870625, 15.281472076599112, -1.064054, 4.331182968383792, 10.645170082275381, -1.064054, 4.331182968383791, 17.045170177642813, -1.064054, 4.242379200225833, 8.839459766571036, -1.064054, 4.242379200225833, 10.68717609805297, -1.064054, 4.242379200225832, 15.239459861938467, -1.064054, 4.242379200225832, 17.087174762908926, -1.064054, 4.15180922723389, 10.719677795593253, -1.064054, 4.151809227233889, 17.119677890960684, -1.064054, 4.149981033569339, 8.806401123229971, -1.064054, 4.149981033569338, 15.206401218597403, -1.064054, 4.05853415704346, 10.743326057617178, -1.064054, 4.058534157043459, 17.14332615298461, -1.064054, 4.054788124328616, 8.782556881134024, -1.064054, 4.054788124328615, 15.182556976501456, -1.064054, 3.9596171495971704, 10.758286823455801, -1.064054, 3.95961714959717, 17.158286918823233, -1.064054, 3.9577159998474145, 8.76815878314208, -1.064054, 3.957715999847414, 15.168158878509512, -1.064054, 3.8597011683044458, 8.763344158355704, -1.064054, 3.8597011683044458, 10.763291229431143, -1.064054, 3.8597011683044453, 15.16334282321166, -1.064054, 3.8597011683044453, 17.163291801635733, -1.064054, 3.7616849062500024, 10.758477081481924, -1.064054, 3.761684906250002, 17.15847622317504, -1.064054, 3.7597842333374043, 15.168348182861319, -1.064054, 3.6646127817688012, 10.744078029815665, -1.064054, 3.664612781768801, 17.14407717150878, -1.064054, 3.6608672258911152, 15.183309902374258, -1.064054, 3.5694198725280786, 10.720235218231192, -1.064054, 3.569419872528078, 17.12023388308715, -1.064054, 3.5675921557006856, 15.206958164398184, -1.064054, 3.4770221827087426, 8.839459766571036, -1.064054, 3.4770221827087426, 10.687175144378653, -1.064054, 3.477022182708742, 15.239459861938467, -1.064054, 3.477022182708742, 17.087174762908926, -1.064054, 3.3882179377136254, 8.881465782348624, -1.064054, 3.388217937713625, 15.281465877716055, -1.064054, 3.3882060167846704, 10.645163883392325, -1.064054, 3.38820601678467, 17.045163978759756, -1.064054, 3.3041510698852563, 8.931856025878897, -1.064054, 3.304151069885256, 15.331856121246329, -1.064054, 3.304135811096194, 10.59476887149047, -1.064054, 3.3041358110961934, 16.994769920532217, -1.064054, 3.22541476464844, 8.990239013854971, -1.064054, 3.2254147646484395, 15.390239109222403, -1.064054, 3.2254037973938012, 10.536386837188711, -1.064054, 3.225403797393801, 16.93638788623046, -1.064054, 3.1525988695678735, 9.056224216644278, -1.064054, 3.1525988695678735, 10.470411171142569, -1.064054, 3.152598869567873, 15.456223835174551, -1.064054, 3.152598869567873, 16.870410789672842, -1.064054, 3.086170208221438, 9.129570831481924, -1.064054, 3.086170208221438, 10.39706503314208, -1.064054, 3.0861702082214375, 15.529570926849356, -1.064054, 3.0861702082214375, 16.797064174835196, -1.064054, 3.0277161715087915, 9.208526004974356, -1.064054, 3.0277161715087915, 10.318109859649649, -1.064054, 3.027716171508791, 15.608526100341788, -1.064054, 3.027716171508791, 16.718109001342764, -1.064054, 2.9775099871215844, 9.292434086029044, -1.064054, 2.9775099871215844, 10.234200824920645, -1.064054, 2.977509987121584, 15.692434181396475, -1.064054, 2.977509987121584, 16.634200920288077, -1.064054, 2.9358229754028344, 9.3806389467163, -1.064054, 2.9358229754028344, 10.14599596423339, -1.064054, 2.935822975402834, 15.780639995758047, -1.064054, 2.935822975402834, 16.54599605960082, -1.064054, 2.9029269335327172, 9.47248588961791, -1.064054, 2.9029269335327172, 10.054149975006094, -1.064054, 2.902926933532717, 15.872485984985342, -1.064054, 2.902926933532717, 16.45414911669921, -1.064054, 2.8790931818542504, 9.56731878680419, -1.064054, 2.8790931818542504, 9.959317077819815, -1.064054, 2.87909318185425, 15.967318882171622, -1.064054, 2.87909318185425, 16.359318126861563, -1.064054, 2.864593994384768, 9.664481033508292, -1.064054, 2.864593994384768, 9.862153877441397, -1.064054, 2.8645939943847676, 16.06448208255004, -1.064054, 2.8645939943847676, 16.26215397280883, -1.064054, 2.8597011683044458, 9.763317932312003, -1.064054, 2.8597011683044453, 16.16331898135375, 0.335946, 4.859701168304437, 16.163318981353772, 0.335946, 4.859701168304436, 9.763317932312024, 0.335946, 4.8548069117126404, 16.262155880157483, 0.335946, 4.8548069117126404, 16.06448208255006, 0.335946, 4.85480691171264, 9.862154831115735, 0.335946, 4.85480691171264, 9.664481033508313, 0.335946, 4.840308201080316, 16.359318126861584, 0.335946, 4.840308201080316, 15.967318882171643, 0.335946, 4.840308201080315, 9.959317077819836, 0.335946, 4.840308201080315, 9.567318786804211, 0.335946, 4.816473972564691, 16.45414911669923, 0.335946, 4.816473972564691, 15.872485984985364, 0.335946, 4.81647397256469, 10.054149975006116, 0.335946, 4.81647397256469, 9.472485889617932, 0.335946, 4.783577930694574, 16.545996059600842, 0.335946, 4.783577930694574, 15.780639995758069, 0.335946, 4.783577930694573, 10.14599596423341, 0.335946, 4.783577930694573, 9.380639900390637, 0.335946, 4.741890918975824, 16.634200920288098, 0.335946, 4.741890918975824, 15.692434181396496, 0.335946, 4.741890918975823, 10.234200824920666, 0.335946, 4.741890918975823, 9.292435039703381, 0.335946, 4.691685211425775, 16.718109001342786, 0.335946, 4.691685211425775, 15.608526100341809, 0.335946, 4.691685211425774, 10.31810985964967, 0.335946, 4.691685211425774, 9.208526958648694, 0.335946, 4.633232128387445, 16.79706608218385, 0.335946, 4.633232128387445, 15.529570926849377, 0.335946, 4.633232128387444, 10.397065033142102, 0.335946, 4.633232128387444, 9.129570831481946, 0.335946, 4.566802036529535, 16.870410789672864, 0.335946, 4.566802036529535, 15.456223835174573, 0.335946, 4.566802036529534, 10.47041117114259, 0.335946, 4.566802036529534, 9.0562242166443, 0.335946, 4.493997108703607, 15.39024816912843, 0.335946, 4.493997108703606, 8.990248073760998, 0.335946, 4.493987095123285, 16.93639599246217, 0.335946, 4.493987095123284, 10.536395897094739, 0.335946, 4.415265095001215, 15.331866134826672, 0.335946, 4.415265095001214, 8.93186603945924, 0.335946, 4.415249836212152, 16.99477993411256, 0.335946, 4.415249836212151, 10.59477983874513, 0.335946, 4.3311958429870545, 15.281472076599133, 0.335946, 4.331195842987054, 8.881471981231702, 0.335946, 4.331182968383783, 17.045170177642834, 0.335946, 4.331182968383782, 10.645170082275403, 0.335946, 4.242379200225824, 17.087174762908948, 0.335946, 4.242379200225824, 15.239459861938489, 0.335946, 4.242379200225823, 10.68717609805299, 0.335946, 4.242379200225823, 8.839459766571057, 0.335946, 4.151809227233881, 17.119677890960705, 0.335946, 4.15180922723388, 10.719677795593274, 0.335946, 4.14998103356933, 15.206401218597424, 0.335946, 4.149981033569329, 8.806401123229993, 0.335946, 4.058534157043451, 17.14332615298463, 0.335946, 4.05853415704345, 10.7433260576172, 0.335946, 4.054788124328607, 15.182556976501477, 0.335946, 4.054788124328606, 8.782556881134045, 0.335946, 3.9596171495971615, 17.158286918823254, 0.335946, 3.9596171495971615, 10.758286823455823, 0.335946, 3.9577159998474056, 15.168158878509534, 0.335946, 3.957715999847405, 8.768158783142102, 0.335946, 3.859701168304437, 17.163291801635754, 0.335946, 3.859701168304437, 15.163342823211682, 0.335946, 3.859701168304437, 10.763291229431164, 0.335946, 3.8597011683044364, 8.763344158355725, 0.335946, 3.7616849062499935, 17.15847622317506, 0.335946, 3.7616849062499935, 10.758477081481946, 0.335946, 3.759784233337396, 15.16834818286134, 0.335946, 3.6646127817687923, 17.1440771715088, 0.335946, 3.6646127817687923, 10.744078029815686, 0.335946, 3.660867225891107, 15.18330990237428, 0.335946, 3.5694198725280697, 17.12023388308717, 0.335946, 3.5694198725280697, 10.720235218231213, 0.335946, 3.567592155700677, 15.206958164398205, 0.335946, 3.4770221827087338, 17.087174762908948, 0.335946, 3.4770221827087338, 15.239459861938489, 0.335946, 3.4770221827087338, 10.687175144378674, 0.335946, 3.4770221827087333, 8.839459766571057, 0.335946, 3.3882179377136166, 15.281465877716077, 0.335946, 3.388217937713616, 8.881465782348645, 0.335946, 3.3882060167846615, 17.045163978759778, 0.335946, 3.3882060167846615, 10.645163883392346, 0.335946, 3.3041510698852474, 15.33185612124635, 0.335946, 3.304151069885247, 8.931856025878918, 0.335946, 3.304135811096185, 16.99476992053224, 0.335946, 3.304135811096185, 10.59476887149049, 0.335946, 3.225414764648431, 15.390239109222424, 0.335946, 3.2254147646484306, 8.990239013854993, 0.335946, 3.2254037973937923, 16.93638788623048, 0.335946, 3.2254037973937923, 10.536386837188733, 0.335946, 3.1525988695678646, 16.870410789672864, 0.335946, 3.1525988695678646, 15.456223835174573, 0.335946, 3.1525988695678646, 10.47041117114259, 0.335946, 3.152598869567864, 9.0562242166443, 0.335946, 3.086170208221429, 16.797064174835217, 0.335946, 3.086170208221429, 15.529570926849377, 0.335946, 3.086170208221429, 10.397065033142102, 0.335946, 3.0861702082214286, 9.129570831481946, 0.335946, 3.0277161715087826, 16.718109001342786, 0.335946, 3.0277161715087826, 15.608526100341809, 0.335946, 3.0277161715087826, 10.31810985964967, 0.335946, 3.027716171508782, 9.208526004974377, 0.335946, 2.9775099871215756, 16.634200920288098, 0.335946, 2.9775099871215756, 15.692434181396496, 0.335946, 2.9775099871215756, 10.234200824920666, 0.335946, 2.977509987121575, 9.292434086029065, 0.335946, 2.9358229754028256, 16.545996059600842, 0.335946, 2.9358229754028256, 15.780639995758069, 0.335946, 2.9358229754028256, 10.14599596423341, 0.335946, 2.935822975402825, 9.38063894671632, 0.335946, 2.9029269335327084, 16.45414911669923, 0.335946, 2.9029269335327084, 15.872485984985364, 0.335946, 2.902926933532708, 10.054149975006116, 0.335946, 2.902926933532708, 9.472485889617932, 0.335946, 2.8790931818542416, 16.359318126861584, 0.335946, 2.8790931818542416, 15.967318882171643, 0.335946, 2.879093181854241, 9.959317077819836, 0.335946, 2.879093181854241, 9.567318786804211, 0.335946, 2.864593994384759, 16.26215397280885, 0.335946, 2.864593994384759, 16.06448208255006, 0.335946, 2.8645939943847587, 9.862153877441418, 0.335946, 2.8645939943847587, 9.664481033508313, 0.335946, 2.859701168304437, 16.163318981353772, 0.335946, 2.8597011683044364, 9.763317932312024
            ],
            vec![
                2, 1, 0, 3, 1, 2, 4, 3, 2, 5, 3, 4, 8, 7, 6, 9, 7, 8, 11, 6, 10, 8, 6, 11, 13, 10, 12, 14, 10, 13, 15, 10, 14, 11, 10, 15, 17, 15, 14, 18, 15, 17, 20, 17, 16, 20, 18, 17, 20, 19, 18, 21, 19, 20, 24, 23, 22, 25, 23, 24, 26, 23, 25, 27, 23, 26, 29, 26, 25, 30, 26, 29, 32, 29, 28, 32, 30, 29, 32, 31, 30, 33, 31, 32, 16, 25, 24, 17, 25, 16, 34, 29, 25, 35, 29, 34, 36, 34, 25, 37, 35, 34, 38, 35, 37, 39, 29, 35, 40, 36, 25, 41, 38, 37, 42, 38, 41, 43, 29, 39, 44, 40, 25, 45, 42, 41, 46, 42, 45, 47, 29, 43, 48, 44, 25, 49, 46, 45, 50, 46, 49, 51, 29, 47, 52, 48, 25, 53, 50, 49, 54, 50, 53, 55, 29, 51, 56, 52, 25, 57, 54, 53, 58, 54, 57, 59, 29, 55, 60, 56, 25, 61, 58, 57, 62, 58, 61, 63, 29, 59, 64, 60, 25, 65, 62, 61, 66, 62, 65, 67, 29, 63, 68, 64, 25, 69, 66, 65, 70, 69, 65, 71, 29, 67, 72, 68, 25, 73, 69, 70, 74, 73, 70, 75, 29, 71, 76, 72, 25, 77, 73, 74, 78, 77, 74, 79, 29, 75, 80, 76, 25, 81, 77, 78, 82, 77, 81, 83, 29, 79, 84, 82, 81, 85, 29, 83, 86, 80, 25, 87, 82, 84, 88, 87, 84, 89, 29, 85, 90, 86, 25, 91, 87, 88, 92, 91, 88, 93, 29, 89, 94, 90, 25, 95, 91, 92, 96, 94, 25, 97, 95, 92, 98, 95, 97, 99, 29, 93, 100, 98, 97, 102, 98, 100, 103, 102, 100, 105, 102, 103, 106, 105, 103, 108, 105, 106, 110, 108, 106, 111, 108, 110, 114, 111, 110, 115, 114, 110, 118, 114, 115, 119, 118, 115, 122, 118, 119, 123, 122, 119, 126, 122, 123, 127, 122, 126, 130, 127, 126, 131, 127, 130, 134, 131, 130, 135, 131, 134, 138, 135, 134, 139, 135, 138, 142, 139, 138, 143, 139, 142, 146, 143, 142, 147, 143, 146, 150, 147, 146, 151, 147, 150, 154, 151, 150, 155, 151, 154, 157, 155, 154, 158, 155, 157, 17, 96, 25, 17, 0, 96, 17, 2, 0, 17, 4, 2, 17, 109, 4, 17, 113, 109, 17, 117, 113, 17, 121, 117, 17, 125, 121, 17, 129, 125, 17, 133, 129, 17, 137, 133, 17, 141, 137, 17, 145, 141, 17, 149, 145, 17, 153, 149, 17, 157, 153, 17, 158, 157, 17, 156, 158, 17, 152, 156, 17, 148, 152, 14, 29, 99, 14, 99, 101, 14, 101, 104, 14, 104, 107, 14, 107, 112, 14, 112, 116, 14, 116, 120, 14, 120, 124, 14, 124, 128, 14, 128, 132, 14, 132, 136, 14, 136, 140, 14, 140, 144, 14, 144, 148, 14, 148, 17, 14, 28, 29, 13, 28, 14, 13, 32, 28, 12, 32, 13, 10, 32, 12, 6, 32, 10, 7, 32, 6, 33, 32, 7, 7, 31, 33, 9, 31, 7, 8, 31, 9, 15, 8, 11, 15, 31, 8, 30, 31, 15, 159, 26, 30, 160, 26, 159, 161, 159, 30, 162, 160, 159, 163, 160, 162, 164, 26, 160, 165, 161, 30, 166, 163, 162, 167, 163, 166, 168, 26, 164, 169, 165, 30, 170, 167, 166, 171, 167, 170, 172, 26, 168, 173, 169, 30, 174, 171, 170, 175, 171, 174, 176, 26, 172, 177, 173, 30, 178, 175, 174, 179, 175, 178, 180, 26, 176, 181, 177, 30, 182, 179, 178, 183, 179, 182, 184, 26, 180, 185, 181, 30, 186, 183, 182, 187, 183, 186, 188, 26, 184, 189, 185, 30, 190, 187, 186, 191, 187, 190, 192, 26, 188, 193, 191, 190, 194, 26, 192, 195, 189, 30, 196, 191, 193, 197, 196, 193, 198, 26, 194, 199, 195, 30, 200, 196, 197, 201, 200, 197, 202, 26, 198, 203, 199, 30, 204, 200, 201, 205, 203, 30, 206, 204, 201, 207, 204, 206, 208, 26, 202, 209, 205, 30, 210, 207, 206, 211, 210, 206, 212, 26, 208, 213, 209, 30, 214, 210, 211, 215, 214, 211, 216, 26, 212, 217, 213, 30, 218, 214, 215, 219, 218, 215, 220, 26, 216, 221, 217, 30, 222, 218, 219, 223, 218, 222, 224, 26, 220, 226, 223, 222, 227, 226, 222, 229, 226, 227, 230, 229, 227, 232, 229, 230, 233, 232, 230, 235, 232, 233, 236, 232, 235, 238, 236, 235, 241, 236, 238, 242, 241, 238, 245, 241, 242, 246, 245, 242, 249, 245, 246, 251, 249, 246, 252, 249, 251, 255, 252, 251, 256, 252, 255, 259, 256, 255, 260, 256, 259, 263, 260, 259, 264, 260, 263, 267, 264, 263, 268, 264, 267, 271, 268, 267, 272, 268, 271, 275, 272, 271, 276, 272, 275, 279, 276, 275, 280, 276, 279, 282, 280, 279, 283, 280, 282, 15, 221, 30, 15, 225, 221, 15, 228, 225, 15, 231, 228, 15, 234, 231, 15, 240, 234, 15, 244, 240, 15, 248, 244, 15, 250, 248, 15, 254, 250, 15, 258, 254, 15, 262, 258, 15, 266, 262, 15, 270, 266, 15, 274, 270, 15, 278, 274, 15, 282, 278, 15, 283, 282, 15, 281, 283, 15, 277, 281, 15, 273, 277, 15, 269, 273, 18, 26, 224, 18, 224, 1, 18, 1, 3, 18, 3, 5, 18, 5, 237, 18, 237, 239, 18, 239, 243, 18, 243, 247, 18, 247, 253, 18, 253, 257, 18, 257, 261, 18, 261, 265, 18, 265, 269, 18, 269, 15, 18, 27, 26, 19, 27, 18, 19, 23, 27, 21, 23, 19, 21, 22, 23, 20, 22, 21, 20, 24, 22, 16, 24, 20, 94, 224, 220, 96, 224, 94, 229, 103, 100, 226, 229, 100, 232, 103, 229, 106, 103, 232, 264, 138, 134, 260, 264, 134, 280, 154, 150, 276, 280, 150, 146, 268, 272, 142, 268, 146, 150, 272, 276, 146, 272, 150, 268, 142, 138, 264, 268, 138, 277, 153, 281, 149, 153, 277, 281, 153, 157, 283, 281, 157, 283, 157, 154, 280, 283, 154, 134, 256, 260, 130, 256, 134, 236, 106, 232, 110, 106, 236, 245, 119, 115, 241, 245, 115, 241, 110, 236, 115, 110, 241, 256, 130, 126, 252, 256, 126, 252, 126, 123, 249, 252, 123, 249, 119, 245, 123, 119, 249, 273, 149, 277, 145, 149, 273, 239, 113, 117, 243, 239, 117, 5, 109, 237, 4, 109, 5, 237, 109, 113, 239, 237, 113, 224, 0, 1, 96, 0, 224, 257, 133, 261, 129, 133, 257, 137, 269, 265, 141, 269, 137, 261, 137, 265, 133, 137, 261, 269, 145, 273, 141, 145, 269, 247, 125, 253, 121, 125, 247, 117, 247, 243, 121, 247, 117, 253, 129, 257, 125, 129, 253, 226, 97, 223, 100, 97, 226, 37, 160, 163, 34, 160, 37, 191, 70, 65, 196, 70, 191, 207, 84, 81, 210, 84, 207, 218, 88, 214, 92, 88, 218, 97, 218, 223, 92, 218, 97, 214, 84, 210, 88, 84, 214, 207, 78, 204, 81, 78, 207, 200, 78, 74, 204, 78, 200, 200, 70, 196, 74, 70, 200, 191, 61, 187, 65, 61, 191, 175, 53, 49, 179, 53, 175, 61, 183, 187, 57, 183, 61, 183, 53, 179, 57, 53, 183, 45, 167, 171, 41, 167, 45, 49, 171, 175, 45, 171, 49, 167, 37, 163, 41, 37, 167, 188, 64, 192, 60, 64, 188, 44, 176, 172, 48, 176, 44, 160, 36, 164, 34, 36, 160, 168, 36, 40, 164, 36, 168, 168, 44, 172, 40, 44, 168, 52, 184, 180, 56, 184, 52, 180, 48, 52, 176, 48, 180, 184, 60, 188, 56, 60, 184, 202, 80, 208, 76, 80, 202, 198, 68, 72, 194, 68, 198, 194, 64, 68, 192, 64, 194, 198, 76, 202, 72, 76, 198, 212, 90, 216, 86, 90, 212, 212, 80, 86, 208, 80, 212, 220, 90, 94, 216, 90, 220, 95, 222, 219, 98, 222, 95, 228, 104, 101, 225, 228, 101, 231, 104, 228, 107, 104, 231, 262, 140, 136, 258, 262, 136, 278, 156, 152, 274, 278, 152, 148, 266, 270, 144, 266, 148, 152, 270, 274, 148, 270, 152, 266, 144, 140, 262, 266, 140, 275, 155, 279, 151, 155, 275, 279, 155, 158, 282, 279, 158, 282, 158, 156, 278, 282, 156, 136, 254, 258, 132, 254, 136, 234, 107, 231, 112, 107, 234, 244, 120, 116, 240, 244, 116, 240, 112, 234, 116, 112, 240, 254, 132, 128, 250, 254, 128, 250, 128, 124, 248, 250, 124, 248, 120, 244, 124, 120, 248, 271, 151, 275, 147, 151, 271, 238, 114, 118, 242, 238, 118, 233, 111, 235, 108, 111, 233, 105, 233, 230, 108, 233, 105, 235, 111, 114, 238, 235, 114, 227, 102, 105, 230, 227, 105, 222, 102, 227, 98, 102, 222, 255, 135, 259, 131, 135, 255, 139, 267, 263, 143, 267, 139, 259, 139, 263, 135, 139, 259, 267, 147, 271, 143, 147, 267, 246, 127, 251, 122, 127, 246, 118, 246, 242, 122, 246, 118, 251, 131, 255, 127, 131, 251, 225, 99, 221, 101, 99, 225, 39, 159, 161, 35, 159, 39, 189, 71, 67, 195, 71, 189, 205, 85, 83, 209, 85, 205, 217, 89, 213, 93, 89, 217, 99, 217, 221, 93, 217, 99, 213, 85, 209, 89, 85, 213, 205, 79, 203, 83, 79, 205, 199, 79, 75, 203, 79, 199, 199, 71, 195, 75, 71, 199, 189, 63, 185, 67, 63, 189, 173, 55, 51, 177, 55, 173, 63, 181, 185, 59, 181, 63, 181, 55, 177, 59, 55, 181, 47, 165, 169, 43, 165, 47, 51, 169, 173, 47, 169, 51, 165, 39, 161, 43, 39, 165, 186, 66, 190, 62, 66, 186, 46, 174, 170, 50, 174, 46, 159, 38, 162, 35, 38, 159, 166, 38, 42, 162, 38, 166, 166, 46, 170, 42, 46, 166, 54, 182, 178, 58, 182, 54, 178, 50, 54, 174, 50, 178, 182, 62, 186, 58, 62, 182, 201, 82, 206, 77, 82, 201, 197, 69, 73, 193, 69, 197, 193, 66, 69, 190, 66, 193, 197, 77, 201, 73, 77, 197, 211, 91, 215, 87, 91, 211, 211, 82, 87, 206, 82, 211, 219, 91, 95, 215, 91, 219
            ]
        );

        println!("{:?}", actual);

        assert!(expected.eq(&actual));
    }

    #[test]
    fn test_get_polygons_for_planar_mesh_star() {
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
                Point::new(67.691681, 34.023804, -5.037391),
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
            ]),
        ];

        assert_eq!(actual.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn test_get_polygons_for_planar_mesh_triangle() {
        let mesh = Mesh::new(
            vec![
                -17.906073, 18.384705, 0.0,
                -19.969576, -4.996209, 0.0,
                -41.047173, 20.427044, 0.0,
                -15.842568, 41.765617, 0.0,
                -13.779064, 65.14653, 0.0,
                -36.920166, 67.188866, 0.0,
                -11.71556, 88.527443, 0.0,
                5.23503, 16.342365, 0.0,
                7.298534, 39.723278, 0.0,
                9.362039, 63.104191, 0.0,
                30.439636, 37.680939, 0.0,
                28.376133, 14.300026, 0.0,
                32.503139, 61.061852, 0.0,
                53.580738, 35.638599, 0.0,
                -54.902267, -40.470875, 0.0,
                -70.378792, -47.673355, 0.0,
                -68.878059, -30.669056, 0.0,
                -43.11068, -2.953869, 0.0,
                -67.377327, -13.664756, 0.0,
                -65.876595, 3.339545, 0.0,
                -64.37587, 20.343845, 0.0,
                -23.949224, -26.065916, 0.0,
                -39.425747, -33.268398, 0.0,
                -62.875137, 37.348145, 0.0,
                -38.983669, 43.807957, 0.0,
                -59.873676, 71.356743, 0.0,
                -61.374409, 54.352448, 0.0,
                -34.856663, 90.569786, 0.0,
                -58.372948, 88.361046, 0.0,
                -56.872219, 105.365349, 0.0,
                -39.894966, 129.572128, 0.0,
                -55.371487, 122.369644, 0.0,
                -11.943384, 109.968483, 0.0,
                -25.919174, 119.770309, 0.0,
                -53.870758, 139.373947, 0.0,
                7.003817, -11.660957, 0.0,
                -8.472704, -18.863438, 0.0,
                22.480339, -4.458477, 0.0,
                53.43338, 9.946482, 0.0,
                37.95686, 2.744002, 0.0,
                84.386421, 24.35144, 0.0,
                68.909904, 17.148962, 0.0,
                85.887154, 41.355743, 0.0,
                99.862946, 31.553921, 0.0,
                16.008198, 90.364845, 0.0,
                29.98399, 80.563026, 0.0,
                2.032407, 100.166664, 0.0,
                71.911362, 51.157562, 0.0,
                57.93557, 60.959381, 0.0,
                43.959782, 70.7612, 0.0,
            ],
            vec![
                0,1,2,
                3,0,2,
                4,3,5,
                6,4,5,
                7,1,0,
                8,7,3,
                9,8,4,
                6,9,4,
                10,11,7,
                12,10,8,
                13,11,10,
                12,13,10,
                14,15,16,
                17,18,19,
                20,2,19,
                18,14,16,
                17,21,22,
                18,17,22,
                23,2,20,
                24,2,23,
                22,14,18,
                25,5,26,
                27,28,29,
                30,29,31,
                28,27,5,
                32,27,33,
                33,27,29,
                31,34,30,
                30,33,29,
                24,23,26,
                1,21,17,
                1,35,36,
                1,7,35,
                37,35,7,
                11,37,7,
                21,1,36,
                38,39,11,
                37,11,39,
                38,11,13,
                40,41,42,
                41,38,13,
                40,42,43,
                9,6,44,
                12,9,45,
                45,9,44,
                6,32,46,
                6,46,44,
                6,27,32,
                13,47,41,
                42,41,47,
                48,47,13,
                49,48,12,
                13,12,48,
                45,49,12,
                1,17,2,
                3,2,24,
                3,24,5,
                6,5,27,
                7,0,3,
                8,3,4,
                10,7,8,
                12,8,9,
                2,17,19,
                5,24,26,
                28,5,25,
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
                Point::new(43.959782, 70.7612, 0.0),
                Point::new(57.93557, 60.959381, 0.0),
                Point::new(71.911362, 51.157562, 0.0),
                Point::new(85.887154, 41.355743, 0.0),
                Point::new(99.862946, 31.553921, 0.0),
                Point::new(84.386421, 24.35144, 0.0),
                Point::new(68.909904, 17.148962, 0.0),
                Point::new(53.43338, 9.946482, 0.0),
                Point::new(37.95686, 2.744002, 0.0),
                Point::new(22.480339, -4.458477, 0.0),
                Point::new(7.003817, -11.660957, 0.0),
                Point::new(-8.472704, -18.863438, 0.0),
                Point::new(-23.949224, -26.065916, 0.0),
                Point::new(-39.425747, -33.268398, 0.0),
                Point::new(-54.902267, -40.470875, 0.0),
                Point::new(-70.378792, -47.673355, 0.0),
                Point::new(-68.878059, -30.669056, 0.0),
                Point::new(-67.377327, -13.664756, 0.0),
                Point::new(-65.876595, 3.339545, 0.0),
                Point::new(-64.37587, 20.343845, 0.0),
                Point::new(-62.875137, 37.348145, 0.0),
                Point::new(-61.374409, 54.352448, 0.0),
                Point::new(-59.873676, 71.356743, 0.0),
                Point::new(-58.372948, 88.361046, 0.0),
                Point::new(-56.872219, 105.365349, 0.0),
                Point::new(-55.371487, 122.369644, 0.0),
                Point::new(-53.870758, 139.373947, 0.0),
                Point::new(-39.894966, 129.572128, 0.0),
                Point::new(-25.919174, 119.770309, 0.0),
                Point::new(-11.943384, 109.968483, 0.0),
                Point::new(2.032407, 100.166664, 0.0),
                Point::new(16.008198, 90.364845, 0.0),
                Point::new(29.98399, 80.563026, 0.0),
            ]),
        ];

        assert_eq!(actual.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn test_get_polygons_for_planar_mesh_rectangle_2_holes() {
        let mesh = Mesh::new(
            vec![
                -39.321667, 43.177273, 21.852428,
                -39.668144, 43.337585, 25.123402,
                -40.014622, 43.497898, 28.394375,
                -40.361099, 43.658211, 31.665348,
                -40.707573, 43.818523, 34.936321,
                -41.05405, 43.978836, 38.207294,
                -41.400528, 44.139149, 41.478268,
                -41.747005, 44.299461, 44.749241,
                -42.093479, 44.45977, 48.020218,
                -42.439957, 44.620083, 51.291187,
                -42.786434, 44.780396, 54.562164,
                -43.132912, 44.940708, 57.833134,
                -43.479385, 45.101021, 61.104111,
                -43.825863, 45.261333, 64.375084,
                -38.281002, 45.426422, 21.852428,
                -38.62748, 45.586735, 25.123402,
                -39.666908, 46.067673, 34.936321,
                -40.013386, 46.227985, 38.207294,
                -40.359863, 46.388298, 41.478268,
                -40.706337, 46.548607, 44.749241,
                -41.052814, 46.70892, 48.020218,
                -41.399292, 46.869232, 51.291187,
                -41.74577, 47.029545, 54.562164,
                -42.092243, 47.189857, 57.833134,
                -42.438721, 47.35017, 61.104111,
                -42.785198, 47.510483, 64.375084,
                -37.240337, 47.675571, 21.852428,
                -37.586811, 47.835884, 25.123402,
                -38.972717, 48.477131, 38.207294,
                -39.319195, 48.637444, 41.478268,
                -39.665672, 48.797756, 44.749241,
                -40.01215, 48.958069, 48.020218,
                -40.705101, 49.278694, 54.562164,
                -41.051579, 49.439007, 57.833134,
                -41.398056, 49.599319, 61.104111,
                -41.744534, 49.759632, 64.375084,
                -36.199669, 49.924721, 21.852428,
                -37.932053, 50.726284, 38.207294,
                -38.27853, 50.886593, 41.478268,
                -38.625008, 51.046906, 44.749241,
                -40.010914, 51.688156, 57.833134,
                -40.357388, 51.848469, 61.104111,
                -40.703865, 52.008781, 64.375084,
                -35.159004, 52.17387, 21.852428,
                -35.505482, 52.334183, 25.123402,
                -36.54491, 52.815117, 34.936321,
                -36.891388, 52.97543, 38.207294,
                -37.237865, 53.135742, 41.478268,
                -37.584339, 53.296055, 44.749241,
                -38.970245, 53.937305, 57.833134,
                -39.316723, 54.097618, 61.104111,
                -39.6632, 54.257931, 64.375084,
                -34.11834, 54.423019, 21.852428,
                -34.464813, 54.583332, 25.123402,
                -34.811291, 54.743645, 28.394375,
                -35.157768, 54.903957, 31.665348,
                -35.504246, 55.064266, 34.936321,
                -35.850719, 55.224579, 38.207294,
                -36.197197, 55.384892, 41.478268,
                -36.543674, 55.545204, 44.749241,
                -37.236626, 55.865829, 51.291187,
                -37.583103, 56.026142, 54.562164,
                -37.929581, 56.186455, 57.833134,
                -38.276058, 56.346767, 61.104111,
                -38.622532, 56.50708, 64.375084,
                -33.077671, 56.672169, 21.852428,
                -33.424149, 56.832481, 25.123402,
                -33.770626, 56.992794, 28.394375,
                -34.117104, 57.153107, 31.665348,
                -34.463577, 57.313416, 34.936321,
                -34.810055, 57.473728, 38.207294,
                -35.156532, 57.634041, 41.478268,
                -35.849483, 57.954666, 48.020218,
                -36.195961, 58.114979, 51.291187,
                -36.542439, 58.275291, 54.562164,
                -36.888916, 58.435604, 57.833134,
                -37.23539, 58.595917, 61.104111,
                -37.581867, 58.756229, 64.375084,
                -32.037006, 58.921314, 21.852428,
                -32.383484, 59.081627, 25.123402,
                -32.729961, 59.24194, 28.394375,
                -33.076435, 59.402252, 31.665348,
                -33.422913, 59.562565, 34.936321,
                -33.76939, 59.722878, 38.207294,
                -34.115868, 59.88319, 41.478268,
                -34.462341, 60.043503, 44.749241,
                -34.808819, 60.203815, 48.020218,
                -35.155296, 60.364128, 51.291187,
                -35.501774, 60.524441, 54.562164,
                -35.848248, 60.684753, 57.833134,
                -36.194725, 60.845062, 61.104111,
                -36.541203, 61.005375, 64.375084,
                -30.996342, 61.170464, 21.852428,
                -31.342817, 61.330776, 25.123402,
                -31.689295, 61.491089, 28.394375,
                -32.03577, 61.651402, 31.665348,
                -32.382248, 61.811714, 34.936321,
                -32.728725, 61.972027, 38.207294,
                -33.075199, 62.132339, 41.478268,
                -33.421677, 62.292652, 44.749241,
                -33.768154, 62.452965, 48.020218,
                -34.114632, 62.613277, 51.291187,
                -34.461105, 62.77359, 54.562164,
                -34.807583, 62.933903, 57.833134,
                -35.15406, 63.094215, 61.104111,
                -35.500538, 63.254524, 64.375084,
                -40.488335, 48.629719, 50.541664,
                -40.313377, 49.582386, 52.608765,
                -40.138416, 50.535053, 54.675861,
                -39.963455, 51.487717, 56.742958,
                -39.788494, 52.440384, 58.810059,
                -39.155083, 53.214073, 56.668308,
                -38.521671, 53.987759, 54.526562,
                -37.88826, 54.761444, 52.384811,
                -37.254848, 55.535133, 50.243061,
                -36.621437, 56.308823, 48.101315,
                -35.988026, 57.082508, 45.959564,
                -35.35461, 57.856194, 43.817818,
                -36.210232, 56.318451, 44.938457,
                -37.065853, 54.780704, 46.059097,
                -37.921474, 53.242958, 47.179741,
                -38.777096, 51.705212, 48.300381,
                -39.632717, 50.167465, 49.421024,
                -35.500019, 54.123981, 31.520412,
                -35.383942, 53.957466, 30.018711,
                -35.267868, 53.790951, 28.517012,
                -35.553463, 52.852646, 27.361885,
                -35.839058, 51.914341, 26.206757,
                -36.417236, 50.562649, 25.83942,
                -36.995419, 49.210957, 25.472082,
                -37.645336, 47.962173, 26.032846,
                -38.29525, 46.713394, 26.593611,
                -38.768661, 46.044518, 27.86828,
                -39.242069, 45.375641, 29.142954,
                -39.358147, 45.542156, 30.644653,
                -39.47422, 45.708672, 32.146351,
                -39.188625, 46.646976, 33.301479,
                -38.903027, 47.585281, 34.456608,
                -38.324848, 48.936974, 34.823944,
                -37.74667, 50.288666, 35.19128,
                -37.096752, 51.537449, 34.63052,
                -36.446838, 52.786228, 34.069756,
                -35.973427, 53.455105, 32.795082,
            ],
            vec![
                15,1,0,
                17,5,4,
                18,6,5,
                19,7,6,
                20,8,7,
                21,9,8,
                22,10,9,
                23,11,10,
                24,12,11,
                25,13,12,
                27,15,14,
                29,18,17,
                30,19,18,
                31,20,19,
                33,23,22,
                34,24,23,
                35,25,24,
                38,29,28,
                39,30,29,
                31,30,39,
                41,34,33,
                42,35,34,
                36,43,44,
                47,38,37,
                48,39,38,
                51,42,41,
                53,44,43,
                57,46,45,
                58,47,46,
                59,48,47,
                63,50,49,
                64,51,50,
                66,53,52,
                67,54,53,
                68,55,54,
                69,56,55,
                70,57,56,
                71,58,57,
                74,61,60,
                75,62,61,
                76,63,62,
                77,64,63,
                79,66,65,
                80,67,66,
                81,68,67,
                82,69,68,
                83,70,69,
                84,71,70,
                87,73,72,
                88,74,73,
                89,75,74,
                90,76,75,
                91,77,76,
                93,79,78,
                94,80,79,
                95,81,80,
                96,82,81,
                97,83,82,
                98,84,83,
                99,85,84,
                100,86,85,
                101,87,86,
                102,88,87,
                103,89,88,
                104,90,89,
                105,91,90,
                15,2,1,
                2,15,132,
                131,132,15,
                133,2,132,
                135,4,3,
                3,134,135,
                3,133,134,
                133,3,2,
                135,16,4,
                15,27,131,
                27,26,36,
                27,44,129,
                27,36,44,
                129,44,128,
                130,27,129,
                137,16,136,
                130,131,27,
                136,16,135,
                138,28,137,
                17,16,137,
                106,20,31,
                137,28,17,
                107,32,22,
                21,106,107,
                108,33,32,
                22,21,107,
                106,21,20,
                122,106,31,
                121,39,120,
                31,121,122,
                39,121,31,
                140,37,139,
                109,33,108,
                40,110,41,
                40,33,109,
                107,108,32,
                37,28,139,
                28,138,139,
                126,127,44,
                128,44,127,
                126,53,125,
                56,141,142,
                53,126,44,
                46,140,45,
                141,45,140,
                45,141,56,
                39,48,120,
                55,123,124,
                125,54,124,
                124,54,55,
                54,125,53,
                142,55,56,
                117,71,84,
                85,117,84,
                58,71,117,
                55,142,123,
                117,118,58,
                48,119,120,
                59,118,119,
                119,48,59,
                112,113,61,
                49,110,111,
                40,109,110,
                62,49,112,
                111,112,49,
                110,49,50,
                41,110,50,
                60,113,114,
                86,72,85,
                116,85,72,
                118,59,58,
                72,115,116,
                113,60,61,
                73,60,114,
                72,73,115,
                114,115,73,
                112,61,62,
                117,85,116,
                37,140,46,
                15,0,14,
                17,4,16,
                18,5,17,
                19,6,18,
                20,7,19,
                21,8,20,
                22,9,21,
                23,10,22,
                24,11,23,
                25,12,24,
                27,14,26,
                29,17,28,
                30,18,29,
                31,19,30,
                33,22,32,
                34,23,33,
                35,24,34,
                38,28,37,
                39,29,38,
                41,33,40,
                42,34,41,
                47,37,46,
                48,38,47,
                51,41,50,
                53,43,52,
                57,45,56,
                58,46,57,
                59,47,58,
                63,49,62,
                64,50,63,
                66,52,65,
                67,53,66,
                68,54,67,
                69,55,68,
                70,56,69,
                71,57,70,
                74,60,73,
                75,61,74,
                76,62,75,
                77,63,76,
                79,65,78,
                80,66,79,
                81,67,80,
                82,68,81,
                83,69,82,
                84,70,83,
                87,72,86,
                88,73,87,
                89,74,88,
                90,75,89,
                91,76,90,
                93,78,92,
                94,79,93,
                95,80,94,
                96,81,95,
                97,82,96,
                98,83,97,
                99,84,98,
                100,85,99,
                101,86,100,
                102,87,101,
                103,88,102,
                104,89,103,
                105,90,104,
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
                Point::new(-35.973427, 53.455105, 32.795082),
                Point::new(-35.500019, 54.123981, 31.520412),
                Point::new(-35.383942, 53.957466, 30.018711),
                Point::new(-35.267868, 53.790951, 28.517012),
                Point::new(-35.553463, 52.852646, 27.361885),
                Point::new(-35.839058, 51.914341, 26.206757),
                Point::new(-36.417236, 50.562649, 25.83942),
                Point::new(-36.995419, 49.210957, 25.472082),
                Point::new(-37.645336, 47.962173, 26.032846),
                Point::new(-38.29525, 46.713394, 26.593611),
                Point::new(-38.768661, 46.044518, 27.86828),
                Point::new(-39.242069, 45.375641, 29.142954),
                Point::new(-39.358147, 45.542156, 30.644653),
                Point::new(-39.47422, 45.708672, 32.146351),
                Point::new(-39.188625, 46.646976, 33.301479),
                Point::new(-38.903027, 47.585281, 34.456608),
                Point::new(-38.324848, 48.936974, 34.823944),
                Point::new(-37.74667, 50.288666, 35.19128),
                Point::new(-37.096752, 51.537449, 34.63052),
                Point::new(-36.446838, 52.786228, 34.069756),
            ]),
            Polygon::new( vec![
                Point::new(-39.632717, 50.167465, 49.421024),
                Point::new(-40.488335, 48.629719, 50.541664),
                Point::new(-40.313377, 49.582386, 52.608765),
                Point::new(-40.138416, 50.535053, 54.675861),
                Point::new(-39.963455, 51.487717, 56.742958),
                Point::new(-39.788494, 52.440384, 58.810059),
                Point::new(-39.155083, 53.214073, 56.668308),
                Point::new(-38.521671, 53.987759, 54.526562),
                Point::new(-37.88826, 54.761444, 52.384811),
                Point::new(-37.254848, 55.535133, 50.243061),
                Point::new(-36.621437, 56.308823, 48.101315),
                Point::new(-35.988026, 57.082508, 45.959564),
                Point::new(-35.35461, 57.856194, 43.817818),
                Point::new(-36.210232, 56.318451, 44.938457),
                Point::new(-37.065853, 54.780704, 46.059097),
                Point::new(-37.921474, 53.242958, 47.179741),
                Point::new(-38.777096, 51.705212, 48.300381),
            ]),
            Polygon::new( vec![
                Point::new(-35.500538, 63.254524, 64.375084),
                Point::new(-36.541203, 61.005375, 64.375084),
                Point::new(-37.581867, 58.756229, 64.375084),
                Point::new(-38.622532, 56.50708, 64.375084),
                Point::new(-39.6632, 54.257931, 64.375084),
                Point::new(-40.703865, 52.008781, 64.375084),
                Point::new(-41.744534, 49.759632, 64.375084),
                Point::new(-42.785198, 47.510483, 64.375084),
                Point::new(-43.825863, 45.261333, 64.375084),
                Point::new(-43.479385, 45.101021, 61.104111),
                Point::new(-43.132912, 44.940708, 57.833134),
                Point::new(-42.786434, 44.780396, 54.562164),
                Point::new(-42.439957, 44.620083, 51.291187),
                Point::new(-42.093479, 44.45977, 48.020218),
                Point::new(-41.747005, 44.299461, 44.749241),
                Point::new(-41.400528, 44.139149, 41.478268),
                Point::new(-41.05405, 43.978836, 38.207294),
                Point::new(-40.707573, 43.818523, 34.936321),
                Point::new(-40.361099, 43.658211, 31.665348),
                Point::new(-40.014622, 43.497898, 28.394375),
                Point::new(-39.668144, 43.337585, 25.123402),
                Point::new(-39.321667, 43.177273, 21.852428),
                Point::new(-38.281002, 45.426422, 21.852428),
                Point::new(-37.240337, 47.675571, 21.852428),
                Point::new(-36.199669, 49.924721, 21.852428),
                Point::new(-35.159004, 52.17387, 21.852428),
                Point::new(-34.11834, 54.423019, 21.852428),
                Point::new(-33.077671, 56.672169, 21.852428),
                Point::new(-32.037006, 58.921314, 21.852428),
                Point::new(-30.996342, 61.170464, 21.852428),
                Point::new(-31.342817, 61.330776, 25.123402),
                Point::new(-31.689295, 61.491089, 28.394375),
                Point::new(-32.03577, 61.651402, 31.665348),
                Point::new(-32.382248, 61.811714, 34.936321),
                Point::new(-32.728725, 61.972027, 38.207294),
                Point::new(-33.075199, 62.132339, 41.478268),
                Point::new(-33.421677, 62.292652, 44.749241),
                Point::new(-33.768154, 62.452965, 48.020218),
                Point::new(-34.114632, 62.613277, 51.291187),
                Point::new(-34.461105, 62.77359, 54.562164),
                Point::new(-34.807583, 62.933903, 57.833134),
                Point::new(-35.15406, 63.094215, 61.104111),
            ]),
        ];

        assert_eq!(actual.len(), expected.len());
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
    }

    #[test]
    fn test_planar_simplify_nonmanifold() {
        let input = Mesh::new(
            vec![0.0, 0.0, 0.0,
                 2.5, 5.0, 0.0,
                 5.0, 0.0, 0.0,
                 7.5, 5.0, 0.0,
                 10.0, 0.0, 0.0,
                 5.0, 10.0, 0.0,
                 5.0, 5.0, 3.0,
            ],
            vec![0, 2, 1, // first face
                 1, 2, 3, // second face
                 2, 4, 3, // third face
                 1, 3, 5, // fourth face
                 1, 3, 6, // fifth face
            ]
        );

        let actual_result = input.get_planar_simplify(0.001, 0.0175);

        assert!(actual_result.is_err());
        assert_eq!(actual_result.err().unwrap().to_string(), "This Mesh has edges with more than 2 neighbouring faces, so it cannot be simplified using this method");
    }

    #[test]
    fn test_get_remesh_for_planar_mesh_triangle() {
        let mesh = Mesh::new(
            vec![
                -17.906073, 18.384705, 0.0,
                -19.969576, -4.996209, 0.0,
                -41.047173, 20.427044, 0.0,
                -15.842568, 41.765617, 0.0,
                -13.779064, 65.14653, 0.0,
                -36.920166, 67.188866, 0.0,
                -11.71556, 88.527443, 0.0,
                5.23503, 16.342365, 0.0,
                7.298534, 39.723278, 0.0,
                9.362039, 63.104191, 0.0,
                30.439636, 37.680939, 0.0,
                28.376133, 14.300026, 0.0,
                32.503139, 61.061852, 0.0,
                53.580738, 35.638599, 0.0,
                -54.902267, -40.470875, 0.0,
                -70.378792, -47.673355, 0.0,
                -68.878059, -30.669056, 0.0,
                -43.11068, -2.953869, 0.0,
                -67.377327, -13.664756, 0.0,
                -65.876595, 3.339545, 0.0,
                -64.37587, 20.343845, 0.0,
                -23.949224, -26.065916, 0.0,
                -39.425747, -33.268398, 0.0,
                -62.875137, 37.348145, 0.0,
                -38.983669, 43.807957, 0.0,
                -59.873676, 71.356743, 0.0,
                -61.374409, 54.352448, 0.0,
                -34.856663, 90.569786, 0.0,
                -58.372948, 88.361046, 0.0,
                -56.872219, 105.365349, 0.0,
                -39.894966, 129.572128, 0.0,
                -55.371487, 122.369644, 0.0,
                -11.943384, 109.968483, 0.0,
                -25.919174, 119.770309, 0.0,
                -53.870758, 139.373947, 0.0,
                7.003817, -11.660957, 0.0,
                -8.472704, -18.863438, 0.0,
                22.480339, -4.458477, 0.0,
                53.43338, 9.946482, 0.0,
                37.95686, 2.744002, 0.0,
                84.386421, 24.35144, 0.0,
                68.909904, 17.148962, 0.0,
                85.887154, 41.355743, 0.0,
                99.862946, 31.553921, 0.0,
                16.008198, 90.364845, 0.0,
                29.98399, 80.563026, 0.0,
                2.032407, 100.166664, 0.0,
                71.911362, 51.157562, 0.0,
                57.93557, 60.959381, 0.0,
                43.959782, 70.7612, 0.0,
            ],
            vec![
                0,1,2,
                3,0,2,
                4,3,5,
                6,4,5,
                7,1,0,
                8,7,3,
                9,8,4,
                6,9,4,
                10,11,7,
                12,10,8,
                13,11,10,
                12,13,10,
                14,15,16,
                17,18,19,
                20,2,19,
                18,14,16,
                17,21,22,
                18,17,22,
                23,2,20,
                24,2,23,
                22,14,18,
                25,5,26,
                27,28,29,
                30,29,31,
                28,27,5,
                32,27,33,
                33,27,29,
                31,34,30,
                30,33,29,
                24,23,26,
                1,21,17,
                1,35,36,
                1,7,35,
                37,35,7,
                11,37,7,
                21,1,36,
                38,39,11,
                37,11,39,
                38,11,13,
                40,41,42,
                41,38,13,
                40,42,43,
                9,6,44,
                12,9,45,
                45,9,44,
                6,32,46,
                6,46,44,
                6,27,32,
                13,47,41,
                42,41,47,
                48,47,13,
                49,48,12,
                13,12,48,
                45,49,12,
                1,17,2,
                3,2,24,
                3,24,5,
                6,5,27,
                7,0,3,
                8,3,4,
                10,7,8,
                12,8,9,
                2,17,19,
                5,24,26,
                28,5,25,
            ]
        );

        let actual = mesh.get_planar_simplify_for_planar_mesh(0.001, 0.01745);

        let expected = Mesh::new(
            vec![
                -53.870758091681715, 139.3739470069094, 0.0, 99.86294597389475, 31.553920927744112, 0.0, -70.37879198032073, -47.67335500055474, 0.0
            ],
            vec![
                2, 0, 1
            ]
        );

        println!("{:?}", actual);

        assert!(actual.is_ok());
        assert!(expected.eq(&actual.unwrap()));
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

        let actual = mesh.get_planar_simplify_for_planar_mesh(0.001, 0.001);

        let expected = Mesh::new(
            vec![
                66.31183414967374, 39.756861917579045, 4.611898152174278, 69.53420661650597, 36.018407141938695, -5.471664778583148, 59.440537200341375, 37.7698406815507, 13.264926563944385, 67.8768121904007, 27.98243766960643, -13.134176423238603, 61.81009227973166, 30.703668424647052, 0.28006701334592865, 59.52663712206086, 31.099887539800804, 4.518777910544081, 62.16413022155516, 28.03998144682352, -3.7345630213309358, 56.18597172012319, 29.077301658145934, 7.362525063450267, 60.453523252048846, 24.126267019048296, -5.991661231412742, 51.54491225162797, 30.816313229300167, 17.182260364099406, 61.97272526740779, 18.718419802759566, -15.44882882781465, 53.06411095414557, 25.408463237234372, 7.725094594254537, 57.331665674379366, 20.457433485654274, -5.629090599302021, 51.35350387447634, 21.494749086464157, 5.46799692139539, 53.990996992450434, 18.43484301686797, -2.785344010479628, 51.70754540013393, 18.831066455622764, 1.4533666449683282, 45.640822398911375, 21.552294487118154, 14.867611612637438, 54.07709820242638, 11.764891629967856, -11.531492502713135, 43.98342750420948, 13.516322194241416, 7.2050970938407595, 47.205800066648784, 9.777868330698647, -2.878464816193616
            ],
            vec![
                5, 4, 0, 6, 1, 4, 6, 4, 5, 7, 5, 2, 7, 6, 5, 8, 3, 6, 8, 6, 7, 9, 8, 7, 10, 8, 9, 11, 10, 9, 12, 10, 11, 13, 12, 11, 14, 12, 13, 15, 14, 13, 16, 13, 11, 17, 12, 14, 18, 15, 13, 19, 14, 15
            ]
        );

        println!("{:?}", actual);

        assert!(actual.is_ok());
        assert!(expected.eq(&actual.unwrap()));
    }

    #[test]
    fn test_get_remesh_for_planar_mesh_rectangle_2_holes() {
        let mesh = Mesh::new(
            vec![
                -39.321667, 43.177273, 21.852428,
                -39.668144, 43.337585, 25.123402,
                -40.014622, 43.497898, 28.394375,
                -40.361099, 43.658211, 31.665348,
                -40.707573, 43.818523, 34.936321,
                -41.05405, 43.978836, 38.207294,
                -41.400528, 44.139149, 41.478268,
                -41.747005, 44.299461, 44.749241,
                -42.093479, 44.45977, 48.020218,
                -42.439957, 44.620083, 51.291187,
                -42.786434, 44.780396, 54.562164,
                -43.132912, 44.940708, 57.833134,
                -43.479385, 45.101021, 61.104111,
                -43.825863, 45.261333, 64.375084,
                -38.281002, 45.426422, 21.852428,
                -38.62748, 45.586735, 25.123402,
                -39.666908, 46.067673, 34.936321,
                -40.013386, 46.227985, 38.207294,
                -40.359863, 46.388298, 41.478268,
                -40.706337, 46.548607, 44.749241,
                -41.052814, 46.70892, 48.020218,
                -41.399292, 46.869232, 51.291187,
                -41.74577, 47.029545, 54.562164,
                -42.092243, 47.189857, 57.833134,
                -42.438721, 47.35017, 61.104111,
                -42.785198, 47.510483, 64.375084,
                -37.240337, 47.675571, 21.852428,
                -37.586811, 47.835884, 25.123402,
                -38.972717, 48.477131, 38.207294,
                -39.319195, 48.637444, 41.478268,
                -39.665672, 48.797756, 44.749241,
                -40.01215, 48.958069, 48.020218,
                -40.705101, 49.278694, 54.562164,
                -41.051579, 49.439007, 57.833134,
                -41.398056, 49.599319, 61.104111,
                -41.744534, 49.759632, 64.375084,
                -36.199669, 49.924721, 21.852428,
                -37.932053, 50.726284, 38.207294,
                -38.27853, 50.886593, 41.478268,
                -38.625008, 51.046906, 44.749241,
                -40.010914, 51.688156, 57.833134,
                -40.357388, 51.848469, 61.104111,
                -40.703865, 52.008781, 64.375084,
                -35.159004, 52.17387, 21.852428,
                -35.505482, 52.334183, 25.123402,
                -36.54491, 52.815117, 34.936321,
                -36.891388, 52.97543, 38.207294,
                -37.237865, 53.135742, 41.478268,
                -37.584339, 53.296055, 44.749241,
                -38.970245, 53.937305, 57.833134,
                -39.316723, 54.097618, 61.104111,
                -39.6632, 54.257931, 64.375084,
                -34.11834, 54.423019, 21.852428,
                -34.464813, 54.583332, 25.123402,
                -34.811291, 54.743645, 28.394375,
                -35.157768, 54.903957, 31.665348,
                -35.504246, 55.064266, 34.936321,
                -35.850719, 55.224579, 38.207294,
                -36.197197, 55.384892, 41.478268,
                -36.543674, 55.545204, 44.749241,
                -37.236626, 55.865829, 51.291187,
                -37.583103, 56.026142, 54.562164,
                -37.929581, 56.186455, 57.833134,
                -38.276058, 56.346767, 61.104111,
                -38.622532, 56.50708, 64.375084,
                -33.077671, 56.672169, 21.852428,
                -33.424149, 56.832481, 25.123402,
                -33.770626, 56.992794, 28.394375,
                -34.117104, 57.153107, 31.665348,
                -34.463577, 57.313416, 34.936321,
                -34.810055, 57.473728, 38.207294,
                -35.156532, 57.634041, 41.478268,
                -35.849483, 57.954666, 48.020218,
                -36.195961, 58.114979, 51.291187,
                -36.542439, 58.275291, 54.562164,
                -36.888916, 58.435604, 57.833134,
                -37.23539, 58.595917, 61.104111,
                -37.581867, 58.756229, 64.375084,
                -32.037006, 58.921314, 21.852428,
                -32.383484, 59.081627, 25.123402,
                -32.729961, 59.24194, 28.394375,
                -33.076435, 59.402252, 31.665348,
                -33.422913, 59.562565, 34.936321,
                -33.76939, 59.722878, 38.207294,
                -34.115868, 59.88319, 41.478268,
                -34.462341, 60.043503, 44.749241,
                -34.808819, 60.203815, 48.020218,
                -35.155296, 60.364128, 51.291187,
                -35.501774, 60.524441, 54.562164,
                -35.848248, 60.684753, 57.833134,
                -36.194725, 60.845062, 61.104111,
                -36.541203, 61.005375, 64.375084,
                -30.996342, 61.170464, 21.852428,
                -31.342817, 61.330776, 25.123402,
                -31.689295, 61.491089, 28.394375,
                -32.03577, 61.651402, 31.665348,
                -32.382248, 61.811714, 34.936321,
                -32.728725, 61.972027, 38.207294,
                -33.075199, 62.132339, 41.478268,
                -33.421677, 62.292652, 44.749241,
                -33.768154, 62.452965, 48.020218,
                -34.114632, 62.613277, 51.291187,
                -34.461105, 62.77359, 54.562164,
                -34.807583, 62.933903, 57.833134,
                -35.15406, 63.094215, 61.104111,
                -35.500538, 63.254524, 64.375084,
                -40.488335, 48.629719, 50.541664,
                -40.313377, 49.582386, 52.608765,
                -40.138416, 50.535053, 54.675861,
                -39.963455, 51.487717, 56.742958,
                -39.788494, 52.440384, 58.810059,
                -39.155083, 53.214073, 56.668308,
                -38.521671, 53.987759, 54.526562,
                -37.88826, 54.761444, 52.384811,
                -37.254848, 55.535133, 50.243061,
                -36.621437, 56.308823, 48.101315,
                -35.988026, 57.082508, 45.959564,
                -35.35461, 57.856194, 43.817818,
                -36.210232, 56.318451, 44.938457,
                -37.065853, 54.780704, 46.059097,
                -37.921474, 53.242958, 47.179741,
                -38.777096, 51.705212, 48.300381,
                -39.632717, 50.167465, 49.421024,
                -35.500019, 54.123981, 31.520412,
                -35.383942, 53.957466, 30.018711,
                -35.267868, 53.790951, 28.517012,
                -35.553463, 52.852646, 27.361885,
                -35.839058, 51.914341, 26.206757,
                -36.417236, 50.562649, 25.83942,
                -36.995419, 49.210957, 25.472082,
                -37.645336, 47.962173, 26.032846,
                -38.29525, 46.713394, 26.593611,
                -38.768661, 46.044518, 27.86828,
                -39.242069, 45.375641, 29.142954,
                -39.358147, 45.542156, 30.644653,
                -39.47422, 45.708672, 32.146351,
                -39.188625, 46.646976, 33.301479,
                -38.903027, 47.585281, 34.456608,
                -38.324848, 48.936974, 34.823944,
                -37.74667, 50.288666, 35.19128,
                -37.096752, 51.537449, 34.63052,
                -36.446838, 52.786228, 34.069756,
                -35.973427, 53.455105, 32.795082,
            ],
            vec![
                15,1,0,
                17,5,4,
                18,6,5,
                19,7,6,
                20,8,7,
                21,9,8,
                22,10,9,
                23,11,10,
                24,12,11,
                25,13,12,
                27,15,14,
                29,18,17,
                30,19,18,
                31,20,19,
                33,23,22,
                34,24,23,
                35,25,24,
                38,29,28,
                39,30,29,
                31,30,39,
                41,34,33,
                42,35,34,
                36,43,44,
                47,38,37,
                48,39,38,
                51,42,41,
                53,44,43,
                57,46,45,
                58,47,46,
                59,48,47,
                63,50,49,
                64,51,50,
                66,53,52,
                67,54,53,
                68,55,54,
                69,56,55,
                70,57,56,
                71,58,57,
                74,61,60,
                75,62,61,
                76,63,62,
                77,64,63,
                79,66,65,
                80,67,66,
                81,68,67,
                82,69,68,
                83,70,69,
                84,71,70,
                87,73,72,
                88,74,73,
                89,75,74,
                90,76,75,
                91,77,76,
                93,79,78,
                94,80,79,
                95,81,80,
                96,82,81,
                97,83,82,
                98,84,83,
                99,85,84,
                100,86,85,
                101,87,86,
                102,88,87,
                103,89,88,
                104,90,89,
                105,91,90,
                15,2,1,
                2,15,132,
                131,132,15,
                133,2,132,
                135,4,3,
                3,134,135,
                3,133,134,
                133,3,2,
                135,16,4,
                15,27,131,
                27,26,36,
                27,44,129,
                27,36,44,
                129,44,128,
                130,27,129,
                137,16,136,
                130,131,27,
                136,16,135,
                138,28,137,
                17,16,137,
                106,20,31,
                137,28,17,
                107,32,22,
                21,106,107,
                108,33,32,
                22,21,107,
                106,21,20,
                122,106,31,
                121,39,120,
                31,121,122,
                39,121,31,
                140,37,139,
                109,33,108,
                40,110,41,
                40,33,109,
                107,108,32,
                37,28,139,
                28,138,139,
                126,127,44,
                128,44,127,
                126,53,125,
                56,141,142,
                53,126,44,
                46,140,45,
                141,45,140,
                45,141,56,
                39,48,120,
                55,123,124,
                125,54,124,
                124,54,55,
                54,125,53,
                142,55,56,
                117,71,84,
                85,117,84,
                58,71,117,
                55,142,123,
                117,118,58,
                48,119,120,
                59,118,119,
                119,48,59,
                112,113,61,
                49,110,111,
                40,109,110,
                62,49,112,
                111,112,49,
                110,49,50,
                41,110,50,
                60,113,114,
                86,72,85,
                116,85,72,
                118,59,58,
                72,115,116,
                113,60,61,
                73,60,114,
                72,73,115,
                114,115,73,
                112,61,62,
                117,85,116,
                37,140,46,
                15,0,14,
                17,4,16,
                18,5,17,
                19,6,18,
                20,7,19,
                21,8,20,
                22,9,21,
                23,10,22,
                24,11,23,
                25,12,24,
                27,14,26,
                29,17,28,
                30,18,29,
                31,19,30,
                33,22,32,
                34,23,33,
                35,24,34,
                38,28,37,
                39,29,38,
                41,33,40,
                42,34,41,
                47,37,46,
                48,38,47,
                51,41,50,
                53,43,52,
                57,45,56,
                58,46,57,
                59,47,58,
                63,49,62,
                64,50,63,
                66,52,65,
                67,53,66,
                68,54,67,
                69,55,68,
                70,56,69,
                71,57,70,
                74,60,73,
                75,61,74,
                76,62,75,
                77,63,76,
                79,65,78,
                80,66,79,
                81,67,80,
                82,68,81,
                83,69,82,
                84,70,83,
                87,72,86,
                88,73,87,
                89,74,88,
                90,75,89,
                91,76,90,
                93,78,92,
                94,79,93,
                95,80,94,
                96,81,95,
                97,82,96,
                98,83,97,
                99,84,98,
                100,85,99,
                101,86,100,
                102,87,101,
                103,88,102,
                104,89,103,
                105,90,104,
            ]
        );

        let actual = mesh.get_planar_simplify_for_planar_mesh(0.001, 0.001);

        let expected = Mesh::new(
            vec![
                -35.50055388155962, 63.25453134415079, 64.37508196105487, -30.99635595304621, 61.17047044567765, 21.852426220715373, -35.35462420575041, 57.85620058599941, 43.817816183046546, -35.500026672996526, 54.12398454124441, 31.520411006415372, -35.26787540164498, 53.7909544322887, 28.5170110576791, -36.44684420698406, 52.786230865691635, 34.069755201994845, -35.83906578136306, 51.9143445909153, 26.206756001917853, -39.78850256621063, 52.440387948344906, 58.8100578880445, -37.74667614689327, 50.288668836556184, 35.19127921916392, -36.995421256657586, 49.210958041849175, 25.472081722929683, -40.488340195644206, 48.62972139955938, 50.54166332484817, -38.90303156019105, 47.58528311258829, 34.45660742669236, -38.295253901619205, 46.71339581353062, 26.59361050592226, -39.474221905116515, 45.70867289390048, 32.146350742854764, -39.24207132899913, 45.37564208865133, 29.142953695054533, -43.82586492660346, 45.26133390171126, 64.37508373713817, -39.32166699809005, 43.17727300323811, 21.852427996798664
            ],
            vec![
                2, 1, 0, 3, 1, 2, 4, 1, 3, 5, 3, 2, 6, 1, 4, 7, 2, 0, 8, 5, 2, 9, 1, 6, 10, 8, 2, 11, 8, 10, 13, 11, 10, 15, 7, 0, 15, 10, 7, 15, 13, 10, 15, 14, 13, 16, 1, 9, 16, 9, 12, 16, 12, 14, 16, 14, 15
            ]
        );

        println!("{:?}", actual);

        assert!(actual.is_ok());
        assert!(expected.eq(&actual.unwrap()));
    }

    #[test]
    fn test_deduplicate_all_unique() {
        let box1 = Mesh::new(
        vec![
            49.216652, 25.228749, -35.809109,
            66.343965, 33.868829, -41.466331,
            57.308002, 7.194164, -38.85604,
            74.435315, 15.834243, -44.513262,
            63.725571, 6.873605, -19.916345,
            80.852884, 15.513684, -25.573567,
            55.634221, 24.90819, -16.869414,
            72.761534, 33.54827, -22.526636,
            57.308002, 7.194164, -38.85604,
            74.435315, 15.834243, -44.513262,
            63.725571, 6.873605, -19.916345,
            80.852884, 15.513684, -25.573567,
            74.435315, 15.834243, -44.513262,
            66.343965, 33.868829, -41.466331,
            80.852884, 15.513684, -25.573567,
            72.761534, 33.54827, -22.526636,
            66.343965, 33.868829, -41.466331,
            49.216652, 25.228749, -35.809109,
            72.761534, 33.54827, -22.526636,
            55.634221, 24.90819, -16.869414,
            49.216652, 25.228749, -35.809109,
            57.308002, 7.194164, -38.85604,
            55.634221, 24.90819, -16.869414,
            63.725571, 6.873605, -19.916345,
        ],
        vec![
            0,1,3,
            4,5,7,
            8,9,11,
            12,13,15,
            16,17,19,
            20,21,23,
            0,3,2,
            4,7,6,
            8,11,10,
            12,15,14,
            16,19,18,
            20,23,22,
        ]);

        let wing = Mesh::new(
            vec![
                23.175826, 37.836765, -7.817897,
                29.620183, 12.342433, -7.786009,
                10.411304, 14.060205, 9.808478,
                9.526543, 55.868835, 10.216331,

            ],
            vec![
                0,1,2,
                0,2,3,
            ]);

        let diamond1 = Mesh::new(
            vec![
                -30.110934, -30.634332, -29.828453,
                -21.45068, -30.634332, -24.828453,
                -23.98721, -24.510608, -24.828453,
                -30.110934, -21.974078, -24.828453,
                -36.234658, -24.510608, -24.828453,
                -38.771188, -30.634332, -24.828453,
                -36.234658, -36.758056, -24.828453,
                -30.110934, -39.294586, -24.828453,
                -23.98721, -36.758056, -24.828453,
                -21.45068, -30.634332, -14.828453,
                -23.98721, -24.510608, -14.828453,
                -30.110934, -21.974078, -14.828453,
                -36.234658, -24.510608, -14.828453,
                -38.771188, -30.634332, -14.828453,
                -36.234658, -36.758056, -14.828453,
                -30.110934, -39.294586, -14.828453,
                -23.98721, -36.758056, -14.828453,
                -30.110934, -30.634332, -9.828453,

            ],
            vec![
                0,2,1,
                0,3,2,
                0,4,3,
                0,5,4,
                0,6,5,
                0,7,6,
                0,8,7,
                0,1,8,
                1,2,10,
                2,3,11,
                3,4,12,
                4,5,13,
                5,6,14,
                6,7,15,
                7,8,16,
                8,1,16,
                9,10,17,
                10,11,17,
                11,12,17,
                12,13,17,
                13,14,17,
                14,15,17,
                15,16,17,
                16,9,17,
                1,10,9,
                2,11,10,
                3,12,11,
                4,13,12,
                5,14,13,
                6,15,14,
                7,16,15,
                1,9,16,
            ]);

        let input = vec![box1, wing, diamond1];

        let actual = Mesh::deduplicate(input, 0.001);

        let expected_box1 = Mesh::new(
            vec![
                -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.3333334368408, 13.333332954932288, -19.999999899853734
            ],
            vec![
                0, 1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 0, 3, 2, 4, 7, 6, 8, 11, 10, 12, 15, 14, 16, 19, 18, 20, 23, 22
            ]);

        let expected_wing = Mesh::new(
            vec![
                -15.413643711502385, -8.44035165205959, -1.7763568394002505e-15, 10.882586400706039, -8.440351652059588, -8.881784197001252e-16, 4.531057310796346, 16.880703304119173, -3.552713678800501e-15, -36.21895891738076, 10.449850067645535, -6.854997638793775
            ],
            vec![
                0, 1, 2, 0, 2, 3
            ]);

        let expected_diamond1 = Mesh::new(
            vec![
                -5.934433476851259, -2.084559503484389, -1.3322676295501878e-15, 1.86886739088493, 4.169119006968773, -1.3322676295501878e-15, 4.065566085966331, -2.0845595034843885, -1.3322676295501878e-15, 1.8688673908849305, -7.8238223025583915, -2.4838191228970192, -3.4344333675556538, -9.686686508995903, -5.996470270246201, -8.737734125996234, -6.581912686815779, -8.480290039815763, -10.934432821077639, -0.3282341763626172, -8.480290039815767, -8.737734125996234, 5.411028622711387, -5.996470916918746, -3.4344333675556538, 7.273892829148897, -2.483819769569566, 6.868867609476145, 5.925444334090544, -8.480290039815763, 9.065566304557546, -0.3282341763626171, -8.480290039815763, 6.868867609476146, -6.067496975436621, -10.964109162712782, 1.5655668510355616, -7.930361181874131, -14.476760310061962, -3.737733907405018, -4.825587359694008, -16.960580079631526, -5.934432602486424, 1.4280911507591543, -16.960580079631526, -3.737733907405018, 7.167353949833158, -14.476760956734509, 1.5655668510355616, 9.030218156270669, -10.964109809385327, 4.0655669603311715, 1.4280911507591543, -16.960580079631526

            ],
            vec![
                0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 7, 6, 0, 8, 7, 0, 1, 8, 1, 2, 10, 2, 3, 11, 3, 4, 12, 4, 5, 13, 5, 6, 14, 6, 7, 15, 7, 8, 16, 8, 1, 16, 9, 10, 17, 10, 11, 17, 11, 12, 17, 12, 13, 17, 13, 14, 17, 14, 15, 17, 15, 16, 17, 16, 9, 17, 1, 10, 9, 2, 11, 10, 3, 12, 11, 4, 13, 12, 5, 14, 13, 6, 15, 14, 7, 16, 15, 1, 9, 16
            ]);

        let expected: Vec<(Mesh, Vec<LocalCoordinateSystem>)> = vec![
            (expected_box1, vec![
                LocalCoordinateSystem::new(
                    Point::new(63.331977333333334,24.977273666666665,-40.596234),
                    Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
                    Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
                )
            ]),
            (expected_wing, vec![
                LocalCoordinateSystem::new(
                    Point::new(21.069104333333332,21.413134333333332,-1.9318093333333337),
                    Vector::new(0.2450677139841467,-0.9695052063057461,0.0012126453055792158),
                    Vector::new(-0.6971401592935829,-0.17535085044260348,0.6951601812166558),
                )
            ]),
            (expected_diamond1, vec![
                LocalCoordinateSystem::new(
                    Point::new(-25.182941333333332,-28.59309066666667,-26.495119666666668),
                    Vector::new(0.6123724267718453,0.6123724267718453,0.5000000218591215),
                    Vector::new(0.6207111069597122,-0.764114477778006,0.17563253271217716),
                )
            ]),
        ];


        for act in &actual {
            println!("Mesh: {:?}", act.0);
            println!("Local:");
            for local in &act.1 {
                println!("{:?}", local);
            }
        }

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
            assert_eq!(expected[i].1.len(), actual[i].1.len());
            for j in 0..expected[i].1.len() {
                assert!(expected[i].1[j].eq_with_tolerance(&actual[i].1[j], 0.0001));
            }
        }
    }

    #[test]
    fn test_deduplicate_3_welded_boxes() {
        let box_a = Mesh::new(
            vec![
                72.761534, 33.54827, -22.526636,
                66.343965, 33.868829, -41.466331,
                80.852884, 15.513684, -25.573567,
                74.435315, 15.834243, -44.513262,
                55.634221, 24.90819, -16.869414,
                49.216652, 25.228749, -35.809109,
                63.725571, 6.873605, -19.916345,
                57.308002, 7.194164, -38.85604,
            ],
            vec![
                5,1,3,
                6,2,0,
                7,3,2,
                3,1,0,
                1,5,4,
                5,7,6,
                5,3,7,
                6,0,4,
                7,2,6,
                3,0,2,
                1,4,0,
                5,6,4,
            ]);

        let box_b = Mesh::new(
            vec![
                -25.420272, 61.131243, 18.969046,
                -28.073985, 77.243424, 7.421083,
                -20.361479, 50.416475, 2.856865,
                -23.015192, 66.528656, -8.691098,
                -44.587077, 56.07245, 16.315332,
                -47.24079, 72.18463, 4.76737,
                -39.528284, 45.357682, 0.203151,
                -42.181997, 61.469863, -11.344811,
            ],
            vec![
                5,1,3,
                6,2,0,
                7,3,2,
                3,1,0,
                1,5,4,
                5,7,6,
                5,3,7,
                6,0,4,
                7,2,6,
                3,0,2,
                1,4,0,
                5,6,4,
            ]);

        let box_c = Mesh::new(
            vec![
                67.432021, 133.273959, 35.72196,
                65.33002, 139.523639, 16.840141,
                74.376341, 115.712505, 29.136237,
                72.27434, 121.962185, 10.254418,
                48.794478, 126.02573, 35.397681,
                46.692477, 132.27541, 16.515862,
                55.738797, 108.464276, 28.811958,
                53.636796, 114.713956, 9.930139,
            ],
            vec![
                5,1,3,
                6,2,0,
                7,3,2,
                3,1,0,
                1,5,4,
                5,7,6,
                5,3,7,
                6,0,4,
                7,2,6,
                3,0,2,
                1,4,0,
                5,6,4,
            ]);

        let input = vec![box_a, box_b, box_c];

        let actual = Mesh::deduplicate(input, 0.001);

        let expected_box = Mesh::new(
            vec![
                6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8
            ],
            vec![
                5,1,3,
                6,2,0,
                7,3,2,
                3,1,0,
                1,5,4,
                5,7,6,
                5,3,7,
                6,0,4,
                7,2,6,
                3,0,2,
                1,4,0,
                5,6,4,
            ]);

        let expected: Vec<(Mesh, Vec<LocalCoordinateSystem>)> = vec![
            (expected_box, vec![
                LocalCoordinateSystem::new(
                    Point::new(63.331977333333334,24.977273666666665,-40.596234),
                    Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
                    Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
                ),
                LocalCoordinateSystem::new(
                    Point::new(-32.77665566666666,71.98557,1.1657849999999996),
                    Vector::new(0.9583402460123103,0.2529396989475088,0.13268564944789007),
                    Vector::new(0.2529396670241323,-0.5357383830896744,-0.80560903031817),
                ),
                LocalCoordinateSystem::new(
                    Point::new(61.432278999999994,131.25374466666665,14.536807000000001),
                    Vector::new(0.931877162134053,0.36241145471899117,0.01621395021112327),
                    Vector::new(0.3472159773829504,-0.8780727160278832,-0.3292861527872357),
                ),
            ]),
        ];


        for act in &actual {
            println!("Mesh: {:?}", act.0);
            println!("Local:");
            for local in &act.1 {
                println!("{:?}", local);
            }
        }

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
            assert_eq!(expected[i].1.len(), actual[i].1.len());
            for j in 0..expected[i].1.len() {
                assert!(expected[i].1[j].eq_with_tolerance(&actual[i].1[j], 0.0001));
            }
        }
    }

    #[test]
    fn test_deduplicate_3_boxes_2_diamonds_1_wing() {
        let box1 = Mesh::new(
            vec![
                49.216652, 25.228749, -35.809109,
                66.343965, 33.868829, -41.466331,
                57.308002, 7.194164, -38.85604,
                74.435315, 15.834243, -44.513262,
                63.725571, 6.873605, -19.916345,
                80.852884, 15.513684, -25.573567,
                55.634221, 24.90819, -16.869414,
                72.761534, 33.54827, -22.526636,
                57.308002, 7.194164, -38.85604,
                74.435315, 15.834243, -44.513262,
                63.725571, 6.873605, -19.916345,
                80.852884, 15.513684, -25.573567,
                74.435315, 15.834243, -44.513262,
                66.343965, 33.868829, -41.466331,
                80.852884, 15.513684, -25.573567,
                72.761534, 33.54827, -22.526636,
                66.343965, 33.868829, -41.466331,
                49.216652, 25.228749, -35.809109,
                72.761534, 33.54827, -22.526636,
                55.634221, 24.90819, -16.869414,
                49.216652, 25.228749, -35.809109,
                57.308002, 7.194164, -38.85604,
                55.634221, 24.90819, -16.869414,
                63.725571, 6.873605, -19.916345,
            ],
            vec![
                0,1,3,
                4,5,7,
                8,9,11,
                12,13,15,
                16,17,19,
                20,21,23,
                0,3,2,
                4,7,6,
                8,11,10,
                12,15,14,
                16,19,18,
                20,23,22,
            ]);

        let box2 = Mesh::new(
            vec![
                -47.24079, 72.18463, 4.76737,
                -28.073985, 77.243424, 7.421083,
                -42.181997, 61.469863, -11.344811,
                -23.015192, 66.528656, -8.691098,
                -39.528284, 45.357682, 0.203151,
                -20.361479, 50.416475, 2.856865,
                -44.587077, 56.07245, 16.315332,
                -25.420272, 61.131243, 18.969046,
                -42.181997, 61.469863, -11.344811,
                -23.015192, 66.528656, -8.691098,
                -39.528284, 45.357682, 0.203151,
                -20.361479, 50.416475, 2.856865,
                -23.015192, 66.528656, -8.691098,
                -28.073985, 77.243424, 7.421083,
                -20.361479, 50.416475, 2.856865,
                -25.420272, 61.131243, 18.969046,
                -28.073985, 77.243424, 7.421083,
                -47.24079, 72.18463, 4.76737,
                -25.420272, 61.131243, 18.969046,
                -44.587077, 56.07245, 16.315332,
                -47.24079, 72.18463, 4.76737,
                -42.181997, 61.469863, -11.344811,
                -44.587077, 56.07245, 16.315332,
                -39.528284, 45.357682, 0.203151,
            ],
            vec![
                0,1,3,
                4,5,7,
                8,9,11,
                12,13,15,
                16,17,19,
                20,21,23,
                0,3,2,
                4,7,6,
                8,11,10,
                12,15,14,
                16,19,18,
                20,23,22,
            ]);

        let box3 = Mesh::new(
            vec![
                46.692477, 132.27541, 16.515862,
                65.33002, 139.523639, 16.840141,
                53.636796, 114.713956, 9.930139,
                72.27434, 121.962185, 10.254418,
                55.738797, 108.464276, 28.811958,
                74.376341, 115.712505, 29.136237,
                48.794478, 126.02573, 35.397681,
                67.432021, 133.273959, 35.72196,
                53.636796, 114.713956, 9.930139,
                72.27434, 121.962185, 10.254418,
                55.738797, 108.464276, 28.811958,
                74.376341, 115.712505, 29.136237,
                72.27434, 121.962185, 10.254418,
                65.33002, 139.523639, 16.840141,
                74.376341, 115.712505, 29.136237,
                67.432021, 133.273959, 35.72196,
                65.33002, 139.523639, 16.840141,
                46.692477, 132.27541, 16.515862,
                67.432021, 133.273959, 35.72196,
                48.794478, 126.02573, 35.397681,
                46.692477, 132.27541, 16.515862,
                53.636796, 114.713956, 9.930139,
                48.794478, 126.02573, 35.397681,
                55.738797, 108.464276, 28.811958,
            ],
            vec![
                0,1,3,
                4,5,7,
                8,9,11,
                12,13,15,
                16,17,19,
                20,21,23,
                0,3,2,
                4,7,6,
                8,11,10,
                12,15,14,
                16,19,18,
                20,23,22,
            ]);

        let wing = Mesh::new(
            vec![
                23.175826, 37.836765, -7.817897,
                29.620183, 12.342433, -7.786009,
                10.411304, 14.060205, 9.808478,
                9.526543, 55.868835, 10.216331,

            ],
            vec![
                0,1,2,
                0,2,3,
            ]);

        let diamond1 = Mesh::new(
            vec![
                -30.110934, -30.634332, -29.828453,
                -21.45068, -30.634332, -24.828453,
                -23.98721, -24.510608, -24.828453,
                -30.110934, -21.974078, -24.828453,
                -36.234658, -24.510608, -24.828453,
                -38.771188, -30.634332, -24.828453,
                -36.234658, -36.758056, -24.828453,
                -30.110934, -39.294586, -24.828453,
                -23.98721, -36.758056, -24.828453,
                -21.45068, -30.634332, -14.828453,
                -23.98721, -24.510608, -14.828453,
                -30.110934, -21.974078, -14.828453,
                -36.234658, -24.510608, -14.828453,
                -38.771188, -30.634332, -14.828453,
                -36.234658, -36.758056, -14.828453,
                -30.110934, -39.294586, -14.828453,
                -23.98721, -36.758056, -14.828453,
                -30.110934, -30.634332, -9.828453,

            ],
            vec![
                0,2,1,
                0,3,2,
                0,4,3,
                0,5,4,
                0,6,5,
                0,7,6,
                0,8,7,
                0,1,8,
                1,2,10,
                2,3,11,
                3,4,12,
                4,5,13,
                5,6,14,
                6,7,15,
                7,8,16,
                8,1,16,
                9,10,17,
                10,11,17,
                11,12,17,
                12,13,17,
                13,14,17,
                14,15,17,
                15,16,17,
                16,9,17,
                1,10,9,
                2,11,10,
                3,12,11,
                4,13,12,
                5,14,13,
                6,15,14,
                7,16,15,
                1,9,16,
            ]);

        let diamond2 = Mesh::new(
            vec![
                -69.049138, -548.045781, -10.0,
                -60.388884, -548.045781, -5.0,
                -62.925414, -541.922056, -5.0,
                -69.049138, -539.385527, -5.0,
                -75.172863, -541.922056, -5.0,
                -77.709392, -548.045781, -5.0,
                -75.172863, -554.169505, -5.0,
                -69.049138, -556.706035, -5.0,
                -62.925414, -554.169505, -5.0,
                -60.388884, -548.045781, 5.0,
                -62.925414, -541.922056, 5.0,
                -69.049138, -539.385527, 5.0,
                -75.172863, -541.922056, 5.0,
                -77.709392, -548.045781, 5.0,
                -75.172863, -554.169505, 5.0,
                -69.049138, -556.706035, 5.0,
                -62.925414, -554.169505, 5.0,
                -69.049138, -548.045781, 10.0,
            ],
            vec![
                0,2,1,
                0,3,2,
                0,4,3,
                0,5,4,
                0,6,5,
                0,7,6,
                0,8,7,
                0,1,8,
                1,2,10,
                2,3,11,
                3,4,12,
                4,5,13,
                5,6,14,
                6,7,15,
                7,8,16,
                8,1,16,
                9,10,17,
                10,11,17,
                11,12,17,
                12,13,17,
                13,14,17,
                14,15,17,
                15,16,17,
                16,9,17,
                1,10,9,
                2,11,10,
                3,12,11,
                4,13,12,
                5,14,13,
                6,15,14,
                7,16,15,
                1,9,16,
            ]);

        let input = vec![box1, wing, box2, diamond1, box3, diamond2];

        let actual = Mesh::deduplicate(input, 0.001);

        let expected_box1 = Mesh::new(
            vec![
                -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.3333334368408, 13.333332954932288, -19.999999899853734
            ],
            vec![
                0, 1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 0, 3, 2, 4, 7, 6, 8, 11, 10, 12, 15, 14, 16, 19, 18, 20, 23, 22
            ]);

        let expected_wing = Mesh::new(
            vec![
                -15.413643711502385, -8.44035165205959, -1.7763568394002505e-15, 10.882586400706039, -8.440351652059588, -8.881784197001252e-16, 4.531057310796346, 16.880703304119173, -3.552713678800501e-15, -36.21895891738076, 10.449850067645535, -6.854997638793775
            ],
            vec![
                0, 1, 2, 0, 2, 3
            ]);

        let expected_diamond1 = Mesh::new(
            vec![
                -5.934433476851259, -2.084559503484389, -1.3322676295501878e-15, 1.86886739088493, 4.169119006968773, -1.3322676295501878e-15, 4.065566085966331, -2.0845595034843885, -1.3322676295501878e-15, 1.8688673908849305, -7.8238223025583915, -2.4838191228970192, -3.4344333675556538, -9.686686508995903, -5.996470270246201, -8.737734125996234, -6.581912686815779, -8.480290039815763, -10.934432821077639, -0.3282341763626172, -8.480290039815767, -8.737734125996234, 5.411028622711387, -5.996470916918746, -3.4344333675556538, 7.273892829148897, -2.483819769569566, 6.868867609476145, 5.925444334090544, -8.480290039815763, 9.065566304557546, -0.3282341763626171, -8.480290039815763, 6.868867609476146, -6.067496975436621, -10.964109162712782, 1.5655668510355616, -7.930361181874131, -14.476760310061962, -3.737733907405018, -4.825587359694008, -16.960580079631526, -5.934432602486424, 1.4280911507591543, -16.960580079631526, -3.737733907405018, 7.167353949833158, -14.476760956734509, 1.5655668510355616, 9.030218156270669, -10.964109809385327, 4.0655669603311715, 1.4280911507591543, -16.960580079631526

            ],
            vec![
                0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 7, 6, 0, 8, 7, 0, 1, 8, 1, 2, 10, 2, 3, 11, 3, 4, 12, 4, 5, 13, 5, 6, 14, 6, 7, 15, 7, 8, 16, 8, 1, 16, 9, 10, 17, 10, 11, 17, 11, 12, 17, 12, 13, 17, 13, 14, 17, 14, 15, 17, 15, 16, 17, 16, 9, 17, 1, 10, 9, 2, 11, 10, 3, 12, 11, 4, 13, 12, 5, 14, 13, 6, 15, 14, 7, 16, 15, 1, 9, 16
            ]);

        let expected: Vec<(Mesh, Vec<LocalCoordinateSystem>)> = vec![
            (expected_box1, vec![
                LocalCoordinateSystem::new(
                    Point::new(63.331977333333334,24.977273666666665,-40.596234),
                    Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
                    Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
                ),
                LocalCoordinateSystem::new(
                    Point::new(-32.77665566666666,71.98557,1.1657849999999996),
                    Vector::new(0.9583402460123103,0.2529396989475088,0.13268564944789007),
                    Vector::new(0.2529396670241323,-0.5357383830896744,-0.80560903031817),
                ),
                LocalCoordinateSystem::new(
                    Point::new(61.432278999999994,131.25374466666665,14.536807000000001),
                    Vector::new(0.931877162134053,0.36241145471899117,0.01621395021112327),
                    Vector::new(0.3472159773829504,-0.8780727160278832,-0.3292861527872357),
                ),
            ]),
            (expected_wing, vec![
                LocalCoordinateSystem::new(
                    Point::new(21.069104333333332,21.413134333333332,-1.9318093333333337),
                    Vector::new(0.2450677139841467,-0.9695052063057461,0.0012126453055792158),
                    Vector::new(-0.6971401592935829,-0.17535085044260348,0.6951601812166558),
                )
            ]),
            (expected_diamond1, vec![
                LocalCoordinateSystem::new(
                    Point::new(-25.182941333333332,-28.59309066666667,-26.495119666666668),
                    Vector::new(0.6123724267718453,0.6123724267718453,0.5000000218591215),
                    Vector::new(0.6207111069597122,-0.764114477778006,0.17563253271217716),
                ),
                LocalCoordinateSystem::new(
                    Point::new(-64.12114533333333,-546.0045393333334,-6.666666666666667),
                    Vector::new(0.6123723892718436,0.6123724892718458,0.4999999912404968),
                    Vector::new(0.6207111413620897,-0.7641144361177981,0.17563259237781428),
                ),
            ]),
        ];


        for act in &actual {
            println!("Mesh: {:?}", act.0);
            println!("Local:");
            for local in &act.1 {
                println!("{:?}", local);
            }
        }

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
            assert_eq!(expected[i].1.len(), actual[i].1.len());
            for j in 0..expected[i].1.len() {
                assert!(expected[i].1[j].eq_with_tolerance(&actual[i].1[j], 0.0001));
            }
        }
    }

    #[test]
    pub fn test_deduplicate_with_id_info_all_unique() {
        let box1 = Mesh::new_with_id(
            Some(0),
            vec![
                49.216652, 25.228749, -35.809109,
                66.343965, 33.868829, -41.466331,
                57.308002, 7.194164, -38.85604,
                74.435315, 15.834243, -44.513262,
                63.725571, 6.873605, -19.916345,
                80.852884, 15.513684, -25.573567,
                55.634221, 24.90819, -16.869414,
                72.761534, 33.54827, -22.526636,
                57.308002, 7.194164, -38.85604,
                74.435315, 15.834243, -44.513262,
                63.725571, 6.873605, -19.916345,
                80.852884, 15.513684, -25.573567,
                74.435315, 15.834243, -44.513262,
                66.343965, 33.868829, -41.466331,
                80.852884, 15.513684, -25.573567,
                72.761534, 33.54827, -22.526636,
                66.343965, 33.868829, -41.466331,
                49.216652, 25.228749, -35.809109,
                72.761534, 33.54827, -22.526636,
                55.634221, 24.90819, -16.869414,
                49.216652, 25.228749, -35.809109,
                57.308002, 7.194164, -38.85604,
                55.634221, 24.90819, -16.869414,
                63.725571, 6.873605, -19.916345,
            ],
            vec![
                0,1,3,
                4,5,7,
                8,9,11,
                12,13,15,
                16,17,19,
                20,21,23,
                0,3,2,
                4,7,6,
                8,11,10,
                12,15,14,
                16,19,18,
                20,23,22,
            ]);

        let wing = Mesh::new_with_id(
            Some(1),
            vec![
                23.175826, 37.836765, -7.817897,
                29.620183, 12.342433, -7.786009,
                10.411304, 14.060205, 9.808478,
                9.526543, 55.868835, 10.216331,

            ],
            vec![
                0,1,2,
                0,2,3,
            ]);

        let diamond1 = Mesh::new_with_id(
            Some(2),
            vec![
                -30.110934, -30.634332, -29.828453,
                -21.45068, -30.634332, -24.828453,
                -23.98721, -24.510608, -24.828453,
                -30.110934, -21.974078, -24.828453,
                -36.234658, -24.510608, -24.828453,
                -38.771188, -30.634332, -24.828453,
                -36.234658, -36.758056, -24.828453,
                -30.110934, -39.294586, -24.828453,
                -23.98721, -36.758056, -24.828453,
                -21.45068, -30.634332, -14.828453,
                -23.98721, -24.510608, -14.828453,
                -30.110934, -21.974078, -14.828453,
                -36.234658, -24.510608, -14.828453,
                -38.771188, -30.634332, -14.828453,
                -36.234658, -36.758056, -14.828453,
                -30.110934, -39.294586, -14.828453,
                -23.98721, -36.758056, -14.828453,
                -30.110934, -30.634332, -9.828453,

            ],
            vec![
                0,2,1,
                0,3,2,
                0,4,3,
                0,5,4,
                0,6,5,
                0,7,6,
                0,8,7,
                0,1,8,
                1,2,10,
                2,3,11,
                3,4,12,
                4,5,13,
                5,6,14,
                6,7,15,
                7,8,16,
                8,1,16,
                9,10,17,
                10,11,17,
                11,12,17,
                12,13,17,
                13,14,17,
                14,15,17,
                15,16,17,
                16,9,17,
                1,10,9,
                2,11,10,
                3,12,11,
                4,13,12,
                5,14,13,
                6,15,14,
                7,16,15,
                1,9,16,
            ]);

        let input = vec![box1, wing, diamond1];

        let actual = Mesh::deduplicate_with_id_info(input, 0.001);

        let expected_box1 = Mesh::new_with_id(
            Some(0),
            vec![
                -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.3333334368408, 13.333332954932288, -19.999999899853734
            ],
            vec![
                0, 1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 0, 3, 2, 4, 7, 6, 8, 11, 10, 12, 15, 14, 16, 19, 18, 20, 23, 22
            ]);

        let expected_wing = Mesh::new_with_id(
            Some(1),
            vec![
                -15.413643711502385, -8.44035165205959, -1.7763568394002505e-15, 10.882586400706039, -8.440351652059588, -8.881784197001252e-16, 4.531057310796346, 16.880703304119173, -3.552713678800501e-15, -36.21895891738076, 10.449850067645535, -6.854997638793775
            ],
            vec![
                0, 1, 2, 0, 2, 3
            ]);

        let expected_diamond1 = Mesh::new_with_id(
            Some(2),
            vec![
                -5.934433476851259, -2.084559503484389, -1.3322676295501878e-15, 1.86886739088493, 4.169119006968773, -1.3322676295501878e-15, 4.065566085966331, -2.0845595034843885, -1.3322676295501878e-15, 1.8688673908849305, -7.8238223025583915, -2.4838191228970192, -3.4344333675556538, -9.686686508995903, -5.996470270246201, -8.737734125996234, -6.581912686815779, -8.480290039815763, -10.934432821077639, -0.3282341763626172, -8.480290039815767, -8.737734125996234, 5.411028622711387, -5.996470916918746, -3.4344333675556538, 7.273892829148897, -2.483819769569566, 6.868867609476145, 5.925444334090544, -8.480290039815763, 9.065566304557546, -0.3282341763626171, -8.480290039815763, 6.868867609476146, -6.067496975436621, -10.964109162712782, 1.5655668510355616, -7.930361181874131, -14.476760310061962, -3.737733907405018, -4.825587359694008, -16.960580079631526, -5.934432602486424, 1.4280911507591543, -16.960580079631526, -3.737733907405018, 7.167353949833158, -14.476760956734509, 1.5655668510355616, 9.030218156270669, -10.964109809385327, 4.0655669603311715, 1.4280911507591543, -16.960580079631526

            ],
            vec![
                0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 7, 6, 0, 8, 7, 0, 1, 8, 1, 2, 10, 2, 3, 11, 3, 4, 12, 4, 5, 13, 5, 6, 14, 6, 7, 15, 7, 8, 16, 8, 1, 16, 9, 10, 17, 10, 11, 17, 11, 12, 17, 12, 13, 17, 13, 14, 17, 14, 15, 17, 15, 16, 17, 16, 9, 17, 1, 10, 9, 2, 11, 10, 3, 12, 11, 4, 13, 12, 5, 14, 13, 6, 15, 14, 7, 16, 15, 1, 9, 16
            ]);

        let mut expected_box1_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
        expected_box1_map.insert(0,
                            LocalCoordinateSystem::new(
                                Point::new(63.331977333333334,24.977273666666665,-40.596234),
                                Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
                                Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
                            )
        );

        let mut expected_wing_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
        expected_wing_map.insert(1,
                                 LocalCoordinateSystem::new(
                                     Point::new(21.069104333333332,21.413134333333332,-1.9318093333333337),
                                     Vector::new(0.2450677139841467,-0.9695052063057461,0.0012126453055792158),
                                     Vector::new(-0.6971401592935829,-0.17535085044260348,0.6951601812166558),
                                 )
        );

        let mut expected_diamond1_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
        expected_diamond1_map.insert(2,
                                 LocalCoordinateSystem::new(
                                     Point::new(-25.182941333333332,-28.59309066666667,-26.495119666666668),
                                     Vector::new(0.6123724267718453,0.6123724267718453,0.5000000218591215),
                                     Vector::new(0.6207111069597122,-0.764114477778006,0.17563253271217716),
                                 )
        );

        let expected = vec![
            (expected_box1, expected_box1_map),
            (expected_wing, expected_wing_map),
            (expected_diamond1, expected_diamond1_map),
        ];

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
            let expected_map = &expected[i].1;
            let actual_map = &actual[i].1;
            for expected_key_value_pair in expected_map {
                assert!(actual_map.contains_key(expected_key_value_pair.0));
                let expected_value = expected_key_value_pair.1;
                let actual_value = actual_map[expected_key_value_pair.0];
                assert!(expected_value.eq_with_tolerance(&actual_value, 0.0001));
            }
        }
    }

    #[test]
    pub fn test_deduplicate_with_id_info_3_welded_boxes() {
        let box_a = Mesh::new_with_id(
        Some(0),
        vec![
            72.761534, 33.54827, -22.526636,
            66.343965, 33.868829, -41.466331,
            80.852884, 15.513684, -25.573567,
            74.435315, 15.834243, -44.513262,
            55.634221, 24.90819, -16.869414,
            49.216652, 25.228749, -35.809109,
            63.725571, 6.873605, -19.916345,
            57.308002, 7.194164, -38.85604,
        ],
        vec![
            5,1,3,
            6,2,0,
            7,3,2,
            3,1,0,
            1,5,4,
            5,7,6,
            5,3,7,
            6,0,4,
            7,2,6,
            3,0,2,
            1,4,0,
            5,6,4,
        ]);

        let box_b = Mesh::new_with_id(
        Some(1),
        vec![
            -25.420272, 61.131243, 18.969046,
            -28.073985, 77.243424, 7.421083,
            -20.361479, 50.416475, 2.856865,
            -23.015192, 66.528656, -8.691098,
            -44.587077, 56.07245, 16.315332,
            -47.24079, 72.18463, 4.76737,
            -39.528284, 45.357682, 0.203151,
            -42.181997, 61.469863, -11.344811,
        ],
        vec![
            5,1,3,
            6,2,0,
            7,3,2,
            3,1,0,
            1,5,4,
            5,7,6,
            5,3,7,
            6,0,4,
            7,2,6,
            3,0,2,
            1,4,0,
            5,6,4,
        ]);

        let box_c = Mesh::new_with_id(
        Some(2),
        vec![
            67.432021, 133.273959, 35.72196,
            65.33002, 139.523639, 16.840141,
            74.376341, 115.712505, 29.136237,
            72.27434, 121.962185, 10.254418,
            48.794478, 126.02573, 35.397681,
            46.692477, 132.27541, 16.515862,
            55.738797, 108.464276, 28.811958,
            53.636796, 114.713956, 9.930139,
        ],
        vec![
            5,1,3,
            6,2,0,
            7,3,2,
            3,1,0,
            1,5,4,
            5,7,6,
            5,3,7,
            6,0,4,
            7,2,6,
            3,0,2,
            1,4,0,
            5,6,4,
        ]);

        let input = vec![box_a, box_b, box_c];

        let actual = Mesh::deduplicate_with_id_info(input, 0.001);

        let expected_box = Mesh::new_with_id(
        Some(0),
        vec![
            6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8
        ],
        vec![
            5,1,3,
            6,2,0,
            7,3,2,
            3,1,0,
            1,5,4,
            5,7,6,
            5,3,7,
            6,0,4,
            7,2,6,
            3,0,2,
            1,4,0,
            5,6,4,
        ]);

        let mut expected_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
        expected_map.insert(0,
            LocalCoordinateSystem::new(
                Point::new(63.331977333333334,24.977273666666665,-40.596234),
                Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
                Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
            )
        );
        expected_map.insert(1,
            LocalCoordinateSystem::new(
                Point::new(-32.77665566666666,71.98557,1.1657849999999996),
                Vector::new(0.9583402460123103,0.2529396989475088,0.13268564944789007),
                Vector::new(0.2529396670241323,-0.5357383830896744,-0.80560903031817),
            )
        );
        expected_map.insert(2,
            LocalCoordinateSystem::new(
                Point::new(61.432278999999994,131.25374466666665,14.536807000000001),
                Vector::new(0.931877162134053,0.36241145471899117,0.01621395021112327),
                Vector::new(0.3472159773829504,-0.8780727160278832,-0.3292861527872357),
            )
        );

        let expected = vec![(expected_box, expected_map)];

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
            let expected_map = &expected[i].1;
            let actual_map = &actual[i].1;
            for expected_key_value_pair in expected_map {
                assert!(actual_map.contains_key(expected_key_value_pair.0));
                let expected_value = expected_key_value_pair.1;
                let actual_value = actual_map[expected_key_value_pair.0];
                assert!(expected_value.eq_with_tolerance(&actual_value, 0.0001));
            }
        }
    }

    #[test]
    fn test_deduplicate_with_id_info_3_boxes_2_diamonds_1_wing() {
        let box1 = Mesh::new_with_id(
            Some(0),
            vec![
                49.216652, 25.228749, -35.809109,
                66.343965, 33.868829, -41.466331,
                57.308002, 7.194164, -38.85604,
                74.435315, 15.834243, -44.513262,
                63.725571, 6.873605, -19.916345,
                80.852884, 15.513684, -25.573567,
                55.634221, 24.90819, -16.869414,
                72.761534, 33.54827, -22.526636,
                57.308002, 7.194164, -38.85604,
                74.435315, 15.834243, -44.513262,
                63.725571, 6.873605, -19.916345,
                80.852884, 15.513684, -25.573567,
                74.435315, 15.834243, -44.513262,
                66.343965, 33.868829, -41.466331,
                80.852884, 15.513684, -25.573567,
                72.761534, 33.54827, -22.526636,
                66.343965, 33.868829, -41.466331,
                49.216652, 25.228749, -35.809109,
                72.761534, 33.54827, -22.526636,
                55.634221, 24.90819, -16.869414,
                49.216652, 25.228749, -35.809109,
                57.308002, 7.194164, -38.85604,
                55.634221, 24.90819, -16.869414,
                63.725571, 6.873605, -19.916345,
            ],
            vec![
                0,1,3,
                4,5,7,
                8,9,11,
                12,13,15,
                16,17,19,
                20,21,23,
                0,3,2,
                4,7,6,
                8,11,10,
                12,15,14,
                16,19,18,
                20,23,22,
            ]);

        let box2 = Mesh::new_with_id(
            Some(1),
            vec![
                -47.24079, 72.18463, 4.76737,
                -28.073985, 77.243424, 7.421083,
                -42.181997, 61.469863, -11.344811,
                -23.015192, 66.528656, -8.691098,
                -39.528284, 45.357682, 0.203151,
                -20.361479, 50.416475, 2.856865,
                -44.587077, 56.07245, 16.315332,
                -25.420272, 61.131243, 18.969046,
                -42.181997, 61.469863, -11.344811,
                -23.015192, 66.528656, -8.691098,
                -39.528284, 45.357682, 0.203151,
                -20.361479, 50.416475, 2.856865,
                -23.015192, 66.528656, -8.691098,
                -28.073985, 77.243424, 7.421083,
                -20.361479, 50.416475, 2.856865,
                -25.420272, 61.131243, 18.969046,
                -28.073985, 77.243424, 7.421083,
                -47.24079, 72.18463, 4.76737,
                -25.420272, 61.131243, 18.969046,
                -44.587077, 56.07245, 16.315332,
                -47.24079, 72.18463, 4.76737,
                -42.181997, 61.469863, -11.344811,
                -44.587077, 56.07245, 16.315332,
                -39.528284, 45.357682, 0.203151,
            ],
            vec![
                0,1,3,
                4,5,7,
                8,9,11,
                12,13,15,
                16,17,19,
                20,21,23,
                0,3,2,
                4,7,6,
                8,11,10,
                12,15,14,
                16,19,18,
                20,23,22,
            ]);

        let box3 = Mesh::new_with_id(
            Some(2),
            vec![
                46.692477, 132.27541, 16.515862,
                65.33002, 139.523639, 16.840141,
                53.636796, 114.713956, 9.930139,
                72.27434, 121.962185, 10.254418,
                55.738797, 108.464276, 28.811958,
                74.376341, 115.712505, 29.136237,
                48.794478, 126.02573, 35.397681,
                67.432021, 133.273959, 35.72196,
                53.636796, 114.713956, 9.930139,
                72.27434, 121.962185, 10.254418,
                55.738797, 108.464276, 28.811958,
                74.376341, 115.712505, 29.136237,
                72.27434, 121.962185, 10.254418,
                65.33002, 139.523639, 16.840141,
                74.376341, 115.712505, 29.136237,
                67.432021, 133.273959, 35.72196,
                65.33002, 139.523639, 16.840141,
                46.692477, 132.27541, 16.515862,
                67.432021, 133.273959, 35.72196,
                48.794478, 126.02573, 35.397681,
                46.692477, 132.27541, 16.515862,
                53.636796, 114.713956, 9.930139,
                48.794478, 126.02573, 35.397681,
                55.738797, 108.464276, 28.811958,
            ],
            vec![
                0,1,3,
                4,5,7,
                8,9,11,
                12,13,15,
                16,17,19,
                20,21,23,
                0,3,2,
                4,7,6,
                8,11,10,
                12,15,14,
                16,19,18,
                20,23,22,
            ]);

        let wing = Mesh::new_with_id(
            Some(3),
            vec![
                23.175826, 37.836765, -7.817897,
                29.620183, 12.342433, -7.786009,
                10.411304, 14.060205, 9.808478,
                9.526543, 55.868835, 10.216331,

            ],
            vec![
                0,1,2,
                0,2,3,
            ]);

        let diamond1 = Mesh::new_with_id(
            Some(4),
            vec![
                -30.110934, -30.634332, -29.828453,
                -21.45068, -30.634332, -24.828453,
                -23.98721, -24.510608, -24.828453,
                -30.110934, -21.974078, -24.828453,
                -36.234658, -24.510608, -24.828453,
                -38.771188, -30.634332, -24.828453,
                -36.234658, -36.758056, -24.828453,
                -30.110934, -39.294586, -24.828453,
                -23.98721, -36.758056, -24.828453,
                -21.45068, -30.634332, -14.828453,
                -23.98721, -24.510608, -14.828453,
                -30.110934, -21.974078, -14.828453,
                -36.234658, -24.510608, -14.828453,
                -38.771188, -30.634332, -14.828453,
                -36.234658, -36.758056, -14.828453,
                -30.110934, -39.294586, -14.828453,
                -23.98721, -36.758056, -14.828453,
                -30.110934, -30.634332, -9.828453,

            ],
            vec![
                0,2,1,
                0,3,2,
                0,4,3,
                0,5,4,
                0,6,5,
                0,7,6,
                0,8,7,
                0,1,8,
                1,2,10,
                2,3,11,
                3,4,12,
                4,5,13,
                5,6,14,
                6,7,15,
                7,8,16,
                8,1,16,
                9,10,17,
                10,11,17,
                11,12,17,
                12,13,17,
                13,14,17,
                14,15,17,
                15,16,17,
                16,9,17,
                1,10,9,
                2,11,10,
                3,12,11,
                4,13,12,
                5,14,13,
                6,15,14,
                7,16,15,
                1,9,16,
            ]);

        let diamond2 = Mesh::new_with_id(
            Some(5),
            vec![
                -69.049138, -548.045781, -10.0,
                -60.388884, -548.045781, -5.0,
                -62.925414, -541.922056, -5.0,
                -69.049138, -539.385527, -5.0,
                -75.172863, -541.922056, -5.0,
                -77.709392, -548.045781, -5.0,
                -75.172863, -554.169505, -5.0,
                -69.049138, -556.706035, -5.0,
                -62.925414, -554.169505, -5.0,
                -60.388884, -548.045781, 5.0,
                -62.925414, -541.922056, 5.0,
                -69.049138, -539.385527, 5.0,
                -75.172863, -541.922056, 5.0,
                -77.709392, -548.045781, 5.0,
                -75.172863, -554.169505, 5.0,
                -69.049138, -556.706035, 5.0,
                -62.925414, -554.169505, 5.0,
                -69.049138, -548.045781, 10.0,
            ],
            vec![
                0,2,1,
                0,3,2,
                0,4,3,
                0,5,4,
                0,6,5,
                0,7,6,
                0,8,7,
                0,1,8,
                1,2,10,
                2,3,11,
                3,4,12,
                4,5,13,
                5,6,14,
                6,7,15,
                7,8,16,
                8,1,16,
                9,10,17,
                10,11,17,
                11,12,17,
                12,13,17,
                13,14,17,
                14,15,17,
                15,16,17,
                16,9,17,
                1,10,9,
                2,11,10,
                3,12,11,
                4,13,12,
                5,14,13,
                6,15,14,
                7,16,15,
                1,9,16,
            ]);

        let input = vec![box1, wing, box2, diamond1, box3, diamond2];

        let actual = Mesh::deduplicate_with_id_info(input, 0.001);

        let expected_box1 = Mesh::new_with_id(
            Some(0),
            vec![
                -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, -13.3333334368408, 13.333332954932288, -19.999999899853734, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666058742171, 13.333333758877597, -3.9968028886505635e-15, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, 6.666665975246522, 13.333333856661543, -19.999999915881677, 6.666666809178926, -6.666666781654859, -19.999999915881677, 6.666666892674573, -6.666666879438801, -4.6629367034256575e-15, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, 6.666666809178926, -6.666666781654859, -19.999999915881677, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.333332951416747, -6.666666879438802, -5.329070518200751e-15, -13.333333353345152, 13.333332857148344, 1.6027941862617467e-8, -13.333333034912403, -6.666666781654859, -19.99999991588168, -13.3333334368408, 13.333332954932288, -19.999999899853734
            ],
            vec![
                0, 1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 0, 3, 2, 4, 7, 6, 8, 11, 10, 12, 15, 14, 16, 19, 18, 20, 23, 22
            ]);

        let expected_wing = Mesh::new_with_id(
            Some(3),
            vec![
                -15.413643711502385, -8.44035165205959, -1.7763568394002505e-15, 10.882586400706039, -8.440351652059588, -8.881784197001252e-16, 4.531057310796346, 16.880703304119173, -3.552713678800501e-15, -36.21895891738076, 10.449850067645535, -6.854997638793775
            ],
            vec![
                0, 1, 2, 0, 2, 3
            ]);

        let expected_diamond1 = Mesh::new_with_id(
            Some(4),
            vec![
                -5.934433476851259, -2.084559503484389, -1.3322676295501878e-15, 1.86886739088493, 4.169119006968773, -1.3322676295501878e-15, 4.065566085966331, -2.0845595034843885, -1.3322676295501878e-15, 1.8688673908849305, -7.8238223025583915, -2.4838191228970192, -3.4344333675556538, -9.686686508995903, -5.996470270246201, -8.737734125996234, -6.581912686815779, -8.480290039815763, -10.934432821077639, -0.3282341763626172, -8.480290039815767, -8.737734125996234, 5.411028622711387, -5.996470916918746, -3.4344333675556538, 7.273892829148897, -2.483819769569566, 6.868867609476145, 5.925444334090544, -8.480290039815763, 9.065566304557546, -0.3282341763626171, -8.480290039815763, 6.868867609476146, -6.067496975436621, -10.964109162712782, 1.5655668510355616, -7.930361181874131, -14.476760310061962, -3.737733907405018, -4.825587359694008, -16.960580079631526, -5.934432602486424, 1.4280911507591543, -16.960580079631526, -3.737733907405018, 7.167353949833158, -14.476760956734509, 1.5655668510355616, 9.030218156270669, -10.964109809385327, 4.0655669603311715, 1.4280911507591543, -16.960580079631526

            ],
            vec![
                0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 7, 6, 0, 8, 7, 0, 1, 8, 1, 2, 10, 2, 3, 11, 3, 4, 12, 4, 5, 13, 5, 6, 14, 6, 7, 15, 7, 8, 16, 8, 1, 16, 9, 10, 17, 10, 11, 17, 11, 12, 17, 12, 13, 17, 13, 14, 17, 14, 15, 17, 15, 16, 17, 16, 9, 17, 1, 10, 9, 2, 11, 10, 3, 12, 11, 4, 13, 12, 5, 14, 13, 6, 15, 14, 7, 16, 15, 1, 9, 16
            ]);

        let mut expected_box1_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
        expected_box1_map.insert(0,
                                 LocalCoordinateSystem::new(
                                     Point::new(63.331977333333334,24.977273666666665,-40.596234),
                                     Vector::new(0.8563656566757418,0.43200400336765865,-0.282861102205025),
                                     Vector::new(0.40456752279544955,-0.9017292532074647,-0.15234655693208665),
                                 )
        );
        expected_box1_map.insert(1,
                                 LocalCoordinateSystem::new(
                                     Point::new(-32.77665566666666,71.98557,1.1657849999999996),
                                     Vector::new(0.9583402460123103,0.2529396989475088,0.13268564944789007),
                                     Vector::new(0.2529396670241323,-0.5357383830896744,-0.80560903031817),
                                 )
        );
        expected_box1_map.insert(2,
                                 LocalCoordinateSystem::new(
                                     Point::new(61.432278999999994,131.25374466666665,14.536807000000001),
                                     Vector::new(0.931877162134053,0.36241145471899117,0.01621395021112327),
                                     Vector::new(0.3472159773829504,-0.8780727160278832,-0.3292861527872357),
                                 )
        );

        let mut expected_wing_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
        expected_wing_map.insert(3,
                                 LocalCoordinateSystem::new(
                                     Point::new(21.069104333333332,21.413134333333332,-1.9318093333333337),
                                     Vector::new(0.2450677139841467,-0.9695052063057461,0.0012126453055792158),
                                     Vector::new(-0.6971401592935829,-0.17535085044260348,0.6951601812166558),
                                 )
        );

        let mut expected_diamond1_map: HashMap<usize, LocalCoordinateSystem> = HashMap::new();
        expected_diamond1_map.insert(4,
                                     LocalCoordinateSystem::new(
                                         Point::new(-25.182941333333332,-28.59309066666667,-26.495119666666668),
                                         Vector::new(0.6123724267718453,0.6123724267718453,0.5000000218591215),
                                         Vector::new(0.6207111069597122,-0.764114477778006,0.17563253271217716),
                                     )
        );
        expected_diamond1_map.insert(5,
                                     LocalCoordinateSystem::new(
                                         Point::new(-64.12114533333333,-546.0045393333334,-6.666666666666667),
                                         Vector::new(0.6123723892718436,0.6123724892718458,0.4999999912404968),
                                         Vector::new(0.6207111413620897,-0.7641144361177981,0.17563259237781428),
                                     )
        );

        let expected = vec![
            (expected_box1, expected_box1_map),
            (expected_wing, expected_wing_map),
            (expected_diamond1, expected_diamond1_map),
        ];

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].0.eq_with_tolerance(&actual[i].0, 0.0001));
            let expected_map = &expected[i].1;
            let actual_map = &actual[i].1;
            for expected_key_value_pair in expected_map {
                assert!(actual_map.contains_key(expected_key_value_pair.0));
                let expected_value = expected_key_value_pair.1;
                let actual_value = actual_map[expected_key_value_pair.0];
                assert!(expected_value.eq_with_tolerance(&actual_value, 0.0001));
            }
        }
    }
}