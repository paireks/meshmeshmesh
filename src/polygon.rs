use crate::point::Point;

/// Represents a three-dimensional closed polygon.
///
/// Polygon should contain at least 3 vertices, all vertices should be unique.
///
/// # Example
///
/// Here below is an example of defining polygon which represents a triangle shape, that's why
/// there are 3 vertices.
///
/// ```
/// use meshmeshmesh::point::Point;
/// use meshmeshmesh::polygon::Polygon;
///
/// let input = vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)];
///
/// let result = Polygon::new(input);
///
/// assert_eq!(result.vertices, vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
///
/// ```
#[derive(Debug)]
pub struct Polygon {
    /// Vertices which define closed [Polygon].
    pub vertices: Vec<Point>,
}

impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        if self.vertices.len() != other.vertices.len() {
            return false;
        }
        for i in 0..self.vertices.len() {
            if self.vertices[i] != other.vertices[i] {
                return false;
            }
        }

        true
    }
}

impl Polygon {
    /// Returns a new [Polygon].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    ///
    /// let input = vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)];
    ///
    /// let result = Polygon::new(input);
    ///
    /// assert_eq!(result.vertices, vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
    ///
    /// ```
    pub fn new(vertices: Vec<Point>) -> Polygon { Polygon { vertices } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)];
        let result = Polygon::new(input);
        assert_eq!(result.vertices, vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, -1.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_middle_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 3.0), Point::new(10.0, 0.0, 5.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_last_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 6.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(-0.1, 0.0, 5.0), Point::new(5.0, 11.0, 5.0), Point::new(10.0, 0.0, 33.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_size_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}