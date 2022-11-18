use super::{intersectable::Intersectable, intersection::Intersection};
use std::rc::Rc;

#[derive(Debug)]
pub struct IntersectableCollection {
    children: Vec<Rc<dyn Intersectable>>,
}

impl IntersectableCollection {
    pub fn new(children: Vec<Rc<dyn Intersectable>>) -> Self {
        Self { children }
    }
}

impl Intersectable for IntersectableCollection {
    fn intersect(&self, ray: &super::ray::Ray) -> Option<Intersection> {
        let mut maybe_closest_intersection: Option<Intersection> = None;

        for geometry in &self.children {
            let Some(intersection) = geometry.intersect(ray) else {
                continue;
            };

            if let Some(closest_intersection) = maybe_closest_intersection {
                if intersection.entrance_distance < closest_intersection.entrance_distance {
                    maybe_closest_intersection = Some(intersection);
                }
            } else {
                maybe_closest_intersection = Some(intersection);
            }
        }

        maybe_closest_intersection
    }
}
