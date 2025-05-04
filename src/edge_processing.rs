use std::collections::HashSet;
use crate::edge::Edge;

impl Edge {
    /// Creates a `HashSet` of [Edge]s, so that the output will describe set of *undirected* Edges.
    /// 
    /// Internally undirected edge is stored as 2 [Edge]s with the same vertices, but different
    /// directions.
    /// 
    /// In other words what this algorithm is doing is it duplicates [Edge]s that seems to be
    /// directional in the input and reverse these duplicates.
    /// 
    /// # Example
    /// 
    /// ```
    /// use std::collections::HashSet;
    /// use meshmeshmesh::edge::Edge;
    /// let input = vec![
    ///     Edge::new(0, 1), // this one won't be duplicated, cause already has a reversed version below
    ///     Edge::new(1, 0), // reversed version here
    ///     Edge::new(1, 2), // this one will be duplicated and reversed
    ///     Edge::new(3, 5), // this one will be duplicated and reversed
    ///     Edge::new(3, 5), // this is a duplication that should be removed
    /// ];
    ///
    /// let actual = Edge::get_unique_undirected(&input);
    ///
    /// let mut expected = HashSet::new();
    /// expected.insert(Edge::new(0, 1));
    /// expected.insert(Edge::new(1, 0));
    /// expected.insert(Edge::new(1, 2));
    /// expected.insert(Edge::new(2, 1));
    /// expected.insert(Edge::new(3, 5));
    /// expected.insert(Edge::new(5, 3));
    /// 
    /// assert_eq!(expected, actual);
    /// 
    /// ```
    pub fn get_unique_undirected(edges: &Vec<Edge>) -> HashSet<Edge> {
        let mut set: HashSet<Edge> = HashSet::new();

        for edge in edges {
            set.insert(edge.clone());
            set.insert(edge.get_reversed());
        }
        
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_unique_undirected() {
        let input = vec![
            Edge::new(0, 1), // this one won't be duplicated, cause already has a reversed version below
            Edge::new(1, 0), // reversed version here
            Edge::new(1, 2), // this one will be duplicated and reversed
            Edge::new(3, 5), // this one will be duplicated and reversed
            Edge::new(3, 5), // this is a duplication that should be removed
        ];
        
        let actual = Edge::get_unique_undirected(&input);
        
        let mut expected = HashSet::new();
        expected.insert(Edge::new(0, 1));
        expected.insert(Edge::new(1, 0));
        expected.insert(Edge::new(1, 2));
        expected.insert(Edge::new(2, 1));
        expected.insert(Edge::new(3, 5));
        expected.insert(Edge::new(5, 3));
        
        assert_eq!(expected, actual);
        
    }
}