use crate::mesh::Mesh;
use crate::polygon::Polygon;

impl Polygon {
    /// Triangulates the [Polygon] using raw method introduced by iTriangle library.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input_points = vec![
    ///     Point::new(-15.519542, 33.6924, 54.752506),
    ///     Point::new(-6.776692, 72.957549, 102.8696),
    ///     Point::new(38.186615, 79.290175, 45.436313),
    ///     Point::new(20.315263, 45.368737, 19.312824),
    ///     Point::new(4.753062, 55.839337, 58.928299),
    /// ];
    ///
    /// let input = Polygon::new(input_points);
    ///
    /// let actual = input.triangulate_raw();
    /// let expected = Mesh::new(
    /// vec![20.315263073219796,45.368736910106094,19.31282409741451,-15.519541989587127,33.6923999962421,54.752505978555725,4.753061984918318, 55.839336994745345,58.92829901438469,38.18661511178051,79.29017488240531,45.43631310822596,-6.77669199116391,72.95754898916059,102.86959996987775],
    /// vec![3,0,2,4,3,2,4,2,1]
    /// );
    ///
    /// assert_eq!(expected, actual);
    /// ```
    pub fn triangulate_raw(&self) -> Mesh {
        let local_coordinate_system = self.get_local_coordinate_system();
        let polygon_2d = self.to_polygon2d(&local_coordinate_system);
        let mesh_2d = polygon_2d.triangulate_raw();
        mesh_2d.get_in_local_coordinate_system(&local_coordinate_system)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn test_triangulate_raw() {
        let input_points = vec![
            Point::new(-15.519542, 33.6924, 54.752506),
            Point::new(-6.776692, 72.957549, 102.8696),
            Point::new(38.186615, 79.290175, 45.436313),
            Point::new(20.315263, 45.368737, 19.312824),
            Point::new(4.753062, 55.839337, 58.928299),
        ];
        
        let input = Polygon::new(input_points);
        
        let actual = input.triangulate_raw();
        let expected = Mesh::new(
            vec![20.315263073219796,45.368736910106094,19.31282409741451,-15.519541989587127,33.6923999962421,54.752505978555725,4.753061984918318, 55.839336994745345,58.92829901438469,38.18661511178051,79.29017488240531,45.43631310822596,-6.77669199116391,72.95754898916059,102.86959996987775],
            vec![3,0,2,4,3,2,4,2,1]
        );
        
        assert_eq!(expected, actual);
    }
}