use crate::point::Point;
use crate::triangle::Triangle;

impl Triangle {
    
    /// Creates a new [Triangle], but flipped one.
    /// 
    /// In other words: the normal Vector of the new created one is reversed.
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602), // A
    /// Point::new(-38.634947, 13.199458, 23.94433), // B
    /// Point::new(-21.698671, -49.7235, -32.888206)); // C
    ///
    /// let expected = Triangle::new(
    /// Point::new(-21.698671, -49.7235, -32.888206), // C
    /// Point::new(-38.634947, 13.199458, 23.94433), // B
    /// Point::new(35.704653, 37.253023, -22.626602)); // A
    /// 
    /// let actual = input.get_flipped();
    /// let expected_normal = input.get_normal_vector_unitized().get_reversed();
    /// let actual_normal = actual.get_normal_vector_unitized();
    ///
    /// assert_eq!(expected, actual);
    /// assert_eq!(expected_normal.eq_with_tolerance(&actual_normal, 0.001), true);
    /// ```
    pub fn get_flipped(&self) -> Triangle {
        Triangle::new(self.third_point, self.second_point, self.first_point)
    }
    
    /// Divides the [Triangle] into 3 new ones. It uses given [Point] to divide it.
    ///
    /// Expected behaviour is to have a `division_point` somewhere inside the original [Triangle].
    ///
    /// If the original [Triangle] is ABC, and the division [Point] is P, then returned is:
    /// 1. BCP
    /// 2. CAP
    /// 3. ABP
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602), // A
    /// Point::new(-38.634947, 13.199458, 23.94433), // B
    /// Point::new(-21.698671, -49.7235, -32.888206)); // C
    ///
    /// let division_point = Point::new(1.922108, 12.641649, -10.707643);
    ///
    /// let expected_bcp = Triangle::new(
    /// Point::new(-38.634947, 13.199458, 23.94433), // B
    /// Point::new(-21.698671, -49.7235, -32.888206), // C
    /// Point::new(1.922108, 12.641649, -10.707643)); // P
    ///
    /// let expected_cap = Triangle::new(
    /// Point::new(-21.698671, -49.7235, -32.888206), // C
    /// Point::new(35.704653, 37.253023, -22.626602), // A
    /// Point::new(1.922108, 12.641649, -10.707643)); // P
    ///
    /// let expected_abp = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602), // A
    /// Point::new(-38.634947, 13.199458, 23.94433), // B
    /// Point::new(1.922108, 12.641649, -10.707643)); // P
    ///
    /// let actual = input.get_divided_by_point(division_point);
    ///
    /// assert_eq!(expected_bcp.eq(&actual.0), true);
    /// assert_eq!(expected_cap.eq(&actual.1), true);
    /// assert_eq!(expected_abp.eq(&actual.2), true);
    /// ```
    pub fn get_divided_by_point(&self, division_point: Point) -> (Triangle, Triangle, Triangle) {
        (Triangle::new(self.second_point, self.third_point, division_point),
         Triangle::new(self.third_point, self.first_point, division_point),
         Triangle::new(self.first_point, self.second_point, division_point))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_flipped() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602), // A
        Point::new(-38.634947, 13.199458, 23.94433), // B
        Point::new(-21.698671, -49.7235, -32.888206)); // C
        
        let expected = Triangle::new(
        Point::new(-21.698671, -49.7235, -32.888206), // C
        Point::new(-38.634947, 13.199458, 23.94433), // B
        Point::new(35.704653, 37.253023, -22.626602)); // A
        
        let actual = input.get_flipped();
        let expected_normal = input.get_normal_vector_unitized().get_reversed();
        let actual_normal = actual.get_normal_vector_unitized();
        
        assert_eq!(expected, actual);
        assert_eq!(expected_normal.eq_with_tolerance(&actual_normal, 0.001), true);
    }
    
    #[test]
    fn test_get_divided_by_point() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602), // A
        Point::new(-38.634947, 13.199458, 23.94433), // B
        Point::new(-21.698671, -49.7235, -32.888206)); // C

        let division_point = Point::new(1.922108, 12.641649, -10.707643);

        let expected_bcp = Triangle::new(
        Point::new(-38.634947, 13.199458, 23.94433), // B
        Point::new(-21.698671, -49.7235, -32.888206), // C
        Point::new(1.922108, 12.641649, -10.707643)); // P

        let expected_cap = Triangle::new(
        Point::new(-21.698671, -49.7235, -32.888206), // C
        Point::new(35.704653, 37.253023, -22.626602), // A
        Point::new(1.922108, 12.641649, -10.707643)); // P

        let expected_abp = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602), // A
        Point::new(-38.634947, 13.199458, 23.94433), // B
        Point::new(1.922108, 12.641649, -10.707643)); // P

        let actual = input.get_divided_by_point(division_point);

        assert_eq!(expected_bcp.eq(&actual.0), true);
        assert_eq!(expected_cap.eq(&actual.1), true);
        assert_eq!(expected_abp.eq(&actual.2), true);
    }
}