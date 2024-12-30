use serde::{Deserialize, Serialize};

/// Represents a three-dimensional point with double-precision floating-point coordinates.
#[derive(Deserialize, Serialize)]
pub struct Point {
    /// The x-coordinate of the point.
    pub x: f64,
    /// The y-coordinate of the point.
    pub y: f64,
    /// The z-coordinate of the point.
    pub z: f64
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Point {
    /// Returns a new Point
    pub fn new(x: f64, y: f64, z: f64) -> Point { Point { x, y, z } }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use serde_json::to_string;
    use super::*;

    #[test]
    fn test_new() {
        let result = Point::new(1.5, -2.3, 3.9);
        assert_eq!(result.x, 1.5);
        assert_eq!(result.y, -2.3);
        assert_eq!(result.z, 3.9);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5, -2.3, 3.9);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.49, -2.3, 3.9);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5, -2.24321, 3.9);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5, -2.3, -4.05);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(-1.5, 5.01, 11.0);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = Point::new(1.5, -2.3, 3.9);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"x\":1.5,\"y\":-2.3,\"z\":3.9}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"x\":1.5,\"y\":-2.3,\"z\":3.9}";
        let actual_result = from_str::<Point>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = Point::new(1.5, -2.3, 3.9);
        assert_eq!(expected.eq(&actual), true);
    }
}