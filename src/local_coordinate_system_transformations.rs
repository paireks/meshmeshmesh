use std::ops;
use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::quaternion::Quaternion;
use crate::vector::Vector;

impl ops::Add<Vector> for LocalCoordinateSystem {
    type Output = LocalCoordinateSystem;

    /// Adds [Vector] to the [LocalCoordinateSystem].
    ///
    /// The result is a new LocalCoordinateSystem.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let origin = Point::new(0.0, 5.0, -1.2);
    /// let x = Vector::new(0.0, 0.0, 1.0);
    /// let y = Vector::new(0.0, -1.0, 0.0);
    ///
    /// let input = LocalCoordinateSystem::new(origin, x, y);
    /// let vector = Vector::new(-12.564, 5.642, 7.731);
    /// 
    /// let actual = input + vector;
    /// let expected = LocalCoordinateSystem::new(Point::new(0.0-12.564, 5.0+5.642, -1.2+7.731), x, y);
    ///
    /// assert_eq!(actual.eq(&expected), true);
    /// ```
    fn add(self, vector: Vector) -> LocalCoordinateSystem {
        LocalCoordinateSystem::new(self.origin + vector, self.x, self.y)
    }
}

impl ops::Sub<Vector> for LocalCoordinateSystem {
    type Output = LocalCoordinateSystem;

    /// Subtracts [Vector] from the [LocalCoordinateSystem].
    ///
    /// The result is a new LocalCoordinateSystem.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let origin = Point::new(0.0, 5.0, -1.2);
    /// let x = Vector::new(0.0, 0.0, 1.0);
    /// let y = Vector::new(0.0, -1.0, 0.0);
    ///
    /// let input = LocalCoordinateSystem::new(origin, x, y);
    /// let vector = Vector::new(-12.564, 5.642, 7.731);
    ///
    /// let actual = input - vector;
    /// let expected = LocalCoordinateSystem::new(Point::new(0.0 - (-12.564), 5.0-5.642, -1.2-7.731), x, y);
    ///
    /// assert_eq!(actual.eq(&expected), true);
    /// ```
    fn sub(self, vector: Vector) -> LocalCoordinateSystem {
        LocalCoordinateSystem::new(self.origin - vector, self.x, self.y)
    }
}

impl LocalCoordinateSystem {

    /// Returns the rotated [LocalCoordinateSystem] using given [Quaternion].
    /// 
    /// It rotates this system around its own origin.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::quaternion::Quaternion;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let origin = Point::new(3.2, 5.0, -1.2);
    /// let x = Vector::new(1.0, 0.0, 0.0);
    /// let y = Vector::new(0.0, 1.0, 0.0);
    /// let input = LocalCoordinateSystem::new(origin, x, y);
    /// let quaternion = Quaternion::new(0.54418103763310099, -0.16946900809404691, -0.24431282685273129, 0.78451162911164585);
    ///
    /// let actual = input.get_rotated_by_quaternion_around_its_origin(quaternion);
    /// let expected = LocalCoordinateSystem::new(origin, Vector::new(0.823183,-0.567776,6.325037e-9), Vector::new(0.198889,0.288356,0.93664));
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001));
    ///
    /// ```
    pub fn get_rotated_by_quaternion_around_its_origin(&self, quaternion: Quaternion) -> LocalCoordinateSystem {
        let rotated_x = self.x.get_rotated_by_quaternion(quaternion);
        let rotated_y = self.y.get_rotated_by_quaternion(quaternion);

        LocalCoordinateSystem::new(self.origin, rotated_x, rotated_y)
    }
    
    /// Creates the new [LocalCoordinateSystem] but flipped, meaning the z-axis and x-axis are
    /// reversed, but the y_axis stays the same.
    /// 
    /// It can be useful if you'd like to e.g. reverse the z-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    /// 
    /// let origin = Point::new(0.0, 5.0, -1.2);
    /// let x = Vector::new(0.0, 0.0, 1.0);
    /// let y = Vector::new(0.0, -1.0, 0.0);
    ///
    /// let input = LocalCoordinateSystem::new(origin, x, y);
    /// let actual = input.get_flipped_around_y_axis();
    ///
    /// assert_eq!(actual.origin, Point::new(0.0, 5.0, -1.2));
    /// assert_eq!(actual.x, Vector::new(0.0, 0.0, -1.0));
    /// assert_eq!(actual.y, Vector::new(0.0, -1.0, 0.0));
    /// ```
    pub fn get_flipped_around_y_axis(&self) -> LocalCoordinateSystem {
        LocalCoordinateSystem::new(self.origin, self.x.get_reversed(), self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vector::Vector;
    use super::*;
    
    #[test]
    fn test_add_vector() {
        let origin = Point::new(0.0, 5.0, -1.2);
        let x = Vector::new(0.0, 0.0, 1.0);
        let y = Vector::new(0.0, -1.0, 0.0);
        
        let input = LocalCoordinateSystem::new(origin, x, y);
        let vector = Vector::new(-12.564, 5.642, 7.731);
        
        let actual = input + vector;
        let expected = LocalCoordinateSystem::new(Point::new(0.0 + (-12.564), 5.0+5.642, -1.2+7.731), x, y);
        
        assert_eq!(actual.eq(&expected), true);
    }

    #[test]
    fn test_subtract_vector() {
        let origin = Point::new(0.0, 5.0, -1.2);
        let x = Vector::new(0.0, 0.0, 1.0);
        let y = Vector::new(0.0, -1.0, 0.0);
        
        let input = LocalCoordinateSystem::new(origin, x, y);
        let vector = Vector::new(-12.564, 5.642, 7.731);
        
        let actual = input - vector;
        let expected = LocalCoordinateSystem::new(Point::new(0.0 - (-12.564), 5.0-5.642, -1.2-7.731), x, y);
        
        assert_eq!(actual.eq(&expected), true);
    }

    #[test]
    fn test_get_flipped_around_y_axis() {
        let origin = Point::new(0.0, 5.0, -1.2);
        let x = Vector::new(0.0, 0.0, 1.0);
        let y = Vector::new(0.0, -1.0, 0.0);
        
        let input = LocalCoordinateSystem::new(origin, x, y);
        let actual = input.get_flipped_around_y_axis();
        
        assert_eq!(actual.origin, Point::new(0.0, 5.0, -1.2));
        assert_eq!(actual.x, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(actual.y, Vector::new(0.0, -1.0, 0.0));
    }
    
    #[test]
    fn test_get_rotated_by_quaternion_around_its_origin() {
        let origin = Point::new(3.2, 5.0, -1.2);
        let x = Vector::new(1.0, 0.0, 0.0);
        let y = Vector::new(0.0, 1.0, 0.0);
        let input = LocalCoordinateSystem::new(origin, x, y);
        let quaternion = Quaternion::new(0.54418103763310099, -0.16946900809404691, -0.24431282685273129, 0.78451162911164585);
        
        let actual = input.get_rotated_by_quaternion_around_its_origin(quaternion);
        let expected = LocalCoordinateSystem::new(origin, Vector::new(0.823183,-0.567776,6.325037e-9), Vector::new(0.198889,0.288356,0.93664));
        
        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }
}