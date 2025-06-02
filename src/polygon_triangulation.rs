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

    /// Triangulates the [Polygon] using raw method introduced by iTriangle library.
    ///
    /// It also allows to define the holes.
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
    /// let hole1 = Polygon::new(vec![
    ///     Point::new(-2.924185, 62.225129, 80.373895),
    ///     Point::new(20.185889, 71.417946, 60.110316),
    ///     Point::new(-8.776735, 48.74057, 68.11553),
    /// ]);
    ///
    /// let hole2 = Polygon::new(vec![
    ///     Point::new(13.477064, 54.448289, 43.701334),
    ///     Point::new(11.676458, 62.943122, 59.637808),
    ///     Point::new(22.774106, 72.530136, 57.969704),
    ///     Point::new(14.188262, 62.46664, 55.135255),
    /// ]);
    ///
    /// let actual = input.triangulate_raw_with_holes(vec![hole1, hole2]);
    /// let expected = Mesh::new(
    ///     vec![20.315263073219796,45.368736910106094,19.31282409741451,-15.519541989587127,33.6923999962421,54.752505978555725,13.47706429545419,54.44828873947391,43.70133418810492,4.753061984918318,55.839336994745345,58.92829901438469,-8.77673518069329,48.74057015539112, 68.11552990722693,14.188262225723733, 62.46663978597737,55.13525516632474, 11.676457660496215,62.943122312782386, 59.63780778399595,38.18661511178051,79.29017488240531,45.43631310822596, 22.774106108979495,72.53013587908147, 57.96970408308002,20.18588893088867, 71.41794609990754,60.11031592295312, -2.924185173340555,62.22512918203788, 80.37389489322362,-6.77669199116391,72.95754898916059, 102.86959996987775],
    ///     vec![3, 2, 0, 4, 3, 1, 5, 0, 2, 6, 2, 3, 6, 3, 4, 7, 0, 5, 8, 6, 4, 8, 7, 5, 9, 8, 4, 9, 7, 8, 10, 4, 1, 10, 7, 9, 11, 7, 10, 11, 10, 1]
    /// );
    ///
    /// assert_eq!(expected, actual);
    /// ```
    pub fn triangulate_raw_with_holes(&self, holes: Vec<Polygon>) -> Mesh {
        let local_coordinate_system = self.get_local_coordinate_system();
        let polygon_2d = self.to_polygon2d(&local_coordinate_system);
        let mut holes_2d = Vec::with_capacity(holes.len());
        for hole in holes {
            holes_2d.push(hole.to_polygon2d(&local_coordinate_system));
        }
        let mesh_2d = polygon_2d.triangulate_raw_with_holes(holes_2d);
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

    #[test]
    fn test_triangulate_raw_with_holes() {
        let input_points = vec![
            Point::new(-15.519542, 33.6924, 54.752506),
            Point::new(-6.776692, 72.957549, 102.8696),
            Point::new(38.186615, 79.290175, 45.436313),
            Point::new(20.315263, 45.368737, 19.312824),
            Point::new(4.753062, 55.839337, 58.928299),
        ];

        let input = Polygon::new(input_points);

        let hole1 = Polygon::new(vec![
            Point::new(-2.924185, 62.225129, 80.373895),
            Point::new(20.185889, 71.417946, 60.110316),
            Point::new(-8.776735, 48.74057, 68.11553),
        ]);

        let hole2 = Polygon::new(vec![
            Point::new(13.477064, 54.448289, 43.701334),
            Point::new(11.676458, 62.943122, 59.637808),
            Point::new(22.774106, 72.530136, 57.969704),
            Point::new(14.188262, 62.46664, 55.135255),
        ]);

        let actual = input.triangulate_raw_with_holes(vec![hole1, hole2]);
        let expected = Mesh::new(
        vec![20.315263073219796,45.368736910106094,19.31282409741451,-15.519541989587127,33.6923999962421,54.752505978555725,13.47706429545419,54.44828873947391,43.70133418810492,4.753061984918318,55.839336994745345,58.92829901438469,-8.77673518069329,48.74057015539112, 68.11552990722693,14.188262225723733, 62.46663978597737,55.13525516632474, 11.676457660496215,62.943122312782386, 59.63780778399595,38.18661511178051,79.29017488240531,45.43631310822596, 22.774106108979495,72.53013587908147, 57.96970408308002,20.18588893088867, 71.41794609990754,60.11031592295312, -2.924185173340555,62.22512918203788, 80.37389489322362,-6.77669199116391,72.95754898916059, 102.86959996987775],
        vec![3, 2, 0, 4, 3, 1, 5, 0, 2, 6, 2, 3, 6, 3, 4, 7, 0, 5, 8, 6, 4, 8, 7, 5, 9, 8, 4, 9, 7, 8, 10, 4, 1, 10, 7, 9, 11, 7, 10, 11, 10, 1]
        );

        assert_eq!(expected, actual);
    }
}