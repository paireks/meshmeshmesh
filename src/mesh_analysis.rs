use crate::bounding_box::BoundingBox;
use crate::edge::Edge;
use crate::graph::Graph;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::mesh::Mesh;
use crate::point::Point;
use crate::three_edge_group::ThreeEdgeGroup;
use crate::triangle::Triangle;
use crate::vector::Vector;
use std::collections::HashSet;

impl Mesh {

    /// Compares given [Mesh] to other one, but with a `f64` tolerance.
    ///
    /// If any coordinate absolute difference is > tolerance, then it should return `false`.
    ///
    /// It also compares indices: but only if they are exactly the same. This means, that if 
    /// the indices are different, e.g. in different order, then even if they represent at the end
    /// the same triangles: the `false` should be returned.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// 
    /// let a = Mesh::new(vec![0.0, 0.0, 0.0002,
    ///                        10.0, 0.0001, 0.0,
    ///                        10.0, -15.0001, 0.0],
    /// vec![0, 1, 2]);
    /// let b = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    /// 
    /// assert_eq!(a.eq_with_tolerance(&b,0.0002), true);
    /// assert_eq!(b.eq_with_tolerance(&a, 0.0002), true);
    /// ```
    ///
    /// In this example we can see the coordinates absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// 
    /// let a = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                        10.0, 2.0, 0.0,
    ///                        10.0, -15.0003, 0.0],
    /// vec![0, 1, 2]);
    /// let b = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    /// assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
    /// assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    /// ```
    /// 
    /// In this example we can see the difference in indices, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// 
    /// let a = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 2, 1]);
    /// let b = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    /// assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
    /// assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Mesh, tolerance: f64) -> bool {

        if self.id != other.id { 
            return false;
        }
        
        self.eq_with_tolerance_without_id(other, tolerance)
    }

    /// Compares given [Mesh] to other one, but with a `f64` tolerance.
    ///
    /// It is same as `eq_with_tolerance` method, but without the [Mesh] `id` comparison.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0002,
    ///                        10.0, 0.0001, 0.0,
    ///                        10.0, -15.0001, 0.0],
    /// vec![0, 1, 2]);
    /// let b = Mesh::new_with_id(Some(6), vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    ///
    /// assert_eq!(a.eq_with_tolerance_without_id(&b,0.0002), true);
    /// assert_eq!(b.eq_with_tolerance_without_id(&a, 0.0002), true);
    /// ```
    ///
    /// In this example we can see the coordinates absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0,
    ///                        10.0, 2.0, 0.0,
    ///                        10.0, -15.0003, 0.0],
    /// vec![0, 1, 2]);
    /// let b = Mesh::new_with_id(Some(6), vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    /// assert_eq!(a.eq_with_tolerance_without_id(&b,0.0002), false);
    /// assert_eq!(b.eq_with_tolerance_without_id(&a, 0.0002), false);
    /// ```
    ///
    /// In this example we can see the difference in indices, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 2, 1]);
    /// let b = Mesh::new_with_id(Some(6), vec![0.0, 0.0, 0.0,
    ///                        10.0, 0.0, 0.0,
    ///                        10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    /// assert_eq!(a.eq_with_tolerance_without_id(&b,0.0002), false);
    /// assert_eq!(b.eq_with_tolerance_without_id(&a, 0.0002), false);
    /// ```
    pub fn eq_with_tolerance_without_id(&self, other:&Mesh, tolerance: f64) -> bool {

        if self.indices.len() != other.indices.len() {
            return false;
        }
        for i in 0..self.indices.len() {
            if self.indices[i] != other.indices[i] {
                return false;
            }
        }

        if self.coordinates.len() != other.coordinates.len() {
            return false;
        }
        for i in 0..self.coordinates.len() {
            if (self.coordinates[i] - other.coordinates[i]).abs() > tolerance {
                return false;
            }
        }

        true
    }
    
    /// Checks if given [Mesh] is connected, meaning all vertices are connected together into
    /// single graph.
    /// 
    /// # Examples
    /// 
    /// Here is an example with 1 face separated from others so `false` is returned.
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Mesh::new(
    ///     vec![0.0, 0.0, 0.0,
    ///          2.5, 5.0, 0.0,
    ///          5.0, 0.0, 0.0,
    ///          7.5, 5.0, 0.0,
    ///          10.0, 0.0, 0.0,
    ///          5.0, 10.0, 0.0,
    ///          5.0, 5.0, 3.0,
    ///          2.5, 5.0, 3.0,
    ///          0.0, 0.0, 3.0,
    ///          ],
    ///     vec![0, 2, 1, // first face
    ///          1, 2, 3, // second face
    ///          2, 4, 3, // third face
    ///          1, 3, 5, // fourth face
    ///          7, 8, 6, // fifth face separated
    ///          ]
    /// );
    /// 
    /// assert!(!input.is_connected());
    /// 
    /// ```
    ///
    /// Here below is an example with all vertices connected so `true` is returned.
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Mesh::new(
    ///     vec![0.0, 0.0, 0.0,
    ///          2.5, 5.0, 0.0,
    ///          5.0, 0.0, 0.0,
    ///          7.5, 5.0, 0.0,
    ///          10.0, 0.0, 0.0,
    ///          5.0, 10.0, 0.0,
    ///          ],
    ///     vec![0, 2, 1, // first face
    ///          1, 2, 3, // second face
    ///          2, 4, 3, // third face
    ///          1, 3, 5, // fourth face
    ///          ]
    /// );
    ///
    /// assert!(input.is_connected());
    ///
    /// ```
    pub fn is_connected(&self) -> bool {
        let number_of_vertices = self.get_number_of_vertices();
        Graph::from_edges_into_undirected(number_of_vertices, &self.to_edges()).is_connected()
    }

    /// Gets number of all faces (triangles) which defines a [Mesh]
    ///
    /// # Example
    ///
    /// Here is an example with pyramid Mesh
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
    ///     0,1,2, // first face
    ///     0,2,3, // second face
    ///
    ///     // Side faces
    ///     0,1,4, // third face
    ///     1,2,4, // fourth face
    ///     2,3,4, // fifth face
    ///     3,0,4 // sixth face
    /// ]);
    ///
    /// let expected = 6;
    /// let actual = input.get_number_of_faces();
    /// assert_eq!(expected, actual);
    /// ```
    ///
    pub fn get_number_of_faces(&self) -> usize {
        self.indices.len() / 3
    }

    /// Gets number of all vertices (points) that defines a [Mesh]
    ///
    /// # Example
    ///
    /// Here is an example with pyramid Mesh
    ///
    /// ```
    ///
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input = Mesh::new(
    ///    vec![
    ///        // Base
    ///        -2.0,1.0,0.0, // first vertex
    ///        8.0,1.0,0.0, // second vertex
    ///        8.0,11.0,0.0, // third vertex
    ///        -2.0,11.0,0.0, // fourth vertex
    ///
    ///        // Top
    ///        3.0,6.0,4.0 // fifth vertex
    ///    ],
    ///    vec![
    ///        // Base faces
    ///        0,1,2,
    ///        0,2,3,
    ///
    ///        // Side faces
    ///        0,1,4,
    ///        1,2,4,
    ///        2,3,4,
    ///        3,0,4
    ///    ]);
    ///
    /// let expected = 5;
    /// let actual = input.get_number_of_vertices();
    /// assert_eq!(expected, actual);
    ///
    /// ```
    ///
    pub fn get_number_of_vertices(&self) -> usize {
        self.coordinates.len() / 3
    }

    /// Calculates the area for given [Mesh]
    ///
    /// # Example
    ///
    /// ```
    ///
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let input = Mesh::new(
    ///    vec![
    ///        // Base
    ///        -2.0,1.0,0.0, // first vertex
    ///        8.0,1.0,0.0, // second vertex
    ///        8.0,11.0,0.0, // third vertex
    ///        -2.0,11.0,0.0, // fourth vertex
    ///
    ///        // Top
    ///        3.0,6.0,4.0 // fifth vertex
    ///    ],
    ///    vec![
    ///        // Base faces
    ///        0,1,2,
    ///        0,2,3,
    ///
    ///        // Side faces
    ///        0,1,4,
    ///        1,2,4,
    ///        2,3,4,
    ///        3,0,4
    ///    ]);
    ///
    /// let expected = 228.062485;
    /// let actual = input.get_area();
    /// assert_eq!(((expected - actual).abs() < 0.00001), true);
    ///
    /// ```
    pub fn get_area(&self) -> f64 {
        let triangles = self.to_triangles();
        let mut sum = 0.0;
        for triangle in triangles {
            sum += triangle.get_area();
        }

        sum
    }

    /// Calculates the Bounding Box (AABB) for given [Mesh]
    ///
    /// # Example
    ///
    /// Here is an example of getting AABB for pyramid Mesh
    ///
    /// ```
    ///
    /// use meshmeshmesh::bounding_box::BoundingBox;
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
    /// let expected = BoundingBox::new(-2.0, 8.0, 1.0, 11.0, 0.0, 4.0);
    /// let actual = input.get_bounding_box();
    ///
    /// assert_eq!(expected.eq(&actual), true);
    ///
    /// ```
    pub fn get_bounding_box(&self) -> BoundingBox {
        let x_coordinates = self.get_x_coordinates();
        let y_coordinates = self.get_y_coordinates();
        let z_coordinates = self.get_z_coordinates();

        let min_x = x_coordinates.iter().copied().reduce(f64::min).unwrap();
        let max_x = x_coordinates.iter().copied().reduce(f64::max).unwrap();

        let min_y = y_coordinates.iter().copied().reduce(f64::min).unwrap();
        let max_y = y_coordinates.iter().copied().reduce(f64::max).unwrap();

        let min_z = z_coordinates.iter().copied().reduce(f64::min).unwrap();
        let max_z = z_coordinates.iter().copied().reduce(f64::max).unwrap();

        BoundingBox::new(min_x, max_x, min_y, max_y, min_z, max_z)
    }
    
    /// Gets local coordinate system, which is a local coordinate system of the very first face
    /// of the given [Mesh].
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::bounding_box::BoundingBox;
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
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
    /// let actual = input.get_local_coordinate_system_for_first_face();
    ///
    /// let expected_origin = Point::new(4.666666666666667, 4.333333333333333, 0.0 );
    /// let expected_x = Vector::new(1.0, 0.0, 0.0);
    /// let expected_y = Vector::new(6.123031769111886e-17, 1.0, 0.0);
    ///
    /// let expected = LocalCoordinateSystem::new(expected_origin, expected_x, expected_y);
    ///
    /// assert_eq!(expected, actual);
    /// ```
    pub fn get_local_coordinate_system_for_first_face(&self) -> LocalCoordinateSystem {

        let offset0 = self.indices[0];
        let index00 = usize::try_from(offset0*3).unwrap();
        let index01 = usize::try_from(offset0*3 + 1).unwrap();
        let index02 = usize::try_from(offset0*3 + 2).unwrap();
        let point0: Point = Point::new(self.coordinates[index00], self.coordinates[index01], self.coordinates[index02]);

        let offset1 = self.indices[1];
        let index10 = usize::try_from(offset1*3).unwrap();
        let index11 = usize::try_from(offset1*3 + 1).unwrap();
        let index12 = usize::try_from(offset1*3 + 2).unwrap();
        let point1: Point = Point::new(self.coordinates[index10], self.coordinates[index11], self.coordinates[index12]);

        let offset2 = self.indices[2];
        let index20 = usize::try_from(offset2*3).unwrap();
        let index21 = usize::try_from(offset2*3 + 1).unwrap();
        let index22 = usize::try_from(offset2*3 + 2).unwrap();
        let point2: Point = Point::new(self.coordinates[index20], self.coordinates[index21], self.coordinates[index22]);

        Triangle::new(point0, point1, point2).get_local_coordinate_system()
    }

    /// Gets ids of faces of the [Mesh] that probably have normals pointing inside the Mesh.
    ///
    /// It offsets an origin of the normal (using given value for `offset` argument) and checks
    /// if it's inside or outside the Mesh using XYZ check.
    ///
    /// # Example
    ///
    /// Here is an example with pyramid Mesh which has 3 faces flipped inside (with ids: 0, 3, and 5).
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
    ///     0,1,2, // flipped (0)
    ///     3,2,0,
    ///
    ///     // Side faces flipped
    ///     0,1,4,
    ///     4,2,1, // flipped (3)
    ///     2,3,4,
    ///     4,0,3 // flipped (5)
    /// ]);
    /// let actual = input.get_ids_of_faces_flipped_inside_using_offset(0.001);
    /// let expected = HashSet::from([0, 3, 5]);
    /// 
    /// for act in &actual {
    ///     println!("{0}", act);
    /// }
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_ids_of_faces_flipped_inside_using_offset(&self, offset:f64) -> HashSet<usize> {
        let triangles_to_check = self.to_triangles();
        let triangles_length = triangles_to_check.len();

        let mut ids_of_faces_flipped: HashSet<usize> = HashSet::new();
        for i in 0..triangles_length {
            let current_face = triangles_to_check[i];
            let normal = current_face.get_normal_ray();
            let point_to_check = normal.get_point_at(offset);
            if point_to_check.is_inside_mesh_using_xyz(self) {
                ids_of_faces_flipped.insert(i);
            }
        }

        ids_of_faces_flipped
    }

    /// Gets only x coordinates of [Mesh]
    ///
    /// In other words: it gets all vertices, and then takes only X coordinate from each
    ///
    /// # Example
    ///
    /// Here is an example of getting all x coordinates only
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
    /// let expected_x = vec![-2.0, 8.0, 8.0, -2.0, 3.0];
    /// let actual_x = input.get_x_coordinates(); // <- getting x coordinates only
    /// assert_eq!(expected_x, actual_x);
    /// ```
    pub fn get_x_coordinates(&self) -> Vec<f64> {
        self.coordinates.iter().skip(0).step_by(3).copied().collect()
    }

    /// Gets only y coordinates of [Mesh]
    ///
    /// In other words: it gets all vertices, and then takes only Y coordinate from each
    ///
    /// # Example
    ///
    /// Here is an example of getting all y coordinates only
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
    ///
    /// let expected_y = vec![1.0, 1.0, 11.0, 11.0, 6.0];
    /// let actual_y = input.get_y_coordinates(); // <- getting y coordinates only
    /// assert_eq!(expected_y, actual_y);
    /// ```
    pub fn get_y_coordinates(&self) -> Vec<f64> {
        self.coordinates.iter().skip(1).step_by(3).copied().collect()
    }

    /// Gets only z coordinates of [Mesh]
    ///
    /// In other words: it gets all vertices, and then takes only Z coordinate from each
    ///
    /// # Example
    ///
    /// Here is an example of getting all z coordinates only
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
    ///
    /// let expected_z = vec![0.0, 0.0, 0.0, 0.0, 4.0];
    /// let actual_z = input.get_z_coordinates(); // <- getting z coordinates only
    /// assert_eq!(expected_z, actual_z);
    /// ```
    pub fn get_z_coordinates(&self) -> Vec<f64> {
        self.coordinates.iter().skip(2).step_by(3).copied().collect()
    }

    /// Gets all faces' start indices
    ///
    /// In other words: it gets all faces, and then takes only start index from each
    ///
    /// # Example
    ///
    /// Here is an example of getting all starting indices only
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
    ///
    /// let expected = vec![0, 0, 0, 1, 2, 3];
    /// let actual = input.get_start_indices();
    /// assert_eq!(expected, actual);
    /// ```
    pub fn get_start_indices(&self) -> Vec<usize> {
        self.indices.iter().skip(0).step_by(3).copied().collect()
    }

    /// Gets all faces' middle indices
    ///
    /// In other words: it gets all faces, and then takes only middle index from each
    ///
    /// # Example
    ///
    /// Here is an example of getting all middle indices only
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
    ///
    /// let expected = vec![1, 2, 1, 2, 3, 0];
    /// let actual = input.get_middle_indices();
    /// assert_eq!(expected, actual);
    /// ```
    pub fn get_middle_indices(&self) -> Vec<usize> {
        self.indices.iter().skip(1).step_by(3).copied().collect()
    }

    /// Gets all faces' end indices
    ///
    /// In other words: it gets all faces, and then takes only end index from each
    ///
    /// # Example
    ///
    /// Here is an example of getting all ending indices only.
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
    ///
    /// let expected = vec![2, 3, 4, 4, 4, 4];
    /// let actual = input.get_end_indices();
    /// assert_eq!(expected, actual);
    /// ```
    pub fn get_end_indices(&self) -> Vec<usize> {
        self.indices.iter().skip(2).step_by(3).copied().collect()
    }

    /// Gets list of [Vector]s that represent [Mesh] normals for faces.
    ///
    /// For each [Mesh] face there is 1 corresponding normal [Vector].
    ///
    /// These [Vector]s should be unitized.
    ///
    /// # Example
    ///
    /// Here is an example with pyramid for which all normals are calculated.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
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
    ///     0,1,2, //0
    ///     0,2,3, //1
    ///
    ///     // Side faces
    ///     0,1,4, //2
    ///     1,2,4, //3
    ///     2,3,4, //4
    ///     3,0,4  //5
    /// ]);
    ///
    ///
    /// let expected = vec![
    /// // Base faces normals
    /// Vector::new(0.0,0.0,1.0), //0
    /// Vector::new(0.0,0.0,1.0), //1
    ///
    /// // Side faces normals
    /// Vector::new(0.0,-0.624695,0.780869), //2
    /// Vector::new(0.624695,0.0,0.780869),  //3
    /// Vector::new(0.0,0.624695,0.780869),  //4
    /// Vector::new(-0.624695,0.0,0.780869), //5
    /// ];
    ///
    ///
    /// let actual = input.get_face_normal_vectors_unitized();
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert_eq!(expected[i].eq_with_tolerance(&actual[i], 0.0001), true);
    /// }
    ///
    /// ```
    pub fn get_face_normal_vectors_unitized(&self) -> Vec<Vector> {
        let triangles = self.to_triangles();

        triangles.iter().map(|triangle| triangle.get_normal_vector_unitized()).collect()
    }

    /// Gets edges with only 1 face-neighbour.
    ///
    /// Normally in manifold [Mesh]es each edge should have 2 face-neighbours. That's why it's
    /// sometimes useful to detect edges with only 1 face-neighbour.
    ///
    /// # Examples
    ///
    /// In the example below there is a planar [Mesh] with some edges connected only on 1 side,
    /// that's why these [Edge]s should be returned.
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Mesh::new(
    ///     vec![0.0, 0.0, 0.0,
    ///          2.5, 5.0, 0.0,
    ///          5.0, 0.0, 0.0,
    ///          7.5, 5.0, 0.0,
    ///          10.0, 0.0, 0.0,
    ///          5.0, 10.0, 0.0,
    ///          ],
    ///     vec![0, 2, 1, // first face
    ///          1, 2, 3, // second face
    ///          2, 4, 3, // third face
    ///          1, 3, 5, // fourth face
    ///          ]
    /// );
    ///
    /// let mut actual = input.get_edges_with_missing_neighbour();
    /// let mut expected = vec![
    ///     Edge::new(0, 2), // first face, first edge
    ///     Edge::new(1, 0), // first face, third edge
    ///     Edge::new(2, 4), // third face, first edge
    ///     Edge::new(4, 3), // third face, second edge
    ///     Edge::new(3, 5), // fourth face, second edge
    ///     Edge::new(5, 1), // fourth face, third edge
    /// ];
    /// actual.sort();
    /// expected.sort();
    /// assert_eq!(actual, expected);
    ///
    /// ```
    ///
    /// In the example below there is a pyramid [Mesh] with a manifold edges, that's why
    /// empty `vec` of [Edge]s should be returned.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
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
    ///     0,1,2, //0
    ///     0,2,3, //1
    ///
    ///     // Side faces
    ///     0,1,4, //2
    ///     1,2,4, //3
    ///     2,3,4, //4
    ///     3,0,4  //5
    /// ]);
    ///
    /// let actual = input.get_edges_with_missing_neighbour();
    /// assert_eq!(actual.len(), 0);
    ///
    /// ```
    pub fn get_edges_with_missing_neighbour(&self) -> Vec<Edge> {

        let three_edge_groups = self.to_three_edge_groups();
        let edge_hashmap = ThreeEdgeGroup::get_edge_with_face_ids_hashmap_with_reversed_edges_merged(&three_edge_groups);
        let mut edges_with_missing_neighbour: Vec<Edge> = Vec::new();
        
        for (key, value) in edge_hashmap.into_iter() {
            let current_edge = key;
            let number_of_neighbour_faces = value.len();
            if number_of_neighbour_faces == 1 {
                edges_with_missing_neighbour.push(current_edge);
            }
        }
        
        edges_with_missing_neighbour
    }

    /// Gets edges with less or more than 2 face-neighbours.
    /// 
    /// Such edges are sometimes called non-manifold.
    /// 
    /// Manifold edges are only the edges that have 2 faces connected with them.
    ///
    /// # Examples
    ///
    /// In the example below there is a [Mesh] with some non-manifold [Edge]s.
    /// that's why these [Edge]s should be returned.
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Mesh::new(
    ///     vec![0.0, 0.0, 0.0,
    ///          2.5, 5.0, 0.0,
    ///          5.0, 0.0, 0.0,
    ///          7.5, 5.0, 0.0,
    ///          10.0, 0.0, 0.0,
    ///          5.0, 10.0, 0.0,
    ///          5.0, 5.0, 3.0,
    ///          ],
    ///     vec![0, 2, 1, // first face
    ///          1, 2, 3, // second face
    ///          2, 4, 3, // third face
    ///          1, 3, 5, // fourth face
    ///          1, 3, 6, // fifth face
    ///          ]
    /// );
    ///
    /// let mut actual = input.get_non_manifold_edges();
    /// let mut expected = vec![
    ///     Edge::new(0, 2), // first face, first edge, 1 neighbour
    ///     Edge::new(1, 0), // first face, third edge, 1 neighbour
    ///     Edge::new(3, 1), // second face, third edge, 3 neighbours
    ///     Edge::new(2, 4), // third face, first edge, 1 neighbour
    ///     Edge::new(4, 3), // third face, second edge, 1 neighbour
    ///     Edge::new(3, 5), // fourth face, second edge, 1 neighbour
    ///     Edge::new(5, 1), // fourth face, third edge, 1 neighbour
    ///     Edge::new(3, 6), // fifth face, second edge, 1 neighbour
    ///     Edge::new(6, 1), // fifth face, third edge, 1 neighbour
    /// ];
    /// actual.sort();
    /// expected.sort();
    /// assert_eq!(actual, expected);
    ///
    /// ```
    ///
    /// In the example below there is a pyramid [Mesh] with a manifold edges, that's why
    /// empty `vec` of [Edge]s should be returned.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::vector::Vector;
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
    ///     0,1,2, //0
    ///     0,2,3, //1
    ///
    ///     // Side faces
    ///     0,1,4, //2
    ///     1,2,4, //3
    ///     2,3,4, //4
    ///     3,0,4  //5
    /// ]);
    ///
    /// let actual = input.get_non_manifold_edges();
    /// assert_eq!(actual.len(), 0);
    ///
    /// ```
    pub fn get_non_manifold_edges(&self) -> Vec<Edge> {
        
        let three_edge_groups = self.to_three_edge_groups();
        let edge_hashmap = ThreeEdgeGroup::get_edge_with_face_ids_hashmap_with_reversed_edges_merged(&three_edge_groups);
        let mut non_manifold_edges: Vec<Edge> = Vec::new();

        for (key, value) in edge_hashmap.into_iter() {
            let current_edge = key;
            let number_of_neighbour_faces = value.len();
            if number_of_neighbour_faces < 2 || number_of_neighbour_faces > 2 {
                non_manifold_edges.push(current_edge);
            }
        }

        non_manifold_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_connected_false() {
        let input = Mesh::new(
            vec![0.0, 0.0, 0.0,
                 2.5, 5.0, 0.0,
                 5.0, 0.0, 0.0,
                 7.5, 5.0, 0.0,
                 10.0, 0.0, 0.0,
                 5.0, 10.0, 0.0,
                 5.0, 5.0, 3.0,
                 2.5, 5.0, 3.0,
                 0.0, 0.0, 3.0,
                 ],
            vec![0, 2, 1, // first face
                 1, 2, 3, // second face
                 2, 4, 3, // third face
                 1, 3, 5, // fourth face
                 7, 8, 6, // fifth face separated
                 ]
        );
        
        assert!(!input.is_connected());
    }

    #[test]
    fn test_is_connected_true() {
        let input = Mesh::new(
            vec![0.0, 0.0, 0.0,
                 2.5, 5.0, 0.0,
                 5.0, 0.0, 0.0,
                 7.5, 5.0, 0.0,
                 10.0, 0.0, 0.0,
                 5.0, 10.0, 0.0,
                 ],
            vec![0, 2, 1, // first face
                 1, 2, 3, // second face
                 2, 4, 3, // third face
                 1, 3, 5, // fourth face
                 ]
        );
        
        assert!(input.is_connected());
    }

    #[test]
    fn test_get_number_of_faces() {
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

        let expected = 6;
        let actual = input.get_number_of_faces();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_number_of_vertices() {
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

        let expected = 5;
        let actual = input.get_number_of_vertices();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_area() {
        let input = Mesh::new(
           vec![
               // Base
               -2.0,1.0,0.0, // first vertex
               8.0,1.0,0.0, // second vertex
               8.0,11.0,0.0, // third vertex
               -2.0,11.0,0.0, // fourth vertex

               // Top
               3.0,6.0,4.0 // fifth vertex
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

        let expected = 228.062485;
        let actual = input.get_area();
        assert_eq!(((expected - actual).abs() < 0.00001), true);
    }

    #[test]
    fn test_get_bounding_box() {
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
        let expected = BoundingBox::new(-2.0, 8.0, 1.0, 11.0, 0.0, 4.0);
        let actual = input.get_bounding_box();

        assert_eq!(expected.eq(&actual), true);
    }
    
    #[test]
    fn test_get_local_coordinate_system_for_first_face() {
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
        
        let actual = input.get_local_coordinate_system_for_first_face();
        
        let expected_origin = Point::new(4.666666666666667, 4.333333333333333, 0.0 );
        let expected_x = Vector::new(1.0, 0.0, 0.0);
        let expected_y = Vector::new(6.123031769111886e-17, 1.0, 0.0);
        
        let expected = LocalCoordinateSystem::new(expected_origin, expected_x, expected_y);
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_get_ids_of_faces_flipped_inside_using_offset() {
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
            0,1,2, // flipped (0)
            3,2,0,
        
            // Side faces flipped
            0,1,4,
            4,2,1, // flipped (3)
            2,3,4,
            4,0,3 // flipped (5)
        ]);
        let actual = input.get_ids_of_faces_flipped_inside_using_offset(0.001);
        let expected = HashSet::from([0, 3, 5]);
        
        for act in &actual {
            println!("{0}", act);
        }
        
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_different_coordinates() {
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
        let expected_x = vec![-2.0, 8.0, 8.0, -2.0, 3.0];
        let actual_x = input.get_x_coordinates();
        assert_eq!(expected_x, actual_x);

        let expected_y = vec![1.0, 1.0, 11.0, 11.0, 6.0];
        let actual_y = input.get_y_coordinates();
        assert_eq!(expected_y, actual_y);

        let expected_z = vec![0.0, 0.0, 0.0, 0.0, 4.0];
        let actual_z = input.get_z_coordinates();
        assert_eq!(expected_z, actual_z);
    }

    #[test]
    fn test_get_different_indices() {
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
        let expected_start = vec![0, 0, 0, 1, 2, 3];
        let actual_start = input.get_start_indices();
        assert_eq!(expected_start, actual_start);

        let expected_middle = vec![1, 2, 1, 2, 3, 0];
        let actual_middle = input.get_middle_indices();
        assert_eq!(expected_middle, actual_middle);

        let expected_end = vec![2, 3, 4, 4, 4, 4];
        let actual_end = input.get_end_indices();
        assert_eq!(expected_end, actual_end);
    }

    #[test]
    fn test_get_face_normal_vectors_unitized() {
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
            0,1,2, //0
            0,2,3, //1

            // Side faces
            0,1,4, //2
            1,2,4, //3
            2,3,4, //4
            3,0,4  //5
        ]);


        let expected = vec![
        // Base faces normals
        Vector::new(0.0,0.0,1.0), //0
        Vector::new(0.0,0.0,1.0), //1

        // Side faces normals
        Vector::new(0.0,-0.624695,0.780869), //2
        Vector::new(0.624695,0.0,0.780869),  //3
        Vector::new(0.0,0.624695,0.780869),  //4
        Vector::new(-0.624695,0.0,0.780869), //5
        ];


        let actual = input.get_face_normal_vectors_unitized();

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq_with_tolerance(&actual[i], 0.0001), true);
        }
    }

    #[test]
    fn test_get_edges_with_missing_neighbour() {
        let input = Mesh::new(
            vec![0.0, 0.0, 0.0,
                 2.5, 5.0, 0.0,
                 5.0, 0.0, 0.0,
                 7.5, 5.0, 0.0,
                 10.0, 0.0, 0.0,
                 5.0, 10.0, 0.0,
                 ],
            vec![0, 2, 1, // first face
                 1, 2, 3, // second face
                 2, 4, 3, // third face
                 1, 3, 5, // fourth face
                 ]
        );
        
        let mut actual = input.get_edges_with_missing_neighbour();
        let mut expected = vec![
            Edge::new(0, 2), // first face, first edge
            Edge::new(1, 0), // first face, third edge
            Edge::new(2, 4), // third face, first edge
            Edge::new(4, 3), // third face, second edge
            Edge::new(3, 5), // fourth face, second edge
            Edge::new(5, 1), // fourth face, third edge
        ];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
        
    }

    #[test]
    fn test_get_edges_with_missing_neighbour_manifold() {
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
            0,1,2, //0
            0,2,3, //1
        
            // Side faces
            0,1,4, //2
            1,2,4, //3
            2,3,4, //4
            3,0,4  //5
        ]);
        
        let actual = input.get_edges_with_missing_neighbour();
        assert_eq!(actual.len(), 0);
    }
    
    #[test]
    fn test_get_non_manifold_edges(){
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
        
        let mut actual = input.get_non_manifold_edges();
        let mut expected = vec![
            Edge::new(0, 2), // first face, first edge, 1 neighbour
            Edge::new(1, 0), // first face, third edge, 1 neighbour
            Edge::new(3, 1), // second face, third edge, 3 neighbours
            Edge::new(2, 4), // third face, first edge, 1 neighbour
            Edge::new(4, 3), // third face, second edge, 1 neighbour
            Edge::new(3, 5), // fourth face, second edge, 1 neighbour
            Edge::new(5, 1), // fourth face, third edge, 1 neighbour
            Edge::new(3, 6), // fifth face, second edge, 1 neighbour
            Edge::new(6, 1), // fifth face, third edge, 1 neighbour
        ];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
        
    }

    #[test]
    fn test_get_non_manifold_edges_manifold(){
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
            0,1,2, //0
            0,2,3, //1
        
            // Side faces
            0,1,4, //2
            1,2,4, //3
            2,3,4, //4
            3,0,4  //5
        ]);
        
        let actual = input.get_non_manifold_edges();
        assert_eq!(actual.len(), 0);
        
    }

    #[test]
    fn test_partialeq_true() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0002,
                               10.0, 0.0001, 0.0,
                               10.0, -15.0001, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), true);
    }

    #[test]
    fn test_partialeq_with_id_true() {
        let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0002,
                               10.0, 0.0001, 0.0,
                               10.0, -15.0001, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), true);
    }

    #[test]
    fn test_partialeq_with_id_and_without_true() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0002,
                                                10.0, 0.0001, 0.0,
                                                10.0, -15.0001, 0.0],
                                  vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0,
                                                10.0, 0.0, 0.0,
                                                10.0, -15.0, 0.0],
                                  vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_partialeq_with_id_false() {
        let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0002,
                                                10.0, 0.0001, 0.0,
                                                10.0, -15.0001, 0.0],
                                  vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(6), vec![0.0, 0.0, 0.0,
                                                10.0, 0.0, 0.0,
                                                10.0, -15.0, 0.0],
                                  vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_partialeq_with_id_without_id_check_true() {
        let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0002,
                                                10.0, 0.0001, 0.0,
                                                10.0, -15.0001, 0.0],
                                  vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(6), vec![0.0, 0.0, 0.0,
                                                10.0, 0.0, 0.0,
                                                10.0, -15.0, 0.0],
                                  vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance_without_id(&b,0.0002), true);
        assert_eq!(b.eq_with_tolerance_without_id(&a, 0.0002), true);
    }

    #[test]
    fn test_partialeq_coordinates_count_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0,
                               5.0, 1.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_partialeq_different_coordinates_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 2.0, 0.0,
                               10.0, -15.0003, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_partialeq_different_coordinates_without_id_check_false() {
        let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0,
                               10.0, 2.0, 0.0,
                               10.0, -15.0003, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(6), vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_partialeq_indices_count_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2, 2, 1, 0]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_partialeq_different_indices_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 2, 1]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }
}