use crate::intersection::Intersection;
use crate::ray::Ray;

pub trait Shape: std::fmt::Debug {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>>;
}