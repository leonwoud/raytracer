use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    // 1, 0, 0, X,
    // 0, 1, 0, Y,
    // 0, 0, 1, Z,
    // 0, 0, 0, 1
    let mut m = Matrix::identity(4);
    m.set(0, 3, x);
    m.set(1, 3, y);
    m.set(2, 3, z);
    m
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    // X, 0, 0, 0,
    // 0, Y, 0, 0,
    // 0, 0, Z, 0,
    // 0, 0, 0, 1
    let mut m = Matrix::identity(4);
    m.set(0, 0, x);
    m.set(1, 1, y);
    m.set(2, 2, z);
    m
}

pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    // 1, XY, XZ, 0,
    // YX, 1, YZ, 0,
    // ZX, ZY, 1, 0,
    // 0, 0, 0, 1
    let mut m = Matrix::identity(4);
    m.set(0, 1, xy);
    m.set(0, 2, xz);
    m.set(1, 0, yx);
    m.set(1, 2, yz);
    m.set(2, 0, zx);
    m.set(2, 1, zy);
    m
}

pub fn rotate_x(radians: f64) -> Matrix {
    // 1, 0, 0, 0,
    // 0, cos(r), -sin(r), 0,
    // 0, sin(r), cos(r), 0,
    // 0, 0, 0, 1
    let mut m = Matrix::identity(4);
    m.set(1, 1, radians.cos());
    m.set(1, 2, -radians.sin());
    m.set(2, 1, radians.sin());
    m.set(2, 2, radians.cos());
    m
}

pub fn rotate_y(radians: f64) -> Matrix {
    // cos(r), 0, sin(r), 0,
    // 0, 1, 0, 0,
    // -sin(r), 0, cos(r), 0,
    // 0, 0, 0, 1
    let mut m = Matrix::identity(4);
    m.set(0, 0, radians.cos());
    m.set(0, 2, radians.sin());
    m.set(2, 0, -radians.sin());
    m.set(2, 2, radians.cos());
    m
}

pub fn rotate_z(radians: f64) -> Matrix {
    // cos(r), -sin(r), 0, 0,
    // sin(r), cos(r), 0, 0,
    // 0, 0, 1, 0,
    // 0, 0, 0, 1
    let mut m = Matrix::identity(4);
    m.set(0, 0, radians.cos());
    m.set(0, 1, -radians.sin());
    m.set(1, 0, radians.sin());
    m.set(1, 1, radians.cos());
    m
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation() {
        let transform = translate(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let expected = Tuple::point(2.0, 1.0, 7.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_inverse_translation() {
        // The inverse of a translation should move in the opposite direction
        let transform = translate(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let expected = Tuple::point(-8.0, 7.0, 3.0);
        let result = inv * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_vector_translation() {
        // Translation should not affect vectors
        let transform = translate(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        let result = transform * v;
        assert!(result.equal(&v, None));
    }

    #[test]
    fn test_scaling() {
        let transform = scale(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        let expected = Tuple::point(-8.0, 18.0, 32.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_inverse_scaling() {
        let transform = scale(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let p = Tuple::point(-4.0, 6.0, 8.0);
        let expected = Tuple::point(-2.0, 2.0, 2.0);
        let result = inv * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_reflection_x() {
        let transform = scale(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(-2.0, 3.0, 4.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_rotation_x() {
        let half_quarter = rotate_x(std::f64::consts::PI / 4.0);
        let full_quarter = rotate_x(std::f64::consts::PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);
        let expected_half = Tuple::point(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0);
        let expected_full = Tuple::point(0.0, 0.0, 1.0);
        let result_half = half_quarter * p;
        let result_full = full_quarter * p;
        assert!(result_half.equal(&expected_half, Some(1e-5)));
        assert!(result_full.equal(&expected_full, Some(1e-5)));
    }

    #[test]
    fn test_rotation_y() {
        let half_quarter = rotate_y(std::f64::consts::PI / 4.0);
        let full_quarter = rotate_y(std::f64::consts::PI / 2.0);
        let p = Tuple::point(0.0, 0.0, 1.0);
        let expected_half = Tuple::point(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0);
        let expected_full = Tuple::point(1.0, 0.0, 0.0);
        let result_half = half_quarter * p;
        let result_full = full_quarter * p;
        assert!(result_half.equal(&expected_half, Some(1e-5)));
        assert!(result_full.equal(&expected_full, Some(1e-5)));
    }

    #[test]
    fn test_rotation_z() {
        let half_quarter = rotate_z(std::f64::consts::PI / 4.0);
        let full_quarter = rotate_z(std::f64::consts::PI / 2.0);
        let p = Tuple::point(0.0, 1.0, 0.0);
        let expected_half = Tuple::point(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0);
        let expected_full = Tuple::point(-1.0, 0.0, 0.0);
        let result_half = half_quarter * p;
        let result_full = full_quarter * p;
        assert!(result_half.equal(&expected_half, Some(1e-5)));
        assert!(result_full.equal(&expected_full, Some(1e-5)));
    }

    #[test]
    fn test_inverse_rotation() {
        let half_quarter = rotate_x(std::f64::consts::PI / 4.0);
        let inv = half_quarter.inverse();
        let p = Tuple::point(0.0, 1.0, 0.0);
        let expected = Tuple::point(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let result = inv * p;
        assert!(result.equal(&expected, Some(1e-5)));
    }

    #[test]
    fn test_shearing_x_proportional_to_y() {
        let transform = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(5.0, 3.0, 4.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_shearing_x_proportional_to_z() {
        let transform = shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(6.0, 3.0, 4.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_shearing_y_proportional_to_x() {
        let transform = shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 5.0, 4.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_shearing_y_proportional_to_z() {
        let transform = shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 7.0, 4.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_shearing_z_proportional_to_x() {
        let transform = shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 3.0, 6.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_shearing_z_proportional_to_y() {
        let transform = shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let expected = Tuple::point(2.0, 3.0, 7.0);
        let result = transform * p;
        assert!(result.equal(&expected, None));
    }

    #[test]
    fn test_chained_transformations() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotate_x(std::f64::consts::PI /2.0);
        let b = scale(5.0, 5.0, 5.0);
        let c = translate(10.0, 5.0, 7.0);
        let p2 = a * p;
        let p3 = b * p2;
        let p4 = c * p3;
        let expected = Tuple::point(15.0, 0.0, 7.0);
        assert!(p4.equal(&expected, Some(1e-5)));
    }

    #[test]
    fn test_chained_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotate_x(std::f64::consts::PI /2.0);
        let b = scale(5.0, 5.0, 5.0);
        let c = translate(10.0, 5.0, 7.0);
        let t = c * b * a;
        let result = t * p;
        let expected = Tuple::point(15.0, 0.0, 7.0);
        assert!(result.equal(&expected, Some(1e-5)));
    }

}
