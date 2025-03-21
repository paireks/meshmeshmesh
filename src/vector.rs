use crate::point::Point;

/// Represents a three-dimensional vector with double-precision floating-point coordinates.
///
/// # Example
/// ```
/// use meshmeshmesh::vector::Vector;
///
/// let result = Vector::new(1.5, -2.3, 3.9);
/// assert_eq!(result.x, 1.5);
/// assert_eq!(result.y, -2.3);
/// assert_eq!(result.z, 3.9);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    /// The x-dimension of the vector.
    pub x: f64,
    /// The y-dimension of the vector.
    pub y: f64,
    /// The z-dimension of the vector.
    pub z: f64
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Vector {
    /// Creates a new [Vector]
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Vector::new(1.5, -2.3, 3.9);
    /// assert_eq!(result.x, 1.5);
    /// assert_eq!(result.y, -2.3);
    /// assert_eq!(result.z, 3.9);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vector { Vector { x, y, z } }

    /// Returns the X unit [Vector]
    ///
    /// X unit is a (1.0,0.0,0.0) Vector.
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Vector::x_unit();
    /// assert_eq!(result.x, 1.0);
    /// assert_eq!(result.y, 0.0);
    /// assert_eq!(result.z, 0.0);
    /// ```
    pub fn x_unit() -> Vector { Vector { x: 1.0, y: 0.0, z:0.0 } }

    /// Returns the Y unit [Vector]
    ///
    /// Y unit is a (0.0,1.0,0.0) Vector.
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Vector::y_unit();
    /// assert_eq!(result.x, 0.0);
    /// assert_eq!(result.y, 1.0);
    /// assert_eq!(result.z, 0.0);
    /// ```
    pub fn y_unit() -> Vector { Vector { x: 0.0, y: 1.0, z:0.0 } }

    /// Returns the Z unit [Vector]
    ///
    /// Z unit is a (0.0,0.0,1.0) Vector.
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Vector::z_unit();
    /// assert_eq!(result.x, 0.0);
    /// assert_eq!(result.y, 0.0);
    /// assert_eq!(result.z, 1.0);
    /// ```
    pub fn z_unit() -> Vector { Vector { x: 0.0, y: 0.0, z:1.0 } }

    /// Returns 0,0,0 [Vector]
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let result = Vector::zero();
    /// assert_eq!(result.x, 0.0);
    /// assert_eq!(result.y, 0.0);
    /// assert_eq!(result.z, 0.0);
    /// ```
    pub fn zero() -> Vector { Vector { x: 0.0, y: 0.0, z:0.0 } }

    /// Returns a new [Vector] created from 2 [Point]s
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let start = Point::new(0.541, 4.051, -8.031);
    /// let end = Point::new(-3.093, 11.391, 15.0341);
    /// let result = Vector::from_2_points(&start, &end);
    /// assert_eq!(result.x, -3.093-0.541);
    /// assert_eq!(result.y, 11.391-4.051);
    /// assert_eq!(result.z, 15.0341--8.031);
    /// ```
    pub fn from_2_points(start: &Point, end: &Point) -> Vector {
        Vector::new(end.x - start.x, end.y - start.y, end.z - start.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Vector::new(1.5, -2.3, 3.9);
        assert_eq!(result.x, 1.5);
        assert_eq!(result.y, -2.3);
        assert_eq!(result.z, 3.9);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5, -2.3, 3.9);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.49, -2.3, 3.9);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5, -2.24321, 3.9);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(1.5, -2.3, -4.05);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Vector::new(1.5, -2.3, 3.9);
        let b = Vector::new(-1.5, 5.01, 11.0);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_x_unit_vector() {
        let result = Vector::x_unit();
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }
    #[test]
    fn test_y_unit_vector() {
        let result = Vector::y_unit();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 1.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_z_unit_vector() {
        let result = Vector::z_unit();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 1.0);
    }

    #[test]
    fn test_zero_vector() {
        let result = Vector::zero();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_from_2_points() {
        let start = Point::new(0.541, 4.051, -8.031);
        let end = Point::new(-3.093, 11.391, 15.0341);
        let result = Vector::from_2_points(&start, &end);
        assert_eq!(result.x, -3.093-0.541);
        assert_eq!(result.y, 11.391-4.051);
        assert_eq!(result.z, 15.0341--8.031);
    }
}