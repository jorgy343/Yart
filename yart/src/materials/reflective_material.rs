use super::material::Material;
use crate::{
    geometries::{geometry::Geometry, ray::Ray},
    math::{color3::Color3, vector3::Vector3},
    normalize,
    scene::Scene,
};
use rand::RngCore;

#[derive(Debug)]
pub struct ReflectiveMaterial {}

impl ReflectiveMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for ReflectiveMaterial {
    fn calculate_rendering_equation(
        &self,
        rng: &mut dyn RngCore,
        scene: &Scene,
        current_depth: u16,
        _hit_geometry: &dyn Geometry,
        hit_position: &Vector3,
        hit_normal: &Vector3,
        incoming_direction: &Vector3,
    ) -> Color3 {
        let reflected_direction = normalize!(incoming_direction.reflect(hit_normal));
        let outgoing_ray = Ray::new(hit_position, &reflected_direction);

        scene.cast_ray_color(rng, &outgoing_ray, current_depth + 1)
    }
}
