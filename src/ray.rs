use crate::tuple::Tuple;
use crate::transform;
use crate::matrix::Matrix;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}

pub fn transform(ray: &Ray, matrix: &Matrix) -> Ray {
    let new_origin = matrix * ray.origin;
    let new_direction = matrix * ray.direction;
    Ray::new(new_origin, new_direction)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_constructor() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let ray = Ray { origin, direction };

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_ray_position() {
        let ray = Ray {
            origin: Tuple::point(2.0, 3.0, 4.0),
            direction: Tuple::vector(1.0, 0.0, 0.0),
        };

        assert_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_ray_transform() {
        let ray = Ray {
            origin: Tuple::point(1.0, 2.0, 3.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };
        let matrix = transform::translate(3.0, 4.0, 5.0);
        let transformed_ray = transform(&ray, &matrix);
        let expected_origin = Tuple::point(4.0, 6.0, 8.0);
        let expected_direction = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(transformed_ray.origin, expected_origin);
        assert_eq!(transformed_ray.direction, expected_direction);
    }

    #[test]
    fn test_ray_transform_scaling() {
        let ray = Ray {
            origin: Tuple::point(1.0, 2.0, 3.0),
            direction: Tuple::vector(0.0, 1.0, 0.0),
        };
        let matrix = transform::scale(2.0, 3.0, 4.0);
        let transformed_ray = transform(&ray, &matrix);
        let expected_origin = Tuple::point(2.0, 6.0, 12.0);
        let expected_direction = Tuple::vector(0.0, 3.0, 0.0);
        assert_eq!(transformed_ray.origin, expected_origin);
        assert_eq!(transformed_ray.direction, expected_direction);
    }
}
