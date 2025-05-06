use crate::point2d::Point2D;

/// Represents a two-dimensional closed polygon.
///
/// Polygon should contain at least 3 vertices, all vertices should be unique.
///
/// # Example
///
/// Here below is an example of defining polygon which represents a triangle shape, that's why
/// there are 3 vertices.
///
/// ```
/// use meshmeshmesh::point2d::Point2D;
/// use meshmeshmesh::polygon2d::Polygon2D;
///
/// let input = vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)];
///
/// let result = Polygon2D::new(input);
///
/// assert_eq!(result.vertices, vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
///
/// ```
#[derive(Debug)]
pub struct Polygon2D {
    /// Vertices which define closed [Polygon2D].
    pub vertices: Vec<Point2D>,
}

enum MonotoneVertexType {
    Start,
    Split,
    End,
    Merge,
    Regular
}

impl PartialEq for Polygon2D {
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

impl Polygon2D {
    /// Returns a new [Polygon2D].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)];
    ///
    /// let result = Polygon2D::new(input);
    ///
    /// assert_eq!(result.vertices, vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    ///
    /// ```
    pub fn new(vertices: Vec<Point2D>) -> Polygon2D { Polygon2D { vertices } }

    /// Checks if given [Polygon2D] is clockwise.
    ///
    /// If `true` is returned: it should be clockwise.
    /// If `false`: it should be counter-clockwise.
    ///
    /// This method assumes normal cartesian coordinate system with the Y-axis pointing up.
    /// 
    /// # Examples
    /// 
    /// Here below there is an example of a clockwise [Polygon2D], so `true` is expected.
    /// 
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let result = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, -10.0)]);
    ///
    /// assert!(result.is_clockwise());
    ///
    /// ```
    /// 
    /// Here below there is an example of counter-clockwise [Polygon2D], so `false` is expected.
    /// 
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let result = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, -10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0)]);
    ///
    /// assert!(!result.is_clockwise());
    ///
    /// ```
    pub fn is_clockwise(&self) -> bool {
        let vertices_length = self.vertices.len();
        let mut sum = 0.0;
        for i in 0..vertices_length-1 {
            let x1 = self.vertices[i].x;
            let y1 = self.vertices[i].y;
            let x2 = self.vertices[i + 1].x;
            let y2 = self.vertices[i + 1].y;
            sum += (x2-x1) * (y2+y1);
        }
        let x1 = self.vertices[vertices_length - 1].x; // Last closing segment
        let y1 = self.vertices[vertices_length - 1].y;
        let x2 = self.vertices[0].x;
        let y2 = self.vertices[0].y;
        sum += (x2-x1) * (y2+y1);

        sum >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)];
        let result = Polygon2D::new(input);
        assert_eq!(result.vertices, vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        let b = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        let b = Polygon2D::new(vec![Point2D::new(1.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_middle_different_false() {
        let a = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        let b = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, -10.0), Point2D::new(10.0, 0.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_last_different_false() {
        let a = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        let b = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.1)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        let b = Polygon2D::new(vec![Point2D::new(0.0, 0.1), Point2D::new(5.1, 10.0), Point2D::new(-10.1, 0.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_size_different_false() {
        let a = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        let b = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
    
    #[test]
    fn test_is_clockwise_true() {
        let result = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, -10.0)]);
        
        assert!(result.is_clockwise());
    }

    #[test]
    fn test_is_clockwise_false() {
        let result = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, -10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0)]);
        
        assert!(!result.is_clockwise());
    }
}