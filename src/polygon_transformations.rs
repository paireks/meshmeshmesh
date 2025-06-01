use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::point::Point;
use crate::polygon::Polygon;

impl Polygon {

    /// Creates a new [Polygon], but with coordinates in the given [LocalCoordinateSystem].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input_points = vec![
    ///     Point::new(28.305465, 44.53041, 14.292343),
    ///     Point::new(37.662834, -14.359948, -15.339571),
    ///     Point::new(-30.621039, 23.839233, 0.0),
    /// ];
    ///
    /// let input = Polygon::new(input_points);
    ///
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = Polygon::new( vec![
    ///     Point::new(-27.389497, 1.526485, 56.415518),
    ///     Point::new(-22.137294, 9.566763, -9.474188),
    ///     Point::new(-72.857472, -36.20055, 31.635226),
    /// ]);
    ///
    /// let actual = input.get_in_local_coordinate_system(&local_coordinate_system);
    ///
    /// assert_eq!(expected.vertices.len(), actual.vertices.len());
    /// for i in 0..expected.vertices.len() {
    ///     assert!(expected.vertices[i].eq_with_tolerance(&actual.vertices[i], 0.001));
    /// }
    /// ```
    pub fn get_in_local_coordinate_system(&self, local_coordinate_system: &LocalCoordinateSystem) -> Polygon {
        Polygon::new(Point::get_all_in_local_coordinate_system(&self.vertices, local_coordinate_system))
    }

    /// Creates a new [Polygon], but with coordinates in the Global Coordinate System.
    ///
    /// Global Coordinate System is cartesian with the origin in 0.0,0.0,0.0, with Z axis
    /// defined by the right hand thumb rule.
    /// 
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input_points = vec![
    ///     Point::new(-27.389497, 1.526485, 56.415518),
    ///     Point::new(-22.137294, 9.566763, -9.474188),
    ///     Point::new(-72.857472, -36.20055, 31.635226),
    /// ];
    ///
    /// let input = Polygon::new(input_points);
    ///
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = Polygon::new( vec![
    ///     Point::new(28.305465, 44.53041, 14.292343),
    ///     Point::new(37.662834, -14.359948, -15.339571),
    ///     Point::new(-30.621039, 23.839233, 0.0),
    /// ]);
    ///
    /// let actual = input.get_in_global_coordinate_system(&local_coordinate_system);
    ///
    /// assert_eq!(expected.vertices.len(), actual.vertices.len());
    /// for i in 0..expected.vertices.len() {
    ///     assert!(expected.vertices[i].eq_with_tolerance(&actual.vertices[i], 0.001));
    /// }
    /// ```
    pub fn get_in_global_coordinate_system(&self, local_coordinate_system: &LocalCoordinateSystem) -> Polygon {
        Polygon::new(Point::get_all_in_global_coordinate_system(&self.vertices, local_coordinate_system))
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vector::Vector;
    use super::*;

    #[test]
    fn test_get_in_local_coordinate_system() {
        let input_points = vec![
            Point::new(28.305465, 44.53041, 14.292343),
            Point::new(37.662834, -14.359948, -15.339571),
            Point::new(-30.621039, 23.839233, 0.0),
        ];
        
        let input = Polygon::new(input_points);
        
        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );
        
        let expected = Polygon::new( vec![
            Point::new(-27.389497, 1.526485, 56.415518),
            Point::new(-22.137294, 9.566763, -9.474188),
            Point::new(-72.857472, -36.20055, 31.635226),
        ]);
        
        let actual = input.get_in_local_coordinate_system(&local_coordinate_system);
        
        assert_eq!(expected.vertices.len(), actual.vertices.len());
        for i in 0..expected.vertices.len() {
            assert!(expected.vertices[i].eq_with_tolerance(&actual.vertices[i], 0.001));
        }
    }
    
    #[test]
    fn test_get_in_global_coordinate_system() {
        let input_points = vec![
            Point::new(-27.389497, 1.526485, 56.415518),
            Point::new(-22.137294, 9.566763, -9.474188),
            Point::new(-72.857472, -36.20055, 31.635226),
        ];
        
        let input = Polygon::new(input_points);
        
        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );
        
        let expected = Polygon::new( vec![
            Point::new(28.305465, 44.53041, 14.292343),
            Point::new(37.662834, -14.359948, -15.339571),
            Point::new(-30.621039, 23.839233, 0.0),
        ]);
        
        let actual = input.get_in_global_coordinate_system(&local_coordinate_system);
        
        assert_eq!(expected.vertices.len(), actual.vertices.len());
        for i in 0..expected.vertices.len() {
            assert!(expected.vertices[i].eq_with_tolerance(&actual.vertices[i], 0.001));
        }
    }
}