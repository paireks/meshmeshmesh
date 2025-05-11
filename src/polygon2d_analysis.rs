use std::collections::{HashMap, VecDeque};
use crate::graph::Graph;
use crate::point2d::{MonotoneVertexType, Point2D};
use crate::polygon2d::Polygon2D;

impl Polygon2D {
    /// Checks if given [Polygon2D] is clockwise.
    ///
    /// If `true` is returned: it should be clockwise.
    /// If `false`: it should be counter-clockwise.
    ///
    /// This method assumes normal cartesian coordinate system with the Y-axis pointing up.
    ///
    /// # Examples
    ///
    /// Here below there is an example of a clockwise [Polygon2D], so `true` is expected.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, -10.0)]);
    ///
    /// assert!(input.is_clockwise());
    ///
    /// ```
    ///
    /// Here below there is an example of counter-clockwise [Polygon2D], so `false` is expected.
    ///
    /// ```
    /// use meshmeshmesh::point2d::Point2D;
    /// use meshmeshmesh::polygon2d::Polygon2D;
    ///
    /// let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, -10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0)]);
    ///
    /// assert!(!input.is_clockwise());
    ///
    /// ```
    pub fn is_clockwise(&self) -> bool {
        let vertices_length = self.vertices.len();
        let mut sum = 0.0;
        for i in 0..vertices_length-1 {
            let x1 = self.vertices[i].x;
            let y1 = self.vertices[i].y;
            let x2 = self.vertices[i + 1].x;
            let y2 = self.vertices[i + 1].y;
            sum += (x2-x1) * (y2+y1);
        }
        let x1 = self.vertices[vertices_length - 1].x; // Last closing segment
        let y1 = self.vertices[vertices_length - 1].y;
        let x2 = self.vertices[0].x;
        let y2 = self.vertices[0].y;
        sum += (x2-x1) * (y2+y1);

        sum >= 0.0
    }

    /// Gets [MonotoneVertexType]s for given [Polygon2D] assuming it's clockwise.
    pub(crate) fn get_monotone_vertices_types_for_clockwise(&self) -> Vec<MonotoneVertexType> {
        let number_of_vertices = self.vertices.len();
        let mut monotone_vertex_types: Vec<MonotoneVertexType> = Vec::with_capacity(number_of_vertices);

        let previous = self.vertices[number_of_vertices - 1];
        let current = self.vertices[0];
        let next = self.vertices[1];
        monotone_vertex_types.push(current.get_monotone_vertex_type_for_clockwise(&previous, &next)); // first
        for i in 1..number_of_vertices - 1 {
            let previous = self.vertices[i-1];
            let current = self.vertices[i];
            let next = self.vertices[i+1];
            monotone_vertex_types.push(current.get_monotone_vertex_type_for_clockwise(&previous, &next)); // middle ones
        }
        let previous = self.vertices[number_of_vertices - 2];
        let current = self.vertices[number_of_vertices - 1];
        let next = self.vertices[0];
        monotone_vertex_types.push(current.get_monotone_vertex_type_for_clockwise(&previous, &next)); // last

        monotone_vertex_types
    }

    /// Gets [MonotoneVertexType]s for specific vertex of [Polygon2D] assuming it's clockwise.
    fn get_monotone_vertex_type_for_vertex_id_for_clockwise(&self, vertex_id: usize) -> MonotoneVertexType {
        let number_of_vertices = self.vertices.len();

        let current_vertex = self.vertices[vertex_id];
        let previous_vertex;
        let next_vertex;

        if vertex_id == 0 {
            previous_vertex = self.vertices[number_of_vertices - 1];
            next_vertex = self.vertices[1];
        }
        else if vertex_id == number_of_vertices - 1 {
            previous_vertex = self.vertices[number_of_vertices - 2];
            next_vertex = self.vertices[0];
        }
        else {
            previous_vertex = self.vertices[vertex_id - 1];
            next_vertex = self.vertices[vertex_id + 1];
        }

        current_vertex.get_monotone_vertex_type_for_clockwise(&previous_vertex, &next_vertex)
    }

    /// Gets the intersection between given [Polygon2D] and the y-line (horizontal infinite
    /// length line).
    ///
    /// The output is the [HashMap] where key is an id of the input [Polygon2D] segment, and the
    /// value is the x-position of intersection point. This way it is easier to know
    /// which segment is crossed and where exactly. The y-position is obviously the input y.
    ///
    /// It shouldn't take into an account parallel segments.
    fn get_hashmap_intersections_with_y(&self, y: f64) -> HashMap<usize, f64> {
        let mut intersections: HashMap<usize, f64> = HashMap::new();
        let number_of_vertices= self.vertices.len();

        for i in 0..number_of_vertices-1 { // Iterating segments
            let intersection = self.get_intersection_with_y_for_segment(y, i, i+1);
            if intersection.is_some() {
                intersections.insert(i, intersection.unwrap());
            }
        }

        let intersection = self.get_intersection_with_y_for_segment(y, number_of_vertices - 1, 0); // Last segment
        if intersection.is_some() {
            intersections.insert(number_of_vertices - 1, intersection.unwrap());
        }

        intersections
    }

    /// Gets the intersection between given [Polygon2D]s' specified segment and the y-line
    /// (horizontal infinite length line).
    ///
    /// The output is the `Option`: if `None` then there is no intersection, if value then it is
    /// the x-value of the intersection point. The y-value of intersection is obvious, because it
    /// is the input y value.
    ///
    /// It shouldn't take into an account parallel segments.
    fn get_intersection_with_y_for_segment(&self, y: f64, start_vertex_id: usize, end_vertex_id: usize) -> Option<f64> {
        let start_point = self.vertices[start_vertex_id];
        let end_point = self.vertices[end_vertex_id];

        if (start_point.y >= y && end_point.y < y) || (start_point.y <= y && end_point.y > y) { // Check if it even intersects
            let a = start_point.y - end_point.y; // Preparing Ax + By + C = 0 equation for that segment
            let b = end_point.x - start_point.x;
            let c = start_point.x * end_point.y - end_point.x * start_point.y;

            return Some((-b * y - c) / a); // x = (-By - C) / A
        }

        None
    }

    /// Creates the queue of vertices' ids where vertices at the front are vertices with the highest
    /// Y coordinate.
    ///
    /// If there are 2 points at the same height: left one will come first.
    fn get_queue_of_vertices_top_to_bottom_then_left_right(&self) -> VecDeque<usize> {
        let mut vertices_ids: Vec<usize> = Vec::from_iter(0..self.vertices.len());
        vertices_ids.sort_by(|&a, &b| self.vertices[a].total_cmp_top_bottom_then_left_right(&self.vertices[b]));
        VecDeque::from(vertices_ids)
    }

/*    /// Creates the queue of edges' ids that are intersecting Y axis on the given height.
    ///
    /// The order is from the left to right.
    fn get_queue_of_edges_intersecting_y_from_left_to_right(&self, y: f64) -> VecDeque<usize> {

    }*/

/*    /// Gets monotone [Graph].
    fn get_monotone_graph(&self) -> Graph {

        let mut d = Graph::from_polygon2d_into_directed(self);
        let mut q:VecDeque<usize> = self.get_queue_of_vertices_from_top_to_bottom();

        while q.len() != 0 {
            let current_vertex_id = q.pop_front().unwrap();
            let current_vertex_type = self.get_monotone_vertex_type_for_vertex_id_for_clockwise(current_vertex_id);
            d = Self::handle_vertex(d, current_vertex_id, current_vertex_type);
        }

        d
    }

   fn handle_vertex(mut d: Graph, vertex_id: usize, vertex_type: MonotoneVertexType) -> Graph {

    }*/
}

#[cfg(test)]
mod tests {
    use crate::point2d::Point2D;
    use super::*;

    #[test]
    fn test_is_clockwise_true() {
        let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, 10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, -10.0)]);

        assert!(input.is_clockwise());
    }

    #[test]
    fn test_is_clockwise_false() {
        let input = Polygon2D::new(vec![Point2D::new(0.0, 0.0), Point2D::new(5.0, -10.0), Point2D::new(10.0, 0.0), Point2D::new(5.0, 10.0)]);

        assert!(!input.is_clockwise());
    }
    
    #[test]
    fn test_get_monotone_vertices_types_for_clockwise() {
        let input = Polygon2D::new(vec![
           Point2D::new(-5.981672, 50.875287),
           Point2D::new(3.075768, 55.323137),
           Point2D::new(7.725793, 50.996592),
           Point2D::new(15.044527, 59.892292),
           Point2D::new(13.184517, 53.665302),
           Point2D::new(17.025842, 49.055712),
           Point2D::new(16.864102, 41.777413),
           Point2D::new(12.456687, 46.063523),
           Point2D::new(12.375817, 37.208258),
           Point2D::new(7.829037, 32.495452),
           Point2D::new(3.106803, 37.191157),
           Point2D::new(-1.456255, 32.548511),
           Point2D::new(-8.141664, 35.174922),
           Point2D::new(-10.590682, 46.392687),
           Point2D::new(-5.091522, 42.510927),
           Point2D::new(-1.290632, 46.433122),
        ]);
        
        let actual = input.get_monotone_vertices_types_for_clockwise();
        
        let expected: Vec<MonotoneVertexType> = vec![
            MonotoneVertexType::Regular,
            MonotoneVertexType::Start,
            MonotoneVertexType::Merge,
            MonotoneVertexType::Start,
            MonotoneVertexType::Regular,
            MonotoneVertexType::Regular,
            MonotoneVertexType::End,
            MonotoneVertexType::Split,
            MonotoneVertexType::Regular,
            MonotoneVertexType::End,
            MonotoneVertexType::Split,
            MonotoneVertexType::End,
            MonotoneVertexType::Regular,
            MonotoneVertexType::Start,
            MonotoneVertexType::Merge,
            MonotoneVertexType::Regular
        ];
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_hashmap_intersections_for_given_y_first() {
        let input = Polygon2D::new(vec![
            Point2D::new(-5.981672, 50.875287),
            Point2D::new(3.075768, 55.323137),
            Point2D::new(7.725793, 50.996592),
            Point2D::new(15.044527, 59.892292),
            Point2D::new(13.184517, 53.665302),
            Point2D::new(17.025842, 49.055712),
            Point2D::new(16.864102, 41.777413),
            Point2D::new(12.456687, 46.063523),
            Point2D::new(12.375817, 37.208258),
            Point2D::new(7.829037, 32.495452),
            Point2D::new(3.106803, 37.191157),
            Point2D::new(-1.456255, 32.548511),
            Point2D::new(-8.141664, 35.174922),
            Point2D::new(-10.590682, 46.392687),
            Point2D::new(-5.091522, 42.510927),
            Point2D::new(-1.290632, 46.433122),
        ]);

        let mut expected: HashMap<usize, f64> = HashMap::new();

        expected.insert(5, 17.01299610934615);
        expected.insert(15, -3.449702758515264);

        let actual = input.get_hashmap_intersections_with_y(48.477647);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_hashmap_intersections_for_given_y_second() {
        let input = Polygon2D::new(vec![
            Point2D::new(-5.981672, 50.875287),
            Point2D::new(3.075768, 55.323137),
            Point2D::new(7.725793, 50.996592),
            Point2D::new(15.044527, 59.892292),
            Point2D::new(13.184517, 53.665302),
            Point2D::new(17.025842, 49.055712),
            Point2D::new(16.864102, 41.777413),
            Point2D::new(12.456687, 46.063523),
            Point2D::new(12.375817, 37.208258),
            Point2D::new(7.829037, 32.495452),
            Point2D::new(3.106803, 37.191157),
            Point2D::new(-1.456255, 32.548511),
            Point2D::new(-8.141664, 35.174922),
            Point2D::new(-10.590682, 46.392687),
            Point2D::new(-5.091522, 42.510927),
            Point2D::new(-1.290632, 46.433122),
        ]);

        let mut expected: HashMap<usize, f64> = HashMap::new();

        expected.insert(8, 9.513355006380916);
        expected.insert(9, 6.073352755756808);
        expected.insert(10, 0.20748830202991556);
        expected.insert(11, -5.7651031999420415);

        let actual = input.get_hashmap_intersections_with_y(34.241273);

        assert_eq!(expected, actual);
    }
    
    #[test]
    fn test_get_queue_of_vertices_from_top_to_bottom_then_left_to_right_unregular() {
        let input = Polygon2D::new(vec![
            Point2D::new(-5.981672, 50.875287),
            Point2D::new(3.075768, 55.323137),
            Point2D::new(7.725793, 50.996592),
            Point2D::new(15.044527, 59.892292),
            Point2D::new(13.184517, 53.665302),
            Point2D::new(17.025842, 49.055712),
            Point2D::new(16.864102, 41.777413),
            Point2D::new(12.456687, 46.063523),
            Point2D::new(12.375817, 37.208258),
            Point2D::new(7.829037, 32.495452),
            Point2D::new(3.106803, 37.191157),
            Point2D::new(-1.456255, 32.548511),
            Point2D::new(-8.141664, 35.174922),
            Point2D::new(-10.590682, 46.392687),
            Point2D::new(-5.091522, 42.510927),
            Point2D::new(-1.290632, 46.433122),
        ]);
        
        let mut expected = VecDeque::new();
        expected.push_back(3);
        expected.push_back(1);
        expected.push_back(4);
        expected.push_back(2);
        expected.push_back(0);
        expected.push_back(5);
        expected.push_back(15);
        expected.push_back(13);
        expected.push_back(7);
        expected.push_back(14);
        expected.push_back(6);
        expected.push_back(8);
        expected.push_back(10);
        expected.push_back(12);
        expected.push_back(11);
        expected.push_back(9);

        let actual = input.get_queue_of_vertices_top_to_bottom_then_left_right();
        
        assert_eq!(expected, actual);
        
    }

    #[test]
    fn test_get_queue_of_vertices_from_top_to_bottom_then_left_to_right_letter_f() {
        let input = Polygon2D::new(vec![
            Point2D::new(50.0, 25.0),
            Point2D::new(50.0, 50.0),
            Point2D::new(65.0, 50.0),
            Point2D::new(65.0, 45.0),
            Point2D::new(55.0, 45.0),
            Point2D::new(55.0, 40.0),
            Point2D::new(65.0, 40.0),
            Point2D::new(65.0, 35.0),
            Point2D::new(55.0, 35.0),
            Point2D::new(55.0, 25.0),
        ]);

        let mut expected = VecDeque::new();
        expected.push_back(1);
        expected.push_back(2);
        expected.push_back(4);
        expected.push_back(3);
        expected.push_back(5);
        expected.push_back(6);
        expected.push_back(8);
        expected.push_back(7);
        expected.push_back(0);
        expected.push_back(9);

        let actual = input.get_queue_of_vertices_top_to_bottom_then_left_right();

        assert_eq!(expected, actual);

    }
}