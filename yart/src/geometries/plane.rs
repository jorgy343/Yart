use crate::{
    common::Real,
    geometries::{
        bound_by_box::BoundByBox, has_material::HasMaterial, intersectable::Intersectable, intersection::Intersection,
        normal_calculator::NormalCalculator, ray::Ray,
    },
    materials::material::MaterialIndex,
    math::vector3::Vector3,
};

#[derive(Debug)]
pub struct Plane {
    normal: Vector3,
    distance: Real,
    material_index: MaterialIndex,
}

impl Plane {
    pub fn new(normal: &Vector3, distance: Real, material_index: MaterialIndex) -> Self {
        Self {
            normal: *normal,
            distance,
            material_index,
        }
    }

    pub fn from_point(normal: &Vector3, point: &Vector3, material_index: MaterialIndex) -> Self {
        Self {
            normal: *normal,
            distance: -(normal ^ point),
            material_index,
        }
    }
}

impl HasMaterial for Plane {
    fn material_index(&self) -> MaterialIndex {
        self.material_index
    }
}

impl NormalCalculator for Plane {
    fn calculate_normal(&self, ray: &super::ray::Ray, _hit_position: &Vector3) -> Vector3 {
        if ray.direction() ^ self.normal < 0.0 {
            self.normal
        } else {
            -self.normal
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let normal_dot_ray_direction = self.normal ^ ray.direction();
        let normal_dot_ray_position = self.normal ^ ray.position();

        let distance = -(self.distance + normal_dot_ray_position) * Real::recip(normal_dot_ray_direction);

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

impl BoundByBox for Plane {}
