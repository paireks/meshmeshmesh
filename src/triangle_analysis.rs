use crate::point::Point;
use crate::triangle::Triangle;

impl Triangle {

    /// Calculates area of given [Triangle] using Heron's formula.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_area();
    ///
    /// let expected = 3746.086182;
    ///
    /// assert_eq!(((expected - actual).abs() < 0.00001), true);
    /// ```
    pub fn get_area(&self) -> f64 {
        let a = self.first_point.get_distance_to_point(&self.second_point);
        let b = self.second_point.get_distance_to_point(&self.third_point);
        let c = self.third_point.get_distance_to_point(&self.first_point);

        let s = (a + b + c) / 2.0;

        f64::sqrt(s * (s - a) * (s - b) * (s - c))
    }

    /// Gets the centroid of the [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_centroid();
    ///
    /// let expected = Point::new((35.704653 + -38.634947 + -21.698671)/3.0, (37.253023 + 13.199458 + -49.7235)/3.0, (-22.626602 + 23.94433 + -32.888206)/3.0);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_centroid(&self) -> Point {
        Point::new((self.first_point.x + self.second_point.x + self.third_point.x) / 3.0, (self.first_point.y + self.second_point.y + self.third_point.y) / 3.0, (self.first_point.z + self.second_point.z + self.third_point.z) / 3.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;
    #[test]
    pub fn test_get_area() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let actual = input.get_area();

        let expected = 3746.086182;

        assert_eq!(((expected - actual).abs() < 0.00001), true); // Both distances should be the same
    }

    #[test]
    pub fn test_get_area_zero() {
        let input = Triangle::new(
            Point::new(35.704653, 37.253023, -22.626602),
            Point::new(35.704653, 37.253023, -22.626602),
            Point::new(35.704653, 37.253023, -22.626602));

        let actual = input.get_area();

        let expected = 0.0;

        assert_eq!(expected, actual); // Both distances should be the same
    }

    #[test]
    pub fn test_get_centroid() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let actual = input.get_centroid();

        let expected = Point::new((35.704653 + -38.634947 + -21.698671)/3.0, (37.253023 + 13.199458 + -49.7235)/3.0, (-22.626602 + 23.94433 + -32.888206)/3.0);

        assert_eq!(expected.eq(&actual), true);
    }
}

