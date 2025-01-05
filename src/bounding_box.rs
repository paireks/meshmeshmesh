use serde::{Deserialize, Serialize};

/// Represents a three-dimensional bounding box (AABB: axis-aligned bounding box).
///
/// # Example
///
/// ```
/// use meshmeshmesh::bounding_box::BoundingBox;
///
/// let result = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
/// assert_eq!(result.min_x, 1.5);
/// assert_eq!(result.max_x, 1.65);
/// assert_eq!(result.min_y, -2.3);
/// assert_eq!(result.max_y, 0.7);
/// assert_eq!(result.min_z, 3.9);
/// assert_eq!(result.max_z, 4.1);
/// ```
#[derive(Deserialize, Serialize)]
pub struct BoundingBox {
    /// Minimum x value.
    pub min_x: f64,
    /// Maximum x value.
    pub max_x: f64,
    /// Minimum y value.
    pub min_y: f64,
    /// Maximum y value.
    pub max_y: f64,
    /// Minimum z value.
    pub min_z: f64,
    /// Maximum z value.
    pub max_z: f64,
}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.min_x == other.min_x && self.max_x == other.max_x &&
        self.min_y == other.min_y && self.max_y == other.max_y &&
        self.min_z == other.min_z && self.max_z == other.max_z
    }
}

impl BoundingBox {
    /// Creates a new [Bounding Box](BoundingBox)
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::bounding_box::BoundingBox;
    ///
    /// let result = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
    /// assert_eq!(result.min_x, 1.5);
    /// assert_eq!(result.max_x, 1.65);
    /// assert_eq!(result.min_y, -2.3);
    /// assert_eq!(result.max_y, 0.7);
    /// assert_eq!(result.min_z, 3.9);
    /// assert_eq!(result.max_z, 4.1);
    /// ```
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64, min_z: f64, max_z: f64) -> BoundingBox {
        if min_x > max_x {
            panic!("Invalid BoundingBox (min x > max_x)");
        }
        if min_y > max_y {
            panic!("Invalid BoundingBox (min y > max_y)");
        }
        if min_z > max_z {
            panic!("Invalid BoundingBox (min z > max_z)");
        }

        BoundingBox {min_x, max_x, min_y, max_y, min_z, max_z}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use serde_json::to_string;
    use super::*;

    #[test]
    fn test_new() {
        let result = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
        assert_eq!(result.min_x, 1.5);
        assert_eq!(result.max_x, 1.65);
        assert_eq!(result.min_y, -2.3);
        assert_eq!(result.max_y, 0.7);
        assert_eq!(result.min_z, 3.9);
        assert_eq!(result.max_z, 4.1);
    }

    #[test]
    fn test_new_empty_y() {
        let result = BoundingBox::new(1.5, 1.65, -2.3, -2.3, 3.9, 4.1);
        assert_eq!(result.min_x, 1.5);
        assert_eq!(result.max_x, 1.65);
        assert_eq!(result.min_y, -2.3);
        assert_eq!(result.max_y, -2.3);
        assert_eq!(result.min_z, 3.9);
        assert_eq!(result.max_z, 4.1);
    }

    #[test]
    #[should_panic(expected = "Invalid BoundingBox (min x > max_x)")]
    fn test_new_wrong_x_should_panic() {
        BoundingBox::new(1.5, 1.45, -2.3, 0.7, 3.9, 4.1);
    }

    #[test]
    #[should_panic(expected = "Invalid BoundingBox (min y > max_y)")]
    fn test_new_wrong_y_should_panic() {
        BoundingBox::new(1.5, 1.65, -2.3, -2.30001, 3.9, 4.1);
    }

    #[test]
    #[should_panic(expected = "Invalid BoundingBox (min z > max_z)")]
    fn test_new_wrong_z_should_panic() {
        BoundingBox::new(1.5, 1.65, -2.3, 0.7, 4.101, 4.1);
    }

    #[test]
    fn test_partialeq_true() {
        let a = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
        let b = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_x_different_false() {
        let a = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
        let b = BoundingBox::new(1.5, 1.66, -2.3, 0.7, 3.9, 4.1);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_y_different_false() {
        let a = BoundingBox::new(1.5, 1.65, -2.3, 0.71, 3.9, 4.1);
        let b = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_z_different_false() {
        let a = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.12);
        let b = BoundingBox::new(1.5, 1.65, -2.3, 0.7, 3.9, 4.1);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = BoundingBox::new(1.51, 1.65, -2.31, 0.7, 3.91, 4.1);
        let b = BoundingBox::new(1.5, 1.651, -2.3, 0.71, 3.9, 4.12);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = BoundingBox::new(1.5, 1.65, -2.3, -2.3, 3.9, 4.1);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"min_x\":1.5,\"max_x\":1.65,\"min_y\":-2.3,\"max_y\":-2.3,\"min_z\":3.9,\"max_z\":4.1}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"min_x\":1.5,\"max_x\":1.65,\"min_y\":-2.3,\"max_y\":-2.3,\"min_z\":3.9,\"max_z\":4.1}";
        let actual_result = from_str::<BoundingBox>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = BoundingBox::new(1.5, 1.65, -2.3, -2.3, 3.9, 4.1);
        assert_eq!(expected.eq(&actual), true);
    }
}
