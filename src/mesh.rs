use crate::edge::Edge;
use crate::point::Point;
use crate::three_edge_group::ThreeEdgeGroup;
use crate::triangle::Triangle;

/// Represents a Mesh object in three-dimensional space.
///
/// This Mesh contains triangle faces only.
///
/// It is described by coordinates and indices.
///
/// *Coordinates* is a flat list of `f64` describing the vertices (points) like this:
///
/// [x0, y0, z0, x1, y1, z1, ... , xN, yN, zN]
///
/// *Indices* is a flat list of `usize` describing the faces (triangles) like this:
///
/// [face0_start_index, face0_middle_index, face0_end_index, face1_start_index, face1_middle_index, face1_end_index, ... , faceN_start_index, faceN_middle_index, faceN_end_index]
///
/// It tells you information which points construct which face.
///
/// The orientation of a face is described by a right hand thumb.
///
/// # Example
///
/// Here is an example with simple 1-triangle Mesh
///
/// ```
/// use meshmeshmesh::mesh::Mesh;
///
/// let result = Mesh::new(vec![0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, -15.0, 0.0], vec![0, 1, 2]);
/// assert_eq!(result.coordinates, vec![0.0, 0.0, 0.0,
///                                    10.0, 0.0, 0.0,
///                                    10.0, -15.0, 0.0]); // We have 3 vertices there.
/// assert_eq!(result.indices, vec![0, 1, 2]); // We create 1 face there using point0, point1 and point2.
/// ```
///
#[derive(Debug)]
pub struct Mesh {
    /// Optional identifier
    pub id: Option<usize>,
    /// The list of coordinates for the mesh vertices.
    pub coordinates: Vec<f64>,
    /// The list of indices for the mesh triangles.
    pub indices: Vec<usize>,
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {

        if self.id != other.id {
            return false;
        }
        if self.coordinates.len() != other.coordinates.len() {
            return false;
        }
        for i in 0..self.coordinates.len() {
            if self.coordinates[i] != other.coordinates[i] {
                return false;
            }
        }
        if self.indices.len() != other.indices.len() {
            return false;
        }
        for i in 0..self.indices.len() {
            if self.indices[i] != other.indices[i] {
                return false;
            }
        }

        true
    }
}

impl Mesh {
    /// Creates a new [Mesh]
    ///
    /// # Example
    ///
    /// Here is an example with simple 1-triangle Mesh
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let result = Mesh::new(vec![0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, -15.0, 0.0], vec![0, 1, 2]);
    /// assert_eq!(result.coordinates, vec![0.0, 0.0, 0.0,
    ///                                    10.0, 0.0, 0.0,
    ///                                    10.0, -15.0, 0.0]);
    /// assert_eq!(result.indices, vec![0, 1, 2]);
    /// ```
    pub fn new(coordinates: Vec<f64>, indices: Vec<usize>) -> Mesh {Mesh {id: None, coordinates, indices}}

    /// Creates a new [Mesh] with already set identifier
    ///
    /// # Example
    ///
    /// Here is an example with simple 1-triangle Mesh
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    ///
    /// let result = Mesh::new_with_id(Some(5), vec![0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, -15.0, 0.0], vec![0, 1, 2]);
    /// assert_eq!(result.id, Some(5));
    /// assert_eq!(result.coordinates, vec![0.0, 0.0, 0.0,
    ///                                    10.0, 0.0, 0.0,
    ///                                    10.0, -15.0, 0.0]);
    /// assert_eq!(result.indices, vec![0, 1, 2]);
    /// ```
    pub fn new_with_id(id: Option<usize>, coordinates: Vec<f64>, indices: Vec<usize>) -> Mesh {Mesh {id, coordinates, indices}}

    /// Converts [Mesh] into list of [Point]s
    ///
    /// # Example
    ///
    /// Here is an example with simple 1-triangle Mesh being converted to 3 Points
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    ///
    /// let input = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                            10.0, 0.0, 0.0,
    ///                            10.0, -15.0, 0.0],
    ///                       vec![0, 1, 2]);
    /// let actual = input.to_points();
    /// let expected = vec![Point::new(0.0, 0.0, 0.0),
    ///                     Point::new(10.0, 0.0, 0.0),
    ///                     Point::new(10.0, -15.0, 0.0)];
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert_eq!(expected[i].eq(&actual[i]), true);
    /// }
    /// ```
    pub fn to_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();
        let coordinates_length: usize = self.coordinates.len();
        let mut i = 0;
        while i < coordinates_length {
            points.push(Point::new(self.coordinates[i], self.coordinates[i+1], self.coordinates[i+2]));
            i = i + 3;
        }
        points
    }

    /// Converts [Mesh] into list of [Triangle]s
    ///
    /// # Example
    ///
    /// Here is an example with simple 1-triangle Mesh being converted to 1 Triangle
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                            10.0, 0.0, 0.0,
    ///                            10.0, -15.0, 0.0],
    /// vec![0, 1, 2]);
    /// let actual = input.to_triangles();
    /// let expected = vec![Triangle::new(
    ///     Point::new(0.0, 0.0, 0.0),
    ///     Point::new(10.0, 0.0, 0.0),
    ///     Point::new(10.0, -15.0, 0.0))];
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert_eq!(expected[i].eq(&actual[i]), true);
    /// }
    /// ```
    pub fn to_triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::<Triangle>::new();
        let indices_length: usize = self.indices.len();
        let mut i = 0;
        while i < indices_length {
            let offset0 = self.indices[i] * 3;
            let index00 = usize::try_from(offset0).unwrap();
            let index01 = usize::try_from(offset0 + 1).unwrap();
            let index02 = usize::try_from(offset0 + 2).unwrap();
            let point0: Point = Point::new(self.coordinates[index00], self.coordinates[index01], self.coordinates[index02]);

            let offset1 = self.indices[i+1] * 3;
            let index10 = usize::try_from(offset1).unwrap();
            let index11 = usize::try_from(offset1 + 1).unwrap();
            let index12 = usize::try_from(offset1 + 2).unwrap();
            let point1: Point = Point::new(self.coordinates[index10], self.coordinates[index11], self.coordinates[index12]);

            let offset2 = self.indices[i+2] * 3;
            let index20 = usize::try_from(offset2).unwrap();
            let index21 = usize::try_from(offset2 + 1).unwrap();
            let index22 = usize::try_from(offset2 + 2).unwrap();
            let point2: Point = Point::new(self.coordinates[index20], self.coordinates[index21], self.coordinates[index22]);

            triangles.push(Triangle::new(point0, point1, point2));
            i = i + 3;
        }

        triangles
    }

    /// Creates [Mesh] from list of [Triangle]s
    ///
    /// # Example
    ///
    /// Here is an example with creating a simple 1-triangle Mesh using a Triangle
    ///
    /// ```
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = vec![Triangle::new(
    ///     Point::new(0.0, 0.0, 0.0),
    ///     Point::new(10.0, 0.0, 0.0),
    ///     Point::new(10.0, -15.0, 0.0))];
    /// let actual = Mesh::from_triangles(input);
    /// let expected = Mesh::new(vec![0.0, 0.0, 0.0,
    ///                               10.0, 0.0, 0.0,
    ///                               10.0, -15.0, 0.0], vec![0, 1, 2]);
    ///
    /// assert_eq!(expected.eq(&actual), true);
    /// ```
    pub fn from_triangles(triangles: Vec<Triangle>) -> Mesh {
        let number_of_triangles: usize = triangles.len();
        let number_of_indices: usize = number_of_triangles * 3;
        let mut indices: Vec<usize> = Vec::<usize>::new();
        for i in 0..number_of_indices {
            indices.push(i);
        }
        let mut coordinates: Vec<f64> = Vec::<f64>::new();
        for i in 0..number_of_triangles {
            let current_triangle = &triangles[i];
            coordinates.push(current_triangle.first_point.x);
            coordinates.push(current_triangle.first_point.y);
            coordinates.push(current_triangle.first_point.z);

            coordinates.push(current_triangle.second_point.x);
            coordinates.push(current_triangle.second_point.y);
            coordinates.push(current_triangle.second_point.z);

            coordinates.push(current_triangle.third_point.x);
            coordinates.push(current_triangle.third_point.y);
            coordinates.push(current_triangle.third_point.z);
        }

        Mesh::new(coordinates, indices)
    }
    /// Converts [Mesh] into list of [ThreeEdgeGroup]s
    ///
    /// # Example
    ///
    /// Here is an example with simple 4 face planar [Mesh] converted into 4 [ThreeEdgeGroup]s.
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::three_edge_group::ThreeEdgeGroup;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Mesh::new(
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
    /// let actual = input.to_three_edge_groups();
    /// let expected = vec![
    ///     ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
    ///     ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
    ///     ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
    ///     ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
    /// ];
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert_eq!(expected[i].eq(&actual[i]), true);
    /// }
    /// ```
    pub fn to_three_edge_groups(&self) -> Vec<ThreeEdgeGroup> {
        let number_of_faces = self.get_number_of_faces();
        let mut three_edge_groups: Vec<ThreeEdgeGroup> = Vec::with_capacity(number_of_faces);
        for i in 0..number_of_faces {
            let first = Edge::new(self.indices[i*3], self.indices[i*3+1]);
            let second = Edge::new(self.indices[i*3+1], self.indices[i*3+2]);
            let third = Edge::new(self.indices[i*3+2], self.indices[i*3]);
            three_edge_groups.push(ThreeEdgeGroup::new(first, second, third));
        }

        three_edge_groups
    }

    /// Converts [Mesh] into list of [Edge]s
    ///
    /// # Example
    ///
    /// Here is an example with simple 4 face planar [Mesh] converted into [Edge]s.
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::mesh::Mesh;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::three_edge_group::ThreeEdgeGroup;
    /// use meshmeshmesh::triangle::Triangle;
    ///
    /// let input = Mesh::new(
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
    /// let actual = input.to_edges();
    /// let expected = vec![
    ///     Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0),
    ///     Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1),
    ///     Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2),
    ///     Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)
    /// ];
    /// assert_eq!(expected.len(), actual.len());
    /// for i in 0..expected.len() {
    ///     assert_eq!(expected[i].eq(&actual[i]), true);
    /// }
    /// ```
    pub fn to_edges(&self) -> Vec<Edge> {
        let number_of_faces = self.get_number_of_faces();
        let mut edges: Vec<Edge> = Vec::with_capacity(number_of_faces*3);
        for i in 0..number_of_faces {
            edges.push(Edge::new(self.indices[i*3], self.indices[i*3+1]));
            edges.push(Edge::new(self.indices[i*3+1], self.indices[i*3+2]));
            edges.push(Edge::new(self.indices[i*3+2], self.indices[i*3]));
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let result = Mesh::new(vec![0.0, 0.0, 0.0,
                                    10.0, 0.0, 0.0,
                                    10.0, -15.0, 0.0],
                               vec![0, 1, 2]);
        assert_eq!(result.coordinates, vec![0.0, 0.0, 0.0,
                                            10.0, 0.0, 0.0,
                                            10.0, -15.0, 0.0]);
        assert_eq!(result.indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_new_with_id() {
        let result = Mesh::new_with_id(Some(5), vec![0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 10.0, -15.0, 0.0], vec![0, 1, 2]);
        assert_eq!(result.id, Some(5));
        assert_eq!(result.coordinates, vec![0.0, 0.0, 0.0,
                                           10.0, 0.0, 0.0,
                                           10.0, -15.0, 0.0]);
        assert_eq!(result.indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_to_points() {
        let input = Mesh::new(vec![0.0, 0.0, 0.0,
                                   10.0, 0.0, 0.0,
                                   10.0, -15.0, 0.0],
                              vec![0, 1, 2]);
        let actual = input.to_points();
        let expected = vec![Point::new(0.0, 0.0, 0.0),
                            Point::new(10.0, 0.0, 0.0),
                            Point::new(10.0, -15.0, 0.0)];
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }

    #[test]
    fn test_to_triangles_1face() {
        let input = Mesh::new(vec![0.0, 0.0, 0.0,
                                   10.0, 0.0, 0.0,
                                   10.0, -15.0, 0.0],
                              vec![0, 1, 2]);
        let actual = input.to_triangles();
        let expected = vec![Triangle::new(
                            Point::new(0.0, 0.0, 0.0),
                            Point::new(10.0, 0.0, 0.0),
                            Point::new(10.0, -15.0, 0.0))];
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }

    #[test]
    fn test_to_triangles_pyramid() {
        let input = Mesh::new(
            vec![
                // Base
                0.0,0.0,0.0,
                10.0,0.0,0.0,
                10.0,10.0,0.0,
                0.0,10.0,0.0,

                // Top
                5.0,5.0,4.0
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
            ]
        );
        let actual = input.to_triangles();
        let expected = vec![
            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(10.0, 0.0, 0.0),
                Point::new(10.0,10.0,0.0)),
            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(10.0,10.0,0.0),
                Point::new(0.0,10.0,0.0)),

            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(10.0, 0.0, 0.0),
                Point::new(5.0,5.0,4.0)),
            Triangle::new(
                Point::new(10.0, 0.0, 0.0),
                Point::new(10.0,10.0,0.0),
                Point::new(5.0,5.0,4.0)),
            Triangle::new(
                Point::new(10.0,10.0,0.0),
                Point::new(0.0,10.0,0.0),
                Point::new(5.0,5.0,4.0)),
            Triangle::new(
                Point::new(0.0,10.0,0.0),
                Point::new(0.0,0.0,0.0),
                Point::new(5.0,5.0,4.0)),
        ];
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }

    #[test]
    fn test_from_triangles_1face() {
        let input = vec![Triangle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(10.0, 0.0, 0.0),
            Point::new(10.0, -15.0, 0.0))];
        let actual = Mesh::from_triangles(input);
        let expected = Mesh::new(vec![0.0, 0.0, 0.0,
                                   10.0, 0.0, 0.0,
                                   10.0, -15.0, 0.0],
                              vec![0, 1, 2]);
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_from_triangles_pyramid() {
        let input = vec![
            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(10.0, 0.0, 0.0),
                Point::new(10.0,10.0,0.0)),
            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(10.0,10.0,0.0),
                Point::new(0.0,10.0,0.0)),

            Triangle::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(10.0, 0.0, 0.0),
                Point::new(5.0,5.0,4.0)),
            Triangle::new(
                Point::new(10.0, 0.0, 0.0),
                Point::new(10.0,10.0,0.0),
                Point::new(5.0,5.0,4.0)),
            Triangle::new(
                Point::new(10.0,10.0,0.0),
                Point::new(0.0,10.0,0.0),
                Point::new(5.0,5.0,4.0)),
            Triangle::new(
                Point::new(0.0,10.0,0.0),
                Point::new(0.0,0.0,0.0),
                Point::new(5.0,5.0,4.0)),
        ];

        let actual = Mesh::from_triangles(input);
        let expected= Mesh::new(
            vec![
                0.0, 0.0, 0.0,
                10.0, 0.0, 0.0,
                10.0,10.0,0.0,

                0.0, 0.0, 0.0,
                10.0,10.0,0.0,
                0.0,10.0,0.0,


                0.0, 0.0, 0.0,
                10.0, 0.0, 0.0,
                5.0,5.0,4.0,

                10.0, 0.0, 0.0,
                10.0,10.0,0.0,
                5.0,5.0,4.0,

                10.0,10.0,0.0,
                0.0,10.0,0.0,
                5.0,5.0,4.0,

                0.0,10.0,0.0,
                0.0,0.0,0.0,
                5.0,5.0,4.0,
            ],
            vec![
                // Base faces
                0,1,2,
                3,4,5,

                // Side faces
                6,7,8,
                9,10,11,
                12,13,14,
                15,16,17
            ]
        );

        assert_eq!(expected.eq(&actual), true);
    }
    
    #[test]
    fn test_to_three_edge_groups() {
        let input = Mesh::new(
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
        let actual = input.to_three_edge_groups();
        let expected = vec![
            ThreeEdgeGroup::new(Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0)), // first face
            ThreeEdgeGroup::new(Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1)), // second face
            ThreeEdgeGroup::new(Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2)), // third face
            ThreeEdgeGroup::new(Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)), // fourth face
        ];
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }
    
    #[test]
    fn test_to_edges() {
        let input = Mesh::new(
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
        let actual = input.to_edges();
        let expected = vec![
            Edge::new(0, 2), Edge::new(2, 1), Edge::new(1, 0),
            Edge::new(1, 2), Edge::new(2, 3), Edge::new(3, 1),
            Edge::new(2, 4), Edge::new(4, 3), Edge::new(3, 2),
            Edge::new(1, 3), Edge::new(3, 5), Edge::new(5, 1)
        ];
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i].eq(&actual[i]), true);
        }
    }

    #[test]
    fn test_partialeq_true() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_with_id_true() {
        let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(7),vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_with_id_and_without_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                                                10.0, 0.0, 0.0,
                                                10.0, -15.0, 0.0],
                                  vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(7),vec![0.0, 0.0, 0.0,
                                               10.0, 0.0, 0.0,
                                               10.0, -15.0, 0.0],
                                  vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_with_id_false() {
        let a = Mesh::new_with_id(Some(7), vec![0.0, 0.0, 0.0,
                                                10.0, 0.0, 0.0,
                                                10.0, -15.0, 0.0],
                                  vec![0, 1, 2]);
        let b = Mesh::new_with_id(Some(6),vec![0.0, 0.0, 0.0,
                                               10.0, 0.0, 0.0,
                                               10.0, -15.0, 0.0],
                                  vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_coordinates_count_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0,
                               5.0, 1.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_coordinates_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 2.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_indices_count_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2, 2, 1, 0]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_different_indices_false() {
        let a = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 2, 1]);
        let b = Mesh::new(vec![0.0, 0.0, 0.0,
                               10.0, 0.0, 0.0,
                               10.0, -15.0, 0.0],
                          vec![0, 1, 2]);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }
}