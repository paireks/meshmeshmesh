
/// Represents a two-dimensional point with double-precision floating-point coordinates.
///
/// # Example
///
/// ```
/// use meshmeshmesh::point2d::Point2d;
///
/// let result = Point2d::new(1.5, -2.3);
/// assert_eq!(result.x, 1.5);
/// assert_eq!(result.y, -2.3);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Point2d {
    /// The x-coordinate of the point.
    pub x: f64,
    /// The y-coordinate of the point.
    pub y: f64,
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point2d {
    /// Returns a new [Point2d]
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2d;
    ///
    /// let result = Point2d::new(1.5, -2.3);
    /// assert_eq!(result.x, 1.5);
    /// assert_eq!(result.y, -2.3);
    /// ```
    pub fn new(x: f64, y: f64) -> Point2d { Point2d { x, y} }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Point2d::new(1.5, -2.3);
        assert_eq!(result.x, 1.5);
        assert_eq!(result.y, -2.3);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Point2d::new(1.5, -2.3);
        let b = Point2d::new(1.5, -2.3);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Point2d::new(1.5, -2.3);
        let b = Point2d::new(1.501, -2.3);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Point2d::new(1.5, -2.3);
        let b = Point2d::new(1.5, -2.5);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Point2d::new(1.5, -2.3);
        let b = Point2d::new(-1.5, 0.0);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}