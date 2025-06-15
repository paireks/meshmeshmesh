use serde::{Deserialize, Serialize};

/// Represents a color using red, green, blue, and alpha (transparency) values.
#[derive(Deserialize, Serialize)]
pub struct Color {
    /// The red component of the color.
    pub r: i32,
    /// The green component of the color.
    pub g: i32,
    /// The blue component of the color.
    pub b: i32,
    /// The alpha (transparency) component of the color.
    pub a: i32,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl Color {
    /// Returns a new Color
    pub fn new(r: i32, g: i32, b: i32, a: i32) -> Color { Color { r, g, b, a } }
}

#[cfg(test)]
mod tests {
    use serde_json::to_string;
    use serde_json::from_str;
    use super::*;

    #[test]
    fn test_new() {
        let result = Color::new(11, 22, 33, 44);
        assert_eq!(result.r, 11);
        assert_eq!(result.g, 22);
        assert_eq!(result.b, 33);
        assert_eq!(result.a, 44);
    }

    #[test]
    fn test_partialeq_true() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 22, 33, 44);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(10, 22, 33, 44);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 9, 33, 44);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 22, 5, 44);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_fourth_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(11, 22, 33, 43);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Color::new(11, 22, 33, 44);
        let b = Color::new(10, 201, 35, 43);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = Color::new(11, 22, 33, 44);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"r\":11,\"g\":22,\"b\":33,\"a\":44}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"r\":11,\"g\":22,\"b\":33,\"a\":44}";
        let actual_result = from_str::<Color>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = Color::new(11, 22, 33, 44);
        assert_eq!(expected.eq(&actual), true);
    }
}