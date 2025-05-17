use std::collections::{HashMap, VecDeque};
use crate::graph::Graph;
use crate::polygon2d::Polygon2D;

impl Polygon2D {
/*    /// Converts given [Polygon2D] into y-monotone pieces.
    pub fn to_y_monotone_pieces(&self) -> Vec<Polygon2D> {
        let y_monotone_graph = self.to_y_monotone_graph();
        self.convert_monotone_graph_to_polygon2ds(&y_monotone_graph)
    }

    /// Converts given [Polygon2D] into y-monotone [Graph].
    fn to_y_monotone_graph(&self) -> Graph {

        let vertex_types = self.get_monotone_vertices_types_for_clockwise();
        let mut d = Graph::from_polygon2d_into_directed(self);
        let mut q:VecDeque<usize> = self.get_queue_of_vertices_top_to_bottom_then_left_right();
        let mut helpers: HashMap<usize, usize> = HashMap::new(); // Key: edge id, value: vertex id of helper

        while q.len() != 0 {
            let current_vertex_id = q.pop_front().unwrap();
            let current_vertex_type = vertex_types[current_vertex_id];
            (d, helpers) = self.handle_from_top_to_bottom(d, helpers, current_vertex_id, current_vertex_type);
        }

        q = self.get_queue_of_vertices_bottom_to_top_then_right_left();
        helpers = HashMap::new();

        while q.len() != 0 {
            let current_vertex_id = q.pop_front().unwrap();
            let current_vertex_type = vertex_types[current_vertex_id];
            (d, helpers) = self.handle_from_bottom_to_top(d, helpers, current_vertex_id, current_vertex_type);
        }

        d
    }
    
    fn convert_monotone_graph_to_polygon2ds(&self, d: &Graph) -> Vec<Polygon2D> {
        
    }*/
}