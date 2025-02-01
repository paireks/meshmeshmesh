use crate::ray::Ray;
use crate::vector::Vector;

impl Ray {

    /// Compares given [Ray] to other one, but with a `f64` tolerance.
    ///
    /// If any value absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
    /// let b = Ray::new(Point::new(0.0, 1.0 + 0.001, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the Y-coordinate absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
    /// let b = Ray::new(Point::new(0.0, 1.0 + 0.0011, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Ray, tolerance: f64) -> bool {
        if !self.origin.eq_with_tolerance(&other.origin, tolerance) {
            return false;
        }

        if !self.direction.eq_with_tolerance(&other.direction, tolerance) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    pub fn test_eq_with_tolerance_true() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.001, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    pub fn test_eq_with_tolerance_different_origin_false() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.0011, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    pub fn test_eq_with_tolerance_different_direction_false() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.001, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0011));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    pub fn test_eq_with_tolerance_different_all_false() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.0011, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0011));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }
}