use crate::point::Point;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::vector::Vector;

impl Ray {

    /// Compares given [Ray] to other one, but with a `f64` tolerance.
    ///
    /// If any value absolute difference is > tolerance, then it should return `false`.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
    /// let b = Ray::new(Point::new(0.0, 1.0 + 0.001, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the Y-coordinate absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
    /// let b = Ray::new(Point::new(0.0, 1.0 + 0.0011, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Ray, tolerance: f64) -> bool {
        if !self.origin.eq_with_tolerance(&other.origin, tolerance) {
            return false;
        }

        if !self.direction.eq_with_tolerance(&other.direction, tolerance) {
            return false;
        }

        true
    }

    /// Creates a [Point] which is located on [Ray] with the given `distance` from the Ray's `origin`.
    ///
    /// Negative value of distance is also accepted, will create a [Point] in the reversed direction.
    ///
    /// # Example
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    /// let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.660831,0.569323,0.489054));
    /// let distance = 5.0;
    /// let expected = Point::new(4.304155, 4.846615, 5.445269);
    /// let actual = ray.get_point_at(distance);
    ///
    /// assert_eq!(expected.eq_with_tolerance(&actual, 0.001), true);
    /// ```
    pub fn get_point_at(&self, distance:f64) -> Point {

        let move_vector = self.direction * distance;

        self.origin + move_vector
    }

    /// Calculates intersection of the [Ray] with given [Triangle] using Möller–Trumbore intersection algorithm.
    ///
    /// It uses `epsilon` value for check if the [Ray] is parallel to [Triangle].
    ///
    /// # Examples
    ///
    /// Here below there is an example of hitting the Triangle with the Ray
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::triangle::Triangle;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let triangle = Triangle::new(Point::new(18.106339, 26.580607, 7.381013), Point::new(27.733604, 26.580607, 28.757986), Point::new(24.296286, -0.019341, 19.121015));
    /// let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.660831,0.569323,0.489054));
    ///
    /// let expected = Point::new(23.94358, 21.766485, 19.979597);
    /// let actual = ray.get_intersection_with_triangle(&triangle).unwrap();
    ///
    /// assert_eq!(expected.eq_with_tolerance(&actual, 0.001), true);
    /// ```
    ///
    /// Below is an example of Ray that misses the Triangle
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::triangle::Triangle;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let triangle = Triangle::new(Point::new(18.106339, 26.580607, 7.381013), Point::new(27.733604, 26.580607, 28.757986), Point::new(24.296286, -0.019341, 19.121015));
    /// let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.590527,0.508754,0.626457));
    ///
    /// let actual_option = ray.get_intersection_with_triangle(&triangle);
    ///
    /// assert_eq!(actual_option.is_none(), true);
    /// ```
    pub fn get_intersection_with_triangle(&self, triangle:&Triangle) -> Option<Point> {
        let ab = triangle.get_first_side_as_vector();
        let ac = triangle.get_third_side_as_vector().get_reversed();

        let direction_ac_cross_product = self.direction.get_cross_product(&ac);
        let det = ab.get_dot_product(&direction_ac_cross_product);

        if det.abs() < f64::EPSILON {
            return None; // That means this Ray is parallel to this Triangle.
        }

        let inverted_det = 1.0 / det;

        let vector_t = Vector::from_2_points(&triangle.first_point, &self.origin);
        let u = vector_t.get_dot_product(&direction_ac_cross_product) * inverted_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let vector_q = vector_t.get_cross_product(&ab);
        let v = self.direction.get_dot_product(&vector_q) * inverted_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let distance_to_intersection = ac.get_dot_product(&vector_q) * inverted_det;

        if distance_to_intersection > 0.0 {
            return Some(self.get_point_at(distance_to_intersection))
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vector::Vector;
    use super::*;

    #[test]
    pub fn test_eq_with_tolerance_true() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.001, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    pub fn test_eq_with_tolerance_different_origin_false() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.0011, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0009));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    pub fn test_eq_with_tolerance_different_direction_false() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.001, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0011));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    pub fn test_eq_with_tolerance_different_all_false() {
        let tolerance: f64 = 0.001;
        let a = Ray::new(Point::new(0.0, 1.0, -2.5), Vector::new(1.0, 0.0, 0.0));
        let b = Ray::new(Point::new(0.0, 1.0 + 0.0011, -2.5), Vector::new(1.0, 0.0, 0.0 + 0.0011));

        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    pub fn test_get_point_at() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.660831,0.569323,0.489054));
        let distance = 5.0;
        let expected = Point::new(4.304155, 4.846615, 5.445269);
        let actual = ray.get_point_at(distance);

        assert_eq!(expected.eq_with_tolerance(&actual, 0.001), true);
    }

    #[test]
    pub fn test_get_intersection_with_triangle_hit() {
        let triangle = Triangle::new(Point::new(18.106339, 26.580607, 7.381013), Point::new(27.733604, 26.580607, 28.757986), Point::new(24.296286, -0.019341, 19.121015));
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.660831,0.569323,0.489054));

        let expected = Point::new(23.94358, 21.766485, 19.979597);
        let actual = ray.get_intersection_with_triangle(&triangle).unwrap();

        assert_eq!(expected.eq_with_tolerance(&actual, 0.001), true);
    }

    #[test]
    pub fn test_get_intersection_with_triangle_miss() {
        let triangle = Triangle::new(Point::new(18.106339, 26.580607, 7.381013), Point::new(27.733604, 26.580607, 28.757986), Point::new(24.296286, -0.019341, 19.121015));
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.590527,0.508754,0.626457));

        let actual_option = ray.get_intersection_with_triangle(&triangle);

        assert_eq!(actual_option.is_none(), true);
    }
}