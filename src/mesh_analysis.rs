use crate::bounding_box::BoundingBox;
use crate::mesh::Mesh;
use crate::vector::Vector;

impl Mesh {

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
    ///     assert_eq!(((expected[i].x - actual[i].x).abs() < 0.00001), true);
    ///     assert_eq!(((expected[i].y - actual[i].y).abs() < 0.00001), true);
    ///     assert_eq!(((expected[i].z - actual[i].z).abs() < 0.00001), true);
    /// }
    ///
    /// ```
    pub fn get_face_normal_vectors_unitized(&self) -> Vec<Vector> {
        let triangles = self.to_triangles();

        triangles.iter().map(|triangle| triangle.get_normal_vector_unitized()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert_eq!(((expected[i].x - actual[i].x).abs() < 0.00001), true);
            assert_eq!(((expected[i].y - actual[i].y).abs() < 0.00001), true);
            assert_eq!(((expected[i].z - actual[i].z).abs() < 0.00001), true);
        }
    }
}