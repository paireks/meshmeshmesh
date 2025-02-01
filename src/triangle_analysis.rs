use crate::point::Point;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::vector::Vector;

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

    /// Gets the normal [Vector] of the [Triangle].
    ///
    /// This output [Vector] will be unitized during the process.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_normal_vector_unitized();
    ///
    /// let expected = Vector::new(0.573586,-0.458635,0.678714);
    ///
    /// assert_eq!(expected.eq_with_tolerance(&actual, 0.00001), true);
    /// ```
    pub fn get_normal_vector_unitized(&self) -> Vector {
        let first_vector = self.get_first_side_as_vector();
        let second_vector = self.get_second_side_as_vector();

        first_vector.get_cross_product(&second_vector).get_unitized()
    }

    /// Gets the normal [Ray] of the [Triangle].
    ///
    /// This [Ray] has an `origin` which is a centroid and `direction` which is a unitized normal.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_normal_ray();
    ///
    /// let expected_origin = Point::new((35.704653 + -38.634947 + -21.698671)/3.0, (37.253023 + 13.199458 + -49.7235)/3.0, (-22.626602 + 23.94433 + -32.888206)/3.0);
    /// let expected_direction = Vector::new(0.573586,-0.458635,0.678714);
    /// let expected = Ray::new(expected_origin, expected_direction);
    ///
    /// assert_eq!(expected.eq_with_tolerance(&actual, 0.00001), true);
    /// ```
    pub fn get_normal_ray(&self) -> Ray {
        let origin = self.get_centroid();
        let direction = self.get_normal_vector_unitized();
        Ray::new(origin, direction)
    }

    /// Gets the first side (AB) of the [Triangle] (ABC) and returns it as an AB [Vector].
    ///
    /// This [Vector] starts at the first [Point] of the [Triangle], and ends at the second [Point] of the [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_first_side_as_vector();
    ///
    /// let expected = Vector::from_2_points(&Point::new(35.704653, 37.253023, -22.626602), &Point::new(-38.634947, 13.199458, 23.94433));
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_first_side_as_vector(&self) -> Vector {
        Vector::from_2_points(&self.first_point, &self.second_point)
    }

    /// Gets the second side (BC) of the [Triangle] (ABC) and returns it as an BC [Vector].
    ///
    /// This [Vector] starts at the second [Point] of the [Triangle], and ends at the third [Point] of the [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_second_side_as_vector();
    ///
    /// let expected = Vector::from_2_points(&Point::new(-38.634947, 13.199458, 23.94433), &Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_second_side_as_vector(&self) -> Vector {
        Vector::from_2_points(&self.second_point, &self.third_point)
    }

    /// Gets the third side (CA) of the [Triangle] (ABC) and returns it as an CA [Vector].
    ///
    /// This [Vector] starts at the third [Point] of the [Triangle], and ends at the first [Point] of the [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_third_side_as_vector();
    ///
    /// let expected = Vector::from_2_points(&Point::new(-21.698671, -49.7235, -32.888206), &Point::new(35.704653, 37.253023, -22.626602));
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn get_third_side_as_vector(&self) -> Vector {
        Vector::from_2_points(&self.third_point, &self.first_point)
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

    #[test]
    pub fn test_get_normal_vector_unitized() {
            let input = Triangle::new(
            Point::new(35.704653, 37.253023, -22.626602),
            Point::new(-38.634947, 13.199458, 23.94433),
            Point::new(-21.698671, -49.7235, -32.888206));

        let actual = input.get_normal_vector_unitized();

        let expected = Vector::new(0.573586,-0.458635,0.678714);

        assert_eq!(expected.eq_with_tolerance(&actual, 0.00001), true);
    }

    #[test]
    pub fn test_get_normal_ray() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let actual = input.get_normal_ray();

        let expected_origin = Point::new((35.704653 + -38.634947 + -21.698671)/3.0, (37.253023 + 13.199458 + -49.7235)/3.0, (-22.626602 + 23.94433 + -32.888206)/3.0);
        let expected_direction = Vector::new(0.573586,-0.458635,0.678714);
        let expected = Ray::new(expected_origin, expected_direction);

        assert_eq!(expected.eq_with_tolerance(&actual, 0.00001), true);
    }

    #[test]
    pub fn test_get_first_side_as_vector() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let actual = input.get_first_side_as_vector();

        let expected = Vector::from_2_points(&Point::new(35.704653, 37.253023, -22.626602), &Point::new(-38.634947, 13.199458, 23.94433));

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    pub fn test_get_second_side_as_vector() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let actual = input.get_second_side_as_vector();

        let expected = Vector::from_2_points(&Point::new(-38.634947, 13.199458, 23.94433), &Point::new(-21.698671, -49.7235, -32.888206));

        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    pub fn test_get_third_side_as_vector() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let actual = input.get_third_side_as_vector();

        let expected = Vector::from_2_points(&Point::new(-21.698671, -49.7235, -32.888206), &Point::new(35.704653, 37.253023, -22.626602));

        assert_eq!(expected.eq(&actual), true);
    }
}

