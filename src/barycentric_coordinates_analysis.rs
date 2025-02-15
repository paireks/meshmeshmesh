use crate::barycentric_coordinates::BarycentricCoordinates;

impl BarycentricCoordinates {
    /// Checks if u + v + w = 1.0 with given `tolerance`.
    /// 
    /// # Examples
    /// 
    /// In this example the coordinates are not normalized, so the expected value is `false`.
    /// 
    /// ```
    /// use meshmeshmesh::barycentric_coordinates::BarycentricCoordinates;
    /// let input = BarycentricCoordinates::new(0.5, 0.3, 0.4);
    /// 
    /// let actual = input.are_normalized_within_tolerance(0.01);
    /// 
    /// assert_eq!(actual, false);
    /// ```
    /// 
    /// In second example the coordinates are normalized, so the expected value is `true`.
    ///
    /// ```
    /// use meshmeshmesh::barycentric_coordinates::BarycentricCoordinates;
    /// let input = BarycentricCoordinates::new(0.5, 0.3, 0.2);
    ///
    /// let actual = input.are_normalized_within_tolerance(0.01);
    ///
    /// assert_eq!(actual, true);
    /// ```
    pub fn are_normalized_within_tolerance(&self, tolerance: f64) -> bool {
        
        let sum = self.u + self.v + self.w;
        (sum-1.0).abs() < tolerance
    }

    /// Compares given [BarycentricCoordinates] to other one, but with a `f64` tolerance.
    ///
    /// If any coordinate absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::barycentric_coordinates::BarycentricCoordinates;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = BarycentricCoordinates::new(0.5, 0.2, 0.3);
    /// let b = BarycentricCoordinates::new(0.5 + 0.0005, 0.2 - 0.0005, 0.3 + 0.0009999);
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the v-coordinate absolute difference is > tolerance, so we expect `false`.
    ///
    /// ```
    /// use meshmeshmesh::barycentric_coordinates::BarycentricCoordinates;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = BarycentricCoordinates::new(0.5, 0.2, 0.3);
    /// let b = BarycentricCoordinates::new(0.5 + 0.0005, 0.2 - 0.00101, 0.3 + 0.0009999);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&BarycentricCoordinates, tolerance: f64) -> bool {
        if (self.u - other.u).abs() > tolerance {
            false
        }
        else if (self.v - other.v).abs() > tolerance {
            false
        }
        else if (self.w - other.w).abs() > tolerance {
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
    fn test_are_normalized_within_tolerance_false_too_big(){
        let input = BarycentricCoordinates::new(0.5, 0.3, 0.4);
        
        let actual = input.are_normalized_within_tolerance(0.01);
        
        assert_eq!(actual, false);
    }

    #[test]
    fn test_are_normalized_within_tolerance_false_too_small(){
        let input = BarycentricCoordinates::new(0.49, 0.3, 0.2);

        let actual = input.are_normalized_within_tolerance(0.01);

        assert_eq!(actual, false);
    }

    #[test]
    fn test_are_normalized_within_tolerance_true(){
        let input = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        
        let actual = input.are_normalized_within_tolerance(0.01);
        
        assert_eq!(actual, true);
    }

    #[test]
    fn test_are_normalized_within_tolerance_true_close(){
        let input = BarycentricCoordinates::new(0.5001, 0.299, 0.1999);

        let actual = input.are_normalized_within_tolerance(0.01);

        assert_eq!(actual, true);
    }

    #[test]
    fn test_eq_within_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = BarycentricCoordinates::new(0.5, 0.2, 0.3);
        let b = BarycentricCoordinates::new(0.5 + 0.0005, 0.2 - 0.0005, 0.3 + 0.0009999);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_within_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = BarycentricCoordinates::new(0.5, 0.2, 0.3);
        let b = BarycentricCoordinates::new(0.5 + 0.0011, 0.2 - 0.0005, 0.3 + 0.0009999);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_within_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = BarycentricCoordinates::new(0.5, 0.2, 0.3);
        let b = BarycentricCoordinates::new(0.5 + 0.0005, 0.2 - 0.00101, 0.3 + 0.0009999);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_within_tolerance_different_z_false(){
        let tolerance: f64 = 0.001;
        let a = BarycentricCoordinates::new(0.5, 0.2, 0.3);
        let b = BarycentricCoordinates::new(0.5 + 0.0005, 0.2 - 0.0005, 0.3 + 0.0013);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_within_tolerance_different_xyz_false(){
        let tolerance: f64 = 0.001;
        let a = BarycentricCoordinates::new(0.5, 0.2, 0.3);
        let b = BarycentricCoordinates::new(0.5 + 0.0011, 0.2 - 0.00101, 0.3 + 0.0013);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }
}