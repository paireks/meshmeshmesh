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

    /// Creates a new clockwise version of given [Polygon2D].
    /// 
    /// Sometimes it's easier to have all Polygons in one direction, that's when such method
    /// might be useful.
    ///
    /// # Examples
    /// 
    /// This example shows that the counter-clockwise [Polygon2D] is turned into clockwise.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
    ///
    /// let expected = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    ///
    /// let actual = input.get_clockwise();
    ///
    /// assert!(expected.eq(&actual));
    ///
    /// ```
    /// 
    /// This example shows that the input Polygon is already clockwise, so new same direction
    /// Polygon is being returned.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    ///
    /// let expected = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    ///
    /// let actual = input.get_clockwise();
    ///
    /// assert!(expected.eq(&actual));
    ///
    /// ```
    pub fn get_clockwise(&self) -> Polygon2D {
        let mut new_vertices = self.vertices.clone();
        
        if !self.is_clockwise() { 
            new_vertices.reverse();
        }

        Polygon2D::new(new_vertices)
    }

    /// Creates a new anticlockwise version of given [Polygon2D].
    ///
    /// Sometimes it's easier to have all Polygons in one direction, that's when such method
    /// might be useful.
    ///
    /// # Examples
    ///
    /// This example shows that the clockwise [Polygon2D] is turned into anticlockwise.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
    ///
    /// let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
    ///
    /// let actual = input.get_anticlockwise();
    ///
    /// assert!(expected.eq(&actual));
    ///
    /// ```
    ///
    /// This example shows that the input Polygon is already anticlockwise, so new same direction
    /// Polygon is being returned.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
    ///
    /// let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
    ///
    /// let actual = input.get_anticlockwise();
    ///
    /// assert!(expected.eq(&actual));
    ///
    /// ```
    pub fn get_anticlockwise(&self) -> Polygon2D {
        let mut new_vertices = self.vertices.clone();

        if self.is_clockwise() {
            new_vertices.reverse();
        }

        Polygon2D::new(new_vertices)
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

    #[test]
    fn test_get_clockwise_counterclockwise() {
        let input = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
        
        let expected = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        
        let actual = input.get_clockwise();
        
        assert!(expected.eq(&actual));
    }

    #[test]
    fn test_get_clockwise_clockwise() {
        let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        
        let expected = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);
        
        let actual = input.get_clockwise();
        
        assert!(expected.eq(&actual));
    }

    #[test]
    fn test_get_anticlockwise_counterclockwise() {
        let input = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);

        let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
        
        let actual = input.get_anticlockwise();
        
        assert!(expected.eq(&actual));
    }

    #[test]
    fn test_get_anticlockwise_clockwise() {
        let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0)]);

        let expected = Polygon2D::new(vec![Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(0.0, 0.0)]);
        
        let actual = input.get_anticlockwise();
        
        assert!(expected.eq(&actual));
    }
}