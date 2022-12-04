use crate::geometries::{
    bound_by_box::BoundByBox, bounding_box::BoundingBox, intersectable::Intersectable, intersection::Intersection,
    ray::Ray,
};

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
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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

impl BoundByBox for IntersectableCollection {
    fn calculate_bounding_box(&self) -> BoundingBox {
        BoundingBox::from_geometries(self.children.iter().map(|x| x.as_ref()))
    }
}
