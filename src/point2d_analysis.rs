use std::cmp::Ordering;
use crate::point2d::{MonotoneVertexType, Point2D};
use crate::vector2d::Vector2D;

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
    
    /// Gets [MonotoneVertexType] for given vertex, assuming whole polygon is clockwise.
    pub(crate) fn get_monotone_vertex_type_for_clockwise(&self, previous: &Point2D, next: &Point2D) -> MonotoneVertexType {
        let inner_angle = self.get_inner_angle_for_clockwise(previous, next);

        if self.are_both_neighbours_lower(previous, next) {
            if inner_angle < std::f64::consts::PI {
                return MonotoneVertexType::Start;
            }
            if inner_angle > std::f64::consts::PI {
                return MonotoneVertexType::Split;
            }
        }

        if self.are_both_neighbours_higher(previous, next) {
            if inner_angle < std::f64::consts::PI {
                return MonotoneVertexType::End;
            }
            if inner_angle > std::f64::consts::PI {
                return MonotoneVertexType::Merge;
            }
        }

        MonotoneVertexType::Regular
    }

    /// Gets value of the inner angle for the given vertex, assuming that the whole polygon 
    /// is clockwise.
    pub(crate) fn get_inner_angle_for_clockwise(&self, previous: &Point2D, next: &Point2D) -> f64 {
        let a = Vector2D::from_2_points(self, previous);
        let b = Vector2D::from_2_points(self, next);
        let angle = a.get_angle(&b);

        if self.is_clockwise(previous, next) {
            return angle;
        }

        (2.0 * std::f64::consts::PI) - angle
    }

    /// Check if the segment constructed with previous, self and next vertex is clockwise,
    /// used for calculation of inner angle.
    fn is_clockwise(&self, previous: &Point2D, next: &Point2D) -> bool {
        let mut sum = 0.0;

        let x1 = previous.x;
        let y1 = previous.y;
        let x2 = self.x;
        let y2 = self.y;
        sum += (x2-x1) * (y2+y1);

        let x1 = self.x;
        let y1 = self.y;
        let x2 = next.x;
        let y2 = next.y;
        sum += (x2-x1) * (y2+y1);

        let x1 = next.x;
        let y1 = next.y;
        let x2 = previous.x;
        let y2 = previous.y;
        sum += (x2-x1) * (y2+y1);

        sum >= 0.0
    }

    /// Check used for Monotones vertex if the neighbouring vertices are lower than the one
    /// checking
    fn are_both_neighbours_lower(&self, first: &Point2D, second: &Point2D) -> bool {
        first.y < self.y && second.y < self.y
    }

    /// Check used for Monotones vertex if the neighbouring vertices are higher than the one
    /// checking
    fn are_both_neighbours_higher(&self, first: &Point2D, second: &Point2D) -> bool {
        first.y > self.y && second.y > self.y
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
}