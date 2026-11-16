use crate::geometry::Shape;
use crate::spheres::Sphere;


#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Shape) -> Self {
        Intersection { t, object }
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::addr_eq(self.object, other.object)
    }
}

pub fn intersections(mut xs: Vec<Intersection>) -> Vec<Intersection> {
    xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    xs
}

pub fn hit<'a>(xs: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    xs.iter()
        .filter(|i| i.t >= 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_creation() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(3.5, &sphere);

        assert_eq!(intersection.t, 3.5);
        //assert_eq!(intersection.object, sphere);
    }

    #[test]
    fn test_aggregating_intersections() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let intersections = intersections(vec![i1, i2]);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[1].t, 2.0);
    }

    #[test]
    fn test_hit_all_positive_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        let xs = intersections(vec![i2, i1]);
        let h = hit(&xs).unwrap();
        assert_eq!(h, i1);
    }

    #[test]
    fn test_hit_some_negative_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(1.0, &sphere);
        let xs = intersections(vec![i2, i1]);
        let h = hit(&xs).unwrap();
        assert_eq!(h, i2);
    }

    #[test]
    fn test_hit_all_negative_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-2.0, &sphere);
        let i2 = Intersection::new(-1.0, &sphere);
        let xs = intersections(vec![i2, i1]);
        let h = hit(&xs);
        assert!(h.is_none());
    }

    #[test]
    fn test_hit_lowest_nonnegative_t() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(5.0, &sphere);
        let i2 = Intersection::new(7.0, &sphere);
        let i3 = Intersection::new(-3.0, &sphere);
        let i4 = Intersection::new(2.0, &sphere);
        let xs = intersections(vec![i1, i2, i3, i4]);
        let h = hit(&xs).unwrap();
        assert_eq!(h, i4);
    }
}
