use serde::{Deserialize, Serialize};
use crate::point::Point;

/// Represents a triangle in three-dimensional space.
#[derive(Deserialize, Serialize)]
pub struct Triangle {
    /// First point.
    pub point1: Point,
    /// Second point.
    pub point2: Point,
    /// Third point.
    pub point3: Point,
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.point1.eq(&other.point1) && self.point2.eq(&other.point2) && self.point3.eq(&other.point3)
    }
}

impl Triangle {
    /// Returns a new Triangle
    pub fn new(point1: Point, point2: Point, point3: Point) -> Triangle { Triangle { point1, point2, point3 } }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use serde_json::to_string;
    use super::*;

    #[test]
    fn test_new() {
        let result = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(result.point1.eq(&Point::new(0.0, 0.0, 0.0)), true);
        assert_eq!(result.point2.eq(&Point::new(10.0, 0.0, 0.0)), true);
        assert_eq!(result.point3.eq(&Point::new(10.0, -15.0, 0.0)), true);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, -7.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 2.1, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(101.0, -15.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let b = Triangle::new(
            Point::new(0.0, -0.1, 0.0),
            Point::new(-10.0, 0.0, 5.0),
            Point::new(101.0, 76.0, 0.0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"point1\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"point2\":{\"x\":10.0,\"y\":0.0,\"z\":0.0},\"point3\":{\"x\":10.0,\"y\":-15.0,\"z\":0.0}}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"point1\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"point2\":{\"x\":10.0,\"y\":0.0,\"z\":0.0},\"point3\":{\"x\":10.0,\"y\":-15.0,\"z\":0.0}}";
        let actual_result = from_str::<Triangle>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0));
        assert_eq!(expected.eq(&actual), true);
    }
}