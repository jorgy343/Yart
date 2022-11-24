use crate::geometries::{bound_by_box::BoundByBox, intersection::Intersection, ray::Ray};
use std::fmt::Debug;

pub trait Intersectable: Debug + BoundByBox {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
