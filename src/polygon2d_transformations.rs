use crate::polygon2d::Polygon2D;

impl Polygon2D {
    /// Reverse given [Polygon2D].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let mut input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    ///
    /// let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
    ///
    /// input.reverse();
    ///
    /// assert!(expected.eq(&input));
    ///
    /// ```
    pub fn reverse(&mut self) {
        self.vertices.reverse();
    }
    
    /// Creates a new reversed version of given [Polygon2D].
    /// 
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    ///
    /// let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
    /// 
    /// let actual = input.get_reversed();
    /// 
    /// assert!(expected.eq(&actual));
    /// 
    /// ```
    pub fn get_reversed(&self) -> Polygon2D {
        let mut reversed_vertices = self.vertices.clone();
        reversed_vertices.reverse();
        
        Polygon2D::new(reversed_vertices)
    }
}

#[cfg(test)]
mod tests {
    use crate::point2d::Point2D;
    use super::*;

    #[test]
    fn test_reverse() {
        let mut input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        
        let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
        
        input.reverse();
        
        assert!(expected.eq(&input));
    }

    #[test]
    fn test_get_reversed() {
        let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        
        let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
        
        let actual = input.get_reversed();
        
        assert!(expected.eq(&actual));
    }
}