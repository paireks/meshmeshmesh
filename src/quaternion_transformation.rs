use std::ops;
use crate::quaternion::Quaternion;

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    /// Multiplies a [Quaternion] with another one.
    ///
    /// The result is a new Quaternion.
    ///
    /// # Example
    ///
    /// ```
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let a = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
    /// let b = Quaternion::new(0.12038213521910773, -0.637288205465847, 0.747937726665425, -0.14128355079330665);
    ///
    /// let actual = a * b;
    ///
    /// let expected = Quaternion::new(0.04636,0.1773,0.22009,-0.95811);
    ///
    /// assert!(expected.eq_with_tolerance(&actual, 0.0001));
    ///
    /// ```
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion::new(
            self.qw * other.qx + self.qx * other.qw + self.qy * other.qz - self.qz * other.qy,
            self.qw * other.qy - self.qx * other.qz + self.qy * other.qw + self.qz * other.qx,
            self.qw * other.qz + self.qx * other.qy - self.qy * other.qx + self.qz * other.qw,
            self.qw * other.qw - self.qx * other.qx - self.qy * other.qy - self.qz * other.qz,
        )
    }
}

impl Quaternion {
    /// Gets inverted version of this [Quaternion].
    /// 
    /// # Example
    /// 
    /// ```
    /// use meshmeshmesh::quaternion::Quaternion;
    ///
    /// let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
    /// let actual = input.get_inverted();
    /// let expected = Quaternion::new(0.16408259600363556,  0.62745925514862166, -0.7364020684484639, 0.1925714890680042);
    ///
    /// assert_eq!(expected, actual);
    ///
    /// ```
    pub fn get_inverted(&self) -> Quaternion {
        Quaternion::new(-self.qx, -self.qy, -self.qz, self.qw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let a = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
        let b = Quaternion::new(0.12038213521910773, -0.637288205465847, 0.747937726665425, -0.14128355079330665);
        
        let actual = a * b;
        
        let expected = Quaternion::new(0.04636,0.1773,0.22009,-0.95811);
        
        assert!(expected.eq_with_tolerance(&actual, 0.0001));
    }
    
    #[test]
    fn test_get_inverted() {
        let input = Quaternion::new(-0.16408259600363556,  -0.62745925514862166, 0.7364020684484639, 0.1925714890680042);
        let actual = input.get_inverted();
        let expected = Quaternion::new(0.16408259600363556,  0.62745925514862166, -0.7364020684484639, 0.1925714890680042);
        
        assert_eq!(expected, actual);
    }
}