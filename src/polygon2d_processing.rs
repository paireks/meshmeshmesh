use crate::polygon2d::Polygon2D;
use crate::vector2d::Vector2D;

impl Polygon2D {
    /// Creates new [Polygon2D], but without vertices (which are neighbours) and `eq_with_tolerance`.
    ///
    /// It can be useful for cleaning the duplicate vertices.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![
    ///     Point2D::new(-5.981672, 50.875287),
    ///     Point2D::new(3.075768, 55.323137),
    ///     Point2D::new(7.725793, 50.996592),
    ///     Point2D::new(15.044527, 59.892292),
    ///     Point2D::new(15.044527, 59.892292), // duplicate
    ///     Point2D::new(15.044527, 59.892292), // duplicate
    ///     Point2D::new(15.044527, 59.892292), // duplicate
    ///     Point2D::new(13.184517, 53.665302),
    ///     Point2D::new(17.025842, 49.055712),
    ///     Point2D::new(16.864102, 41.777413),
    ///     Point2D::new(12.456687, 46.063523),
    ///     Point2D::new(12.375817, 37.208258),
    ///     Point2D::new(12.375818, 37.208257), // duplicate within tolerance
    ///     Point2D::new(7.829037, 32.495452),
    ///     Point2D::new(3.106803, 37.191157),
    ///     Point2D::new(-1.456255, 32.548511),
    ///     Point2D::new(-8.141664, 35.174922),
    ///     Point2D::new(-10.590682, 46.392687),
    ///     Point2D::new(-5.091522, 42.510927),
    ///     Point2D::new(-1.290632, 46.433122),
    ///     Point2D::new(-1.290632, 46.433122), // duplicate
    /// ]);
    ///
    /// let actual = input.get_with_removed_neighbour_duplicates_with_tolerance(0.001);
    ///
    /// let expected = Polygon2D::new(vec![
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
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn get_with_removed_neighbour_duplicates_with_tolerance(&self, tolerance: f64) -> Polygon2D {
        let number_of_vertices = self.vertices.len();
        let mut cleaned_vertices = Vec::with_capacity(number_of_vertices);
        let mut i = 0;
        while i < number_of_vertices {
            let current = self.vertices[i];
            cleaned_vertices.push(current); // Current should always be unique

            if i+1 == number_of_vertices { break } 
            let mut below = self.vertices[i+1];
            
            while current.eq_with_tolerance(&below, tolerance) {
                i += 1;
                
                if i+1 == number_of_vertices { break } 
                below = self.vertices[i+1];
            }
            
            i += 1;
        }

        let current = self.vertices[number_of_vertices-1]; // Check if last one is the duplicate of first one
        let below = self.vertices[0];
        if current.eq_with_tolerance(&below, tolerance) { 
            cleaned_vertices.remove(cleaned_vertices.len() - 1);
        }

        Polygon2D::new(cleaned_vertices)
    }

    /// Creates new [Polygon2D], but without segments (which are neighbours) that are parallel.
    ///
    /// It can be useful for cleaning unnecessary parallel segments.
    ///
    /// `tolerance` is used here for angle measurement: if angle between segments' vector is <=
    /// tolerance, then such segment should be removed.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![
    ///     Point2D::new(-5.981672, 50.875287),
    ///     Point2D::new(3.075768, 55.323137),
    ///     Point2D::new(7.725793, 50.996592),
    ///     Point2D::new(15.044527, 59.892292),
    ///     Point2D::new(13.184517, 53.665302),
    ///     Point2D::new(16.683995, 49.46593), // parallel vertex
    ///     Point2D::new(17.025842, 49.055712),
    ///     Point2D::new(16.864102, 41.777413),
    ///     Point2D::new(12.456687, 46.063523),
    ///     Point2D::new(12.375817, 37.208258),
    ///     Point2D::new(7.829037, 32.495452),
    ///     Point2D::new(3.106803, 37.191157),
    ///     Point2D::new(0.37224, 34.408898), // parallel vertex
    ///     Point2D::new(-0.530578, 33.490333), // parallel vertex
    ///     Point2D::new(-1.456255, 32.548511),
    ///     Point2D::new(-8.141664, 35.174922),
    ///     Point2D::new(-10.590682, 46.392687),
    ///     Point2D::new(-5.091522, 42.510927),
    ///     Point2D::new(-1.290632, 46.433122),
    /// ]);
    ///
    /// let actual = input.get_with_removed_neighbour_parallel_segments_with_tolerance(0.01);
    ///
    /// let expected = Polygon2D::new(vec![
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
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn get_with_removed_neighbour_parallel_segments_with_tolerance(&self, tolerance: f64) -> Polygon2D {
        let number_of_vertices = self.vertices.len();
        let mut cleaned_vertices = Vec::with_capacity(number_of_vertices);
        
        let mut i = 1;
        while i < number_of_vertices {
            let previous = self.vertices[i-1];
            cleaned_vertices.push(previous);

            let this = self.vertices[i];
            let previous_vector = Vector2D::from_2_points(&previous, &this);

            if i+1 == number_of_vertices { break }
            let mut next = self.vertices[i+1];
            let mut next_vector = Vector2D::from_2_points(&this, &next);

            while previous_vector.get_angle(&next_vector) <= tolerance {
                i += 1;

                if i+1 == number_of_vertices { break }
                next = self.vertices[i+1];
                next_vector = Vector2D::from_2_points(&this, &next);
            }

            i += 1;
        }

        let previous = self.vertices[number_of_vertices-2]; // Last segments check with first one
        let this = self.vertices[number_of_vertices-1];
        let next = self.vertices[0];
        let previous_vector = Vector2D::from_2_points(&previous, &this);
        let next_vector = Vector2D::from_2_points(&this, &next);
        if previous_vector.get_angle(&next_vector) > tolerance {
            cleaned_vertices.push(this);
        }

        Polygon2D::new(cleaned_vertices)
    }
}

#[cfg(test)]
mod tests {
    use crate::point2d::Point2D;
    use super::*;

    #[test]
    fn test_get_with_removed_neighbour_duplicates_with_tolerance() {
        let input = Polygon2D::new(vec![
            Point2D::new(-5.981672, 50.875287),
            Point2D::new(3.075768, 55.323137),
            Point2D::new(7.725793, 50.996592),
            Point2D::new(15.044527, 59.892292),
            Point2D::new(15.044527, 59.892292), // duplicate
            Point2D::new(15.044527, 59.892292), // duplicate
            Point2D::new(15.044527, 59.892292), // duplicate
            Point2D::new(13.184517, 53.665302),
            Point2D::new(17.025842, 49.055712),
            Point2D::new(16.864102, 41.777413),
            Point2D::new(12.456687, 46.063523),
            Point2D::new(12.375817, 37.208258),
            Point2D::new(12.375818, 37.208257), // duplicate within tolerance
            Point2D::new(7.829037, 32.495452),
            Point2D::new(3.106803, 37.191157),
            Point2D::new(-1.456255, 32.548511),
            Point2D::new(-8.141664, 35.174922),
            Point2D::new(-10.590682, 46.392687),
            Point2D::new(-5.091522, 42.510927),
            Point2D::new(-1.290632, 46.433122),
            Point2D::new(-1.290632, 46.433122), // duplicate
        ]);

        let actual = input.get_with_removed_neighbour_duplicates_with_tolerance(0.001);

        let expected = Polygon2D::new(vec![
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

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_with_removed_neighbour_duplicates_with_tolerance_last() {
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
            Point2D::new(-5.981672, 50.875287), // This one is same as last one
        ]);

        let actual = input.get_with_removed_neighbour_duplicates_with_tolerance(0.001);

        let expected = Polygon2D::new(vec![
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

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_with_removed_neighbour_parallel_segments_with_tolerance() {
        let input = Polygon2D::new(vec![
            Point2D::new(-5.981672, 50.875287),
            Point2D::new(3.075768, 55.323137),
            Point2D::new(7.725793, 50.996592),
            Point2D::new(15.044527, 59.892292),
            Point2D::new(13.184517, 53.665302),
            Point2D::new(16.683995, 49.46593), // parallel vertex
            Point2D::new(17.025842, 49.055712),
            Point2D::new(16.864102, 41.777413),
            Point2D::new(12.456687, 46.063523),
            Point2D::new(12.375817, 37.208258),
            Point2D::new(7.829037, 32.495452),
            Point2D::new(3.106803, 37.191157),
            Point2D::new(0.37224, 34.408898), // parallel vertex
            Point2D::new(-0.530578, 33.490333), // parallel vertex
            Point2D::new(-1.456255, 32.548511),
            Point2D::new(-8.141664, 35.174922),
            Point2D::new(-10.590682, 46.392687),
            Point2D::new(-5.091522, 42.510927),
            Point2D::new(-1.290632, 46.433122),
        ]);
        
        let actual = input.get_with_removed_neighbour_parallel_segments_with_tolerance(0.01);
        
        let expected = Polygon2D::new(vec![
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
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_with_removed_neighbour_parallel_segments_with_tolerance_removing_last_segment() {
        let input = Polygon2D::new(vec![
            Point2D::new(50.0, 25.0),
            Point2D::new(50.0, 50.0),
            Point2D::new(65.0, 50.0),
            Point2D::new(65.0, 45.0),
            Point2D::new(55.0, 45.0),
            Point2D::new(55.0, 40.0),
            Point2D::new(65.0, 40.0),
            Point2D::new(65.0, 35.0),
            Point2D::new(55.0, 35.0),
            Point2D::new(55.0, 25.0),
            Point2D::new(52.5, 25.0), // This one is parallel to first one
        ]);

        let actual = input.get_with_removed_neighbour_parallel_segments_with_tolerance(0.01);

        let expected = Polygon2D::new(vec![
            Point2D::new(50.0, 25.0),
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

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_with_removed_neighbour_parallel_segments_with_tolerance_first_to_remove() {
        let input = Polygon2D::new(vec![
            Point2D::new(52.5, 25.0), // This one is parallel to first one
            Point2D::new(50.0, 25.0),
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

        let actual = input.get_with_removed_neighbour_parallel_segments_with_tolerance(0.01);

        let expected = Polygon2D::new(vec![
            Point2D::new(50.0, 25.0),
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

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_with_removed_neighbour_parallel_segments_with_tolerance_removing_last_segment_multiple() {
        let input = Polygon2D::new(vec![
            Point2D::new(50.0, 25.0),
            Point2D::new(50.0, 50.0),
            Point2D::new(65.0, 50.0),
            Point2D::new(65.0, 45.0),
            Point2D::new(55.0, 45.0),
            Point2D::new(55.0, 40.0),
            Point2D::new(65.0, 40.0),
            Point2D::new(65.0, 35.0),
            Point2D::new(55.0, 35.0),
            Point2D::new(55.0, 25.0),
            Point2D::new(54.0, 25.0), // This one is parallel to first one
            Point2D::new(53.0, 25.0), // This one is parallel to first one
            Point2D::new(52.0, 25.0), // This one is parallel to first one
            Point2D::new(51.0, 25.0), // This one is parallel to first one
        ]);

        let actual = input.get_with_removed_neighbour_parallel_segments_with_tolerance(0.01);

        let expected = Polygon2D::new(vec![
            Point2D::new(50.0, 25.0),
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

        assert_eq!(expected, actual);
    }
}