use std::collections::VecDeque;
use crate::graph::Graph;

/// Data structure helpful for keeping information related to breadth-first search.
struct BFS {
    /// Vec of previous vertex id for each [Graph] vertex
    pub previous_vertex: Vec<Option<usize>>,

    /// Vec of the bool information if specific vertex was already visited or not
    pub visited: Vec<bool>,

    /// Queue used during BFS loop
    pub queue: VecDeque<usize>,
}

impl Graph {
    /// Checks if this [Graph] is connected or not using breadth-first search.
    /// It will only work properly if the Graph has all the edges undirected. It is like that,
    /// because if some of the edges are directed, then depending on where you start it can still
    /// go visit all vertices, but from the other start it won't. When all the edges are undirected
    /// then it doesn't matter where you start. This method starts arbitrally on the vertex with
    /// id = 0.
    pub fn is_connected(&self) -> bool {
        let number_of_vertices: usize = self.get_number_of_vertices();
        let mut bfs = BFS {
            previous_vertex: vec![None; number_of_vertices],
            visited: vec![false; number_of_vertices],
            queue: VecDeque::new(),
        };
        bfs = self.main_bfs_loop(0, bfs);
        !bfs.visited.contains(&false)
    }


    /// It is the main breadth-first search loop.
    fn main_bfs_loop(&self, start_vertex: usize, mut bfs: BFS) ->  BFS{
        let adjacency_vertices = self.get_adjacency_vertices();

        bfs.queue.push_front(start_vertex);
        bfs.visited[start_vertex] = true;

        while bfs.queue.len() != 0 {
            let vertex = bfs.queue.pop_front().unwrap();
            let neighbours = &adjacency_vertices[vertex];
            for neighbour in neighbours {
                let neighbour_clone = neighbour.clone();
                if bfs.visited[neighbour_clone] {
                    continue
                }

                bfs.queue.push_front(neighbour_clone);
                bfs.visited[neighbour_clone] = true;
                bfs.previous_vertex[neighbour_clone] = Some(vertex);
            }
        }

        bfs
    }
}

#[cfg(test)]
mod tests {
    use crate::edge::Edge;
    use super::*;

    #[test]
    fn test_is_connected_diamond_true() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(0, 2),
            Edge::new(2, 0),
            Edge::new(1, 3),
            Edge::new(3, 1),
            Edge::new(2, 3),
            Edge::new(3, 2),
        ];
        
        let actual = Graph::new(edges).is_connected();
        assert_eq!(actual, true);
        
    }

    #[test]
    fn test_is_connected_triangle_true() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 1),
            Edge::new(2, 0),
            Edge::new(0, 2),
        ];

        let actual = Graph::new(edges).is_connected();
        assert_eq!(actual, true);

    }

    #[test]
    fn test_is_connected_triangle_with_line_true() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 1),
            Edge::new(2, 0),
            Edge::new(0, 2),
            Edge::new(2, 3),
            Edge::new(3, 2),
        ];

        let actual = Graph::new(edges).is_connected();
        assert_eq!(actual, true);

    }

    #[test]
    fn test_is_connected_triangle_with_separate_line_false() {
        let edges = vec![
            Edge::new(0, 1),
            Edge::new(1, 0),
            Edge::new(1, 2),
            Edge::new(2, 1),
            Edge::new(2, 0),
            Edge::new(0, 2),
            Edge::new(3, 4),
            Edge::new(4, 3),
        ];

        let actual = Graph::new(edges).is_connected();
        assert_eq!(actual, false);
    }
}