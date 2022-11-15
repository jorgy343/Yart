use crate::{
    common::Real,
    geometries::{
        has_material::HasMaterial, intersectable::Intersectable, intersection::Intersection,
        normal_calculator::NormalCalculator, ray::Ray,
    },
    materials::material::MaterialIndex,
    math::vector3::Vector3,
    normalize,
};

#[derive(Debug)]
pub struct Triangle {
    vertex0: Vector3,
    vertex1: Vector3,
    vertex2: Vector3,

    normal0: Vector3,
    normal1: Vector3,
    normal2: Vector3,

    material_index: MaterialIndex,
}

impl Triangle {
    pub fn new(
        vertex0: &Vector3,
        vertex1: &Vector3,
        vertex2: &Vector3,
        normal0: &Vector3,
        normal1: &Vector3,
        normal2: &Vector3,
        material_index: MaterialIndex,
    ) -> Self {
        Self {
            vertex0: *vertex0,
            vertex1: *vertex1,
            vertex2: *vertex2,

            normal0: *normal0,
            normal1: *normal1,
            normal2: *normal2,

            material_index,
        }
    }

    pub fn barycentric_coordinates(&self, point_in_triangle: &Vector3) -> Vector3 {
        let v0 = self.vertex1 - self.vertex0;
        let v1 = self.vertex2 - self.vertex0;
        let v2 = point_in_triangle - self.vertex0;

        let d00 = v0 ^ v0;
        let d01 = v0 ^ v1;
        let d11 = v1 ^ v1;
        let d20 = v2 ^ v0;
        let d21 = v2 ^ v1;

        let inverse_denominator = Real::recip(d00 * d11 - d01 * d01);

        let v = (d11 * d20 - d01 * d21) * inverse_denominator;
        let w = (d00 * d21 - d01 * d20) * inverse_denominator;
        let u = 1.0 - v - w;

        Vector3::new(u, v, w)
    }

    pub fn calculate_face_normal(vertex0: &Vector3, vertex1: &Vector3) -> Vector3 {
        // TODO: This is just a cross product?
        normalize!(Vector3::new(
            vertex0.y * vertex1.z - vertex0.z * vertex1.y,
            vertex0.z * vertex1.x - vertex0.x * vertex1.z,
            vertex0.x * vertex1.y - vertex0.y * vertex1.x,
        ))
    }
}

impl HasMaterial for Triangle {
    fn material_index(&self) -> MaterialIndex {
        self.material_index
    }
}

impl NormalCalculator for Triangle {
    fn calculate_normal(&self, ray: &Ray, hit_position: &Vector3) -> Vector3 {
        let barycentric_coordinates = self.barycentric_coordinates(hit_position);

        let normal = normalize!(
            self.normal0 * barycentric_coordinates.x
                + self.normal1 * barycentric_coordinates.y
                + self.normal2 * barycentric_coordinates.z
        );

        if ray.direction() ^ normal < 0.0 {
            normal
        } else {
            -normal
        }
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let edge1 = self.vertex1 - self.vertex0;
        let edge2 = self.vertex2 - self.vertex0;

        let h = ray.direction() % edge2;
        let a = edge1 ^ h;

        // Normally you would check for a parallel ray here but we'll skip that check.

        let f = Real::recip(a);
        let s = ray.position() - self.vertex0;
        let u = f * (s ^ h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s % edge1;
        let v = f * (ray.direction() ^ q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let distance = f * (edge2 ^ q);

        if distance >= 0.0 {
            Some(Intersection {
                hit_geometry: self,
                entrance_distance: distance,
                exit_distance: distance,
                mix_amount: 0.0,
                material_index_override: 0,
            })
        } else {
            None
        }
    }
}
