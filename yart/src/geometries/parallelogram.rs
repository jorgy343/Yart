use crate::{
    common::{Real, EPSILON},
    geometries::{
        area_light::AreaLight, bound_by_box::BoundByBox, bounding_box::BoundingBox, has_material::HasMaterial,
        intersectable::Intersectable, intersection::Intersection, normal_calculator::NormalCalculator, ray::Ray,
    },
    materials::material::MaterialIndex,
    math::{vector::Vector, vector3::Vector3},
    normalize,
    scene::Scene,
};
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Parallelogram {
    position: Vector3,
    edge1: Vector3,
    edge2: Vector3,
    normal: Vector3,
    area: Real,
    material_index: MaterialIndex,
}

impl Parallelogram {
    pub fn new(position: &Vector3, edge1: &Vector3, edge2: &Vector3, material_index: MaterialIndex) -> Self {
        Self {
            position: *position,
            edge1: *edge1,
            edge2: *edge2,
            normal: normalize!(edge1 % edge2),
            area: (edge1 % edge2).length(),
            material_index,
        }
    }
}

impl HasMaterial for Parallelogram {
    fn material_index(&self) -> MaterialIndex {
        self.material_index
    }
}

impl NormalCalculator for Parallelogram {
    fn calculate_normal(&self, ray: &super::ray::Ray, _hit_position: &Vector3) -> Vector3 {
        if ray.direction() ^ self.normal < 0.0 {
            self.normal
        } else {
            -self.normal
        }
    }
}

impl Intersectable for Parallelogram {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let p = ray.direction() % self.edge2;
        let determinant = self.edge1 ^ p;

        if determinant < 0.0 {
            return None;
        }

        let inverse_determinant = Real::recip(determinant);

        let t = ray.position() - self.position;
        let a = inverse_determinant * (t ^ p);

        if a < 0.0 || a > 1.0 {
            return None;
        }

        let q = t % self.edge1;
        let b = inverse_determinant * (ray.direction() ^ q);

        if b < 0.0 || b > 1.0 {
            return None;
        }

        let distance = inverse_determinant * (self.edge2 ^ q);

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

impl AreaLight for Parallelogram {
    fn get_direction_towards_light(
        &self,
        rng: &mut dyn RngCore,
        hit_position: &Vector3,
        hit_normal: &Vector3,
    ) -> Vector3 {
        let point_on_light = self.get_point_on_light(rng, hit_position, hit_normal);
        let direction_to_light = normalize!(point_on_light - hit_position);

        direction_to_light
    }

    fn get_point_on_light(&self, rng: &mut dyn RngCore, _hit_position: &Vector3, _hit_normal: &Vector3) -> Vector3 {
        self.position + (self.edge1 * rng.gen::<Real>()) + (self.edge2 * rng.gen::<Real>())
    }

    fn is_in_shadow(
        &self,
        _rng: &mut dyn RngCore,
        scene: &Scene,
        hit_position: &Vector3,
        _hit_normal: &Vector3,
        point_on_light: &Vector3,
    ) -> bool {
        let direction_to_light = point_on_light - hit_position;
        let distance_to_light = direction_to_light.length();

        let direction_to_light = normalize!(direction_to_light);
        let ray = Ray::new(hit_position, &direction_to_light);

        if let Some(distance) = scene.cast_ray_distance(&ray) {
            !(distance >= distance_to_light - EPSILON)
        } else {
            false
        }
    }

    fn calculate_inverse_pdf(
        &self,
        _rng: &mut dyn RngCore,
        _hit_position: &Vector3,
        _hit_normal: &Vector3,
        _incoming_direction: &Vector3,
        _outgoing_direction: &Vector3,
    ) -> Real {
        self.area
    }
}

impl BoundByBox for Parallelogram {
    fn calculate_bounding_box(&self) -> BoundingBox {
        BoundingBox::from_points(
            [
                self.position,
                self.position + self.edge1,
                self.position + self.edge2,
                self.position + self.edge1 + self.edge2,
            ]
            .iter(),
        )
    }
}
