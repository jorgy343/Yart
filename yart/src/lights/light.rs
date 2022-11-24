use crate::{
    math::{color3::Color3, vector3::Vector3},
    scene::Scene,
};
use rand::RngCore;
use std::fmt::Debug;

pub trait Light: Debug {
    fn color(&self) -> Color3;

    fn get_direction_towards_light(&self, hit_position: &Vector3, hit_normal: &Vector3) -> Vector3;

    fn is_in_shadow(
        &self,
        rng: &mut dyn RngCore,
        scene: &Scene,
        hit_position: &Vector3,
        hit_normal: &Vector3,
        direction_to_light: &Vector3,
    ) -> bool;
}
