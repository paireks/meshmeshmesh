use crate::vector::Vector;

impl Vector {
    /// Checks if this [Vector] is an absolute zero-length
    ///
    /// # Examples
    ///
    /// Here is an example of checking actual zero Vector
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Vector::zero();
    ///
    /// assert_eq!(result.is_zero_length(), true)
    /// ```
    ///
    /// And here is an example of checking a Vector that is not a zero-length Vector
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Vector::new(0.541, 4.051, -8.031);
    ///
    /// assert_eq!(result.is_zero_length(), false)
    /// ```
    pub fn is_zero_length(&self) -> bool {
        self.get_length() == 0.0
    }

    /// Compares given [Vector] to other one, but with a `f64` tolerance.
    ///
    /// If any value absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Vector::new(1.5, -2.3, 3.9);
    /// let b = Vector::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.001);
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the Y-coordinate absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Vector::new(1.5, -2.3, 3.9);
    /// let b = Vector::new(1.5 + 0.0005, -2.3 - 0.00101, 3.9 + 0.001);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Vector, tolerance: f64) -> bool {
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

    /// Returns length of a [Vector]
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let vector = Vector::new(5.231, -0.341, 11.034);
    /// let result = vector.get_length();
    ///
    /// assert_eq!(result, 12.215923951957134);
    /// ```
    pub fn get_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculates a cross product.
    ///
    /// Self [Vector] is the first one (a), and another one is the second one (b).
    ///
    /// result = a x b
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let first_vector = Vector::new(3.0, -3.0, 1.0);
    /// let second_vector = Vector::new(4.0, 9.0, 2.0);
    ///
    /// let actual = first_vector.get_cross_product(&second_vector);
    /// let expected = Vector::new(-15.0, -2.0, 39.0);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_cross_product(&self, second_vector: &Vector) -> Vector {
        let x = self.y * second_vector.z - self.z * second_vector.y;
        let y = self.z * second_vector.x - self.x * second_vector.z;
        let z = self.x * second_vector.y - self.y * second_vector.x;

        Vector::new(x, y, z)
    }

    /// Calculates a dot product.
    ///
    /// Self [Vector] is the first one (a), and another one is the second one (b).
    ///
    /// result = a ⋅ b
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let first_vector = Vector::new(1.0, 2.0, 3.0);
    /// let second_vector = Vector::new(4.0, -5.0, 6.0);
    ///
    /// let actual = first_vector.get_dot_product(&second_vector);
    ///
    /// assert_eq!(actual, 12.0);
    /// ```
    pub fn get_dot_product(&self, second_vector: &Vector) -> f64 {
        self.x * second_vector.x + self.y * second_vector.y + self.z * second_vector.z
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero_vector_true() {
        let result = Vector::zero();
        assert_eq!(result.is_zero_length(), true)
    }

    #[test]
    fn test_zero_vector_false() {
        let result = Vector::new(0.541, 4.051, -8.031);
        assert_eq!(result.is_zero_length(), false)
    }

    #[test]
    fn test_eq_with_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.001);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_with_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5 + 0.0011, -2.3 - 0.0005, 3.9 + 0.001);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5 + 0.0005, -2.3 - 0.00101, 3.9 + 0.001);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_z_false(){
        let tolerance: f64 = 0.001;
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.0013);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_xyz_false(){
        let tolerance: f64 = 0.001;
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5 + 0.0011, -2.3 - 0.00101, 3.9 + 0.0013);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_get_length_vector() {
        let vector = Vector::new(5.231, -0.341, 11.034);
        let result = vector.get_length();
        assert_eq!(result, 12.215923951957134);
    }

    #[test]
    fn test_get_length_zero_vector() {
        let vector = Vector::zero();
        let result = vector.get_length();
        assert_eq!(result, 0.000);
    }

    #[test]
    fn test_get_cross_product_vector() {
        let first_vector = Vector::new(3.0, -3.0, 1.0);
        let second_vector = Vector::new(4.0, 9.0, 2.0);

        let actual = first_vector.get_cross_product(&second_vector);
        let expected = Vector::new(-15.0, -2.0, 39.0);

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_get_dot_product_vector() {
        let first_vector = Vector::new(1.0, 2.0, 3.0);
        let second_vector = Vector::new(4.0, -5.0, 6.0);

        let actual = first_vector.get_dot_product(&second_vector);

        assert_eq!(actual, 12.0);
    }
}