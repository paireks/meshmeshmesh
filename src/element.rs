use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::color::Color;
use crate::rotation::Rotation;
use crate::vector::Vector;

/// Represents an element represented by single [Mesh] in three-dimensional space + metadata.
#[derive(Deserialize, Serialize)]
pub struct Element {
    /// The identifier of the mesh associated with the element.
    pub mesh_id: usize,
    /// The position of the element.
    pub vector: Vector,
    /// The rotation of the element.
    pub rotation: Rotation,
    /// The globally unique identifier for the element.
    pub guid: String,
    /// The type of the element.
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub element_type: String,
    /// The color of the element.
    pub color: Color,
    /// The list of integers, that determine face colors of a mesh.
    /// They should be organized like this: [r1, g1, b1, a1, r2, g2, b2, a2, r3, g3, b3, a3, ... rn, gn, bn, an]
    /// E.g. list like: [255, 0, 0, 255, 135, 206, 235, 255, 255, 255, 255, 255]
    /// means first triangle should be colored as red (255,0,0,255),
    /// second as skyblue (135,206,235,255),
    /// third as white (255,255,255,255).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_colors: Option<Vec<i32>>,
    /// Additional information about the element.
    pub info: HashMap<String, String>
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        if self.mesh_id != other.mesh_id {
            return false;
        }
        if self.vector != other.vector {
            return false;
        }
        if self.rotation != other.rotation {
            return false;
        }
        if self.guid != other.guid {
            return false;
        }
        if self.element_type != other.element_type {
            return false;
        }
        if self.color != other.color {
            return false;
        }
        if self.face_colors.is_none() && other.face_colors.is_none() {

        } else {
            if self.face_colors.is_some() && other.face_colors.is_some() {
                let self_face_colors_unpacked = self.face_colors.as_ref().unwrap();
                let other_face_colors_unpacked = other.face_colors.as_ref().unwrap();
                if self_face_colors_unpacked.len() != other_face_colors_unpacked.len() {
                    return false;
                }
                for i in 0..self_face_colors_unpacked.len() {
                    if self_face_colors_unpacked[i] != other_face_colors_unpacked[i] {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }

        if !self.info.eq(&other.info) {
            return false;
        }

        true
    }
}

impl Element {
    /// Returns a new Element
    pub fn new(mesh_id: usize, vector: Vector, rotation: Rotation, guid: String, element_type: String,
               color: Color, face_colors: Option<Vec<i32>>, info: HashMap<String, String>)
               -> Element { Element { mesh_id, vector, rotation, guid, element_type, color, face_colors, info } }
}

#[cfg(test)]
mod tests {
    use serde_json::to_string;
    use serde_json::from_str;
    use super::*;

    fn get_blue_test_element() -> Element {
        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        )
    }

    fn get_face_colored_test_element() -> Element {
        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        Element::new(
            0,
            Vector::new(0.0, 0.0, 0.0),
            Rotation::new(0.0, 0.0, 0.0, 1.0),
            String::from("3028896f-cd51-4b3a-be54-08841b4e9081"),
            String::from("Cube"),
            Color::new(0,0,255,0),
            Some(vec![
                // Front side
                255, 105, 180, 150, // Hot pink with transparency
                255, 192, 203, 255, // Pink

                // Bottom side
                53, 57, 53, 255, // Onyx
                0, 0, 0, 255, // Black

                // Left side
                243, 229, 171, 255, // Vanilla
                255, 255, 0, 255, // Yellow

                // Right side
                9, 121, 105, 255, // Cadmium Green
                0, 128, 0, 255, // Green

                // Top side
                0, 255, 255, 255, // Cyan
                0, 0, 255, 255, // Blue

                // Back side
                226, 223, 210, 255, // Pearl
                255, 255, 255, 255, // White
            ]),
            info.clone(),
        )
    }

    #[test]
    fn test_new_without_face_colors() {
        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let result = get_blue_test_element();

        assert_eq!(result.mesh_id, 4);
        assert_eq!(Vector::new(0.2, 0.3, 0.4).eq(&result.vector), true);
        assert_eq!(Rotation::new(1.0, 1.5, 2.0, 2.5).eq(&result.rotation), true);
        assert_eq!(String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353").eq(&result.guid), true);
        assert_eq!(String::from("Plate").eq(&result.element_type), true);
        assert_eq!(Color::new(0,0,255,0).eq(&result.color), true);
        assert_eq!(result.face_colors.is_none(), true);
        assert_eq!(result.face_colors.is_some(), false);
        assert_eq!(info.eq(&result.info), true);
    }

    #[test]
    fn test_new_with_face_colors() {
        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));
        let expected_face_colors = vec![
            // Front side
            255, 105, 180, 150, // Hot pink with transparency
            255, 192, 203, 255, // Pink

            // Bottom side
            53, 57, 53, 255, // Onyx
            0, 0, 0, 255, // Black

            // Left side
            243, 229, 171, 255, // Vanilla
            255, 255, 0, 255, // Yellow

            // Right side
            9, 121, 105, 255, // Cadmium Green
            0, 128, 0, 255, // Green

            // Top side
            0, 255, 255, 255, // Cyan
            0, 0, 255, 255, // Blue

            // Back side
            226, 223, 210, 255, // Pearl
            255, 255, 255, 255, // White
        ];

        let result = get_face_colored_test_element();

        assert_eq!(result.mesh_id, 0);
        assert_eq!(Vector::new(0.0, 0.0, 0.0).eq(&result.vector), true);
        assert_eq!(Rotation::new(0.0, 0.0, 0.0, 1.0).eq(&result.rotation), true);
        assert_eq!(String::from("3028896f-cd51-4b3a-be54-08841b4e9081").eq(&result.guid), true);
        assert_eq!(String::from("Cube").eq(&result.element_type), true);
        assert_eq!(Color::new(0,0,255,0).eq(&result.color), true);
        assert_eq!(result.face_colors.is_none(), false);
        assert_eq!(result.face_colors.is_some(), true);
        let face_colors_unpacked = result.face_colors.as_ref().unwrap();
        assert_eq!(face_colors_unpacked.len(), expected_face_colors.len());
        for i in 0..face_colors_unpacked.len() {
            assert_eq!(face_colors_unpacked[i], expected_face_colors[i]);
        }
        assert_eq!(info.eq(&result.info), true);
    }

    #[test]
    fn test_partial_eq_without_face_colors_true(){
        let a = get_blue_test_element();
        let b = get_blue_test_element();
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_mesh_id(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            3, // different
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_vector(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.1, 0.3, 0.4), // different
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_rotation(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, -0.5, 2.0, 2.5), // different
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_guid(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b34a1674-e680-40f2-baa9-0e9b017bea14"), // different
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_type(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Another one"), // different
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_color(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(55,0,255,0), // different
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_existing_face_colors(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            Some(vec![255, 0, 0, 0]), // different
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_face_colors(){
        let a = get_face_colored_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            0,
            Vector::new(0.0, 0.0, 0.0),
            Rotation::new(0.0, 0.0, 0.0, 1.0),
            String::from("3028896f-cd51-4b3a-be54-08841b4e9081"),
            String::from("Cube"),
            Color::new(0,0,255,0),
            Some(vec![
                // Front side
                255, 105, 180, 150, // Hot pink with transparency
                255, 192, 203, 255, // Pink

                // Bottom side
                53, 57, 53, 255, // Onyx
                1, 0, 0, 255, // Black <- Different

                // Left side
                243, 229, 171, 255, // Vanilla
                255, 255, 0, 255, // Yellow

                // Right side
                9, 121, 105, 255, // Cadmium Green
                0, 128, 0, 255, // Green

                // Top side
                0, 255, 255, 255, // Cyan
                0, 0, 255, 255, // Blue

                // Back side
                226, 223, 210, 255, // Pearl
                255, 255, 255, 255, // White
            ]),
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_face_colors_count(){
        let a = get_face_colored_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            0,
            Vector::new(0.0, 0.0, 0.0),
            Rotation::new(0.0, 0.0, 0.0, 1.0),
            String::from("3028896f-cd51-4b3a-be54-08841b4e9081"),
            String::from("Cube"),
            Color::new(0,0,255,0),
            Some(vec![
                // Front side
                255, 105, 180, 150, // Hot pink with transparency
                255, 192, 203, 255, // Pink

                // Bottom side
                53, 57, 53, 255, // Onyx
                1, 0, 0, 255, // Black <- Different

                // Left side
                243, 229, 171, 255, // Vanilla
                255, 255, 0, 255, // Yellow

                // Right side
                9, 121, 105, 255, // Cadmium Green
                0, 128, 0, 255, // Green

                // Top side
                0, 255, 255, 255, // Cyan
                0, 0, 255, 255, // Blue

                // Back side
                226, 223, 210, 255, // Pearl
                255, 255, 255, //255, // White <- different
            ]),
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_info_value(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Another value")); // different

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_info_key(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Another key"), String::from("Value")); // different

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_info_length(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));
        info.insert(String::from("Another key"), String::from("Another value")); // different

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Rotation::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partial_eq_with_face_colors_true(){
        let a = get_face_colored_test_element();
        let b = get_face_colored_test_element();
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_to_json_without_face_colors() {
        let input = get_blue_test_element();
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"mesh_id\":4,\"vector\":{\"x\":0.2,\"y\":0.3,\"z\":0.4},\"rotation\":{\"qx\":1.0,\"qy\":1.5,\"qz\":2.0,\"qw\":2.5},\"guid\":\"b8a7a2ed-0c30-4c20-867e-baa1ef7b8353\",\"type\":\"Plate\",\"color\":{\"r\":0,\"g\":0,\"b\":255,\"a\":0},\"info\":{\"Key\":\"Value\"}}");
    }

    #[test]
    fn test_from_json_without_face_colors() {
        let json = "{\"mesh_id\":4,\"vector\":{\"x\":0.2,\"y\":0.3,\"z\":0.4},\"rotation\":{\"qx\":1.0,\"qy\":1.5,\"qz\":2.0,\"qw\":2.5},\"guid\":\"b8a7a2ed-0c30-4c20-867e-baa1ef7b8353\",\"type\":\"Plate\",\"color\":{\"r\":0,\"g\":0,\"b\":255,\"a\":0},\"info\":{\"Key\":\"Value\"}}";
        let actual_result = from_str::<Element>(json);
        // println!("{}", actual_result.err().unwrap());
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = get_blue_test_element();
        assert_eq!(expected.eq(&actual), true);
    }

    #[test]
    fn test_to_json_with_face_colors() {
        let input = get_face_colored_test_element();
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"mesh_id\":0,\"vector\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"rotation\":{\"qx\":0.0,\"qy\":0.0,\"qz\":0.0,\"qw\":1.0},\"guid\":\"3028896f-cd51-4b3a-be54-08841b4e9081\",\"type\":\"Cube\",\"color\":{\"r\":0,\"g\":0,\"b\":255,\"a\":0},\"face_colors\":[255,105,180,150,255,192,203,255,53,57,53,255,0,0,0,255,243,229,171,255,255,255,0,255,9,121,105,255,0,128,0,255,0,255,255,255,0,0,255,255,226,223,210,255,255,255,255,255],\"info\":{\"Key\":\"Value\"}}");
    }

    #[test]
    fn test_from_json_with_face_colors() {
        let json = "{\"mesh_id\":0,\"vector\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"rotation\":{\"qx\":0.0,\"qy\":0.0,\"qz\":0.0,\"qw\":1.0},\"guid\":\"3028896f-cd51-4b3a-be54-08841b4e9081\",\"type\":\"Cube\",\"color\":{\"r\":0,\"g\":0,\"b\":255,\"a\":0},\"face_colors\":[255,105,180,150,255,192,203,255,53,57,53,255,0,0,0,255,243,229,171,255,255,255,0,255,9,121,105,255,0,128,0,255,0,255,255,255,0,0,255,255,226,223,210,255,255,255,255,255],\"info\":{\"Key\":\"Value\"}}";
        let actual_result = from_str::<Element>(json);
        // println!("{}", actual_result.err().unwrap());
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = get_face_colored_test_element();
        assert_eq!(expected.eq(&actual), true);
    }
}