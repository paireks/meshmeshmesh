use std::ops;
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

    /// Returns the rotated [Vector] using given axis and rotation angle in radians.
    ///
    /// It uses Rodrigue's rotation formula.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Vector::new(-35.950453, 27.164182, 25.901055);
    /// let rotation_axis = Vector::new(27.845467, 18.872629, 11.197679);
    ///
    /// let actual = input.get_rotated(&rotation_axis, 0.797);
    /// let expected = Vector::new(-22.722373, -4.58531, 46.517321);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001));
    /// ```
    pub fn get_rotated(&self, rotation_axis: &Vector, angle: f64) -> Vector {
        let k = rotation_axis.get_unitized();
        let v = self.clone();

        v * f64::cos(angle) + (k.get_cross_product(&v)) * f64::sin(angle) + k * (k.get_dot_product(&v)) * (1.0 - f64::cos(angle))
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    /// Multiplies a [Vector] with `f64`.
    ///
    /// The result is a new Vector with a multiplied values.
    ///
    /// # Example
    ///
    /// Here is an example of multiplying a Vector with a 2.5 value.
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let vector = Vector::new(5.231, -0.341, 11.034);
    /// let result = vector * 2.5;
    /// let expected = Vector::new(5.231*2.5, -0.341*2.5, 11.034*2.5);
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn mul(self, f: f64) -> Vector {
        Vector {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    /// Adds two [Vector]s to each other.
    ///
    /// The result is a new Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Vector::new(5.231, -0.341, 11.034);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a + b;
    /// let expected = Vector::new(5.231+(-12.564), -0.341+5.642, 11.034+7.731);
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn add(self, vector: Vector) -> Vector {
        Vector {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    /// Subtracts two [Vector]s.
    ///
    /// The result is a new Vector.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Vector::new(5.231, -0.341, 11.034);
    /// let b = Vector::new(-12.564, 5.642, 7.731);
    /// let result = a - b;
    /// let expected = Vector::new(5.231-(-12.564), -0.341-5.642, 11.034-7.731);
    /// assert_eq!(result.eq(&expected), true);
    /// ```
    fn sub(self, vector: Vector) -> Vector {
        Vector {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_vector_multiply_f64() {
        let vector = Vector::new(5.231, -0.341, 11.034);
        let result = vector * 2.5;
        let expected = Vector::new(5.231 * 2.5, -0.341 * 2.5, 11.034 * 2.5);
        assert_eq!(result.eq(&expected), true);
    }

    #[test]
    fn test_vector_add_vector() {
        let a = Vector::new(5.231, -0.341, 11.034);
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a + b;
        let expected = Vector::new(5.231 + (-12.564), -0.341 + 5.642, 11.034 + 7.731);
        assert_eq!(result.eq(&expected), true);
    }

    #[test]
    fn test_vector_subtract_vector() {
        let a = Vector::new(5.231, -0.341, 11.034);
        let b = Vector::new(-12.564, 5.642, 7.731);
        let result = a - b;
        let expected = Vector::new(5.231-(-12.564), -0.341-5.642, 11.034-7.731);
        assert_eq!(result.eq(&expected), true);
    }
    
    #[test]
    fn test_get_rotated() {
        let input = Vector::new(-35.950453, 27.164182, 25.901055);
        let rotation_axis = Vector::new(27.845467, 18.872629, 11.197679);
        
        let actual = input.get_rotated(&rotation_axis, 0.797);
        let expected = Vector::new(-22.722373, -4.58531, 46.517321);
        
        assert!(expected.eq_with_tolerance(&actual, 0.001));
    }
}