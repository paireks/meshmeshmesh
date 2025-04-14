use crate::mesh::Mesh;
use crate::three_edge_group::ThreeEdgeGroup;

/// Gets [Mesh]'s face neighbours.
///
/// Each neighbour is an index to a specific face.
/// 
/// There is an assumption that each face's edge has only 1 face neighbour. It might not always be
/// true, because it is possible that some faces have multiple faces as a neighbours.
/// 
/// If index is `None` = there is no neighbour face for that edge.
#[derive(Debug, Clone, Copy)]
pub struct FaceNeighbours {
    /// Neighbour of the first edge of that face.
    pub first: Option<usize>,
    /// Neighbour of the second edge of that face.
    pub second: Option<usize>,
    /// Neighbour of the third edge of that face.
    pub third: Option<usize>,
}

impl PartialEq for FaceNeighbours {
    fn eq(&self, other: &Self) -> bool {
        self.first.eq(&other.first) && self.second.eq(&other.second) && self.third.eq(&other.third)
    }
}

impl FaceNeighbours {

    /// Creates a new [FaceNeighbours].
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    ///
    /// let result = FaceNeighbours::new(Some(0), None, Some(1)); // This means that face has first neighbour on that edge index 0, on the second edge it doesn't have any neighbour, and on the third edge it has neighbour index 1.
    ///
    /// assert!(result.first.eq(&Some(0)));
    /// assert!(result.second.is_none());
    /// assert!(result.third.eq(&Some(1)));
    /// ```
    pub fn new(first: Option<usize>, second: Option<usize>, third: Option<usize>) -> FaceNeighbours { FaceNeighbours { first, second, third } }

    /// Creates a [FaceNeighbours] from [Mesh].
    ///
    /// It tries to figure out which faces in given [Mesh] are neighbours.
    ///
    /// To check which one are neighbours it looks for same indexes of same vertices.
    ///
    /// That's why it's good to do welding of vertices before to have better results.
    ///
    /// The order of the output `vec` should be aligned to the order of faces.
    ///
    /// # Example
    ///
    /// In this example below we have simple 4 face planar [Mesh].
    ///
    /// We will calculate this way what are the neighbours for each face.
    ///
    /// ```
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let mesh = Mesh::new(
    ///     vec![0.0, 0.0, 0.0,
    ///          2.5, 5.0, 0.0,
    ///          5.0, 0.0, 0.0,
    ///          7.5, 5.0, 0.0,
    ///          10.0, 0.0, 0.0,
    ///          10.0, 5.0, 0.0,
    ///          ],
    ///     vec![0, 2, 1, // first face
    ///          1, 2, 3, // second face
    ///          2, 4, 3, // third face
    ///          1, 3, 5, // fourth face
    ///          ]
    /// );
    ///
    /// let actual = FaceNeighbours::from_mesh(&mesh);
    ///
    /// let expected = vec![
    ///     FaceNeighbours::new(None, Some(1), None),
    ///     FaceNeighbours::new(Some(0), Some(2), Some(3)),
    ///     FaceNeighbours::new(None, None, Some(1)),
    ///     FaceNeighbours::new(Some(1), None, None),
    /// ];
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert_eq!(expected[i].eq(&actual[i]), true);
    /// }
    ///
    /// ```
    pub fn from_mesh(mesh: &Mesh) -> Vec<FaceNeighbours> {
        Self::from_three_edge_groups(&mesh.to_three_edge_groups())
    }

    /// Creates a [FaceNeighbours] from `vec` of [ThreeEdgeGroup]s.
    ///
    /// It tries to figure out which faces in given [ThreeEdgeGroup]s `vec` are neighbours.
    ///
    /// To check which one are neighbours it looks for same indexes of same vertices.
    ///
    /// That's why it's good to do welding of vertices before to have better results.
    ///
    /// The order of the output `vec` should be aligned to the order of faces.
    ///
    /// # Example
    ///
    /// In this example below we have simple 4 face planar [Mesh].
    ///
    /// We will calculate this way what are the neighbours for each face.
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::three_edge_group::ThreeEdgeGroup;
    ///
    /// let input = vec![
    ///     ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
    ///     ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
    ///     ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
    ///     ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
    /// ];
    ///
    /// let actual = FaceNeighbours::from_three_edge_groups(&input);
    ///
    /// let expected = vec![
    ///     FaceNeighbours::new(None, Some(1), None),
    ///     FaceNeighbours::new(Some(0), Some(2), Some(3)),
    ///     FaceNeighbours::new(None, None, Some(1)),
    ///     FaceNeighbours::new(Some(1), None, None),
    /// ];
    ///
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert_eq!(expected[i].eq(&actual[i]), true);
    /// }
    ///
    /// ```
    pub fn from_three_edge_groups(three_edge_groups: &Vec<ThreeEdgeGroup>) -> Vec<FaceNeighbours> {

        let number_of_faces = three_edge_groups.len();
        let mut face_neighbours: Vec<FaceNeighbours> = vec![FaceNeighbours::new(None, None, None); number_of_faces];
        
        for i in 0..number_of_faces {
            let current_face = three_edge_groups[i];
            for j in 0..number_of_faces {
                if i != j {
                    let current_neighbour_candidate = three_edge_groups[j];
                    let neighbour_edge_option = current_face.which_edge_is_neighbour_to(&current_neighbour_candidate);
                    if neighbour_edge_option.is_some() { 
                        let neighbour_edge_id = neighbour_edge_option.unwrap();
                        if neighbour_edge_id == 0 {
                            face_neighbours[i].first = Some(j);
                        }
                        else if neighbour_edge_id == 1 {
                            face_neighbours[i].second = Some(j);
                        }
                        else {
                            face_neighbours[i].third = Some(j);
                        }
                    }
                }
            }
        }

        face_neighbours
    }
}

#[cfg(test)]
mod tests {
    use crate::edge::Edge;
    use super::*;

    #[test]
    fn test_new() {
        let result = FaceNeighbours::new(Some(0), None, Some(1)); // This means that face has first neighbour on that edge index 0, on the second edge it doesn't have any neighbour, and on the third edge it has neighbour index 1.

        assert!(result.first.eq(&Some(0)));
        assert!(result.second.is_none());
        assert!(result.third.eq(&Some(1)));
    }

    #[test]
    fn test_partialeq_true() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(0), None, Some(1));
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(2), None, Some(1));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(0), Some(3), Some(1));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(0), None, Some(5));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = FaceNeighbours::new(Some(0), None, Some(1));
        let b  = FaceNeighbours::new(Some(2), Some(3), Some(0));
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
    
    #[test]
    fn test_from_mesh() {
        let mesh = Mesh::new(
            vec![0.0, 0.0, 0.0,
                 2.5, 5.0, 0.0,
                 5.0, 0.0, 0.0,
                 7.5, 5.0, 0.0,
                 10.0, 0.0, 0.0,
                 10.0, 5.0, 0.0,
                 ],
            vec![0, 2, 1, // first face
                 1, 2, 3, // second face
                 2, 4, 3, // third face
                 1, 3, 5, // fourth face
                 ]
        );
        
        let actual = FaceNeighbours::from_mesh(&mesh);
        
        let expected = vec![
            FaceNeighbours::new(None, Some(1), None),
            FaceNeighbours::new(Some(0), Some(2), Some(3)),
            FaceNeighbours::new(None, None, Some(1)),
            FaceNeighbours::new(Some(1), None, None),
        ];
        
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }
    
    #[test]
    fn test_from_three_edge_groups() {
        let input = vec![
            ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
            ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
            ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
            ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
        ];
        
        let actual = FaceNeighbours::from_three_edge_groups(&input);
        
        let expected = vec![
            FaceNeighbours::new(None, Some(1), None),
            FaceNeighbours::new(Some(0), Some(2), Some(3)),
            FaceNeighbours::new(None, None, Some(1)),
            FaceNeighbours::new(Some(1), None, None),
        ];
        
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }
}