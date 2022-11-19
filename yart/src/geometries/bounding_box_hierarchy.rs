use crate::geometries::{
    bounding_box_hierarchy_leaf::BoundingBoxHierarchyLeaf, intersectable::Intersectable, intersection::Intersection,
    ray::Ray,
};

#[derive(Debug)]
pub struct BoundingBoxHierarchy {
    leafs: Vec<BoundingBoxHierarchyLeaf>,
}

impl BoundingBoxHierarchy {
    pub fn new(leafs: Vec<BoundingBoxHierarchyLeaf>) -> Self {
        Self { leafs }
    }
}

impl Intersectable for BoundingBoxHierarchy {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!();
    }
}
