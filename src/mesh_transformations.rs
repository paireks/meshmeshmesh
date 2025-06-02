use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::mesh::Mesh;
use crate::point::Point;

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
        
        Mesh::new(transformed_coordinates, self.indices.clone())
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

        Mesh::new(transformed_coordinates, self.indices.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use super::*;

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
}