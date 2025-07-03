use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::quaternion::Quaternion;
use crate::ray::Ray;
use crate::vector::Vector;

impl LocalCoordinateSystem {

    /// Compares given [LocalCoordinateSystem] to other one, but with a `f64` tolerance.
    ///
    /// If any coordinate absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    /// 
    /// let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
    /// let b = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0001, 1.0), Vector::new(0.0, -1.0002, 0.0));
    ///
    /// assert_eq!(a.eq_with_tolerance(&b,0.0002), true);
    /// assert_eq!(b.eq_with_tolerance(&a, 0.0002), true);
    /// ```
    ///
    /// In this example we can see the coordinates absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
    /// let b = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0001, 1.0), Vector::new(0.0, -1.0, 0.1));
    ///
    /// assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
    /// assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    /// ```
    pub fn eq_with_tolerance(&self, other: &LocalCoordinateSystem, tolerance: f64) -> bool {
        if !self.origin.eq_with_tolerance(&other.origin, tolerance) { 
            return false;
        }
        
        if !self.x.eq_with_tolerance(&other.x, tolerance) { 
            return false;
        }

        if !self.y.eq_with_tolerance(&other.y, tolerance) {
            return false;
        }
        
        true
    }
    
    /// Gets z-axis as [Vector].
    /// 
    /// Should be unitized.
    /// 
    /// It is defined by right hand thumb rule.
    pub fn get_z(&self) -> Vector {
        self.x.get_cross_product(&self.y).get_unitized()
    }
    
    /// Gets x-axis as [Ray]
    pub fn get_x_ray(&self) -> Ray {
        Ray::new(self.origin, self.x)
    }

    /// Gets y-axis as [Ray]
    pub fn get_y_ray(&self) -> Ray {
        Ray::new(self.origin, self.y)
    }

    /// Gets z-axis as [Ray]
    pub fn get_z_ray(&self) -> Ray {
        Ray::new(self.origin, self.get_z())
    }

    /// Gets [Vector] which goes from this `self` [LocalCoordinateSystem] to the `other` one.
    ///
    /// Can be useful in cases when you try to move one coordinate system in the same place as
    /// another one.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
    /// let b = LocalCoordinateSystem::new(Point::new(10.0, -4.0, -1.0), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
    ///
    /// let actual = a.get_vector_to(&b);
    /// let expected = Vector::new(10.0 - 0.0, -4.0 - 5.0, -1.0 - (-1.2));
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.00001));
    /// ```
    pub fn get_vector_to(&self, other: &LocalCoordinateSystem) -> Vector {
        Vector::from_2_points(&self.origin, &other.origin)
    }

    /// Gets [Rotation] which aligns from this `self` [LocalCoordinateSystem] to the `other` one.
    ///
    /// Can be useful in cases when you try to rotate one coordinate system in the same way as
    /// another one.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::quaternion::Quaternion;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = LocalCoordinateSystem::new(Point::new(3.2, 5.0, -1.2), Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    /// let b = LocalCoordinateSystem::new(Point::new(5.6, -8.7, 9.6), Vector::new(0.823183,-0.567776,6.325037e-9), Vector::new(0.198889,0.288356,0.93664));
    ///
    /// let actual = a.get_rotation_to(&b);
    ///
    /// let expected = Quaternion::new(0.54418103763310099, -0.16946900809404691, -0.24431282685273129, 0.78451162911164585);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001));
    /// 
    /// ```
    pub fn get_rotation_to(&self, other: &LocalCoordinateSystem) -> Quaternion {
        let first_quaternion = self.x.get_rotation_to(&other.x);
        let self_after_first_rotation = self.get_rotated_by_quaternion_around_its_origin(first_quaternion);
        let second_quaternion = self_after_first_rotation.get_rotation_to_when_x_aligned(&other);
        second_quaternion * first_quaternion
    }

    /// Gets [Rotation] which aligns from this `self` [LocalCoordinateSystem] to the `other` one.
    ///
    /// This one works only once the X-axes are already aligned.
    fn get_rotation_to_when_x_aligned(&self, other: &LocalCoordinateSystem) -> Quaternion {
        let signed_angle = self.y.get_signed_angle(&other.y, &self.x);
        Quaternion::new_from_axis_angle(&self.x, signed_angle)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn test_eq_with_tolerance_true() {
        let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
        let b = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0001, 1.0), Vector::new(0.0, -1.0002, 0.0));
        
        assert_eq!(a.eq_with_tolerance(&b,0.0002), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), true);
    }

    #[test]
    fn test_eq_with_tolerance_origin_false() {
        let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
        let b = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.201), Vector::new(0.0, 0.0001, 1.0), Vector::new(0.0, -1.0002, 0.0));

        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_eq_with_tolerance_x_false() {
        let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
        let b = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.00021, 1.0), Vector::new(0.0, -1.0002, 0.0));

        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_eq_with_tolerance_y_false() {
        let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
        let b = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0001, 1.0), Vector::new(0.0, -1.0, 0.01));

        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_eq_with_tolerance_all_false() {
        let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
        let b = LocalCoordinateSystem::new(Point::new(0.1, 5.0, -1.2), Vector::new(-0.1, 0.0001, 1.0), Vector::new(0.0, -1.0003, 0.0));

        assert_eq!(a.eq_with_tolerance(&b,0.0002), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.0002), false);
    }

    #[test]
    fn test_get_vector_to() {
        let a = LocalCoordinateSystem::new(Point::new(0.0, 5.0, -1.2), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));
        let b = LocalCoordinateSystem::new(Point::new(10.0, -4.0, -1.0), Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, -1.0, 0.0));

        let actual = a.get_vector_to(&b);
        let expected = Vector::new(10.0 - 0.0, -4.0 - 5.0, -1.0 - (-1.2));

        assert!(expected.eq_with_tolerance(&actual, 0.00001));
    }
    
    #[test]
    fn test_get_rotation_to() {
        let a = LocalCoordinateSystem::new(Point::new(3.2, 5.0, -1.2), Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let b = LocalCoordinateSystem::new(Point::new(5.6, -8.7, 9.6), Vector::new(0.823183,-0.567776,6.325037e-9), Vector::new(0.198889,0.288356,0.93664));
        
        let actual = a.get_rotation_to(&b);
        
        let expected = Quaternion::new(0.54418103763310099, -0.16946900809404691, -0.24431282685273129, 0.78451162911164585);
        
        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }
}