use crate::math::{vector::Vector, vector3::Vector3};

pub struct BoundingBox {
    pub minimum: Vector3,
    pub maximum: Vector3,
}

impl BoundingBox {
    pub fn new(minimum: &Vector3, maximum: &Vector3) -> BoundingBox {
        Self {
            minimum: *minimum,
            maximum: *maximum,
        }
    }

    pub fn new_infinity() -> BoundingBox {
        Self::new(&-Vector3::new_infinity(), &Vector3::new_infinity())
    }

    pub fn new_inverse_infinity() -> BoundingBox {
        Self::new(&Vector3::new_infinity(), &-Vector3::new_infinity())
    }
}
