use crate::vector::Vector;

impl Vector {
    /// Reverses [Vector]
    ///
    /// E.g. Vector (1.0, 2.0, 3.0) will be turned into (-1.0, -2.0, -3.0).
    ///
    /// It mutates given Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// let mut vector = Vector::new(5.231, -0.341, 9.093);
    /// vector.reverse();
    /// assert_eq!(vector.x, -5.231);
    /// assert_eq!(vector.y, 0.341);
    /// assert_eq!(vector.z, -9.093);
    /// ```
    pub fn reverse(&mut self) -> () {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    /// Returns a new reversed [Vector]
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// let vector = Vector::new(5.231, -0.341, 9.093);
    /// let result = vector.get_reversed();
    /// assert_eq!(result.x, -5.231);
    /// assert_eq!(result.y, 0.341);
    /// assert_eq!(result.z, -9.093);
    /// ```
    pub fn get_reversed(&self) -> Vector {
        Vector {x: - self.x, y: - self.y, z: - self.z}
    }

    /// Returns the unitized [Vector]
    ///
    /// Unitized meaning the length will be 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// let vector = Vector::new(5.231, -0.341, 11.034);
    /// let result = vector.get_unitized();
    /// assert_eq!(result.x, 5.231 / 12.215923951957134);
    /// assert_eq!(result.y, -0.341 / 12.215923951957134);
    /// assert_eq!(result.z, 11.034 / 12.215923951957134);
    /// ```
    pub fn get_unitized(&self) -> Vector {
        let length = self.get_length();

        if length == 0.0 {
            panic!("Cannot get unitized Vector if its length is 0.0")
        }

        Vector::new(self.x / length, self.y / length, self.z / length)
    }
}

#[test]
fn test_reverse_vector() {
    let mut vector = Vector::new(5.231, -0.341, 9.093);
    vector.reverse();
    assert_eq!(vector.x, -5.231);
    assert_eq!(vector.y, 0.341);
    assert_eq!(vector.z, -9.093);
}

#[test]
fn test_get_reversed_vector() {
    let vector = Vector::new(5.231, -0.341, 9.093);
    let result = vector.get_reversed();
    assert_eq!(result.x, -5.231);
    assert_eq!(result.y, 0.341);
    assert_eq!(result.z, -9.093);
}

#[test]
fn test_get_unitized_vector() {
    let vector = Vector::new(5.231, -0.341, 11.034);
    let result = vector.get_unitized();
    assert_eq!(result.x, 5.231 / 12.215923951957134);
    assert_eq!(result.y, -0.341 / 12.215923951957134);
    assert_eq!(result.z, 11.034 / 12.215923951957134);
}

#[test]
#[should_panic(expected = "Cannot get unitized Vector if its length is 0.0")]
fn test_get_unitized_vector_0_length_panic() {
    let vector = Vector::zero();
    let _result = vector.get_unitized();
}