use super::{intersectable::Intersectable, intersection::Intersection};

#[derive(Debug)]
pub struct IntersectableCollection {
    children: Vec<Box<dyn Intersectable>>,
}

impl IntersectableCollection {
    pub fn new(children: Vec<Box<dyn Intersectable>>) -> Self {
        Self { children }
    }
}

impl Intersectable for IntersectableCollection {
    fn intersect(&self, ray: &super::ray::Ray) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;

        for geometry in &self.children {
            let intersection = geometry.intersect(ray);

            if intersection.is_some() {
                if closest_intersection.is_some() {
                    if intersection.unwrap().entrance_distance < closest_intersection.unwrap().entrance_distance {
                        closest_intersection = intersection;
                    }
                } else {
                    closest_intersection = intersection;
                }
            }
        }

        closest_intersection
    }
}
