use crate::polygon2d::Polygon2D;

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
}