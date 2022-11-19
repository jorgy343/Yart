use crate::geometries::{bounding_box::BoundingBox, intersectable::Intersectable};
use std::rc::Rc;

#[derive(Debug)]
pub struct BoundingBoxHierarchyLeaf {
    bounding_box: BoundingBox,
    children: Vec<Rc<dyn Intersectable>>,
}

impl BoundingBoxHierarchyLeaf {
    pub fn new(bounding_box: BoundingBox, children: Vec<Rc<dyn Intersectable>>) -> Self {
        Self { bounding_box, children }
    }
}
