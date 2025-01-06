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
    /// let result = Vector::zero();
    /// assert_eq!(result.is_zero_length(), true)
    /// ```
    ///
    /// And here is an example of checking a Vector that is not a zero-length Vector
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// let result = Vector::new(0.541, 4.051, -8.031);
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
    /// let vector = Vector::new(5.231, -0.341, 11.034);
    /// let result = vector.get_length();
    /// assert_eq!(result, 12.215923951957134);
    /// ```
    pub fn get_length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

#[test]
fn test_zero_vector3d_true() {
    let result = Vector::zero();
    assert_eq!(result.is_zero_length(), true)
}

#[test]
fn test_zero_vector3d_false() {
    let result = Vector::new(0.541, 4.051, -8.031);
    assert_eq!(result.is_zero_length(), false)
}

#[test]
fn test_get_length_vector3d() {
    let vector = Vector::new(5.231, -0.341, 11.034);
    let result = vector.get_length();
    assert_eq!(result, 12.215923951957134);
}

#[test]
fn test_get_length_zero_vector3d() {
    let vector = Vector::zero();
    let result = vector.get_length();
    assert_eq!(result, 0.000);
}