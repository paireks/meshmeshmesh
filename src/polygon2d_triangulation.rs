use i_triangle::float::triangulatable::Triangulatable;
use i_triangle::float::triangulation::Triangulation;
use crate::mesh::Mesh;
use crate::point2d::Point2D;
use crate::polygon2d::Polygon2D;

impl Polygon2D {

    /// Triangulates the [Polygon2D] using raw method introduced by iTriangle library.
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
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
    /// let actual = input.triangulate_raw();
    /// let expected = Mesh::new(
    /// vec![-10.59068199054718, 46.39268698813629, 0.0, -8.141663988990784, 35.17492201449585, 0.0, -5.981672009391785, 50.8752869916172, 0.0, -5.091521998806, 42.510926986953734, 0.0, -1.4562550093364717, 32.54851099374008, 0.0, -1.2906320001316072, 46.43312200429153, 0.0, 3.0757680033016204, 55.3231370103569, 0.0, 3.1068030093479155, 37.19115700843048, 0.0, 7.725793013410568, 50.99659201028061, 0.0, 7.829036990242004, 32.495452011844634, 0.0, 12.375817010240555, 37.20825799824905, 0.0, 12.456687012748718, 46.06352298977089, 0.0, 13.184517005519867, 53.665302003643035, 0.0, 15.044527003602981, 59.892291988155364, 0.0, 16.86410198553085, 41.77741300584984, 0.0, 17.02584199054718, 49.05571200968933, 0.0],
    /// vec![3, 0, 1, 4, 3, 1, 5, 3, 4, 6, 2, 5, 6, 5, 4, 7, 6, 4, 8, 6, 7, 9, 8, 7, 10, 8, 9, 11, 8, 10, 12, 8, 11, 13, 8, 12, 14, 12, 11, 15, 12, 14]
    /// );
    ///
    /// assert_eq!(expected, actual);
    /// ```
    pub fn triangulate_raw(&self) -> Mesh {
        let polygon_for_triangulation = self.get_anticlockwise();
        let mut flat_polygon_for_triangulation = Vec::with_capacity(polygon_for_triangulation.vertices.len());
        for i in polygon_for_triangulation.vertices {
            flat_polygon_for_triangulation.push([i.x, i.y]);
        }

        let shape = vec![
            flat_polygon_for_triangulation
        ];

        let triangulation: Triangulation<[f64; 2], usize> = shape.triangulate().to_triangulation();

        let mut points = Vec::new();
        for point2d in triangulation.points {
            points.push(point2d[0]);
            points.push(point2d[1]);
            points.push(0.0);
        }

        let mut indices = Vec::new();
        for index in triangulation.indices {
            indices.push(index);
        }

        Mesh::new(points, indices)
    }
}

#[cfg(test)]
mod tests {
    use crate::point2d::Point2D;
    use super::*;

    #[test]
    fn test_triangulate_raw() {
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

        let actual = input.triangulate_raw();
        let expected = Mesh::new(
            vec![-10.59068199054718, 46.39268698813629, 0.0, -8.141663988990784, 35.17492201449585, 0.0, -5.981672009391785, 50.8752869916172, 0.0, -5.091521998806, 42.510926986953734, 0.0, -1.4562550093364717, 32.54851099374008, 0.0, -1.2906320001316072, 46.43312200429153, 0.0, 3.0757680033016204, 55.3231370103569, 0.0, 3.1068030093479155, 37.19115700843048, 0.0, 7.725793013410568, 50.99659201028061, 0.0, 7.829036990242004, 32.495452011844634, 0.0, 12.375817010240555, 37.20825799824905, 0.0, 12.456687012748718, 46.06352298977089, 0.0, 13.184517005519867, 53.665302003643035, 0.0, 15.044527003602981, 59.892291988155364, 0.0, 16.86410198553085, 41.77741300584984, 0.0, 17.02584199054718, 49.05571200968933, 0.0],
            vec![3, 0, 1, 4, 3, 1, 5, 3, 4, 6, 2, 5, 6, 5, 4, 7, 6, 4, 8, 6, 7, 9, 8, 7, 10, 8, 9, 11, 8, 10, 12, 8, 11, 13, 8, 12, 14, 12, 11, 15, 12, 14]
        );

        assert_eq!(expected, actual);
    }
}
