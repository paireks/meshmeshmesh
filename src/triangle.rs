use crate::point::Point;

/// Represents a triangle in three-dimensional space.
///
/// Sometimes it's easier to work with separate triangles rather than using a `Mesh`.
///
/// # Example
///
/// ```
/// use meshmeshmesh::point::Point;
/// use meshmeshmesh::triangle::Triangle;
///
/// let result = Triangle::new(
/// Point::new(0.0, 0.0, 0.0),
/// Point::new(10.0, 0.0, 0.0),
/// Point::new(10.0, -15.0, 0.0));
///
/// assert_eq!(result.first_point.eq(&Point::new(0.0, 0.0, 0.0)), true);
/// assert_eq!(result.second_point.eq(&Point::new(10.0, 0.0, 0.0)), true);
/// assert_eq!(result.third_point.eq(&Point::new(10.0, -15.0, 0.0)), true);
/// ```
pub struct Triangle {
    /// First point.
    pub first_point: Point,
    /// Second point.
    pub second_point: Point,
    /// Third point.
    pub third_point: Point,
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.first_point.eq(&other.first_point) && self.second_point.eq(&other.second_point) && self.third_point.eq(&other.third_point)
    }
}

impl Triangle {
    /// Returns a new [Triangle]
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let result = Triangle::new(
    /// Point::new(0.0, 0.0, 0.0),
    /// Point::new(10.0, 0.0, 0.0),
    /// Point::new(10.0, -15.0, 0.0));
    ///
    /// assert_eq!(result.first_point.eq(&Point::new(0.0, 0.0, 0.0)), true);
    /// assert_eq!(result.second_point.eq(&Point::new(10.0, 0.0, 0.0)), true);
    /// assert_eq!(result.third_point.eq(&Point::new(10.0, -15.0, 0.0)), true);
    /// ```
    pub fn new(first_point: Point, second_point: Point, third_point: Point) -> Triangle { Triangle { first_point, second_point, third_point } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(result.first_point.eq(&Point::new(0.0, 0.0, 0.0)), true);
        assert_eq!(result.second_point.eq(&Point::new(10.0, 0.0, 0.0)), true);
        assert_eq!(result.third_point.eq(&Point::new(10.0, -15.0, 0.0)), true);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, -7.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 2.1, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(101.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, -0.1, 0.0),
            Point::new(-10.0, 0.0, 5.0),
            Point::new(101.0, 76.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}