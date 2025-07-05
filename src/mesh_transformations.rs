use std::ops;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::mesh::Mesh;
use crate::point::Point;
use crate::quaternion::Quaternion;
use crate::vector::Vector;

impl ops::Add<Vector> for Mesh {
    type Output = Mesh;

    /// Adds [Vector] to the [Mesh].
    ///
    /// The result is a new Mesh.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0,
    ///    ],
    ///    vec![
    ///        // Base faces
    ///        0,1,2, //0
    ///        0,2,3, //1
    ///
    ///       // Side faces
    ///        0,1,4, //2
    ///        1,2,4, //3
    ///        2,3,4, //4
    ///        3,0,4  //5
    ///    ]);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a + b;
    /// let expected = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0-12.564,1.0+5.642,0.0+7.731,
    ///     8.0-12.564,1.0+5.642,0.0+7.731,
    ///     8.0-12.564,11.0+5.642,0.0+7.731,
    ///     -2.0-12.564,11.0+5.642,0.0+7.731,
    ///
    ///     // Top
    ///     3.0-12.564,6.0+5.642,4.0+7.731,
    ///    ],
    ///    vec![
    ///        // Base faces
    ///        0,1,2, //0
    ///        0,2,3, //1
    ///
    ///       // Side faces
    ///        0,1,4, //2
    ///        1,2,4, //3
    ///        2,3,4, //4
    ///        3,0,4  //5
    ///    ]);
    /// 
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn add(self, vector: Vector) -> Mesh {
        let number_of_coordinates = self.coordinates.len();
        let mut moved_coordinates = Vec::with_capacity(number_of_coordinates);

        let mut counter: usize = 0;
        while counter < number_of_coordinates {
            moved_coordinates.push(self.coordinates[counter] + vector.x);
            moved_coordinates.push(self.coordinates[counter+1] + vector.y);
            moved_coordinates.push(self.coordinates[counter+2] + vector.z);
            counter += 3;
        }
        
        Mesh::new_with_id(self.id, moved_coordinates, self.indices.clone())
    }
}

impl ops::Sub<Vector> for Mesh {
    type Output = Mesh;

    /// Subtracts [Vector] from the [Mesh].
    ///
    /// The result is a new Point.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0,
    ///    ],
    ///    vec![
    ///        // Base faces
    ///        0,1,2, //0
    ///        0,2,3, //1
    ///
    ///       // Side faces
    ///        0,1,4, //2
    ///        1,2,4, //3
    ///        2,3,4, //4
    ///        3,0,4  //5
    ///    ]);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a - b;
    /// let expected = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0+12.564,1.0-5.642,0.0-7.731,
    ///     8.0+12.564,1.0-5.642,0.0-7.731,
    ///     8.0+12.564,11.0-5.642,0.0-7.731,
    ///     -2.0+12.564,11.0-5.642,0.0-7.731,
    ///
    ///     // Top
    ///     3.0+12.564,6.0-5.642,4.0-7.731,
    ///    ],
    ///    vec![
    ///        // Base faces
    ///        0,1,2, //0
    ///        0,2,3, //1
    ///
    ///       // Side faces
    ///        0,1,4, //2
    ///        1,2,4, //3
    ///        2,3,4, //4
    ///        3,0,4  //5
    ///    ]);
    ///
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn sub(self, vector: Vector) -> Mesh {
        let number_of_coordinates = self.coordinates.len();
        let mut moved_coordinates = Vec::with_capacity(number_of_coordinates);

        let mut counter: usize = 0;
        while counter < number_of_coordinates {
            moved_coordinates.push(self.coordinates[counter] - vector.x);
            moved_coordinates.push(self.coordinates[counter+1] - vector.y);
            moved_coordinates.push(self.coordinates[counter+2] - vector.z);
            counter += 3;
        }

        Mesh::new_with_id(self.id, moved_coordinates, self.indices.clone())
    }
}

impl Mesh {
    /// Creates a new [Mesh], but with coordinates in the given [LocalCoordinateSystem].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input_points = vec![
    ///     28.305465, 44.53041, 14.292343,
    ///     37.662834, -14.359948, -15.339571,
    ///     -30.621039, 23.839233, 0.0,
    /// ];
    ///
    /// let input = Mesh::new(input_points, vec![0,1,2]);
    ///
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = Mesh::new( vec![
    ///     -27.389505260098144, 1.5264797845139046, 56.41551707505066,
    ///     -22.137283805470638, 9.566759141069753, -9.474188423495871,
    ///     -72.85748048361508, -36.20054627209404, 31.635224628432436,
    /// ], vec![0,1,2]);
    ///
    /// let actual = input.get_in_local_coordinate_system(&local_coordinate_system);
    ///
    /// assert_eq!(expected, actual);
    /// 
    /// ```
    pub fn get_in_local_coordinate_system(&self, local_coordinate_system: &LocalCoordinateSystem) -> Mesh {
        let points = self.to_points();
        let transformed_points = Point::get_all_in_local_coordinate_system(&points, local_coordinate_system);
        let mut transformed_coordinates = Vec::with_capacity(self.coordinates.len());
        for point in transformed_points {
            transformed_coordinates.push(point.x);
            transformed_coordinates.push(point.y);
            transformed_coordinates.push(point.z);
        }
        
        Mesh::new_with_id(self.id, transformed_coordinates, self.indices.clone())
    }

    /// Creates a new [Mesh], but with coordinates in the Global Coordinate System.
    ///
    /// Global Coordinate System is cartesian with the origin in 0.0,0.0,0.0, with Z axis
    /// defined by the right hand thumb rule.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input_points = vec![
    ///     -27.389497, 1.526485, 56.415518,
    ///     -22.137294, 9.566763, -9.474188,
    ///     -72.857472, -36.20055, 31.635226,
    /// ];
    ///
    /// let input = Mesh::new(input_points, vec![0,1,2]);
    ///
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = Mesh::new( vec![
    ///     28.305463832777935, 44.53040319128577, 14.292345166803258,
    ///     37.66283278239673, -14.35995256298574, -15.339579756565103,
    ///     -30.621041211404954, 23.839238031471098, 8.3491367455224e-6
    /// ], vec![0,1,2]);
    ///
    /// let actual = input.get_in_global_coordinate_system(&local_coordinate_system);
    ///
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn get_in_global_coordinate_system(&self, local_coordinate_system: &LocalCoordinateSystem) -> Mesh {
        let points = self.to_points();
        let transformed_points = Point::get_all_in_global_coordinate_system(&points, local_coordinate_system);
        let mut transformed_coordinates = Vec::with_capacity(self.coordinates.len());
        for point in transformed_points {
            transformed_coordinates.push(point.x);
            transformed_coordinates.push(point.y);
            transformed_coordinates.push(point.z);
        }

        Mesh::new_with_id(self.id, transformed_coordinates, self.indices.clone())
    }

    /// Returns the rotated [Mesh] using given [Quaternion].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let input_points = vec![
    ///     -27.389497, 1.526485, 56.415518,
    ///     -22.137294, 9.566763, -9.474188,
    ///     -72.857472, -36.20055, 31.635226,
    /// ];
    ///
    /// let input = Mesh::new(input_points, vec![0,1,2]);
    /// let quaternion = Quaternion::new(0.54418103763310099, -0.16946900809404691, -0.24431282685273129, 0.78451162911164585);
    /// 
    /// let actual = input.get_rotated_by_quaternion(quaternion);
    /// let expected = Mesh::new(vec![
    ///     -52.24483178220444, -27.50654491820647, 21.191812388005097, -11.281932722213432, 22.632509037636527, 5.641853145261749, -83.99858318766411, 6.536496205872773, -22.82522311873513
    /// ], vec![0,1,2]);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001));
    /// ```
    pub fn get_rotated_by_quaternion(&self, quaternion: Quaternion) -> Mesh {
        let mut rotated_coordinates = Vec::with_capacity(self.coordinates.len());
        let vertices = self.to_points();

        for vertex in vertices {
            let rotated_vertex = vertex.get_rotated_by_quaternion(quaternion);
            rotated_coordinates.push(rotated_vertex.x);
            rotated_coordinates.push(rotated_vertex.y);
            rotated_coordinates.push(rotated_vertex.z);
        }
        
        Mesh::new_with_id(self.id, rotated_coordinates, self.indices.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use super::*;
    
    #[test]
    fn test_add_vector() {
        let a = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,
        
            // Top
            3.0,6.0,4.0,
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
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a + b;
        let expected = Mesh::new(
        vec![
            // Base
            -2.0-12.564,1.0+5.642,0.0+7.731,
            8.0-12.564,1.0+5.642,0.0+7.731,
            8.0-12.564,11.0+5.642,0.0+7.731,
            -2.0-12.564,11.0+5.642,0.0+7.731,
        
            // Top
            3.0-12.564,6.0+5.642,4.0+7.731,
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
        
        assert_eq!(result.eq(&expected), true);
    }

    #[test]
    fn test_subtract_vector() {
        let a = Mesh::new(
            vec![
                // Base
                -2.0,1.0,0.0,
                8.0,1.0,0.0,
                8.0,11.0,0.0,
                -2.0,11.0,0.0,

                // Top
                3.0,6.0,4.0,
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
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a - b;
        let expected = Mesh::new(
            vec![
                // Base
                -2.0+12.564,1.0-5.642,0.0-7.731,
                8.0+12.564,1.0-5.642,0.0-7.731,
                8.0+12.564,11.0-5.642,0.0-7.731,
                -2.0+12.564,11.0-5.642,0.0-7.731,

                // Top
                3.0+12.564,6.0-5.642,4.0-7.731,
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

        assert_eq!(result.eq(&expected), true);
    }

    #[test]
    fn test_get_in_local_coordinate_system() {
        let input_points = vec![
            28.305465, 44.53041, 14.292343,
            37.662834, -14.359948, -15.339571,
            -30.621039, 23.839233, 0.0,
        ];
        
        let input = Mesh::new(input_points, vec![0,1,2]);
        
        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );
        
        let expected = Mesh::new( vec![
            -27.389505260098144, 1.5264797845139046, 56.41551707505066,
            -22.137283805470638, 9.566759141069753, -9.474188423495871,
            -72.85748048361508, -36.20054627209404, 31.635224628432436,
        ], vec![0,1,2]);
        
        let actual = input.get_in_local_coordinate_system(&local_coordinate_system);
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_in_global_coordinate_system() {
        let input_points = vec![
            -27.389497, 1.526485, 56.415518,
            -22.137294, 9.566763, -9.474188,
            -72.857472, -36.20055, 31.635226,
        ];
        
        let input = Mesh::new(input_points, vec![0,1,2]);
        
        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );
        
        let expected = Mesh::new( vec![
            28.305463832777935, 44.53040319128577, 14.292345166803258,
            37.66283278239673, -14.35995256298574, -15.339579756565103,
            -30.621041211404954, 23.839238031471098, 8.3491367455224e-6
        ], vec![0,1,2]);
        
        let actual = input.get_in_global_coordinate_system(&local_coordinate_system);
        
        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_get_rotated_by_quaternion() {
        let input_points = vec![
            -27.389497, 1.526485, 56.415518,
            -22.137294, 9.566763, -9.474188,
            -72.857472, -36.20055, 31.635226,
        ];
        
        let input = Mesh::new(input_points, vec![0,1,2]);
        let quaternion = Quaternion::new(0.54418103763310099, -0.16946900809404691, -0.24431282685273129, 0.78451162911164585);
        
        let actual = input.get_rotated_by_quaternion(quaternion);
        let expected = Mesh::new(vec![
            -52.24483178220444, -27.50654491820647, 21.191812388005097, -11.281932722213432, 22.632509037636527, 5.641853145261749, -83.99858318766411, 6.536496205872773, -22.82522311873513
        ], vec![0,1,2]);
        
        println!("{0:?}", actual);
        
        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }
}