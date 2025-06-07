use crate::bounding_area::BoundingArea;
use crate::polygon2d::Polygon2D;
use crate::vector2d::Vector2D;

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

    /// Tries to get the id of the very first vertex, which is actually concave or convex,
    /// not the straight one.
    /// 
    /// Every [Polygon2D] should have every vertex concave or convex, not straight, but sometimes
    /// not cleaned [Polygon2D]s can happen, then such check could be useful.
    /// 
    /// If the `None` is returned, then there is no such vertex, which also means that this
    /// Polygon2D is not correct, and it is a some sort of the straight line.
    /// 
    /// # Example
    /// 
    /// ```
    ///
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![
    ///     Point2D::new(54.5, 25.0),
    ///     Point2D::new(53.5, 25.0),
    ///     Point2D::new(52.5, 25.0),
    ///     Point2D::new(50.0, 25.0), // This one is first one not straight, so 3 should be returned
    ///     Point2D::new(50.0, 50.0),
    ///     Point2D::new(65.0, 50.0),
    ///     Point2D::new(65.0, 45.0),
    ///     Point2D::new(55.0, 45.0),
    ///     Point2D::new(55.0, 40.0),
    ///     Point2D::new(65.0, 40.0),
    ///     Point2D::new(65.0, 35.0),
    ///     Point2D::new(55.0, 35.0),
    ///     Point2D::new(55.0, 25.0),
    /// ]);
    /// 
    /// let actual = input.get_first_not_straight_vertex_id(0.001);
    /// 
    /// assert_eq!(Some(3), actual);
    ///
    /// ```
    pub fn get_first_not_straight_vertex_id(&self, tolerance: f64) -> Option<usize> {
        let number_of_vertices = self.vertices.len();
        let mut not_straight_id = None;

        let previous_vector = Vector2D::from_2_points(&self.vertices[number_of_vertices-1], &self.vertices[0]);
        let next_vector = Vector2D::from_2_points(&self.vertices[0], &self.vertices[1]);
        if previous_vector.get_angle(&next_vector) > tolerance {
            not_straight_id = Some(0);
        }
        else {
            for i in 1..number_of_vertices-1 {
                let previous_vector = Vector2D::from_2_points(&self.vertices[i-1], &self.vertices[i]);
                let next_vector = Vector2D::from_2_points(&self.vertices[i], &self.vertices[i+1]);
                if previous_vector.get_angle(&next_vector) > tolerance {
                    not_straight_id = Some(i);
                    break
                }
            }
        }
        
        not_straight_id
    }
    
    /// Calculates the [BoundingArea] for this [Polygon2D].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::bounding_area::BoundingArea;
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![
    ///     Point2D::new(-5.981672, 50.875287),
    ///     Point2D::new(3.075768, 55.323137),
    ///     Point2D::new(7.725793, 50.996592),
    ///     Point2D::new(15.044527, 59.892292),
    ///     Point2D::new(13.184517, 53.665302),
    ///     Point2D::new(17.025842, 49.055712),
    ///     Point2D::new(16.864102, 41.777413),
    ///     Point2D::new(12.456687, 46.063523),
    ///     Point2D::new(12.375817, 37.208258),
    ///     Point2D::new(7.829037, 32.495452),
    ///     Point2D::new(3.106803, 37.191157),
    ///     Point2D::new(-1.456255, 32.548511),
    ///     Point2D::new(-8.141664, 35.174922),
    ///     Point2D::new(-10.590682, 46.392687),
    ///     Point2D::new(-5.091522, 42.510927),
    ///     Point2D::new(-1.290632, 46.433122),
    /// ]);
    ///
    /// let actual = input.get_bounding_area();
    /// let expected = BoundingArea::new(-10.590682, 17.025842, 32.495452, 59.892292);
    /// 
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn get_bounding_area(&self) -> BoundingArea {
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;

        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;

        for vertex in &self.vertices {
            if vertex.x < min_x { 
                min_x = vertex.x
            }

            if vertex.x > max_x {
                max_x = vertex.x
            }

            if vertex.y < min_y {
                min_y = vertex.y
            }

            if vertex.y > max_y {
                max_y = vertex.y
            }
        }
        
        BoundingArea::new(min_x, max_x, min_y, max_y)
    }
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
    
    #[test]
    fn test_get_first_get_first_not_straight_vertex_id() {
        let input = Polygon2D::new(vec![
            Point2D::new(54.5, 25.0),
            Point2D::new(53.5, 25.0),
            Point2D::new(52.5, 25.0),
            Point2D::new(50.0, 25.0), // This one is first one not straight, so 3 should be returned
            Point2D::new(50.0, 50.0),
            Point2D::new(65.0, 50.0),
            Point2D::new(65.0, 45.0),
            Point2D::new(55.0, 45.0),
            Point2D::new(55.0, 40.0),
            Point2D::new(65.0, 40.0),
            Point2D::new(65.0, 35.0),
            Point2D::new(55.0, 35.0),
            Point2D::new(55.0, 25.0),
        ]);
        
        let actual = input.get_first_not_straight_vertex_id(0.001);
        
        assert_eq!(Some(3), actual);
    }
    
    #[test]
    fn test_get_bounding_area() {
        let input = Polygon2D::new(vec![
            Point2D::new(-5.981672, 50.875287),
            Point2D::new(3.075768, 55.323137),
            Point2D::new(7.725793, 50.996592),
            Point2D::new(15.044527, 59.892292),
            Point2D::new(13.184517, 53.665302),
            Point2D::new(17.025842, 49.055712),
            Point2D::new(16.864102, 41.777413),
            Point2D::new(12.456687, 46.063523),
            Point2D::new(12.375817, 37.208258),
            Point2D::new(7.829037, 32.495452),
            Point2D::new(3.106803, 37.191157),
            Point2D::new(-1.456255, 32.548511),
            Point2D::new(-8.141664, 35.174922),
            Point2D::new(-10.590682, 46.392687),
            Point2D::new(-5.091522, 42.510927),
            Point2D::new(-1.290632, 46.433122),
        ]);
        
        let actual = input.get_bounding_area();
        let expected = BoundingArea::new(-10.590682, 17.025842, 32.495452, 59.892292);
        
        assert_eq!(expected, actual);
    }
}