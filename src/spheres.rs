use crate::geometry::Shape;
use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::ray;
use crate::ray::Ray;
use crate::transform;
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Matrix::identity(4),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let ray = ray::transform(ray, &self.transform.inverse());
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![]; // No intersections
        }
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        return vec![Intersection::new(t1, self), Intersection::new(t2, self)];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert!((intersections[0].t - 4.0).abs() < 1e-6);
        assert!((intersections[1].t - 6.0).abs() < 1e-6);
    }

    #[test]
    fn test_ray_intersects_sphere_at_a_tangent() {
        let ray = Ray {
            origin: Tuple::point(0.0, 1.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert!((intersections[0].t - 5.0).abs() < 1e-6);
        assert!((intersections[1].t - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_ray_misses_sphere() {
        let ray = Ray {
            origin: Tuple::point(0.0, 2.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_sphere() {
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, 0.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert!((intersections[0].t - -1.0).abs() < 1e-6);
        assert!((intersections[1].t - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ray_originates_behind_sphere() {
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, 5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert!((intersections[0].t - -6.0).abs() < 1e-6);
        assert!((intersections[1].t - -4.0).abs() < 1e-6);
    }

    #[test]
    fn test_intersection_scaled_sphere() {
        let mut sphere = Sphere::new();
        let transform = transform::scale(2.0, 2.0, 2.0);
        sphere.set_transform(transform.clone());
        let ray = Ray {
            origin: Tuple::point(0.0, 0.0, -5.0),
            direction: Tuple::vector(0.0, 0.0, 1.0),
        };
        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert!((intersections[0].t - 3.0).abs() < 1e-6);
        assert!((intersections[1].t - 7.0).abs() < 1e-6);
    }
}
