use crate::mesh::Mesh;
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
    
    /// Checks if this [Ray] hits given [Mesh].
    /// 
    /// If it hits: then `true` is returned, if not: `false`.
    /// 
    /// Orientations of [Triangle]s are not taken into an account.
    /// 
    /// # Examples
    /// 
    /// First example shows detecting of hit, so `true` is expected.
    /// 
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    /// let mesh = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
    /// ],
    /// vec![
    ///     // Base faces
    ///     0,1,2,
    ///     0,2,3,
    ///
    ///     // Side faces
    ///     0,1,4,
    ///     1,2,4,
    ///     2,3,4,
    ///     3,0,4
    /// ]);
    ///
    /// let ray = Ray::new(Point::new(-4.912183, 2.730841, 0.76832), Vector::new(0.853281,0.510629,0.105683));
    ///
    /// let actual = ray.does_intersect_with_mesh(&mesh);
    ///
    /// assert_eq!(actual, true);
    ///
    /// ```
    /// 
    /// Second example shows not hitting the Mesh, so `false` is expected.
    /// 
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    /// let mesh = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
    /// ],
    /// vec![
    ///     // Base faces
    ///     0,1,2,
    ///     0,2,3,
    ///
    ///     // Side faces
    ///     0,1,4,
    ///     1,2,4,
    ///     2,3,4,
    ///     3,0,4
    /// ]);
    ///
    /// let ray = Ray::new(Point::new(-4.912183, 7.757342, 0.76832), Vector::new(0.853281,0.510629,0.105683));
    ///
    /// let actual = ray.does_intersect_with_mesh(&mesh);
    ///
    /// assert_eq!(actual, false);
    ///
    /// ```
    pub fn does_intersect_with_mesh(&self, mesh:&Mesh) -> bool {
        let triangles_to_check = mesh.to_triangles();

        for triangle in triangles_to_check {
            let result = self.get_intersection_with_triangle(&triangle);
            if result.is_some() {
                return true;
            }
        }

        false
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

    /// Calculates [Ray]'s intersections with the [Mesh].
    ///
    /// It iterates all the [Triangle]s and for each it tries to get an intersection.
    ///
    /// # Examples
    ///
    /// There is an example below with a Ray hitting the Mesh, returning 2 intersection Points.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    /// let mesh = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
    /// ],
    /// vec![
    ///     // Base faces
    ///     0,1,2,
    ///     0,2,3,
    ///
    ///     // Side faces
    ///     0,1,4,
    ///     1,2,4,
    ///     2,3,4,
    ///     3,0,4
    /// ]);
    ///
    /// let ray = Ray::new(Point::new(-4.912183, 2.730841, 0.76832), Vector::new(0.853281,0.510629,0.105683));
    ///
    /// let actual = ray.get_intersections_with_mesh(&mesh);
    ///
    /// let expected = vec![Point::new(4.790800375717201,8.537397923404011,1.9700816612767906), Point::new(-0.3302282935488474,5.472820429754613,1.3358173651609224)];
    ///
    /// assert_eq!(actual.len(), 2);
    /// assert_eq!(actual[0].eq_with_tolerance(&expected[0], 0.001), true);
    /// assert_eq!(actual[1].eq_with_tolerance(&expected[1], 0.001), true);
    ///
    /// ```
    ///
    /// The example below shows the case where Ray misses the Mesh, so there is an empty `vector` returned.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::ray::Ray;
    /// use meshmeshmesh::vector::Vector;
    /// let mesh = Mesh::new(
    /// vec![
    ///     // Base
    ///     -2.0,1.0,0.0,
    ///     8.0,1.0,0.0,
    ///     8.0,11.0,0.0,
    ///     -2.0,11.0,0.0,
    ///
    ///     // Top
    ///     3.0,6.0,4.0
    /// ],
    /// vec![
    ///     // Base faces
    ///     0,1,2,
    ///     0,2,3,
    ///
    ///     // Side faces
    ///     0,1,4,
    ///     1,2,4,
    ///     2,3,4,
    ///     3,0,4
    /// ]);
    ///
    /// let ray = Ray::new(Point::new(-4.912183, 7.757342, 0.76832), Vector::new(0.853281,0.510629,0.105683));
    ///
    /// let actual = ray.get_intersections_with_mesh(&mesh);
    ///
    /// assert_eq!(actual.is_empty(), true);
    ///
    /// ```
    pub fn get_intersections_with_mesh(&self, mesh:&Mesh) -> Vec<Point> {
        let triangles_to_check = mesh.to_triangles();

        let mut intersection_points: Vec<Point> = Vec::new();

        for triangle in triangles_to_check {
            let result = self.get_intersection_with_triangle(&triangle);
            if result.is_some() {
                intersection_points.push(result.unwrap())
            }
        }

        intersection_points
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
    pub fn test_does_intersect_with_mesh_true() {
        let mesh = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,
        
            // Top
            3.0,6.0,4.0
        ],
        vec![
            // Base faces
            0,1,2,
            0,2,3,
        
            // Side faces
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]);
        
        let ray = Ray::new(Point::new(-4.912183, 2.730841, 0.76832), Vector::new(0.853281,0.510629,0.105683));
        
        let actual = ray.does_intersect_with_mesh(&mesh);
        
        assert_eq!(actual, true);
    }

    #[test]
    pub fn test_does_intersect_with_mesh_false() {
        let mesh = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,
        
            // Top
            3.0,6.0,4.0
        ],
        vec![
            // Base faces
            0,1,2,
            0,2,3,
        
            // Side faces
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]);
        
        let ray = Ray::new(Point::new(-4.912183, 7.757342, 0.76832), Vector::new(0.853281,0.510629,0.105683));
        
        let actual = ray.does_intersect_with_mesh(&mesh);
        
        assert_eq!(actual, false);
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

    #[test]
    pub fn test_get_intersection_with_triangle_hit_flipped() {
        let triangle = Triangle::new(Point::new(18.106339, 26.580607, 7.381013), Point::new(27.733604, 26.580607, 28.757986), Point::new(24.296286, -0.019341, 19.121015));
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.660831,0.569323,0.489054));
        let triangle_flipped = triangle.get_flipped();

        let expected = Point::new(23.94358, 21.766485, 19.979597);
        let actual = ray.get_intersection_with_triangle(&triangle_flipped).unwrap();

        assert_eq!(expected.eq_with_tolerance(&actual, 0.001), true);
    }

    #[test]
    pub fn test_get_intersections_with_mesh_2_intersections() {
        let mesh = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,

            // Top
            3.0,6.0,4.0
        ],
        vec![
            // Base faces
            0,1,2,
            0,2,3,

            // Side faces
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]);

        let ray = Ray::new(Point::new(-4.912183, 2.730841, 0.76832), Vector::new(0.853281,0.510629,0.105683));

        let actual = ray.get_intersections_with_mesh(&mesh);

        let expected = vec![Point::new(4.790800375717201,8.537397923404011,1.9700816612767906), Point::new(-0.3302282935488474,5.472820429754613,1.3358173651609224)];

        for act in &actual {
            println!("{0},{1},{2}", act.x, act.y, act.z)
        }

        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].eq_with_tolerance(&expected[0], 0.001), true);
        assert_eq!(actual[1].eq_with_tolerance(&expected[1], 0.001), true);
    }

    #[test]
    pub fn test_get_intersections_with_mesh_0_intersections() {
        let mesh = Mesh::new(
        vec![
            // Base
            -2.0,1.0,0.0,
            8.0,1.0,0.0,
            8.0,11.0,0.0,
            -2.0,11.0,0.0,

            // Top
            3.0,6.0,4.0
        ],
        vec![
            // Base faces
            0,1,2,
            0,2,3,

            // Side faces
            0,1,4,
            1,2,4,
            2,3,4,
            3,0,4
        ]);

        let ray = Ray::new(Point::new(-4.912183, 7.757342, 0.76832), Vector::new(0.853281,0.510629,0.105683));

        let actual = ray.get_intersections_with_mesh(&mesh);

        for act in &actual {
            println!("{0},{1},{2}", act.x, act.y, act.z)
        }

        assert_eq!(actual.is_empty(), true);
    }
}