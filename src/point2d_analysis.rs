use crate::point2d::{MonotoneVertexType, Point2D};
use crate::vector2d::Vector2D;

impl Point2D {
    pub fn get_monotone_vertex_type_for_clockwise(&self, previous: &Point2D, next: &Point2D) -> MonotoneVertexType {
        let inner_angle = self.get_inner_angle_for_clockwise(previous, next);

        if self.are_both_neighbours_lower(previous, next) {
            if inner_angle < std::f64::consts::PI {
                return MonotoneVertexType::Start;
            }
            if inner_angle > std::f64::consts::PI {
                return MonotoneVertexType::Split;
            }
        }

        if self.are_both_neighbours_higher(previous, next) {
            if inner_angle < std::f64::consts::PI {
                return MonotoneVertexType::End;
            }
            if inner_angle > std::f64::consts::PI {
                return MonotoneVertexType::Merge;
            }
        }

        MonotoneVertexType::Regular
    }

    pub fn get_inner_angle_for_clockwise(&self, previous: &Point2D, next: &Point2D) -> f64 {
        let a = Vector2D::from_2_points(self, previous);
        let b = Vector2D::from_2_points(self, next);
        let angle = a.get_angle(&b);

        if self.is_clockwise(previous, next) {
            return angle;
        }

        (2.0 * std::f64::consts::PI) - angle
    }

    pub fn is_clockwise(&self, previous: &Point2D, next: &Point2D) -> bool {
        let mut sum = 0.0;

        let x1 = previous.x;
        let y1 = previous.y;
        let x2 = self.x;
        let y2 = self.y;
        sum += (x2-x1) * (y2+y1);

        let x1 = self.x;
        let y1 = self.y;
        let x2 = next.x;
        let y2 = next.y;
        sum += (x2-x1) * (y2+y1);

        let x1 = next.x;
        let y1 = next.y;
        let x2 = previous.x;
        let y2 = previous.y;
        sum += (x2-x1) * (y2+y1);

        sum >= 0.0
    }

    pub fn are_both_neighbours_lower(&self, first: &Point2D, second: &Point2D) -> bool {
        first.y < self.y && second.y < self.y
    }

    pub fn are_both_neighbours_higher(&self, first: &Point2D, second: &Point2D) -> bool {
        first.y > self.y && second.y > self.y
    }
}