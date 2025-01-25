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
        let x = self.y * second_vector.x - self.z * second_vector.y;
        let y = self.z * second_vector.x - self.x * second_vector.z;
        let z = self.x * second_vector.y - self.y * second_vector.x;

        Vector::new(x, y, z)
    }
}

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