use crate::{common::Real, math::vector3::Vector3, scene::Scene};
use rand::RngCore;

pub trait AreaLight {
    fn get_direction_towards_light(
        &self,
        rng: &mut dyn RngCore,
        hit_position: &Vector3,
        hit_normal: &Vector3,
    ) -> Vector3;
    fn get_point_on_light(&self, rng: &mut dyn RngCore, hit_position: &Vector3, hit_normal: &Vector3) -> Vector3;
    fn is_in_shadow(
        &self,
        scene: &Scene,
        hit_position: &Vector3,
        hit_normal: &Vector3,
        point_on_light: &Vector3,
    ) -> bool;

    fn calculate_inverse_pdf(
        &self,
        rng: &mut dyn RngCore,
        hit_position: &Vector3,
        hit_normal: &Vector3,
        incoming_direction: &Vector3,
        outgoing_direction: &Vector3,
    ) -> Real;
}
