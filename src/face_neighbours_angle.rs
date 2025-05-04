use crate::face_neighbours::FaceNeighbours;
use crate::triangle::Triangle;

/// Mesh's face neighbours angles.
///
/// Each neighbour is a value of angle between specific faces' normals or `None` if there is no
/// neighbour for that edge.
///
/// Should be used most of the time together with [FaceNeighbours], so it is known then which
/// faces are neighbouring, so then we know between which faces this angle is measured.
///
/// There is an assumption that each face's edge has only 1 face neighbour. It might not always be
/// true, because it is possible that some faces have multiple faces as a neighbours for the same
/// edge.
#[derive(Debug, Clone, Copy)]
pub struct FaceNeighboursAngle {
    /// Angle of the first edge of that face.
    pub first: Option<f64>,
    /// Angle of the second edge of that face.
    pub second: Option<f64>,
    /// Angle of the third edge of that face.
    pub third: Option<f64>,
}

impl PartialEq for FaceNeighboursAngle {
    fn eq(&self, other: &Self) -> bool {
        self.first.eq(&other.first) && self.second.eq(&other.second) && self.third.eq(&other.third)
    }
}

impl FaceNeighboursAngle {
    /// Creates a new [FaceNeighboursAngle].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::face_neighbours_angle::FaceNeighboursAngle;
    ///
    /// let result = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51)); // This means that face has first neighbour on that edge with angle 0.02, on the second edge it doesn't have any neighbour, and on the third edge it has neighbour with angle 1.51.
    ///
    /// assert!(result.first.eq(&Some(0.02)));
    /// assert!(result.second.is_none());
    /// assert!(result.third.eq(&Some(1.51)));
    /// ```
    pub fn new(first: Option<f64>, second: Option<f64>, third: Option<f64>) -> FaceNeighboursAngle { FaceNeighboursAngle { first, second, third } }

    /// Creates a `vec` of [FaceNeighboursAngle]s from [FaceNeighbours] and [Triangle]s.
    ///
    /// To make it correct both input lists (`face_neighbours` and `triangles`) have to match.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    /// use meshmeshmesh::face_neighbours_angle::FaceNeighboursAngle;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    /// 
    /// let triangles = vec![
    ///     Triangle::new(Point::new(0.0, 0.0, -0.5), Point::new(5.0, 0.0, 0.3), Point::new(2.5, 5.0, 0.5)),
    ///     Triangle::new(Point::new(2.5, 5.0, 0.5), Point::new(5.0, 0.0, 0.3), Point::new(7.5, 5.0, -0.4)),
    ///     Triangle::new(Point::new(5.0, 0.0, 0.3), Point::new(10.0, 0.0, 0.1), Point::new(7.5, 5.0, -0.4)),
    ///     Triangle::new(Point::new(2.5, 5.0, 0.5), Point::new(7.5, 5.0, -0.4), Point::new(5.0, 10.0, 0.9)),
    /// ];
    ///
    /// let face_neighbours = vec![
    ///     FaceNeighbours::new(None, Some(1), None),
    ///     FaceNeighbours::new(Some(0), Some(2), Some(3)),
    ///     FaceNeighbours::new(None, None, Some(1)),
    ///     FaceNeighbours::new(Some(1), None, None),
    /// ];
    ///
    /// let actual = FaceNeighboursAngle::from_face_neighbours_and_triangles(&face_neighbours, &triangles);
    ///
    /// let expected = vec![
    ///     FaceNeighboursAngle::new(None, Some(0.37540037779770735), None),
    ///     FaceNeighboursAngle::new(Some(0.37540037779770735), Some(0.15445199884596061), Some(0.21494519445616783)),
    ///     FaceNeighboursAngle::new(None, None, Some(0.15445199884596061)),
    ///     FaceNeighboursAngle::new(Some(0.21494519445616783), None, None),
    /// ];
    /// 
    /// assert_eq!(expected, actual);
    /// 
    /// ```
    pub fn from_face_neighbours_and_triangles(face_neighbours: &Vec<FaceNeighbours>, triangles: &Vec<Triangle>) -> Vec<FaceNeighboursAngle> {
        let number_of_faces = triangles.len();
        let mut face_neighbours_angles: Vec<FaceNeighboursAngle> = vec![FaceNeighboursAngle::new(None, None, None); number_of_faces];
        if number_of_faces != face_neighbours.len() {
            panic!("The input of the from_face_neighbours_and_triangles (for both FaceNeighbours and Triangles) should be the same length.")
        }

        for i in 0..number_of_faces {
            let current_face_neighbours = face_neighbours[i];
            let current_face = triangles[i];

            if current_face_neighbours.first.is_some() {
                let neighbour_face = triangles[current_face_neighbours.first.unwrap()];
                face_neighbours_angles[i].first = Some(current_face.get_normals_angle(&neighbour_face));
            }

            if current_face_neighbours.second.is_some() {
                let neighbour_face = triangles[current_face_neighbours.second.unwrap()];
                face_neighbours_angles[i].second = Some(current_face.get_normals_angle(&neighbour_face));
            }

            if current_face_neighbours.third.is_some() {
                let neighbour_face = triangles[current_face_neighbours.third.unwrap()];
                face_neighbours_angles[i].third = Some(current_face.get_normals_angle(&neighbour_face));
            }
        }

        face_neighbours_angles
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use super::*;

    #[test]
    fn test_new() {
        let result = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51)); // This means that face has first neighbour on that edge with angle 0.02, on the second edge it doesn't have any neighbour, and on the third edge it has neighbour with angle 1.51.

        assert!(result.first.eq(&Some(0.02)));
        assert!(result.second.is_none());
        assert!(result.third.eq(&Some(1.51)));
    }

    #[test]
    fn test_partialeq_true() {
        let a = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51));
        let b  = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51));
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51));
        let b  = FaceNeighboursAngle::new(Some(0.03), None, Some(1.51));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51));
        let b  = FaceNeighboursAngle::new(Some(0.02), Some(0.4), Some(1.51));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51));
        let b  = FaceNeighboursAngle::new(Some(0.02), None, Some(1.52));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = FaceNeighboursAngle::new(Some(0.02), None, Some(1.51));
        let b  = FaceNeighboursAngle::new(Some(0.03), Some(0.4), Some(1.52));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
    
    #[test]
    fn test_from_face_neighbours_and_triangles() {
        let triangles = vec![
            Triangle::new(Point::new(0.0, 0.0, -0.5), Point::new(5.0, 0.0, 0.3), Point::new(2.5, 5.0, 0.5)),
            Triangle::new(Point::new(2.5, 5.0, 0.5), Point::new(5.0, 0.0, 0.3), Point::new(7.5, 5.0, -0.4)),
            Triangle::new(Point::new(5.0, 0.0, 0.3), Point::new(10.0, 0.0, 0.1), Point::new(7.5, 5.0, -0.4)),
            Triangle::new(Point::new(2.5, 5.0, 0.5), Point::new(7.5, 5.0, -0.4), Point::new(5.0, 10.0, 0.9)),
        ];
        
        let face_neighbours = vec![
            FaceNeighbours::new(None, Some(1), None),
            FaceNeighbours::new(Some(0), Some(2), Some(3)),
            FaceNeighbours::new(None, None, Some(1)),
            FaceNeighbours::new(Some(1), None, None),
        ];
        
        let actual = FaceNeighboursAngle::from_face_neighbours_and_triangles(&face_neighbours, &triangles);
        
        let expected = vec![
            FaceNeighboursAngle::new(None, Some(0.37540037779770735), None),
            FaceNeighboursAngle::new(Some(0.37540037779770735), Some(0.15445199884596061), Some(0.21494519445616783)),
            FaceNeighboursAngle::new(None, None, Some(0.15445199884596061)),
            FaceNeighboursAngle::new(Some(0.21494519445616783), None, None),
        ];
        
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "The input of the from_face_neighbours_and_triangles (for both FaceNeighbours and Triangles) should be the same length.")]
    fn test_from_face_neighbours_and_triangles_different_length_of_inputs() {
        let triangles = vec![
            Triangle::new(Point::new(0.0, 0.0, -0.5), Point::new(5.0, 0.0, 0.3), Point::new(2.5, 5.0, 0.5)),
            Triangle::new(Point::new(2.5, 5.0, 0.5), Point::new(5.0, 0.0, 0.3), Point::new(7.5, 5.0, -0.4)),
            Triangle::new(Point::new(5.0, 0.0, 0.3), Point::new(10.0, 0.0, 0.1), Point::new(7.5, 5.0, -0.4)),
            Triangle::new(Point::new(2.5, 5.0, 0.5), Point::new(7.5, 5.0, -0.4), Point::new(5.0, 10.0, 0.9)),
        ];

        let face_neighbours = vec![
            FaceNeighbours::new(None, Some(1), None),
            FaceNeighbours::new(Some(0), Some(2), Some(3)),
            FaceNeighbours::new(None, None, Some(1)),
        ];

        FaceNeighboursAngle::from_face_neighbours_and_triangles(&face_neighbours, &triangles);
    }
}