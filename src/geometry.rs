use crate::intersection::Intersection;
use crate::tuple::Tuple;
use crate::ray::Ray;

pub trait Shape: std::fmt::Debug {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>>;
    fn normal_at(&self, point: Tuple) -> Tuple;
    fn material(&self) -> &crate::material::Material;
}