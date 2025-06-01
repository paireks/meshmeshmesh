use std::ops;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::point::Point;
use crate::vector::Vector;

impl ops::Add<Vector> for Point {
    type Output = Point;

    /// Adds [Vector] to the [Point].
    ///
    /// The result is a new Point.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Point::new(5.231, -0.341, 11.034);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a + b;
    /// let expected = Point::new(5.231+(-12.564), -0.341+5.642, 11.034+7.731);
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn add(self, vector: Vector) -> Point {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    /// Subtracts [Vector] from the [Point].
    ///
    /// The result is a new Point.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Point::new(5.231, -0.341, 11.034);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a - b;
    /// let expected = Point::new(5.231-(-12.564), -0.341-5.642, 11.034-7.731);
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn sub(self, vector: Vector) -> Point {
        Point {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }
}

impl Point {
    /// Creates a new [Point], but with coordinates in the given [LocalCoordinateSystem].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Point::new(44.545922, 27.392431, 12.289269);
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = Point::new(-11.949745, 8.895507, 40.020804);
    /// 
    /// let actual = input.get_in_local_coordinate_system(&local_coordinate_system);
    /// 
    /// assert!(expected.eq_with_tolerance(&actual, 0.001));
    /// 
    /// ```
    pub fn get_in_local_coordinate_system(&self, local_coordinate_system: &LocalCoordinateSystem) -> Point {
        local_coordinate_system.origin + local_coordinate_system.x * self.x + local_coordinate_system.y * self.y + local_coordinate_system.get_z() * self.z
    }

    /// Creates a new `vec` of [Point]s, but with coordinates in the given [LocalCoordinateSystem].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = vec![
    ///     Point::new(28.305465, 44.53041, 14.292343),
    ///     Point::new(37.662834, -14.359948, -15.339571),
    ///     Point::new(-30.621039, 23.839233, 0.0),
    /// ];
    /// 
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = vec![
    ///     Point::new(-27.389497, 1.526485, 56.415518),
    ///     Point::new(-22.137294, 9.566763, -9.474188),
    ///     Point::new(-72.857472, -36.20055, 31.635226),
    /// ];
    ///
    /// let actual = Point::get_all_in_local_coordinate_system(&input, &local_coordinate_system);
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert!(expected[i].eq_with_tolerance(&actual[i], 0.001));
    /// }
    /// ```
    pub fn get_all_in_local_coordinate_system(points: &Vec<Point>, local_coordinate_system: &LocalCoordinateSystem) -> Vec<Point> {
        let mut translated: Vec<Point> = Vec::with_capacity(points.len());

        for point in points {
            translated.push(point.get_in_local_coordinate_system(local_coordinate_system))
        }

        translated
    }

    /// Creates a new [Point], but with coordinates in the Global Coordinate System.
    ///
    /// Global Coordinate System is cartesian with the origin in 0.0,0.0,0.0, with Z axis
    /// defined by the right hand thumb rule.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Point::new(-11.949745, 8.895507, 40.020804);
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = Point::new(44.545922, 27.392431, 12.289269);
    ///
    /// let actual = input.get_in_global_coordinate_system(&local_coordinate_system);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001));
    ///
    /// ```
    pub fn get_in_global_coordinate_system(&self, local_coordinate_system: &LocalCoordinateSystem) -> Point {
        Point::new(
            local_coordinate_system.get_x_ray().get_distance_from_origin_to_closest_point(self),
            local_coordinate_system.get_y_ray().get_distance_from_origin_to_closest_point(self),
            local_coordinate_system.get_z_ray().get_distance_from_origin_to_closest_point(self),
        )
    }

    /// Creates a new `vec` of [Point]s, but with coordinates in the Global Coordinate System.
    ///
    /// Global Coordinate System is cartesian with the origin in 0.0,0.0,0.0, with Z axis
    /// defined by the right hand thumb rule.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = vec![
    ///     Point::new(-27.389497, 1.526485, 56.415518),
    ///     Point::new(-22.137294, 9.566763, -9.474188),
    ///     Point::new(-72.857472, -36.20055, 31.635226),
    /// ];
    ///
    /// let local_coordinate_system = LocalCoordinateSystem::new(
    ///     Point::new(-43.836955, -22.211852, 10.0),
    ///     Vector::new(0.721276,0.692648,0.0),
    ///     Vector::new(-0.290878,0.3029,0.907547)
    /// );
    ///
    /// let expected = vec![
    ///     Point::new(28.305465, 44.53041, 14.292343),
    ///     Point::new(37.662834, -14.359948, -15.339571),
    ///     Point::new(-30.621039, 23.839233, 0.0),
    /// ];
    ///
    /// let actual = Point::get_all_in_global_coordinate_system(&input, &local_coordinate_system);
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert!(expected[i].eq_with_tolerance(&actual[i], 0.001));
    /// }
    /// ```
    pub fn get_all_in_global_coordinate_system(points: &Vec<Point>, local_coordinate_system: &LocalCoordinateSystem) -> Vec<Point> {
        let mut translated: Vec<Point> = Vec::with_capacity(points.len());
        let x_ray = local_coordinate_system.get_x_ray();
        let y_ray = local_coordinate_system.get_y_ray();
        let z_ray = local_coordinate_system.get_z_ray();
        
        for point in points {
            translated.push(
                Point::new(
                    x_ray.get_distance_from_origin_to_closest_point(point),
                    y_ray.get_distance_from_origin_to_closest_point(point),
                    z_ray.get_distance_from_origin_to_closest_point(point),
                )
            )
        }

        translated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add_vector() {
        let a = Point::new(5.231, -0.341, 11.034);
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a + b;
        let expected = Point::new(5.231+(-12.564), -0.341+5.642, 11.034+7.731);
        assert_eq!(result.eq(&expected), true);
    }

    #[test]
    fn test_point_subtract_vector() {
        let a = Point::new(5.231, -0.341, 11.034);
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a - b;
        let expected = Point::new(5.231-(-12.564), -0.341-5.642, 11.034-7.731);
        assert_eq!(result.eq(&expected), true);
    }
    
    #[test]
    fn test_get_in_local_coordinate_system() {
        let input = Point::new(44.545922, 27.392431, 12.289269);
        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );
        
        let expected = Point::new(-11.949745, 8.895507, 40.020804);
        
        let actual = input.get_in_local_coordinate_system(&local_coordinate_system);
        
        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }
    
    #[test]
    fn test_get_all_in_local_coordinate_system() {
        let input = vec![
            Point::new(28.305465, 44.53041, 14.292343),
            Point::new(37.662834, -14.359948, -15.339571),
            Point::new(-30.621039, 23.839233, 0.0),
        ];
        
        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );
        
        let expected = vec![
            Point::new(-27.389497, 1.526485, 56.415518),
            Point::new(-22.137294, 9.566763, -9.474188),
            Point::new(-72.857472, -36.20055, 31.635226),
        ];
        
        let actual = Point::get_all_in_local_coordinate_system(&input, &local_coordinate_system);
        
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].eq_with_tolerance(&actual[i], 0.001));
        }
    }

    #[test]
    fn test_get_in_global_coordinate_system() {
        let input = Point::new(-11.949745, 8.895507, 40.020804);
        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );

        let expected = Point::new(44.545922, 27.392431, 12.289269);

        let actual = input.get_in_global_coordinate_system(&local_coordinate_system);

        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }

    #[test]
    fn test_get_all_in_global_coordinate_system() {
        let input = vec![
            Point::new(-27.389497, 1.526485, 56.415518),
            Point::new(-22.137294, 9.566763, -9.474188),
            Point::new(-72.857472, -36.20055, 31.635226),
        ];

        let local_coordinate_system = LocalCoordinateSystem::new(
            Point::new(-43.836955, -22.211852, 10.0),
            Vector::new(0.721276,0.692648,0.0),
            Vector::new(-0.290878,0.3029,0.907547)
        );

        let expected = vec![
            Point::new(28.305465, 44.53041, 14.292343),
            Point::new(37.662834, -14.359948, -15.339571),
            Point::new(-30.621039, 23.839233, 0.0),
        ];

        let actual = Point::get_all_in_global_coordinate_system(&input, &local_coordinate_system);

        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert!(expected[i].eq_with_tolerance(&actual[i], 0.001));
        }
    }
}