use crate::rotation::Rotation;

impl Rotation {
    /// Compares given [Rotation] to other one, but with a `f64` tolerance.
    ///
    /// If any value absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::rotation::Rotation;
    /// 
    /// let tolerance: f64 = 0.001;
    /// let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
    /// let b = Rotation::new(1.5, -2.3+0.001, 3.9, 5.5);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the qy absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::rotation::Rotation;
    /// 
    /// let tolerance: f64 = 0.001;
    /// let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
    /// let b = Rotation::new(1.5, -2.3-0.0011, 3.9, 5.5);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Rotation, tolerance: f64) -> bool {
        if (self.qx - other.qx).abs() > tolerance {
            false
        }
        else if (self.qy - other.qy).abs() > tolerance {
            false
        }
        else if (self.qz - other.qz).abs() > tolerance {
            false
        }
        else if (self.qw - other.qw).abs() > tolerance {
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
    fn test_eq_with_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.3+0.001, 3.9, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_with_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5-0.0011, -2.3, 3.9, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.3-0.0011, 3.9, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_z_false(){
        let tolerance: f64 = 0.001;
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.3, 3.9+0.0011, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_w_false(){
        let tolerance: f64 = 0.001;
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.3, 3.9, 5.5+0.0011);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_xyzw_false(){
        let tolerance: f64 = 0.001;
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5-0.0011, -2.3+0.0011, 3.9-0.0011, 5.5+0.0011);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }
}