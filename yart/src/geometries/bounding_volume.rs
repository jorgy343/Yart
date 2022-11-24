use crate::geometries::{bound_by_box::BoundByBox, ray::Ray};
use std::fmt::Debug;

pub trait BoundingVolume: Debug + BoundByBox {
    fn ray_intersects(&self, ray: &Ray) -> bool;
}
