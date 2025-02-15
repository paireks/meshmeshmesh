
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