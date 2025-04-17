use crate::face_neighbours::FaceNeighbours;
use crate::point::Point;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::vector::Vector;

impl Triangle {

    /// Check is the [Triangle] is parallel to given [Vector].
    ///
    /// It uses `epsilon` for the check.
    ///
    /// # Examples
    ///
    /// In this example there is a [Triangle] that is not parallel, so `false` is returned.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input_triangle = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let input_vector = Vector::new(3.456, 0.9831, -9.761);
    ///
    /// assert_eq!(input_triangle.is_parallel_to_vector_with_epsilon(&input_vector), false);
    /// ```
    ///
    /// In this example there is a [Triangle] that is parallel, so `true` is returned.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input_triangle = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let input_vector = Vector::new(-45.637938,19.4346965,51.701734);
    ///
    /// assert_eq!(input_triangle.is_parallel_to_vector_with_epsilon(&input_vector), true);
    /// ```
    pub fn is_parallel_to_vector_with_epsilon(&self, vector: &Vector) -> bool{

        self.get_normal_vector_unitized().is_perpendicular_to_vector_with_epsilon(vector)
    }

    /// Check is the [Triangle] is parallel to given [Ray].
    ///
    /// It uses `epsilon` for the check.
    ///
    /// # Examples
    ///
    /// In this example there is a [Triangle] that is not parallel, so `false` is returned.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input_triangle = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let input_ray = Ray::new(Point::new(1.000, -2.000, 3.000), Vector::new(3.456, 0.9831, -9.761));
    ///
    /// assert_eq!(input_triangle.is_parallel_to_ray_with_epsilon(&input_ray), false);
    /// ```
    ///
    /// In this example there is a [Triangle] that is parallel, so `true` is returned.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input_triangle = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let input_ray = Ray::new(Point::new(1.000, -2.000, 3.000), Vector::new(-45.637938,19.4346965,51.701734));
    ///
    /// assert_eq!(input_triangle.is_parallel_to_ray_with_epsilon(&input_ray), true);
    /// ```
    pub fn is_parallel_to_ray_with_epsilon(&self, ray: &Ray) -> bool{

        self.is_parallel_to_vector_with_epsilon(&ray.direction)
    }

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

    /// Gets the middle of the first side of the [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(0.0, 0.0, 1.0),
    /// Point::new(10.0, 0.0, 1.0),
    /// Point::new(10.0, -15.0, 1.0));
    ///
    /// let actual = input.get_first_side_middle();
    /// let expected = Point::new(5.0, 0.0, 1.0);
    ///
    /// assert!(expected.eq(&actual))
    /// ```
    pub fn get_first_side_middle(&self) -> Point {
        self.first_point.get_middle_to(&self.second_point)
    }

    /// Gets the middle of the second side of the [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(0.0, 0.0, 1.0),
    /// Point::new(10.0, 0.0, 1.0),
    /// Point::new(10.0, -15.0, 1.0));
    ///
    /// let actual = input.get_second_side_middle();
    /// let expected = Point::new(10.0, -7.5, 1.0);
    ///
    /// assert!(expected.eq(&actual))
    /// ```
    pub fn get_second_side_middle(&self) -> Point {
        self.second_point.get_middle_to(&self.third_point)
    }

    /// Gets the middle of the third side of the [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Triangle::new(
    /// Point::new(0.0, 0.0, 1.0),
    /// Point::new(10.0, 0.0, 1.0),
    /// Point::new(10.0, -15.0, 1.0));
    ///
    /// let actual = input.get_third_side_middle();
    /// let expected = Point::new(5.0, -7.5, 1.0);
    ///
    /// assert!(expected.eq(&actual))
    /// ```
    pub fn get_third_side_middle(&self) -> Point {
        self.third_point.get_middle_to(&self.first_point)
    }

    /// Gets [Vector] from the middle of the first side to the centroid of this [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_vector_from_first_side_middle_to_centroid();
    /// let expected = Vector::new(-6.744508,-24.983247,-11.182357);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001))
    /// ```
    pub fn get_vector_from_first_side_middle_to_centroid(&self) -> Vector {
        Vector::from_2_points(&self.get_first_side_middle(), &self.get_centroid())
    }

    /// Gets [Vector] from the middle of the second side to the centroid of this [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_vector_from_second_side_middle_to_centroid();
    /// let expected = Vector::new(21.957154,18.505015,-6.051555);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001))
    /// ```
    pub fn get_vector_from_second_side_middle_to_centroid(&self) -> Vector {
        Vector::from_2_points(&self.get_second_side_middle(), &self.get_centroid())
    }

    /// Gets [Vector] from the middle of the third side to the centroid of this [Triangle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    ///
    /// let actual = input.get_vector_from_third_side_middle_to_centroid();
    /// let expected = Vector::new(-15.212646,6.478232,17.233911);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.001))
    /// ```
    pub fn get_vector_from_third_side_middle_to_centroid(&self) -> Vector {
        Vector::from_2_points(&self.get_third_side_middle(), &self.get_centroid())
    }

    /// Calculates an angle between [Triangle]s' normals.
    ///
    /// Self [Triangle] is the first one (a), and another one is the second one (b).
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let a = Triangle::new(
    /// Point::new(35.704653, 37.253023, -22.626602),
    /// Point::new(-38.634947, 13.199458, 23.94433),
    /// Point::new(-21.698671, -49.7235, -32.888206));
    /// let b = Triangle::new(
    /// Point::new(51.3242, 19.2342, 28.461254),
    /// Point::new(-21.698671, -49.7235, -32.888206),
    /// Point::new(-38.634947, 13.199458, 23.94433));
    ///
    /// let actual = a.get_normals_angle(&b);
    ///
    /// assert!((actual - 2.524541).abs() < 0.00001);
    /// ```
    pub fn get_normals_angle(&self, another: &Triangle) -> f64 {

        let first_vector = self.get_normal_vector_unitized();
        let second_vector = another.get_normal_vector_unitized();

        first_vector.get_angle(&second_vector)
    }

    
/*    pub fn get_all_normal_angles_between(all_faces: &Vec<Triangle>, all_face_neighbours: &Vec<FaceNeighbours>) -> Vec<[Option<f64>; 3]> {
        let faces_length = all_faces.len();
        if faces_length != all_face_neighbours.len() { 
            panic!("The input of the get_all_angles_between (for both Faces and FaceNeighbours) should be the same length.")
        }
        let mut angles = vec![[None, None, None]; faces_length];

        for i in 0..faces_length {
            let current_triangle = all_faces[i];
            let current_neighbours = all_face_neighbours[i];

            if current_neighbours.first.is_some() {
                let neighbour_triangle = all_faces[current_neighbours.first.unwrap()];
                angles[i][0] = Some(current_triangle.get_normals_angle(&neighbour_triangle));
            }
            
            if current_neighbours.second.is_some() {
                let neighbour_triangle = all_faces[current_neighbours.second.unwrap()];
                angles[i][1] = Some(current_triangle.get_normals_angle(&neighbour_triangle));
            }

            if current_neighbours.third.is_some() {
                let neighbour_triangle = all_faces[current_neighbours.third.unwrap()];
                angles[i][2] = Some(current_triangle.get_normals_angle(&neighbour_triangle));
            }
        }

        angles
    }*/
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    pub fn test_is_parallel_to_vector_with_epsilon_false() {
        let input_triangle = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let input_vector = Vector::new(3.456, 0.9831, -9.761);

        assert_eq!(input_triangle.is_parallel_to_vector_with_epsilon(&input_vector), false);
    }

    #[test]
    pub fn test_is_parallel_to_vector_with_epsilon_true() {
        let input_triangle = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let input_vector = Vector::new(-45.637938,19.4346965,51.701734);

        assert_eq!(input_triangle.is_parallel_to_vector_with_epsilon(&input_vector), true);
    }

    #[test]
    pub fn test_is_parallel_to_ray_with_epsilon_false() {
        let input_triangle = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let input_ray = Ray::new(Point::new(1.000, -2.000, 3.000), Vector::new(3.456, 0.9831, -9.761));

        assert_eq!(input_triangle.is_parallel_to_ray_with_epsilon(&input_ray), false);
    }

    #[test]
    pub fn test_is_parallel_to_ray_with_epsilon_true() {
        let input_triangle = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));

        let input_ray = Ray::new(Point::new(1.000, -2.000, 3.000), Vector::new(-45.637938,19.4346965,51.701734));

        assert_eq!(input_triangle.is_parallel_to_ray_with_epsilon(&input_ray), true);
    }

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

    #[test]
    pub fn test_get_first_side_middle() {
        let input = Triangle::new(
        Point::new(0.0, 0.0, 1.0),
        Point::new(10.0, 0.0, 1.0),
        Point::new(10.0, -15.0, 1.0));

        let actual = input.get_first_side_middle();
        let expected = Point::new(5.0, 0.0, 1.0);

        assert!(expected.eq(&actual))
    }

    #[test]
    pub fn test_get_second_side_middle() {
        let input = Triangle::new(
        Point::new(0.0, 0.0, 1.0),
        Point::new(10.0, 0.0, 1.0),
        Point::new(10.0, -15.0, 1.0));

        let actual = input.get_second_side_middle();
        let expected = Point::new(10.0, -7.5, 1.0);

        assert!(expected.eq(&actual))
    }

    #[test]
    pub fn test_get_third_side_middle() {
        let input = Triangle::new(
        Point::new(0.0, 0.0, 1.0),
        Point::new(10.0, 0.0, 1.0),
        Point::new(10.0, -15.0, 1.0));

        let actual = input.get_third_side_middle();
        let expected = Point::new(5.0, -7.5, 1.0);

        assert!(expected.eq(&actual))
    }
    
    #[test]
    pub fn test_get_vector_from_first_side_middle_to_centroid() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));
        
        let actual = input.get_vector_from_first_side_middle_to_centroid();
        let expected = Vector::new(-6.744508,-24.983247,-11.182357);
        
        assert!(expected.eq_with_tolerance(&actual, 0.001))
    }

    #[test]
    pub fn test_get_vector_from_second_side_middle_to_centroid() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));
        
        let actual = input.get_vector_from_second_side_middle_to_centroid();
        let expected = Vector::new(21.957154,18.505015,-6.051555);
        
        assert!(expected.eq_with_tolerance(&actual, 0.001))
    }

    #[test]
    pub fn test_get_vector_from_third_side_middle_to_centroid() {
        let input = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));
        
        let actual = input.get_vector_from_third_side_middle_to_centroid();
        let expected = Vector::new(-15.212646,6.478232,17.233911);
        
        assert!(expected.eq_with_tolerance(&actual, 0.001))
    }
    
    #[test]
    pub fn test_get_normals_angle() {
        let a = Triangle::new(
        Point::new(35.704653, 37.253023, -22.626602),
        Point::new(-38.634947, 13.199458, 23.94433),
        Point::new(-21.698671, -49.7235, -32.888206));
        let b = Triangle::new(
        Point::new(51.3242, 19.2342, 28.461254),
        Point::new(-21.698671, -49.7235, -32.888206),
        Point::new(-38.634947, 13.199458, 23.94433));
        
        let actual = a.get_normals_angle(&b);
        
        assert!((actual - 2.524541).abs() < 0.00001);
    }

    #[test]
    pub fn test_get_normals_angle_planar() {
        let a = Triangle::new(
            Point::new(0.0,0.0,0.0),
            Point::new(10.0,0.0,0.0),
            Point::new(10.0,5.0,0.0));
        let b = Triangle::new(
            Point::new(0.0,0.0,0.0),
            Point::new(10.0,5.0,0.0),
            Point::new(0.0,5.0,0.0));

        let actual = a.get_normals_angle(&b);

        assert!(actual < 0.00001);
    }
}

