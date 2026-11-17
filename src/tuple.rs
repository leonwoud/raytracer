use approx::AbsDiffEq;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        let out  = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
        out
    }

    pub fn cross(&self, other: &Tuple) -> Self {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: &Tuple) -> Self {
        *self - *normal * 2.0 * self.dot(normal)
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
            && self.w.abs_diff_eq(&other.w, epsilon)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Tuple) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Tuple) -> Self {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_point() {
        let a = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
    }

    #[test]
    fn test_vector() {
        let a = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
    }

    #[test]
    fn test_equal() {
        let a = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        assert_abs_diff_eq!(a, b);
    }

    #[test]
    fn test_add() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let result = a1 + a2;
        assert_eq!(result, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_sub() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        let result = p1 - p2;
        assert_eq!(result, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_neg() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = -a;
        assert_eq!(result, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_mul() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = a * 3.5;
        assert_eq!(result, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn test_div() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result = a / 2.0;
        assert_eq!(result, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_vector_magnitude_x() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_vector_magnitude_y() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_vector_magnitude_z() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn test_vector_magnitude_pos_xyz() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn test_vector_magnitude_neg_xyz() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn test_vector_normalize() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        let result = v.normalize();
        assert_eq!(result, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_vector_normalize_non_unit() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let result = v.normalize();
        let expected = Tuple::vector(
            1.0 / (14.0 as f64).sqrt(),
            2.0 / (14.0 as f64).sqrt(),
            3.0 / (14.0 as f64).sqrt(),
        );
        assert_abs_diff_eq!(result, expected);
    }

    #[test]
    fn test_vector_magnitude_normalized() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let result = a.dot(&b);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let result1 = a.cross(&b);
        let result2 = b.cross(&a);
        assert_eq!(result1, Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(result2, Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn test_reflect_vector_at_45_degrees() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflect_vector_off_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(&n);
        assert_abs_diff_eq!(r, Tuple::vector(1.0, 0.0, 0.0), epsilon = 1e-5);
    }
}
