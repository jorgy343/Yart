use crate::geometries::bounding_box::BoundingBox;

pub trait BoundByBox {
    fn calculate_bounding_box(&self) -> BoundingBox {
        BoundingBox::new_infinity()
    }
}
