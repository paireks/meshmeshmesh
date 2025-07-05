use crate::element::Element;
use crate::local_coordinate_system::LocalCoordinateSystem;

impl Element {
    /// Compares given [Element] to other one, but with a `f64` tolerance for fields
    /// having `f64`.
    ///
    /// If any fields' value absolute difference is > tolerance, then it should return `false`.
    /// 
    /// Other fields are compared same way as `PartialEq`.
    pub fn eq_with_tolerance(&self, other: &Self, tolerance: f64) -> bool {
        if self.mesh_id != other.mesh_id {
            return false;
        }
        if !self.vector.eq_with_tolerance(&other.vector, tolerance) {
            return false;
        }
        if !self.rotation.eq_with_tolerance(&other.rotation, tolerance) {
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

    /// Gets [LocalCoordinateSystem] of this specific [Element].
    ///
    /// In other words, it turns the `vector` and `rotation` of the Element into the
    /// LocalCoordinateSystem.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use meshmeshmesh::color::Color;
    /// use meshmeshmesh::element::Element;
    /// use meshmeshmesh::local_coordinate_system::LocalCoordinateSystem;
    /// use meshmeshmesh::point::Point;
    /// use meshmeshmesh::quaternion::Quaternion;
    /// use meshmeshmesh::vector::Vector;
    ///
    /// let input = Element::new(
    ///     2,
    ///     Vector::new(7.5186233686734392, 7.3149022009468743, 105.9714472336252),
    ///     Quaternion::new(0.6008800094747585, 0.24407245125781549, -0.28644964837820225, -0.70520809105867444),
    ///     String::from("45255b3c-e672-4467-a891-c329c95fe35d"),
    ///     String::from("Brick"),
    ///     Color::new(174,166,129,255),
    ///     None,
    ///     HashMap::new(),
    /// );
    ///
    /// let actual = input.get_local_coordinate_system();
    /// let expected = LocalCoordinateSystem::new(
    ///     Point::new(7.518623368673439,7.314902200946874, 105.9714472336252),
    ///     Vector::new(0.7167504749620112,0.6973297330830885,5.551115123125783e-17),
    ///     Vector::new(-0.11069670578579367,0.11377962631523676, -0.9873198245572398)
    /// );
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.0001));
    ///
    /// ```
    pub fn get_local_coordinate_system(&self) -> LocalCoordinateSystem {
        let mut local_coordinate_system = LocalCoordinateSystem::global().get_rotated_by_quaternion_around_its_origin(self.rotation);
        local_coordinate_system.origin = self.vector.to_point();
        local_coordinate_system
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::color::Color;
    use crate::point::Point;
    use crate::quaternion::Quaternion;
    use crate::vector::Vector;
    use super::*;

    #[test]
    fn test_get_local_coordinate_system() {
        let input = Element::new(
            2,
            Vector::new(7.5186233686734392, 7.3149022009468743, 105.9714472336252),
            Quaternion::new(0.6008800094747585, 0.24407245125781549, -0.28644964837820225, -0.70520809105867444),
            String::from("45255b3c-e672-4467-a891-c329c95fe35d"),
            String::from("Brick"),
            Color::new(174,166,129,255),
            None,
            HashMap::new(),
        );

        let actual = input.get_local_coordinate_system();
        let expected = LocalCoordinateSystem::new(
            Point::new(7.518623368673439,7.314902200946874, 105.9714472336252),
            Vector::new(0.7167504749620112,0.6973297330830885,5.551115123125783e-17),
            Vector::new(-0.11069670578579367,0.11377962631523676, -0.9873198245572398)
        );

        println!("{0:?}", actual);

        assert!(expected.eq_with_tolerance(&actual, 0.0001));
    }

    fn get_blue_test_element() -> Element {
        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
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
            Quaternion::new(0.0, 0.0, 0.0, 1.0),
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
    fn test_partial_eq_without_face_colors_true(){
        let a = get_blue_test_element();
        let b = get_blue_test_element();
        assert_eq!(a.eq_with_tolerance(&b, 0.001), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), true);
    }

    #[test]
    fn test_partial_eq_without_face_colors_same_within_tolerance_true(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4-0.0005),
            Quaternion::new(1.0+0.0005, 1.5, 2.0, 2.5-0.000001),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), true);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_mesh_id(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            3, // different
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_vector(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.1, 0.3, 0.4), // different
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_rotation(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, -0.5, 2.0, 2.5), // different
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_guid(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b34a1674-e680-40f2-baa9-0e9b017bea14"), // different
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_type(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Another one"), // different
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_color(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(55,0,255,0), // different
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_existing_face_colors(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            Some(vec![255, 0, 0, 0]), // different
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_face_colors(){
        let a = get_face_colored_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            0,
            Vector::new(0.0, 0.0, 0.0),
            Quaternion::new(0.0, 0.0, 0.0, 1.0),
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

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_face_colors_count(){
        let a = get_face_colored_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Value"));

        let b = Element::new(
            0,
            Vector::new(0.0, 0.0, 0.0),
            Quaternion::new(0.0, 0.0, 0.0, 1.0),
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

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_info_value(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Key"), String::from("Another value")); // different

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_without_face_colors_different_info_key(){
        let a = get_blue_test_element();

        let mut info: HashMap<String, String> = HashMap::new();
        info.insert(String::from("Another key"), String::from("Value")); // different

        let b = Element::new(
            4,
            Vector::new(0.2, 0.3, 0.4),
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
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
            Quaternion::new(1.0, 1.5, 2.0, 2.5),
            String::from("b8a7a2ed-0c30-4c20-867e-baa1ef7b8353"),
            String::from("Plate"),
            Color::new(0,0,255,0),
            None,
            info.clone(),
        );

        assert_eq!(a.eq_with_tolerance(&b, 0.001), false);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), false);
    }

    #[test]
    fn test_partial_eq_with_face_colors_true(){
        let a = get_face_colored_test_element();
        let b = get_face_colored_test_element();
        assert_eq!(a.eq_with_tolerance(&b, 0.001), true);
        assert_eq!(b.eq_with_tolerance(&a, 0.001), true);
    }
}