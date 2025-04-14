use crate::mesh::Mesh;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

impl Point {
    /// Compares given [Point] to other one, but with a `f64` tolerance.
    ///
    /// If any coordinate absolute difference is > tolerance, then it should return `false`.
    ///
    /// As you can see, it doesn't compare distances between [Point]s, but rather coordinates themselves.
    ///
    /// # Examples
    ///
    /// In this example we can see the differences of coordinates are not > tolerance, so we expect `true`.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Point::new(1.5, -2.3, 3.9);
    /// let b = Point::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.001);
    ///
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    /// ```
    ///
    /// In this example we can see the Y-coordinate absolute difference is > tolerance, so we expect 'false'.
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    ///
    /// let tolerance: f64 = 0.001;
    /// let a = Point::new(1.5, -2.3, 3.9);
    /// let b = Point::new(1.5 + 0.0005, -2.3 - 0.00101, 3.9 + 0.001);
    /// assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    /// ```
    pub fn eq_with_tolerance(&self, other:&Point, tolerance: f64) -> bool {
        if (self.x - other.x).abs() > tolerance {
            false
        }
        else if (self.y - other.y).abs() > tolerance {
            false
        }
        else if (self.z - other.z).abs() > tolerance {
            false
        }
        else {
            true
        }
    }
    
    /// Checks is the [Point] is inside the given [Mesh].
    /// 
    /// The method used for check is shooting the [Ray]s in 6 directions:
    /// 1. X
    /// 2. -X
    /// 3. Y
    /// 4. -Y
    /// 5. Z
    /// 6. -Z
    /// 
    /// If all of these Rays hits the Mesh - then it counts this Point as being inside the Mesh.
    /// 
    /// # Examples
    /// 
    /// Here in this example [Point] is inside the pyramid [Mesh], so `true` is returned.
    /// 
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
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
    /// let point = Point::new(2.632568, 4.836652, 0.767105);
    /// 
    /// let actual = point.is_inside_mesh_using_xyz(&mesh);
    /// 
    /// assert_eq!(actual, true);
    /// 
    /// ```
    /// 
    /// Here in another example [Point] is outside the pyramid [Mesh], so `false` is returned.
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
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
    /// let point = Point::new(8.975928, 4.836652, 0.767105);
    ///
    /// let actual = point.is_inside_mesh_using_xyz(&mesh);
    ///
    /// assert_eq!(actual, false);
    ///
    /// ```
    pub fn is_inside_mesh_using_xyz(&self, mesh:&Mesh) -> bool {
        let origin = self.clone();
        
        let mut ray = Ray::new(origin, Vector::new(1.0, 0.0, 0.0));
        if !ray.does_intersect_with_mesh(mesh) { // 1. X
            return false;
        }
        
        ray.direction = Vector::new(-1.0, 0.0, 0.0);
        if !ray.does_intersect_with_mesh(mesh) { // 2. -X
            return false;
        }

        ray.direction = Vector::new(0.0, 1.0, 0.0);
        if !ray.does_intersect_with_mesh(mesh) { // 3. Y
            return false;
        }

        ray.direction = Vector::new(0.0, -1.0, 0.0);
        if !ray.does_intersect_with_mesh(mesh) { // 4. -Y
            return false;
        }

        ray.direction = Vector::new(0.0, 0.0, 1.0);
        if !ray.does_intersect_with_mesh(mesh) { // 5. Z
            return false;
        }

        ray.direction = Vector::new(0.0, 0.0, -1.0);
        if !ray.does_intersect_with_mesh(mesh) { // 6. -Z
            return false;
        }
        
        true
    }

    /// Gets distance to another [Point].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::point::Point;
    /// let a = Point::new(35.704653, 37.253023, -22.626602);
    /// let b = Point::new(-38.634947, 13.199458, 23.94433);
    /// let actual_ab = a.get_distance_to_point(&b);
    /// let actual_ba = b.get_distance_to_point(&a);
    ///
    /// let expected = 90.960441;
    ///
    /// assert_eq!(((expected - actual_ab).abs() < 0.00001), true); // Both distances should be the same
    /// assert_eq!(((expected - actual_ba).abs() < 0.00001), true);
    /// ```
    pub fn get_distance_to_point(&self, other:&Point) -> f64 {
        f64::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
    }
    
    /// Gets the [Point] in between this one and other, specifically in the middle between these 2.
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::point::Point;
    /// let a = Point::new(10.0, 0.0, 1.2);
    /// let b = Point::new(11.0, -10.10, 3.6);
    /// 
    /// let actual = a.get_middle_to(&b);
    /// let expected = Point::new(10.5, -5.05, 2.4);
    /// 
    /// assert!(actual.eq(&expected))
    /// ```
    pub fn get_middle_to(&self, other:&Point) -> Point {
        Point::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0, (self.z + other.z) / 2.0)
    }

    /// Scans given `vec` of [Point]s and creates a `vec` of tuples with information about duplicates.
    ///
    /// Duplicates are checked with given tolerance.
    ///
    /// This information is stored in the `vec` inside tuples:
    ///
    /// - Each [Point] in input `vec` has corresponding tuple in output `vec`. The output is sorted
    /// same way as input.
    /// - `usize` in related `tuple` tells the index of first occurrence of this Point.
    /// - `bool` in related `tuple` tells if this index was already there.
    /// `true` = it occurred before in the Vector (it is a duplicate).
    /// `false` = it is the first occurrence (it's not a duplicate).
    ///
    /// This way it is clear which Points are the duplicates and of which Point.
    ///
    /// # Examples
    ///
    /// ```
    ///  use meshmeshmesh::point::Point;
    /// let points = vec![
    ///     Point::new(1.5, -2.3, 3.9),
    ///     Point::new(0.6, -7.8, 9.1),
    ///     Point::new(0.6, -7.8, 9.1), // duplicate of 1
    ///     Point::new(1.5, -2.3, 3.9), // duplicate of 0
    ///     Point::new(8.9, 0.5, 35.8),
    /// ];
    /// let expected = vec![(0, false), (1, false), (1, true), (0, true), (4, false)];
    /// let actual = Point::scan_for_duplicates_with_tolerance_info(&points, 0.001);
    /// assert_eq!(actual, expected);
    /// ```
    pub fn scan_for_duplicates_with_tolerance_info(points: &Vec<Point>, tolerance: f64) -> Vec<(usize, bool)>{
        let mut info: Vec<(usize, bool)> = Vec::new();

        let points_length = points.len();

        for i in 0..points_length {
            info.push((i, false));
        }

        for i in 0..points_length {
            if !info[i].1 { // Checks if it's not already a duplicate, cause if it is, then doesn't make sense to check again with Points below
                let current_point = &points[i];
                for j in (i+1)..points_length {
                    let next_point_for_comparison = &points[j];
                    if current_point.eq_with_tolerance(next_point_for_comparison, tolerance) {
                        info[j] = (i, true);
                    }
                }
            }
        }

        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_with_tolerance_true(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.001);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), true);
    }

    #[test]
    fn test_eq_with_tolerance_different_x_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0011, -2.3 - 0.0005, 3.9 + 0.001);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_y_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0005, -2.3 - 0.00101, 3.9 + 0.001);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_z_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0005, -2.3 - 0.0005, 3.9 + 0.0013);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_eq_with_tolerance_different_xyz_false(){
        let tolerance: f64 = 0.001;
        let a = Point::new(1.5, -2.3, 3.9);
        let b = Point::new(1.5 + 0.0011, -2.3 - 0.00101, 3.9 + 0.0013);
        assert_eq!(a.eq_with_tolerance(&b, tolerance), false);
    }

    #[test]
    fn test_is_inside_mesh_using_xyz_true() {
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
        
        let point = Point::new(2.632568, 4.836652, 0.767105);
        
        let actual = point.is_inside_mesh_using_xyz(&mesh);
        
        assert_eq!(actual, true);
    }

    #[test]
    fn test_is_inside_mesh_using_xyz_false() {
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
        
        let point = Point::new(8.975928, 4.836652, 0.767105);
        
        let actual = point.is_inside_mesh_using_xyz(&mesh);
        
        assert_eq!(actual, false);
    }

    #[test]
    fn test_get_distance_to_point(){
        let a = Point::new(35.704653, 37.253023, -22.626602);
        let b = Point::new(-38.634947, 13.199458, 23.94433);
        let actual_ab = a.get_distance_to_point(&b);
        let actual_ba = b.get_distance_to_point(&a);

        let expected = 90.960441;

        assert_eq!(((expected - actual_ab).abs() < 0.00001), true); // Both distances should be the same
        assert_eq!(((expected - actual_ba).abs() < 0.00001), true);
    }

    #[test]
    fn test_get_distance_to_point_the_same(){
        let a = Point::new(35.704653, 37.253023, -22.626602);
        let b = Point::new(35.704653, 37.253023, -22.626602);
        let actual_ab = a.get_distance_to_point(&b);
        let actual_ba = b.get_distance_to_point(&a);

        let expected = 0.0;

        assert_eq!(((expected - actual_ab).abs() < 0.00001), true); // Both distances should be the same
        assert_eq!(((expected - actual_ba).abs() < 0.00001), true);
    }

    #[test]
    fn test_scan_for_duplicates_with_tolerance_info_1_point(){
        let points = vec![Point::new(1.5, -2.3, 3.9)];
        let expected = vec![(0, false)];
        let actual = Point::scan_for_duplicates_with_tolerance_info(&points, 0.001);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scan_for_duplicates_with_tolerance_info_5_points_no_duplicates(){
        let points = vec![
            Point::new(1.5, -2.3, 3.9),
            Point::new(0.6, -7.8, 9.1),
            Point::new(6.2, 6.34, -2.6),
            Point::new(11.54, 7.56, 2.05),
            Point::new(8.9, 0.5, 35.8),
        ];
        let expected = vec![(0, false), (1, false), (2, false), (3, false), (4, false)];
        let actual = Point::scan_for_duplicates_with_tolerance_info(&points, 0.001);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scan_for_duplicates_with_tolerance_info_5_points_4_duplicates(){
        let points = vec![
            Point::new(1.5, -2.3, 3.9),
            Point::new(1.5, -2.3, 3.9), // duplicate of 0
            Point::new(1.5, -2.3, 3.9), // duplicate of 0
            Point::new(1.5, -2.3, 3.9), // duplicate of 0
            Point::new(1.5, -2.3, 3.9), // duplicate of 0
        ];
        let expected = vec![(0, false), (0, true), (0, true), (0, true), (0, true)];
        let actual = Point::scan_for_duplicates_with_tolerance_info(&points, 0.001);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scan_for_duplicates_with_tolerance_info_5_points_3_duplicates(){
        let points = vec![
            Point::new(1.5, -2.3, 3.9),
            Point::new(0.6, -7.8, 9.1),
            Point::new(0.6, -7.8 - 0.0009, 9.1), // duplicate of 1
            Point::new(0.6, -7.8, 9.1 + 0.0005), // duplicate of 1
            Point::new(0.6, -7.8, 9.1), // duplicate of 1
        ];
        let expected = vec![(0, false), (1, false), (1, true), (1, true), (1, true)];
        let actual = Point::scan_for_duplicates_with_tolerance_info(&points, 0.001);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scan_for_duplicates_with_tolerance_info_5_points_1_duplicate(){
        let points = vec![
            Point::new(1.5, -2.3, 3.9),
            Point::new(0.6, -7.8, 9.1),
            Point::new(6.2, 6.34, -2.6),
            Point::new(0.6, -7.8, 9.1), // duplicate of 1
            Point::new(8.9, 0.5, 35.8),
        ];
        let expected = vec![(0, false), (1, false), (2, false), (1, true), (4, false)];
        let actual = Point::scan_for_duplicates_with_tolerance_info(&points, 0.001);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scan_for_duplicates_with_tolerance_info_5_points_2_duplicates(){
        let points = vec![
            Point::new(1.5, -2.3, 3.9),
            Point::new(0.6, -7.8, 9.1),
            Point::new(0.6, -7.8, 9.1), // duplicate of 1
            Point::new(1.5, -2.3, 3.9), // duplicate of 0
            Point::new(8.9, 0.5, 35.8),
        ];
        let expected = vec![(0, false), (1, false), (1, true), (0, true), (4, false)];
        let actual = Point::scan_for_duplicates_with_tolerance_info(&points, 0.001);
        assert_eq!(actual, expected);
    }
}

