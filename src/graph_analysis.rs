use std::collections::{HashMap, VecDeque};
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

    /// Splits [Graph] into separate parts that are isolated.
    pub(crate) fn split_disconnected_vertices(&self) -> Vec<Vec<usize>> {
        let mut isolated_groups: Vec<Vec<usize>> = Vec::new();
        
        let number_of_vertices: usize = self.get_number_of_vertices();
        let mut bfs = BFS {
            previous_vertex: vec![None; number_of_vertices],
            visited: vec![false; number_of_vertices],
            queue: VecDeque::new(),
        };
        
        while bfs.visited.contains(&false) {
            let mut new_isolated_group: Vec<usize> = Vec::new();
            
            let old_visited = bfs.visited.clone();
            let mut new_start_vertex = 0;
            for i in 0..number_of_vertices { // Finding the index of first not visited yet vertex
                let is_visited = bfs.visited[i];
                if !is_visited { 
                    new_start_vertex = i;
                }
            }
            bfs = self.main_bfs_loop(new_start_vertex, bfs); // BFS searching starting from this not visited
            
            for i in 0..number_of_vertices { // Comparison between before/after version of visited to get new isolated island
                if old_visited[i] != bfs.visited[i] { 
                    new_isolated_group.push(i);
                }
            }
            
            isolated_groups.push(new_isolated_group);
        }

        isolated_groups
    }


    pub(crate) fn split_disconnected_loops(&self) -> Vec<Vec<usize>> {
        let mut isolated_groups: Vec<Vec<usize>> = Vec::new();

        let number_of_vertices: usize = self.get_number_of_vertices();
        let mut bfs = BFS {
            previous_vertex: vec![None; number_of_vertices],
            visited: vec![false; number_of_vertices],
            queue: VecDeque::new(),
        };

        while bfs.visited.contains(&false) {
            let mut new_isolated_group: Vec<usize> = Vec::new();

            let old_visited = bfs.visited.clone();
            let mut new_start_vertex = 0;
            for i in 0..number_of_vertices { // Finding the index of first not visited yet vertex
                let is_visited = bfs.visited[i];
                if !is_visited {
                    new_start_vertex = i;
                }
            }
            bfs = self.main_bfs_loop(new_start_vertex, bfs); // BFS searching starting from this not visited

            for i in 0..number_of_vertices { // Comparison between before/after version of visited to get new isolated island
                if old_visited[i] != bfs.visited[i] {
                    new_isolated_group.push(i);
                }
            }

            isolated_groups.push(new_isolated_group);
        }

        let mut paths = Vec::with_capacity(isolated_groups.len());

        for isolated_group in isolated_groups {
            if isolated_group.len() > 2 {
                let mut forward_hashmap: HashMap<usize, usize> = HashMap::new();
                let mut start = usize::MAX;
                for i in isolated_group {
                    let previous_option = bfs.previous_vertex[i];
                    if previous_option.is_none() { 
                        start = i;
                    }
                    else {
                        let previous = previous_option.unwrap();
                        forward_hashmap.insert(previous, i);
                    }
                }
                
                let mut path = Vec::new();

                let mut current = start;
                while forward_hashmap.contains_key(&current) {
                    path.push(current);
                    current = forward_hashmap[&current];
                }
                path.push(current);

                paths.push(path);
            }
        }

        paths
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
        
        let actual = Graph::new(4, edges).is_connected();
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

        let actual = Graph::new(3, edges).is_connected();
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

        let actual = Graph::new(4, edges).is_connected();
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

        let actual = Graph::new(5, edges).is_connected();
        assert_eq!(actual, false);
    }
}