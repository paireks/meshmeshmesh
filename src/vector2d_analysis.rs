use crate::vector2d::Vector2D;

impl Vector2D {
    /// Checks if this [Vector2D] is an absolute zero-length
    ///
    /// # Examples
    ///
    /// Here is an example of checking actual zero Vector2D
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let result = Vector2D::zero();
    ///
    /// assert_eq!(result.is_absolute_zero_length(), true)
    /// ```
    ///
    /// And here is an example of checking a Vector2D that is not a zero-length Vector2D
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let result = Vector2D::new(0.541, 4.051);
    ///
    /// assert_eq!(result.is_absolute_zero_length(), false)
    /// ```
    pub fn is_absolute_zero_length(&self) -> bool {
        self.get_length() == 0.0
    }

    /// Compares given [Vector2D] to other one, but with a `f64` tolerance.
    ///
    /// If any value absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Vector2D::new(1.5, -2.3);
    /// let b = Vector2D::new(1.5 + 0.0005, -2.3 - 0.0005);
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the Y-coordinate absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Vector2D::new(1.5, -2.3);
    /// let b = Vector2D::new(1.5 + 0.0005, -2.3 - 0.00101);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Vector2D, tolerance: f64) -> bool {
        if (self.x - other.x).abs() > tolerance {
            false
        }
        else if (self.y - other.y).abs() > tolerance {
            false
        }
        else {
            true
        }
    }

    /// Returns length of a [Vector2D]
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let vector = Vector2D::new(5.231, -0.341);
    /// let result = vector.get_length();
    ///
    /// assert_eq!(result, 5.242102822341431);
    /// ```
    pub fn get_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Calculates an angle between [Vector2D]s.
    ///
    /// Self [Vector2D] is the first one (a), and another one is the second one (b).
    ///
    /// This angle should be in range 0.0 <-> Pi, so it always returns smallest angle.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let first_vector = Vector2D::new(3.0, -3.0);
    /// let second_vector = Vector2D::new(4.0, 9.0);
    ///
    /// let actual = first_vector.get_angle(&second_vector);
    ///
    /// assert!((actual - 1.937970160613120).abs() < 0.00001);
    /// ```
    pub fn get_angle(&self, second_vector: &Vector2D) -> f64 {
        f64::acos((self.get_dot_product(second_vector) / (self.get_length() * second_vector.get_length())).clamp(-1.0, 1.0))
    }

    /// Gets the signed angle in radians between the given [Vector2D] & X-axis of Global Coordinate System.
    ///
    /// This angle should be in range -Pi <-> Pi.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let input = Vector2D::new(-6.028681, -14.381407);
    ///
    /// let actual = input.get_signed_angle_to_x();
    ///
    /// assert!((actual - (-1.967744)).abs() < 0.00001);
    /// ```
    pub fn get_signed_angle_to_x(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    /// Calculates a dot product.
    ///
    /// Self [Vector2D] is the first one (a), and another one is the second one (b).
    ///
    /// result = a ⋅ b
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let first_vector = Vector2D::new(1.0, 2.0);
    /// let second_vector = Vector2D::new(4.0, -5.0);
    ///
    /// let actual = first_vector.get_dot_product(&second_vector);
    ///
    /// assert_eq!(actual, -6.0);
    /// ```
    pub fn get_dot_product(&self, second_vector: &Vector2D) -> f64 {
        self.x * second_vector.x + self.y * second_vector.y
    }
}


#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use super::*;
    #[test]
    fn test_is_absolute_zero_length_true() {
        let result = Vector2D::zero();
        assert_eq!(result.is_absolute_zero_length(), true)
    }

    #[test]
    fn test_is_absolute_zero_length_false() {
        let result = Vector2D::new(0.541, 4.051);
        assert_eq!(result.is_absolute_zero_length(), false)
    }

    #[test]
    fn test_eq_with_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(1.5 + 0.0005, -2.3 - 0.0005);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_with_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(1.5 + 0.0011, -2.3 - 0.0005);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(1.5 + 0.0005, -2.3 - 0.00101);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_xy_false(){
        let tolerance: f64 = 0.001;
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(1.5 + 0.0011, -2.3 - 0.00101);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_get_length_vector() {
        let vector = Vector2D::new(5.231, -0.341);
        let result = vector.get_length();
        assert_eq!(result, 5.242102822341431);
    }

    #[test]
    fn test_get_length_zero_vector() {
        let vector = Vector2D::zero();
        let result = vector.get_length();
        assert_eq!(result, 0.000);
    }

    #[test]
    fn test_get_angle() {
        let first_vector = Vector2D::new(3.0, -3.0);
        let second_vector = Vector2D::new(4.0, 9.0);

        let actual = first_vector.get_angle(&second_vector);

        assert!((actual - 1.937970160613120).abs() < 0.00001);
    }

    #[test]
    fn test_get_angle_same() {
        let first_vector = Vector2D::new(3.0, -3.0);
        let second_vector = Vector2D::new(3.0, -3.0);

        let actual = first_vector.get_angle(&second_vector);

        assert!(actual < 0.00001);
    }

    #[test]
    fn test_get_angle_reversed() {
        let first_vector = Vector2D::new(3.0, -3.0);
        let second_vector = Vector2D::new(-3.0, 3.0);

        let actual = first_vector.get_angle(&second_vector);

        assert!((actual - std::f64::consts::PI).abs() < 0.00001);
    }
    
    #[test]
    fn test_get_signed_angle_to_x() {
        let input = Vector2D::new(-6.028681, -14.381407);
        
        let actual = input.get_signed_angle_to_x();
        
        println!("{0:?}", actual);
        
        assert!((actual - (-1.967744)).abs() < 0.00001);
    }

    #[test]
    fn test_get_signed_angle_to_x_x_axis() {
        let input = Vector2D::new(1.0, 0.0);

        let actual = input.get_signed_angle_to_x();

        println!("{0:?}", actual);

        assert!(actual < 0.00001);
    }

    #[test]
    fn test_get_signed_angle_to_x_x_axis_reversed() {
        let input = Vector2D::new(-1.0, 0.0);

        let actual = input.get_signed_angle_to_x();

        println!("{0:?}", actual);

        assert!((actual - PI).abs() < 0.00001);
    }

    #[test]
    fn test_get_dot_product_vector() {
        let first_vector = Vector2D::new(1.0, 2.0);
        let second_vector = Vector2D::new(4.0, -5.0);

        let actual = first_vector.get_dot_product(&second_vector);

        assert_eq!(actual, -6.0);
    }
}