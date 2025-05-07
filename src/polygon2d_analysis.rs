use crate::polygon2d::Polygon2D;

impl Polygon2D {
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
    /// let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, -10.0)]);
    ///
    /// assert!(input.is_clockwise());
    ///
    /// ```
    ///
    /// Here below there is an example of counter-clockwise [Polygon2D], so `false` is expected.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, -10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0)]);
    ///
    /// assert!(!input.is_clockwise());
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

/*    /// Gets [MonotoneVertexType]s for given [Polygon2D] assuming it's clockwise.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::{MonotoneVertexType, Point2D};
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![
    ///    Point2D::new(-5.981672, 50.875287),
    ///    Point2D::new(3.075768, 55.323137),
    ///    Point2D::new(7.725793, 50.996592),
    ///    Point2D::new(15.044527, 59.892292),
    ///    Point2D::new(13.184517, 53.665302),
    ///    Point2D::new(17.025842, 49.055712),
    ///    Point2D::new(16.864102, 41.777413),
    ///    Point2D::new(12.456687, 46.063523),
    ///    Point2D::new(12.375817, 37.208258),
    ///    Point2D::new(7.829037, 32.495452),
    ///    Point2D::new(3.106803, 37.191157),
    ///    Point2D::new(-1.456255, 32.548511),
    ///    Point2D::new(-8.141664, 35.174922),
    ///    Point2D::new(-10.590682, 46.392687),
    ///    Point2D::new(-5.091522, 42.510927),
    ///    Point2D::new(-1.290632, 46.433122),
    /// ]);
    ///
    /// let actual = input.get_monotone_vertices_types_for_clockwise();
    ///
    /// let expected: Vec<MonotoneVertexType> = vec![
    ///     MonotoneVertexType::Regular,
    ///     MonotoneVertexType::Start,
    ///     MonotoneVertexType::Merge,
    ///     MonotoneVertexType::Start,
    ///     MonotoneVertexType::Regular,
    ///     MonotoneVertexType::Regular,
    ///     MonotoneVertexType::End,
    ///     MonotoneVertexType::Split,
    ///     MonotoneVertexType::Regular,
    ///     MonotoneVertexType::End,
    ///     MonotoneVertexType::Split,
    ///     MonotoneVertexType::End,
    ///     MonotoneVertexType::Regular,
    ///     MonotoneVertexType::Start,
    ///     MonotoneVertexType::Merge,
    ///     MonotoneVertexType::Regular
    /// ];
    ///
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn get_monotone_vertices_types_for_clockwise(&self) -> Vec<MonotoneVertexType> {
        let number_of_vertices = self.vertices.len();
        let mut monotone_vertex_types: Vec<MonotoneVertexType> = Vec::with_capacity(number_of_vertices);

        let previous = self.vertices[number_of_vertices - 1];
        let current = self.vertices[0];
        let next = self.vertices[1];
        monotone_vertex_types.push(current.get_monotone_vertex_type_for_clockwise(&previous, &next)); // first
        for i in 1..number_of_vertices - 2 {
            let previous = self.vertices[i-1];
            let current = self.vertices[i];
            let next = self.vertices[i+1];
            monotone_vertex_types.push(current.get_monotone_vertex_type_for_clockwise(&previous, &next)); // middle ones
        }
        let previous = self.vertices[number_of_vertices - 2];
        let current = self.vertices[number_of_vertices - 1];
        let next = self.vertices[0];
        monotone_vertex_types.push(current.get_monotone_vertex_type_for_clockwise(&previous, &next)); // last

        monotone_vertex_types
    }*/
}

#[cfg(test)]
mod tests {
    use crate::point2d::Point2D;
    use super::*;

    #[test]
    fn test_is_clockwise_true() {
        let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, -10.0)]);

        assert!(input.is_clockwise());
    }

    #[test]
    fn test_is_clockwise_false() {
        let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, -10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0)]);

        assert!(!input.is_clockwise());
    }
}