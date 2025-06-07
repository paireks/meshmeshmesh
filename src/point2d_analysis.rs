use std::cmp::Ordering;
use crate::point2d::Point2D;

impl Point2D {
    /// Compares given [Point2D] to other one, but with a `f64` tolerance.
    ///
    /// If any coordinate absolute difference is > tolerance, then it should return `false`.
    ///
    /// As you can see, it doesn't compare distances between [Point2D]s, but rather coordinates themselves.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Point2D::new(1.5, -2.3);
    /// let b = Point2D::new(1.5 + 0.0005, -2.3 - 0.0005);
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the Y-coordinate absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Point2D::new(1.5, -2.3);
    /// let b = Point2D::new(1.5 + 0.0005, -2.3 - 0.00101);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Point2D, tolerance: f64) -> bool {
        if (self.x - other.x).abs() > tolerance {
            false
        }
        else if (self.y - other.y).abs() > tolerance {
            false
        }
        else {
            true
        }
    }

    /// Gets distance to another [Point2D].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// let a = Point2D::new(35.704653, 37.253023);
    /// let b = Point2D::new(-38.634947, 13.199458);
    /// let actual_ab = a.get_distance_to_point(&b);
    /// let actual_ba = b.get_distance_to_point(&a);
    ///
    /// let expected = 78.13418;
    ///
    /// assert_eq!(((expected - actual_ab).abs() < 0.00001), true); // Both distances should be the same
    /// assert_eq!(((expected - actual_ba).abs() < 0.00001), true);
    /// ```
    pub fn get_distance_to_point(&self, other:&Point2D) -> f64 {
        f64::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2))
    }

    /// Creates [Ordering] for [Point2D]s from top to bottom, and then from left to right.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::cmp::Ordering;
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let a = Point2D::new(0.436, 2.515); // should be second
    /// let b = Point2D::new(0.631, 3.715); // should be first (cause Y1 > Y0)
    ///
    /// let actual = a.total_cmp_top_bottom_then_left_right(&b);
    ///
    /// assert_eq!(actual, Ordering::Greater)
    /// ```
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let a = Point2D::new(0.436, 3.715); // should be first (cause Y1 == Y0, but X1 < X0)
    /// let b = Point2D::new(0.631, 3.715); // should be second
    ///
    /// let actual = a.total_cmp_top_bottom_then_left_right(&b);
    ///
    /// assert_eq!(actual, Ordering::Less)
    /// ```
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let a = Point2D::new(0.436, 3.715); // should be equal
    /// let b = Point2D::new(0.436, 3.715); // should be equal
    ///
    /// let actual = a.total_cmp_top_bottom_then_left_right(&b);
    ///
    /// assert_eq!(actual, Ordering::Equal)
    /// ```
    pub fn total_cmp_top_bottom_then_left_right(&self, other: &Self) -> Ordering {
        (-self.y).total_cmp(&(-other.y)).then(self.x.total_cmp(&other.x))
    }

    /// Creates [Ordering] for [Point2D]s from top to bottom, and then from left to right.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let a = Point2D::new(0.436, 2.515); // should be first (cause Y1 > Y0)
    /// let b = Point2D::new(0.631, 3.715); // should be second
    ///
    /// let actual = a.total_cmp_bottom_top_then_right_left(&b);
    ///
    /// assert_eq!(actual, Ordering::Less)
    /// ```
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let a = Point2D::new(0.436, 3.715); // should be second
    /// let b = Point2D::new(0.631, 3.715); // should be first (cause Y1 == Y0, but X1 < X0)
    ///
    /// let actual = a.total_cmp_bottom_top_then_right_left(&b);
    ///
    /// assert_eq!(actual, Ordering::Greater)
    /// ```
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use meshmeshmesh::point2d::Point2D;
    ///
    /// let a = Point2D::new(0.436, 3.715); // should be equal
    /// let b = Point2D::new(0.436, 3.715); // should be equal
    ///
    /// let actual = a.total_cmp_bottom_top_then_right_left(&b);
    ///
    /// assert_eq!(actual, Ordering::Equal)
    /// ```
    pub fn total_cmp_bottom_top_then_right_left(&self, other: &Self) -> Ordering {
        self.y.total_cmp(&other.y).then((-self.x).total_cmp(&(-other.x)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_with_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = Point2D::new(1.5, -2.3);
        let b = Point2D::new(1.5 + 0.0005, -2.3 - 0.0005);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_with_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = Point2D::new(1.5, -2.3);
        let b = Point2D::new(1.5 + 0.0011, -2.3 - 0.0005);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = Point2D::new(1.5, -2.3);
        let b = Point2D::new(1.5 + 0.0005, -2.3 - 0.00101);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_xy_false(){
        let tolerance: f64 = 0.001;
        let a = Point2D::new(1.5, -2.3);
        let b = Point2D::new(1.5 + 0.0011, -2.3 - 0.00101);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_get_distance_to_point() {
        let a = Point2D::new(35.704653, 37.253023);
        let b = Point2D::new(-38.634947, 13.199458);
        let actual_ab = a.get_distance_to_point(&b);
        let actual_ba = b.get_distance_to_point(&a);

        let expected = 78.13418;

        assert_eq!(((expected - actual_ab).abs() < 0.00001), true); // Both distances should be the same
        assert_eq!(((expected - actual_ba).abs() < 0.00001), true);
    }

    #[test]
    fn test_total_cmp_top_bottom_then_left_right_greater_y() {
        let a = Point2D::new(0.436, 2.515);
        let b = Point2D::new(0.631, 3.715);
        
        let actual = a.total_cmp_top_bottom_then_left_right(&b);
        
        assert_eq!(actual, Ordering::Greater)
    }

    #[test]
    fn test_total_cmp_top_bottom_then_left_right_less_y() {
        let a = Point2D::new(0.436, 3.715);
        let b = Point2D::new(0.631, 2.515);

        let actual = a.total_cmp_top_bottom_then_left_right(&b);

        assert_eq!(actual, Ordering::Less)
    }

    #[test]
    fn test_total_cmp_top_bottom_then_left_right_greater_x() {
        let a = Point2D::new(0.631, 3.715);
        let b = Point2D::new(0.436, 3.715);
        
        let actual = a.total_cmp_top_bottom_then_left_right(&b);
        
        assert_eq!(actual, Ordering::Greater)
    }

    #[test]
    fn test_total_cmp_top_bottom_then_left_right_less_x() {
        let a = Point2D::new(0.436, 3.715);
        let b = Point2D::new(0.631, 3.715);

        let actual = a.total_cmp_top_bottom_then_left_right(&b);

        assert_eq!(actual, Ordering::Less)
    }

    #[test]
    fn test_total_cmp_top_bottom_then_left_right_equal() {
        let a = Point2D::new(0.436, 3.715);
        let b = Point2D::new(0.436, 3.715);
        
        let actual = a.total_cmp_top_bottom_then_left_right(&b);
        
        assert_eq!(actual, Ordering::Equal)
    }

    #[test]
    fn test_total_cmp_bottom_top_then_right_left_greater_y() {
        let a = Point2D::new(0.436, 2.515);
        let b = Point2D::new(0.631, 3.715);

        let actual = a.total_cmp_bottom_top_then_right_left(&b);

        assert_eq!(actual, Ordering::Less)
    }

    #[test]
    fn test_total_cmp_bottom_top_then_right_left_less_y() {
        let a = Point2D::new(0.436, 3.715);
        let b = Point2D::new(0.631, 2.515);

        let actual = a.total_cmp_bottom_top_then_right_left(&b);

        assert_eq!(actual, Ordering::Greater)
    }

    #[test]
    fn test_total_cmp_bottom_top_then_right_left_greater_x() {
        let a = Point2D::new(0.631, 3.715);
        let b = Point2D::new(0.436, 3.715);

        let actual = a.total_cmp_bottom_top_then_right_left(&b);

        assert_eq!(actual, Ordering::Less)
    }

    #[test]
    fn test_total_cmp_bottom_top_then_right_left_less_x() {
        let a = Point2D::new(0.436, 3.715);
        let b = Point2D::new(0.631, 3.715);

        let actual = a.total_cmp_bottom_top_then_right_left(&b);

        assert_eq!(actual, Ordering::Greater)
    }

    #[test]
    fn test_total_cmp_bottom_top_then_right_left_equal() {
        let a = Point2D::new(0.436, 3.715);
        let b = Point2D::new(0.436, 3.715);

        let actual = a.total_cmp_bottom_top_then_right_left(&b);

        assert_eq!(actual, Ordering::Equal)
    }
}