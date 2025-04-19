use std::collections::HashMap;
use crate::edge::Edge;
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
    ///          5.0, 10.0, 0.0,
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
    /// It can panic once it will find that specific Edge has more than one neighbour.
    ///
    /// It is because of the simplified convention, that [FaceNeighbours] can only have a single
    /// neighbour for each edge.
    ///
    /// # Examples
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
    ///
    /// In the example below it will panic, because the input [Mesh] has an edge that shares
    /// 3 different faces instead of 2.
    ///
    /// ```should_panic
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
    ///     ThreeEdgeGroup::new(Edge::new(6, 2), Edge::new(2, 3), Edge::new(3, 6)), // fifth face -> you can see the edge (2, 3) is shared here as well
    /// ];
    ///
    /// let _actual = FaceNeighbours::from_three_edge_groups(&input);
    ///
    /// ```
    pub fn from_three_edge_groups(three_edge_groups: &Vec<ThreeEdgeGroup>) -> Vec<FaceNeighbours> {

        let edge_hashmap = ThreeEdgeGroup::get_edge_with_face_ids_hashmap_with_reversed_edges_merged(three_edge_groups);
        FaceNeighbours::from_edge_with_face_ids_hashmap(&edge_hashmap, three_edge_groups)
    }

    /// Private method for turning edge_with_face_ids_hashmap into `vec` of [FaceNeighbours].
    fn from_edge_with_face_ids_hashmap(edge_with_face_ids_hashmap: &HashMap<Edge, Vec<usize>>, three_edge_groups: &Vec<ThreeEdgeGroup>) -> Vec<FaceNeighbours> {

        let number_of_faces = three_edge_groups.len();
        let mut face_neighbours: Vec<FaceNeighbours> = vec![FaceNeighbours::new(None, None, None); number_of_faces];

        for (key, value) in edge_with_face_ids_hashmap.into_iter() {
            let current_edge = key;
            let number_of_neighbour_faces = value.len();
            if number_of_neighbour_faces == 0 { 
                panic!("Seems like the edge_with_face_ids_hashmap is not structured properly")
            }
            else if number_of_neighbour_faces == 1 {
                // This edge has only 1 face, so it's not useful to find FaceNeighbour
            }
            else if number_of_neighbour_faces == 2 {
                let face_a = three_edge_groups[value[0]];
                
                if face_a.first.eq_regardless_of_direction(current_edge) { 
                    face_neighbours[value[0]].first = Some(value[1]);
                }
                else if face_a.second.eq_regardless_of_direction(current_edge) {
                    face_neighbours[value[0]].second = Some(value[1]);
                }
                else if face_a.third.eq_regardless_of_direction(current_edge) {
                    face_neighbours[value[0]].third = Some(value[1]);
                }

                let face_b = three_edge_groups[value[1]];

                if face_b.first.eq_regardless_of_direction(current_edge) {
                    face_neighbours[value[1]].first = Some(value[0]);
                }
                else if face_b.second.eq_regardless_of_direction(current_edge) {
                    face_neighbours[value[1]].second = Some(value[0]);
                }
                else if face_b.third.eq_regardless_of_direction(current_edge) {
                    face_neighbours[value[1]].third = Some(value[0]);
                }
            }
            else {
                panic!("There is more than 2 neighbours for the edge, which is illegal for FaceNeighbours")
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
                 5.0, 10.0, 0.0,
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

    #[test]
    #[should_panic(expected = "There is more than 2 neighbours for the edge, which is illegal for FaceNeighbours")]
    fn test_from_three_edge_groups_first_edge_panic() {
        let input = vec![
            ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
            ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
            ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
            ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
            ThreeEdgeGroup::new(Edge::new(6, 2), Edge::new(2, 3), Edge::new(3, 6)), // fifth face -> you can see the edge (2, 3) is shared here as well
        ];
        
        let _actual = FaceNeighbours::from_three_edge_groups(&input);
    }
}