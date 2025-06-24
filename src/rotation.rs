use serde::{Deserialize, Serialize};
use crate::vector::Vector;

/// Represents a rotation in three-dimensional space using a quaternion.
#[derive(Deserialize, Serialize)]
pub struct Rotation {
    /// The x-coordinate of the quaternion.
    pub qx: f64,
    /// The y-coordinate of the quaternion.
    pub qy: f64,
    /// The z-coordinate of the quaternion.
    pub qz: f64,
    /// The w-coordinate of the quaternion.
    pub qw: f64
}

impl PartialEq for Rotation {
    fn eq(&self, other: &Self) -> bool {
        self.qx == other.qx && self.qy == other.qy && self.qz == other.qz && self.qw == other.qw
    }
}

impl Rotation {
    /// Returns a new Rotation
    pub fn new(qx: f64, qy: f64, qz: f64, qw: f64) -> Rotation { Rotation { qx, qy, qz, qw } }

    /// Returns a new Rotation, but from Axis-Angle rotation.
    pub fn new_from_axis_angle(axis: &Vector, angle: f64) -> Rotation {
        let axis = axis.get_unitized();

        let qx = axis.x * f64::sin(angle/2.0);
        let qy = axis.y * f64::sin(angle/2.0);
        let qz = axis.z * f64::sin(angle/2.0);
        let qw = f64::cos(angle/2.0);

        Rotation::new(qx, qy, qz, qw)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;
    use serde_json::to_string;
    use super::*;

    #[test]
    fn test_new() {
        let result = Rotation::new(1.5, -2.3, 3.9, 5.5);
        assert_eq!(result.qx, 1.5);
        assert_eq!(result.qy, -2.3);
        assert_eq!(result.qz, 3.9);
        assert_eq!(result.qw, 5.5);
    }

    #[test]
    fn test_new_from_axis_angle() {
        let actual = Rotation::new_from_axis_angle(&Vector::new(1.0, 0.0, 0.0), std::f64::consts::PI / 2.0);
        let expected = Rotation::new(std::f64::consts::FRAC_1_SQRT_2, 0.0, 0.0, std::f64::consts::FRAC_1_SQRT_2);
        
        assert!(expected.eq_with_tolerance(&actual, 0.0001))
    }

    #[test]
    fn test_partialeq_true() {
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.3, 3.9, 5.5);
        assert_eq!(a.eq(&b), true);
        assert_eq!(b.eq(&a), true);
    }

    #[test]
    fn test_partialeq_first_different_false() {
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.4, -2.3, 3.9, 5.5);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_second_different_false() {
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.4, 3.9, 5.5);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_third_different_false() {
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.3, 3.4, 5.5);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_fourth_different_false() {
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(1.5, -2.3, 3.4, 5.4);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_partialeq_all_different_false() {
        let a = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let b = Rotation::new(5.5, 2.3, 33.9, 50.51);
        assert_eq!(a.eq(&b), false);
        assert_eq!(b.eq(&a), false);
    }

    #[test]
    fn test_to_json() {
        let input = Rotation::new(1.5, -2.3, 3.9, 5.5);
        let input_serialized = to_string(&input);
        assert_eq!(input_serialized.is_ok(), true);
        let input_serialized_string = input_serialized.ok().unwrap();
        assert_eq!(input_serialized_string, "{\"qx\":1.5,\"qy\":-2.3,\"qz\":3.9,\"qw\":5.5}");
    }

    #[test]
    fn test_from_json() {
        let json = "{\"qx\":1.5,\"qy\":-2.3,\"qz\":3.9,\"qw\":5.5}";
        let actual_result = from_str::<Rotation>(json);
        assert_eq!(actual_result.is_ok(), true);
        let actual = actual_result.ok().unwrap();
        let expected = Rotation::new(1.5, -2.3, 3.9, 5.5);
        assert_eq!(expected.eq(&actual), true);
    }
}