use crate::point2d::Point2D;

/// Represents a two-dimensional vector with double-precision floating-point coordinates.
///
/// # Example
/// ```
/// use meshmeshmesh::vector2d::Vector2D;
///
/// let result = Vector2D::new(1.5, -2.3);
/// assert_eq!(result.x, 1.5);
/// assert_eq!(result.y, -2.3);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    /// The x-dimension of the vector.
    pub x: f64,
    /// The y-dimension of the vector.
    pub y: f64,
}

impl PartialEq for Vector2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector2D {
    /// Creates a new [Vector2D]
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let result = Vector2D::new(1.5, -2.3);
    /// assert_eq!(result.x, 1.5);
    /// assert_eq!(result.y, -2.3);
    /// ```
    pub fn new(x: f64, y: f64) -> Vector2D { Vector2D { x, y } }

    /// Returns the X unit [Vector2D]
    ///
    /// X unit is a (1.0,0.0) Vector2D.
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let result = Vector2D::x_unit();
    /// assert_eq!(result.x, 1.0);
    /// assert_eq!(result.y, 0.0);
    /// ```
    pub fn x_unit() -> Vector2D { Vector2D { x: 1.0, y: 0.0 } }

    /// Returns the Y unit [Vector2D]
    ///
    /// Y unit is a (0.0,1.0) Vector2D.
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let result = Vector2D::y_unit();
    /// assert_eq!(result.x, 0.0);
    /// assert_eq!(result.y, 1.0);
    /// ```
    pub fn y_unit() -> Vector2D { Vector2D { x: 0.0, y: 1.0 } }

    /// Returns 0,0 [Vector2D]
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let result = Vector2D::zero();
    /// assert_eq!(result.x, 0.0);
    /// assert_eq!(result.y, 0.0);
    /// ```
    pub fn zero() -> Vector2D { Vector2D { x: 0.0, y: 0.0 } }

    /// Returns a new [Vector2D] created from 2 [Point2D]s
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::vector2d::Vector2D;
    ///
    /// let start = Point2D::new(0.541, 4.051);
    /// let end = Point2D::new(-3.093, 11.391);
    /// let result = Vector2D::from_2_points(&start, &end);
    /// assert_eq!(result.x, -3.093-0.541);
    /// assert_eq!(result.y, 11.391-4.051);
    /// ```
    pub fn from_2_points(start: &Point2D, end: &Point2D) -> Vector2D {
        Vector2D::new(end.x - start.x, end.y - start.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Vector2D::new(1.5, -2.3);
        assert_eq!(result.x, 1.5);
        assert_eq!(result.y, -2.3);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(1.5, -2.3);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(1.501, -2.3);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(1.5, -2.5);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Vector2D::new(1.5, -2.3);
        let b = Vector2D::new(-1.5, -2.5);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_x_unit_vector2d() {
        let result = Vector2D::x_unit();
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);
    }
    #[test]
    fn test_y_unit_vector2d() {
        let result = Vector2D::y_unit();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 1.0);
    }

    #[test]
    fn test_zero_vector() {
        let result = Vector2D::zero();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
    }

    #[test]
    fn test_from_2_points() {
        let start = Point2D::new(0.541, 4.051);
        let end = Point2D::new(-3.093, 11.391);
        let result = Vector2D::from_2_points(&start, &end);
        assert_eq!(result.x, -3.093-0.541);
        assert_eq!(result.y, 11.391-4.051);
    }
}