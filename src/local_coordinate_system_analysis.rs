use crate::local_coordinate_system::LocalCoordinateSystem;
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
}