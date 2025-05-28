use std::cmp::Ordering;
use crate::point2d::Point2D;

impl Point2D {

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