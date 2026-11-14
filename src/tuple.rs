use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f64) -> Tuple {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: f64) -> Tuple {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    fn equal(&self, other: &Tuple) -> bool {
        let epsilon = f64::EPSILON;
        (self.x - other.x).abs() < epsilon
            && (self.y - other.y).abs() < epsilon
            && (self.z - other.z).abs() < epsilon
            && (self.w - other.w).abs() < epsilon
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(a.equal(&b));
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
        assert!(result.equal(&expected));
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
}
