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
}