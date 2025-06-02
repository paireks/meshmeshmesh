use crate::local_coordinate_system::LocalCoordinateSystem;
use crate::point2d::Point2D;
use crate::point::Point;
use crate::polygon2d::Polygon2D;

/// Represents a three-dimensional closed polygon.
///
/// Polygon should contain at least 3 vertices, all vertices should be unique.
/// 
/// Even though this polygon is three-dimensional: it should be planar.
/// 
/// Polygon shouldn't contain duplicate vertices.
/// 
/// It also shouldn't have parallel segments neighbour segments.
///
/// # Example
///
/// Here below is an example of defining polygon which represents a triangle shape, that's why
/// there are 3 vertices.
///
/// ```
/// use meshmeshmesh::point::Point;
/// use meshmeshmesh::polygon::Polygon;
///
/// let input = vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)];
///
/// let result = Polygon::new(input);
///
/// assert_eq!(result.vertices, vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
///
/// ```
#[derive(Debug)]
pub struct Polygon {
    /// Vertices which define closed [Polygon].
    pub vertices: Vec<Point>,
}

impl PartialEq for Polygon {
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

impl Polygon {
    /// Returns a new [Polygon].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon::Polygon;
    ///
    /// let input = vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)];
    ///
    /// let result = Polygon::new(input);
    ///
    /// assert_eq!(result.vertices, vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
    ///
    /// ```
    pub fn new(vertices: Vec<Point>) -> Polygon { Polygon { vertices } }
    
    /// Converts to the [Polygon2D] using given [LocalCoordinateSystem].
    /// 
    /// It transforms all the vertices using XY plane of the [LocalCoordinateSystem] into Global
    /// Coordinate System, and then it projects onto this global XY plane.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::polygon2d::Polygon2D;
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
    /// let origin = Point::new(-15.519542, 33.6924,54.752506);
    /// let x = Vector::new(0.13940120784477725,0.6260669228918656,0.767207606396162);
    /// let y = Vector::new(0.7651247740152495,0.4237333029589709,-0.4848032262182198);
    /// let local_coordinate_system = LocalCoordinateSystem::new(origin, x, y);
    ///
    /// let expected = Polygon2D::new( vec![
    ///     Point2D::new(0.0,0.0),
    ///     Point2D::new(62.71717537580547,7.105427357601002e-15),
    ///     Point2D::new(28.886507707212203,64.92972746865411),
    ///     Point2D::new(-14.884010122540644,49.54702209072535),
    ///     Point2D::new(19.89519033516472,22.871028401215423),
    /// ]);
    ///
    /// let actual = input.to_polygon2d(&local_coordinate_system);
    ///
    /// assert_eq!(expected, actual);
    /// 
    /// ```
    pub fn to_polygon2d(&self, local_coordinate_system: &LocalCoordinateSystem) -> Polygon2D {
        let polygon_transformed = self.get_in_global_coordinate_system(local_coordinate_system);
        let mut point2ds = Vec::with_capacity(self.vertices.len());
        for vertex in polygon_transformed.vertices {
            point2ds.push(Point2D::new(vertex.x, vertex.y))
        }
        
        Polygon2D::new(point2ds)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use super::*;

    #[test]
    fn test_new() {
        let input = vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)];
        let result = Polygon::new(input);
        assert_eq!(result.vertices, vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, -1.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_middle_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 3.0), Point::new(10.0, 0.0, 5.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_last_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 6.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(-0.1, 0.0, 5.0), Point::new(5.0, 11.0, 5.0), Point::new(10.0, 0.0, 33.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_size_different_false() {
        let a = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0), Point::new(10.0, 0.0, 5.0)]);
        let b = Polygon::new(vec![Point::new(0.0, 0.0, 5.0), Point::new(5.0, 10.0, 5.0)]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
    
    #[test]
    fn test_to_polygon2d() {
        let input_points = vec![
            Point::new(-15.519542, 33.6924, 54.752506),
            Point::new(-6.776692, 72.957549, 102.8696),
            Point::new(38.186615, 79.290175, 45.436313),
            Point::new(20.315263, 45.368737, 19.312824),
            Point::new(4.753062, 55.839337, 58.928299),
        ];
        
        let input = Polygon::new(input_points);
        
        let origin = Point::new(-15.519542, 33.6924,54.752506);
        let x = Vector::new(0.13940120784477725,0.6260669228918656,0.767207606396162);
        let y = Vector::new(0.7651247740152495,0.4237333029589709,-0.4848032262182198);
        let local_coordinate_system = LocalCoordinateSystem::new(origin, x, y);
        
        let expected = Polygon2D::new( vec![
            Point2D::new(0.0,0.0),
            Point2D::new(62.71717537580547,7.105427357601002e-15),
            Point2D::new(28.886507707212203,64.92972746865411),
            Point2D::new(-14.884010122540644,49.54702209072535),
            Point2D::new(19.89519033516472,22.871028401215423),
        ]);
        
        let actual = input.to_polygon2d(&local_coordinate_system);
        
        assert_eq!(expected, actual);
    }
}