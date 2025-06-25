use crate::quaternion::Quaternion;

impl Quaternion {
    /// Checks if it's unit [Quaternion].
    /// 
    /// This check may be useful for many different scenarios, e.g. quaternions which represents
    /// a rotation have to be unit quaternions.
    /// 
    /// # Example
    /// 
    /// This example shows the unit [Quaternion] so `true` should be returned.
    ///
    /// ```
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
    ///
    /// assert!(input.is_unit(0.001));
    ///
    /// ```
    /// 
    /// The example below shows the Quaternion which is not unit, so `false` should be returned.
    ///
    /// ```
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.2925714890680042);
    ///
    /// assert!(!input.is_unit(0.001));
    ///
    /// ```
    pub fn is_unit(&self, tolerance: f64) -> bool {
        (1.0 - self.get_length()).abs() <= tolerance
    }

    /// Compares given [Quaternion] to other one, but with a `f64` tolerance.
    ///
    /// If any value absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
    /// let b = Quaternion::new(1.5, -2.3+0.001, 3.9, 5.5);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the qy absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
    /// let b = Quaternion::new(1.5, -2.3-0.0011, 3.9, 5.5);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Quaternion, tolerance: f64) -> bool {
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

    /// Gets the length (magnitude) of [Quaternion].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
    ///
    /// let actual = input.get_length();
    ///
    /// assert!((1.0 - actual).abs() < 0.00001);
    ///
    /// ```
    pub fn get_length(&self) -> f64 {
        f64::sqrt(self.qx*self.qx + self.qy*self.qy + self.qz*self.qz + self.qw*self.qw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_unit_true() {
        let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
        
        assert!(input.is_unit(0.001));
    }

    #[test]
    fn test_is_unit_false() {
        let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.2925714890680042);
        
        assert!(!input.is_unit(0.001));
    }

    #[test]
    fn test_eq_with_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
        let b = Quaternion::new(1.5, -2.3+0.001, 3.9, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_with_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
        let b = Quaternion::new(1.5-0.0011, -2.3, 3.9, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
        let b = Quaternion::new(1.5, -2.3-0.0011, 3.9, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_z_false(){
        let tolerance: f64 = 0.001;
        let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
        let b = Quaternion::new(1.5, -2.3, 3.9+0.0011, 5.5);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_w_false(){
        let tolerance: f64 = 0.001;
        let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
        let b = Quaternion::new(1.5, -2.3, 3.9, 5.5+0.0011);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_xyzw_false(){
        let tolerance: f64 = 0.001;
        let a = Quaternion::new(1.5, -2.3, 3.9, 5.5);
        let b = Quaternion::new(1.5-0.0011, -2.3+0.0011, 3.9-0.0011, 5.5+0.0011);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_get_length() {
        let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);

        let actual = input.get_length();

        assert!((1.0 - actual).abs() < 0.00001);
    }
}