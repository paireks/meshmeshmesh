use std::collections::HashSet;
use crate::scene::Scene;

impl Scene {
    /// Compares given [Scene] to other one, but with a `f64` tolerance for fields
    /// having `f64`.
    ///
    /// If any fields' value absolute difference is > tolerance, then it should return `false`.
    ///
    /// Other fields are compared same way as `PartialEq`.
    pub fn eq_with_tolerance(&self, other: &Self, tolerance: f64) -> bool {
        if self.schema_version != other.schema_version {
            return false;
        }
        if self.meshes.len() != other.meshes.len() {
            return false;
        }
        for i in 0..self.meshes.len() {
            if !self.meshes[i].eq_with_tolerance(&other.meshes[i], tolerance) {
                return false;
            }
        }
        if self.elements.len() != other.elements.len() {
            return false;
        }
        for i in 0..self.elements.len() {
            if !self.elements[i].eq_with_tolerance(&other.elements[i], tolerance) {
                return false;
            }
        }
        if !self.info.eq(&other.info) {
            return false;
        }
        true
    }
    
    /// Gets all `id`s of the [Mesh]es for which any [Element] applied `face_colors`.
    /// 
    /// This method can be useful for additional checks.
    pub (crate) fn get_mesh_ids_with_face_colors_applied(&self) -> HashSet<usize> {
        let mut set: HashSet<usize> = HashSet::new();

        for element in &self.elements {
            if element.face_colors.is_some() { 
                set.insert(element.mesh_id);
            }
        }
        
        set
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::color::Color;
    use crate::element::Element;
    use crate::mesh::Mesh;
    use crate::quaternion::Quaternion;
    use crate::vector::Vector;
    use super::*;

    fn get_file_with_triangle_blue_plate() -> Scene {
        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        Scene::new(
            String::from("1.0.0"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        )
    }

    #[test]
    fn test_partial_eq_true() {
        let a = get_file_with_triangle_blue_plate();
        let b = get_file_with_triangle_blue_plate();
        assert_eq!(a.eq_with_tolerance(&b, 0.001), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), true);
    }

    #[test]
    fn test_partial_eq_same_within_tolerance_true() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = Scene::new(
            String::from("1.0.0"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.00002,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.000001,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), true);
    }

    #[test]
    fn test_partial_eq_different_schema_version_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = Scene::new(
            String::from("Different"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_different_meshes_count_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = Scene::new(
            String::from("1.0.0"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ],
                ),
                Mesh::new_with_id(
                    Some(1),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ],
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_different_mesh_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = Scene::new(
            String::from("1.0.0"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,2,1 // Different
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_different_elements_count_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = Scene::new(
            String::from("1.0.0"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info.clone()
                ),
                Element::new(
                    0,
                    Vector::new(100.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("882ccb70-9925-4a10-82af-07c6fa2be5e7"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info.clone()
                )
            ],
            file_info
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_different_element_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("Jane Doe"));

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = Scene::new(
            String::from("1.0.0"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Something else"), // Different
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_different_file_info_false() {
        let a = get_file_with_triangle_blue_plate();

        let mut file_info: HashMap<String, String> = HashMap::new();
        file_info.insert(String::from("Author"), String::from("John Doe")); // different

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Name"), String::from("Triangle"));

        let b = Scene::new(
            String::from("1.0.0"),
            vec![
                Mesh::new_with_id(
                    Some(0),
                    vec![
                        0.0,0.0,0.0,
                        10.0,0.0,0.0,
                        10.0,-15.0,0.0
                    ],
                    vec![
                        0,1,2
                    ]
                )
            ],
            vec![
                Element::new(
                    0,
                    Vector::new(0.,0.,0.),
                    Quaternion::new(0.,0.,0.,1.0),
                    String::from("d4f28792-e1e9-4e31-bcee-740dbda61e20"),
                    String::from("Plate"),
                    Color::new(0,120,120,255),
                    None,
                    info
                )
            ],
            file_info
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }
}