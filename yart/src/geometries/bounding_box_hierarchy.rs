use crate::geometries::{
    bound_by_box::BoundByBox, bounding_box::BoundingBox, intersectable::Intersectable,
    intersectable_collection::IntersectableCollection, intersection::Intersection, ray::Ray,
};
use itertools::Itertools;
use std::{cmp::Ordering, rc::Rc};

#[derive(Debug)]
pub struct BoundingBoxLeaf {
    bounding_box: BoundingBox,
    child: Box<dyn Intersectable>,
}

impl BoundingBoxLeaf {
    pub fn new(bounding_box: BoundingBox, child: Box<dyn Intersectable>) -> Self {
        Self { bounding_box, child }
    }
}

impl Intersectable for BoundingBoxLeaf {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bounding_box.ray_intersects(ray) {
            return self.child.intersect(ray);
        }

        None
    }
}

impl BoundByBox for BoundingBoxLeaf {
    fn calculate_bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

fn build_bounding_box_hierarchy_split_by_long_axis_internal(
    _current_depth: u16,
    geometries: Vec<Box<dyn Intersectable>>,
) -> Box<dyn Intersectable> {
    const MAXIMUM_LEAFS: usize = 8;

    let complete_bounding_box = BoundingBox::from_geometries(geometries.iter().map(|x| x.as_ref()));

    // Determine which axis is the longest.
    let axis_lengths = complete_bounding_box.maximum - complete_bounding_box.minimum;

    let longest_axis_index = if axis_lengths.x > axis_lengths.y { 0 } else { 1 };
    let longest_axis_index = if axis_lengths[longest_axis_index] > axis_lengths.z {
        longest_axis_index
    } else {
        2
    };

    // Sort the geometries by the longest axis and chunk them together.
    let geometries_per_level = usize::max(1, f64::ceil(geometries.len() as f64 / MAXIMUM_LEAFS as f64) as usize);

    let geometry_chunks = geometries
        .into_iter()
        .sorted_by(|a, b| {
            let a_center_point = a.calculate_bounding_box().calculate_center_point();
            let b_center_point = b.calculate_bounding_box().calculate_center_point();

            a_center_point[longest_axis_index]
                .partial_cmp(&b_center_point[longest_axis_index])
                .unwrap_or(Ordering::Equal)
        })
        .chunks(geometries_per_level);

    let mut leaves: Vec<Box<dyn Intersectable>> = Vec::new();

    for geometry_chunk in &geometry_chunks {
        let chunk_geometries = geometry_chunk.collect_vec();

        let leaf_bounding_box = BoundingBox::from_geometries(chunk_geometries.iter().map(|x| x.as_ref()));
        let geometry = Box::new(IntersectableCollection::new(chunk_geometries));

        // TODO: Take into account preferred leaf width and current depth.

        let leaf = Box::new(BoundingBoxLeaf::new(leaf_bounding_box, geometry));
        leaves.push(leaf);
    }

    Box::new(IntersectableCollection::new(leaves))
}

pub fn build_bounding_box_hierarchy_split_by_long_axis(
    geometries: Vec<Box<dyn Intersectable>>,
) -> Box<dyn Intersectable> {
    build_bounding_box_hierarchy_split_by_long_axis_internal(1, geometries)
}
