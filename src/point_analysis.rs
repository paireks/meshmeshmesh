use crate::point::Point;

impl Point {
    /// Compares given [Point] to other one, but with a `f64` tolerance.
    ///
    /// If any coordinate absolute difference is > tolerance, then it should return `false`.
    ///
    /// As you can see, it doesn't compare distances between [Point]s, but rather coordinates themselves.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Point::new(1.5, -2.3, 3.9);
    /// let b = Point::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.001);
    ///
    /// assert_eq!(a.eq_within_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the Y-coordinate absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Point::new(1.5, -2.3, 3.9);
    /// let b = Point::new(1.5 + 0.0005, -2.3 - 0.00101, 3.9 + 0.001);
    /// assert_eq!(a.eq_within_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_within_tolerance(&self, other:&Point, tolerance: f64) -> bool {
        if (self.x - other.x).abs() > tolerance {
            false
        }
        else if (self.y - other.y).abs() > tolerance {
            false
        }
        else if (self.z - other.z).abs() > tolerance {
            false
        }
        else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_within_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.001);
        assert_eq!(a.eq_within_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_within_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0011, -2.3 - 0.0005, 3.9 + 0.001);
        assert_eq!(a.eq_within_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_within_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0005, -2.3 - 0.00101, 3.9 + 0.001);
        assert_eq!(a.eq_within_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_within_tolerance_different_z_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.0013);
        assert_eq!(a.eq_within_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_within_tolerance_different_xyz_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0011, -2.3 - 0.00101, 3.9 + 0.0013);
        assert_eq!(a.eq_within_tolerance(&b, tolerance), false);
    }
}

