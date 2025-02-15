use crate::point::Point;
use crate::triangle::Triangle;

/// Represents a barycentric coordinates.
///
/// # Example
///
/// ```
/// use meshmeshmesh::barycentric_coordinates::BarycentricCoordinates;
///
/// let result = BarycentricCoordinates::new(0.5, 0.3, 0.2);
/// assert_eq!(result.u, 0.5);
/// assert_eq!(result.v, 0.3);
/// assert_eq!(result.w, 0.2);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct BarycentricCoordinates {
    /// The u-coordinate.
    pub u: f64,
    /// The v-coordinate.
    pub v: f64,
    /// The w-coordinate.
    pub w: f64
}

impl PartialEq for BarycentricCoordinates {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v && self.w == other.w
    }
}

impl BarycentricCoordinates {
    /// Returns a new [BarycentricCoordinates].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::barycentric_coordinates::BarycentricCoordinates;
    ///
    /// let result = BarycentricCoordinates::new(0.5, 0.3, 0.2);
    /// assert_eq!(result.u, 0.5);
    /// assert_eq!(result.v, 0.3);
    /// assert_eq!(result.w, 0.2);
    /// ```
    pub fn new(u: f64, v: f64, w: f64) -> BarycentricCoordinates { BarycentricCoordinates { u, v, w } }

    /// Converts a [Point] and [Triangle] into [BarycentricCoordinates].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::barycentric_coordinates::BarycentricCoordinates;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input_triangle = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let input_point = Point::new(1.922108, 12.641649, -10.707643);
    ///
    /// let actual = BarycentricCoordinates::new_from_point_and_triangle(input_point, input_triangle);
    /// let expected = BarycentricCoordinates::new(0.5, 0.3, 0.2);
    ///
    /// assert_eq!(actual.eq_with_tolerance(&expected, 0.001), true);
    /// ```
    ///
    pub fn new_from_point_and_triangle(point: Point, triangle: Triangle) -> BarycentricCoordinates {
        let triangles = triangle.get_divided_by_point(point);
        let bcp_area = triangles.0.get_area();
        let cap_area = triangles.1.get_area();
        let abp_area = triangles.2.get_area();
        let abc_area = triangle.get_area();

        let u = bcp_area / abc_area;
        let v = cap_area / abc_area;
        let w = abp_area / abc_area;

        BarycentricCoordinates::new(u, v, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        assert_eq!(result.u, 0.5);
        assert_eq!(result.v, 0.3);
        assert_eq!(result.w, 0.2);
    }

    #[test]
    fn test_new_from_point_and_triangle() {
        let input_triangle = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let input_point = Point::new(1.922108, 12.641649, -10.707643);

        let actual = BarycentricCoordinates::new_from_point_and_triangle(input_point, input_triangle);
        let expected = BarycentricCoordinates::new(0.5, 0.3, 0.2);

        assert_eq!(actual.eq_with_tolerance(&expected, 0.001), true);
    }

    #[test]
    fn test_new_from_point_and_triangle_first_point() {
        let input_triangle = Triangle::new(
            Point::new(35.704653, 37.253023, -22.626602),
            Point::new(-38.634947, 13.199458, 23.94433),
            Point::new(-21.698671, -49.7235, -32.888206));

        let input_point = Point::new(35.704653, 37.253023, -22.626602);

        let actual = BarycentricCoordinates::new_from_point_and_triangle(input_point, input_triangle);
        let expected = BarycentricCoordinates::new(1.0, 0.0, 0.0);

        assert_eq!(actual.eq_with_tolerance(&expected, 0.001), true);
    }

    #[test]
    fn test_new_from_point_and_triangle_second_point() {
        let input_triangle = Triangle::new(
            Point::new(35.704653, 37.253023, -22.626602),
            Point::new(-38.634947, 13.199458, 23.94433),
            Point::new(-21.698671, -49.7235, -32.888206));

        let input_point = Point::new(-38.634947, 13.199458, 23.94433);

        let actual = BarycentricCoordinates::new_from_point_and_triangle(input_point, input_triangle);
        let expected = BarycentricCoordinates::new(0.0, 1.0, 0.0);

        assert_eq!(actual.eq_with_tolerance(&expected, 0.001), true);
    }

    #[test]
    fn test_new_from_point_and_triangle_third_point() {
        let input_triangle = Triangle::new(
            Point::new(35.704653, 37.253023, -22.626602),
            Point::new(-38.634947, 13.199458, 23.94433),
            Point::new(-21.698671, -49.7235, -32.888206));

        let input_point = Point::new(-21.698671, -49.7235, -32.888206);

        let actual = BarycentricCoordinates::new_from_point_and_triangle(input_point, input_triangle);
        let expected = BarycentricCoordinates::new(0.0, 0.0, 1.0);

        assert_eq!(actual.eq_with_tolerance(&expected, 0.001), true);
    }

    #[test]
    fn test_partialeq_true() {
        let a = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        let b = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        let b = BarycentricCoordinates::new(0.51, 0.3, 0.2);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        let b = BarycentricCoordinates::new(0.5, 0.31, 0.2);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        let b = BarycentricCoordinates::new(0.5, 0.3, 0.21);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = BarycentricCoordinates::new(0.5, 0.3, 0.2);
        let b = BarycentricCoordinates::new(0.51, 0.295, 0.195);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}