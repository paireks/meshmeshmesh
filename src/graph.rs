use crate::edge::Edge;
use crate::face_neighbours::FaceNeighbours;

/// Graph representation.
/// 
/// This representation stores data in a specific way internally in a private fields.
/// 
/// These fields are accessible by getters.
/// 
/// They are private, because they are coupled: if the `edges` change, then `adjacency_` fields
/// should also change accordingly. Same with other coupled fields.
pub struct Graph {
    /// Number of all vertices in the [Graph].
    number_of_vertices: usize,
    
    /// List of [Edge]s that define a [Graph].
    edges: Vec<Edge>,

    /// Adjacency vertices. For each vertex it tells you all its neighbour vertices,
    /// by storing their ids.
    adjacency_vertices: Vec<Vec<usize>>,

    /// Adjacency edges. For each vertex it tells you all its neighbour edges, by storing edge ids.
    adjacency_edges: Vec<Vec<usize>>,
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.edges.eq(&other.edges)
    }
}

impl Graph {
    /// Creates a new [Graph].
    ///
    /// During Graph creation it should also internally calculate the adjacency fields.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)];
    ///
    /// let expected_adjacency_vertices = vec![
    ///     vec![1], // 0
    ///     vec![0, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![0], // 5
    /// ];
    ///
    /// let expected_adjacency_edges = vec![
    ///     vec![0], // 0
    ///     vec![1, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![5], // 5
    /// ];
    /// 
    /// let actual = Graph::new(6, edges);
    /// 
    /// assert!(actual.get_edges().eq(&vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)]));
    /// assert_eq!(actual.get_adjacency_vertices().clone(), expected_adjacency_vertices);
    /// assert_eq!(actual.get_adjacency_edges().clone(), expected_adjacency_edges);
    /// 
    /// ```
    pub fn new(number_of_vertices: usize, edges: Vec<Edge>) -> Graph {
        let adjacency_vertices = Self::create_adjacency_vertices(&edges, number_of_vertices);
        let adjacency_edges = Self::create_adjacency_edges(&edges, number_of_vertices);

        Graph {number_of_vertices, edges, adjacency_vertices, adjacency_edges}
    }

    /// Gets number of vertices in the [Graph].
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)];
    ///
    /// let actual = Graph::new(6, edges).get_number_of_vertices();
    /// 
    /// assert_eq!(actual, 6);
    /// 
    /// ```
    pub fn get_number_of_vertices(&self) -> usize {
        self.number_of_vertices.clone()
    }

    /// Gets [Edge]s of [Graph]. These Edges are defining the [Graph].
    ///
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)];
    ///
    /// let input = Graph::new(6, edges);
    /// 
    /// let actual = input.get_edges();
    /// let expected = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)];
    ///
    /// assert!(expected.eq(actual));
    /// 
    /// ```
    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    /// Get adjacency vertices. For each vertex it tells you all its neighbour vertices,
    /// by storing their ids.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)];
    /// let actual = Graph::new(6, edges).get_adjacency_vertices().clone();
    ///
    /// let expected = vec![
    ///     vec![1], // 0
    ///     vec![0, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![0], // 5
    /// ];
    ///
    /// assert_eq!(actual, expected);
    ///
    /// ```
    pub fn get_adjacency_vertices(&self) -> &Vec<Vec<usize>> {
        &self.adjacency_vertices
    }

    /// Adjacency edges. For each vertex it tells you all its neighbour edges, by storing edge ids.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)];
    /// let actual = Graph::new(6, edges).get_adjacency_edges().clone();
    ///
    /// let expected = vec![
    ///     vec![0], // 0
    ///     vec![1, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![5], // 5
    /// ];
    ///
    /// assert_eq!(actual, expected);
    ///
    /// ```
    pub fn get_adjacency_edges(&self) -> &Vec<Vec<usize>> {
        &self.adjacency_edges
    }
    
    /// Creates a [Graph] by looking at the `vec` of [FaceNeighbours].
    /// 
    /// This way it is possible to create a Graph showing which faces are connected together.
    /// 
    /// Indices of Graph edges are indices of faces this way.
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::face_neighbours::FaceNeighbours;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let input = vec![
    ///     FaceNeighbours::new(None, Some(1), None),
    ///     FaceNeighbours::new(Some(0), Some(2), Some(3)),
    ///     FaceNeighbours::new(None, None, Some(1)),
    ///     FaceNeighbours::new(Some(1), None, None),
    /// ];
    ///
    /// let actual = Graph::from_face_neighbours(&input);
    /// let expected = Graph::new(4, vec![
    ///     Edge::new(0, 1),
    ///     Edge::new(1, 0),
    ///     Edge::new(1, 2),
    ///     Edge::new(1, 3),
    ///     Edge::new(2, 1),
    ///     Edge::new(3, 1),
    /// ]);
    ///
    /// assert!(expected.eq(&actual));
    ///
    /// ```
    pub fn from_face_neighbours(face_neighbours: &Vec<FaceNeighbours>) -> Graph {
        let mut graph_edges: Vec<Edge> = Vec::new();
        let number_of_face_neighbours = face_neighbours.len();

        for i in 0..number_of_face_neighbours {
            let current_face_neighbours = face_neighbours[i];
            if current_face_neighbours.first.is_some() { 
                graph_edges.push(Edge::new(i, current_face_neighbours.first.unwrap()));
            }
            if current_face_neighbours.second.is_some() {
                graph_edges.push(Edge::new(i, current_face_neighbours.second.unwrap()));
            }
            if current_face_neighbours.third.is_some() {
                graph_edges.push(Edge::new(i, current_face_neighbours.third.unwrap()));
            }
        }
        
        Graph::new(number_of_face_neighbours, graph_edges)
    }

    /// Creates an undirected [Graph] from the `vec` of [Edge]s.
    pub fn from_edges_into_undirected(number_of_vertices: usize, edges: &Vec<Edge>) -> Graph {
        let graph_edges = Edge::get_unique_undirected(edges).into_iter().collect();

        Graph::new(number_of_vertices, graph_edges)
    }

    fn create_adjacency_vertices(edges: &Vec<Edge>, number_of_vertices: usize) -> Vec<Vec<usize>> {
        let mut adjacency_vertices = vec![Vec::new(); number_of_vertices];

        for edge in edges {
            adjacency_vertices[edge.start].push(edge.end);
        }

        adjacency_vertices
    }

    fn create_adjacency_edges(edges: &Vec<Edge>, number_of_vertices: usize) -> Vec<Vec<usize>> {
        let mut adjacency_edges = vec![Vec::new(); number_of_vertices];
        let number_of_edges = edges.len();
        
        for i in 0..number_of_edges {
            let current_edge = edges[i];
            adjacency_edges[current_edge.start].push(i);
        }

        adjacency_edges
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_new() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 3),
            Edge::new(1, 4),
            Edge::new(5, 0)
        ];
        
        let expected_adjacency_vertices = vec![
            vec![1], // 0
            vec![0, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![0], // 5
        ];
        
        let expected_adjacency_edges = vec![
            vec![0], // 0
            vec![1, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![5], // 5
        ];
        
        let actual = Graph::new(6, edges);
        
        assert!(actual.edges.eq(&vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)]));
        assert_eq!(actual.adjacency_vertices, expected_adjacency_vertices);
        assert_eq!(actual.adjacency_edges, expected_adjacency_edges);
        
    }
    
    #[test]
    fn test_get_number_of_vertices() {
        let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(5, 0)];
        
        let actual = Graph::new(6, edges).get_number_of_vertices();
        
        assert_eq!(actual, 6);
    }

    #[test]
    fn get_adjacency_vertices() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 3),
            Edge::new(1, 4),
            Edge::new(5, 0)
        ];

        let actual = Graph::new(6, edges).get_adjacency_vertices().clone();

        let expected = vec![
            vec![1], // 0
            vec![0, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![0], // 5
        ];

        assert_eq!(actual, expected);

    }

    #[test]
    fn get_adjacency_edges() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 3),
            Edge::new(1, 4),
            Edge::new(5, 0)
        ];
        let actual = Graph::new(6, edges).get_adjacency_edges().clone();

        let expected = vec![
            vec![0], // 0
            vec![1, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![5], // 5
        ];

        assert_eq!(actual, expected);

    }
    
    #[test]
    fn test_from_face_neighbours() {
        let input = vec![
            FaceNeighbours::new(None, Some(1), None),
            FaceNeighbours::new(Some(0), Some(2), Some(3)),
            FaceNeighbours::new(None, None, Some(1)),
            FaceNeighbours::new(Some(1), None, None),
        ];
        
        let actual = Graph::from_face_neighbours(&input);
        let expected = Graph::new(4, vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(1, 3),
            Edge::new(2, 1),
            Edge::new(3, 1),
        ]);
        
        assert!(expected.eq(&actual));
    }

    #[test]
    fn test_from_face_neighbours_isolated_triangles() {
        let input = vec![
            FaceNeighbours::new(None, Some(1), None),
            FaceNeighbours::new(Some(0), Some(2), Some(4)),
            FaceNeighbours::new(None, None, Some(1)),
            FaceNeighbours::new(None, None, None), // isolated triangle
            FaceNeighbours::new(Some(1), None, None),
            FaceNeighbours::new(None, None, None), // isolated triangle
        ];

        let actual = Graph::from_face_neighbours(&input);
        let expected = Graph::new(5 ,vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(1, 4),
            Edge::new(2, 1),
            Edge::new(4, 1),
        ]);

        assert!(expected.eq(&actual));
    }

    #[test]
    fn test_from_edges_into_undirected() {
        let input: Vec<Edge> = vec![
            Edge::new(0, 1),
            Edge::new(1, 2),
            Edge::new(2, 0),
            Edge::new(1, 0),
            Edge::new(0, 3),
            Edge::new(3, 1),
        ];

        let actual = Graph::from_edges_into_undirected(4, &input);

        let expected_edges: Vec<Edge> = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 1),
            Edge::new(2, 0),
            Edge::new(0, 2),
            Edge::new(0, 3),
            Edge::new(3, 0),
            Edge::new(3, 1),
            Edge::new(1, 3),
        ];

        let expected = Graph::new(4, expected_edges);

        let a: HashSet<Edge> = expected.edges.into_iter().collect();
        let b: HashSet<Edge> = actual.edges.into_iter().collect();

        assert_eq!(a, b);
    }
}