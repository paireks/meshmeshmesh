
/// Represents a two-dimensional bounding area - (2d version of AABB: axis-aligned bounding box).
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
/// ```
#[derive(Debug)]
pub struct BoundingArea {
    /// Minimum x value.
    pub min_x: f64,
    /// Maximum x value.
    pub max_x: f64,
    /// Minimum y value.
    pub min_y: f64,
    /// Maximum y value.
    pub max_y: f64,
}

impl PartialEq for BoundingArea {
    fn eq(&self, other: &Self) -> bool {
        self.min_x == other.min_x && self.max_x == other.max_x &&
            self.min_y == other.min_y && self.max_y == other.max_y
    }
}

impl BoundingArea {
    /// Creates a new [Bounding Area](BoundingArea)
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
    /// ```
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> BoundingArea {
        if min_x > max_x {
            panic!("Invalid BoundingArea (min x > max_x)");
        }
        if min_y > max_y {
            panic!("Invalid BoundingArea (min y > max_y)");
        }

        BoundingArea {min_x, max_x, min_y, max_y}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = BoundingArea::new(1.5, 1.65, -2.3, 0.7);
        assert_eq!(result.min_x, 1.5);
        assert_eq!(result.max_x, 1.65);
        assert_eq!(result.min_y, -2.3);
        assert_eq!(result.max_y, 0.7);
    }

    #[test]
    fn test_new_empty_y() {
        let result = BoundingArea::new(1.5, 1.65, -2.3, -2.3);
        assert_eq!(result.min_x, 1.5);
        assert_eq!(result.max_x, 1.65);
        assert_eq!(result.min_y, -2.3);
        assert_eq!(result.max_y, -2.3);
    }

    #[test]
    #[should_panic(expected = "Invalid BoundingArea (min x > max_x)")]
    fn test_new_wrong_x_should_panic() {
        BoundingArea::new(1.5, 1.45, -2.3, 0.7);
    }

    #[test]
    #[should_panic(expected = "Invalid BoundingArea (min y > max_y)")]
    fn test_new_wrong_y_should_panic() {
        BoundingArea::new(1.5, 1.65, -2.3, -2.30001);
    }

    #[test]
    fn test_partialeq_true() {
        let a = BoundingArea::new(1.5, 1.65, -2.3, 0.7);
        let b = BoundingArea::new(1.5, 1.65, -2.3, 0.7);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_x_different_false() {
        let a = BoundingArea::new(1.5, 1.65, -2.3, 0.7);
        let b = BoundingArea::new(1.5, 1.66, -2.3, 0.7);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_y_different_false() {
        let a = BoundingArea::new(1.5, 1.65, -2.3, 0.71);
        let b = BoundingArea::new(1.5, 1.65, -2.3, 0.7);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = BoundingArea::new(1.51, 1.65, -2.31, 0.7);
        let b = BoundingArea::new(1.5, 1.651, -2.3, 0.71);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}
