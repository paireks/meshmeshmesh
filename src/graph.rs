use crate::edge::Edge;

/// Graph representation.
/// 
/// This representation stores data in a specific way internally in a private fields.
/// 
/// These fields are accessible by getters.
/// 
/// They are private, because they are coupled: if the `edges` change, then `adjacency_` fields
/// should also change accordingly.
pub struct Graph {
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
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(6, 0)];
    ///
    /// let expected_adjacency_vertices = vec![
    ///     vec![1], // 0
    ///     vec![0, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![], // 5 (not existent on any edge)
    ///     vec![0], // 6
    /// ];
    ///
    /// let expected_adjacency_edges = vec![
    ///     vec![0], // 0
    ///     vec![1, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![], // 5 (not existent on any edge)
    ///     vec![5], // 6
    /// ];
    /// 
    /// let actual = Graph::new(edges);
    /// 
    /// assert!(actual.get_edges().eq(&vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(6, 0)]));
    /// assert_eq!(actual.get_adjacency_vertices(), expected_adjacency_vertices);
    /// assert_eq!(actual.get_adjacency_edges(), expected_adjacency_edges);
    /// 
    /// ```
    pub fn new(edges: Vec<Edge>) -> Graph {

        let number_of_vertices = Edge::get_flatten_from_edges(&edges).iter().max().unwrap().clone() + 1; // It assumes that max index can tell the number of vertices.

        let adjacency_vertices = Self::create_adjacency_vertices(&edges, number_of_vertices);
        let adjacency_edges = Self::create_adjacency_edges(&edges, number_of_vertices);

        Graph {edges, adjacency_vertices, adjacency_edges}
    }

    /// Gets [Edge]s of [Graph]. These Edges are defining the [Graph].
    ///
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(6, 0)];
    ///
    /// let actual = Graph::new(edges).get_edges();
    ///
    /// assert!(actual.eq(&vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(6, 0)]));
    /// ```
    pub fn get_edges(&self) -> Vec<Edge> {
        self.edges.clone()
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
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(6, 0)];
    /// let actual = Graph::new(edges).get_adjacency_vertices();
    ///
    /// let expected = vec![
    ///     vec![1], // 0
    ///     vec![0, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![], // 5 (not existent on any edge)
    ///     vec![0], // 6
    /// ];
    ///
    /// assert_eq!(actual, expected);
    ///
    /// ```
    pub fn get_adjacency_vertices(&self) -> Vec<Vec<usize>> {
        self.adjacency_vertices.clone()
    }

    /// Adjacency edges. For each vertex it tells you all its neighbour edges, by storing edge ids.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::edge::Edge;
    /// use meshmeshmesh::graph::Graph;
    ///
    /// let edges = vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(6, 0)];
    /// let actual = Graph::new(edges).get_adjacency_edges();
    ///
    /// let expected = vec![
    ///     vec![0], // 0
    ///     vec![1, 2, 4], // 1
    ///     vec![3], // 2
    ///     vec![], // 3
    ///     vec![], // 4
    ///     vec![], // 5 (not existent on any edge)
    ///     vec![5], // 6
    /// ];
    ///
    /// assert_eq!(actual, expected);
    ///
    /// ```
    pub fn get_adjacency_edges(&self) -> Vec<Vec<usize>> {
        self.adjacency_edges.clone()
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
    use super::*;

    #[test]
    fn test_new() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 3),
            Edge::new(1, 4),
            Edge::new(6, 0)
        ];
        
        let expected_adjacency_vertices = vec![
            vec![1], // 0
            vec![0, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![], // 5 (not existent on any edge)
            vec![0], // 6
        ];
        
        let expected_adjacency_edges = vec![
            vec![0], // 0
            vec![1, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![], // 5 (not existent on any edge)
            vec![5], // 6
        ];
        
        let actual = Graph::new(edges);
        
        assert!(actual.edges.eq(&vec![Edge::new(0, 1), Edge::new(1, 0), Edge::new(1, 2), Edge::new(2, 3), Edge::new(1, 4), Edge::new(6, 0)]));
        assert_eq!(actual.adjacency_vertices, expected_adjacency_vertices);
        assert_eq!(actual.adjacency_edges, expected_adjacency_edges);
        
    }

    #[test]
    fn get_adjacency_vertices() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 3),
            Edge::new(1, 4),
            Edge::new(6, 0)
        ];

        let actual = Graph::new(edges).get_adjacency_vertices();

        let expected = vec![
            vec![1], // 0
            vec![0, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![], // 5 (not existent on any edge)
            vec![0], // 6
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
            Edge::new(6, 0)
        ];
        let actual = Graph::new(edges).get_adjacency_edges();

        let expected = vec![
            vec![0], // 0
            vec![1, 2, 4], // 1
            vec![3], // 2
            vec![], // 3
            vec![], // 4
            vec![], // 5 (not existent on any edge)
            vec![5], // 6
        ];

        assert_eq!(actual, expected);

    }
}