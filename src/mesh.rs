use serde::{Deserialize, Serialize};
use crate::point::Point;
use crate::triangle::Triangle;

/// Represents a mesh object in three-dimensional space.
#[derive(Deserialize, Serialize)]
pub struct Mesh {
    /// The list of coordinates for the mesh vertices.
    pub coordinates: Vec<f64>,
    /// The list of indices for the mesh triangles.
    pub indices: Vec<usize>,
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {

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
    /// Returns a new Mesh
    pub fn new(coordinates: Vec<f64>, indices: Vec<usize>) -> Mesh {Mesh {coordinates, indices}}

    /// Converts Mesh into list of Points
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

    /// Converts Mesh into list of Triangles
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

    /// Creates Mesh from list of Triangles
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
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use serde_json::to_string;
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

    #[test]
    fn test_to_json() {
        let input = Mesh::new(vec![0.0, 0.0, 0.0,
                                   10.0, 0.0, 0.0,
                                   10.0, -15.0, 0.0],
                              vec![0, 1, 2]);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"coordinates\":[0.0,0.0,0.0,10.0,0.0,0.0,10.0,-15.0,0.0],\"indices\":[0,1,2]}";
        let actual_result = from_str::<Mesh>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = Mesh::new(vec![0.0, 0.0, 0.0,
                                      10.0, 0.0, 0.0,
                                      10.0, -15.0, 0.0],
                                 vec![0, 1, 2]);
        assert_eq!(expected.eq(&actual), true);
    }
}