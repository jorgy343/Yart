use crate::{
    common::{Real, TWO_PI},
    geometries::geometry::Geometry,
    math::{color3::Color3, vector3::Vector3},
    scene::Scene,
};
use rand::{Rng, RngCore};
use std::fmt::Debug;

pub type MaterialIndex = usize;

pub trait Material: Debug {
    fn calculate_rendering_equation(
        &self,
        rng: &mut dyn RngCore,
        scene: &Scene,
        current_depth: u16,
        hit_geometry: &dyn Geometry,
        hit_position: &Vector3,
        hit_normal: &Vector3,
        incoming_direction: &Vector3,
    ) -> Color3;
}

pub fn generate_cosine_weighted_hemisphere_sample(rng: &mut dyn RngCore, hit_normal: &Vector3) -> Vector3 {
    let random1 = rng.gen::<Real>();
    let random2 = rng.gen::<Real>();

    let random_hemisphere_vector = cosine_weighted_sample_hemisphere(random1, random2);
    let outgoing_direction = transform_from_tangent_space_to_world_space(hit_normal, &random_hemisphere_vector);

    outgoing_direction
}

fn cosine_weighted_sample_hemisphere(random1: Real, random2: Real) -> Vector3 {
    // Source: https://www.scratchapixel.com/code.php?id=34&origin=/lessons/3d-basic-rendering/global-illumination-path-tracing

    let sin_theta = Real::sqrt(1.0 - random1 * random1);
    let phi = TWO_PI * random2;

    let z = sin_theta * Real::sin(phi);
    let x = sin_theta * Real::cos(phi);

    Vector3::new(x, random1, z)
}

fn transform_from_tangent_space_to_world_space(hit_normal: &Vector3, vector_to_transform: &Vector3) -> Vector3 {
    // Source: https://www.scratchapixel.com/code.php?id=34&origin=/lessons/3d-basic-rendering/global-illumination-path-tracing

    let nt = if Real::abs(hit_normal.x) > Real::abs(hit_normal.y) {
        Vector3::new(hit_normal.z, 0.0, -hit_normal.x)
            / Real::sqrt((hit_normal.x * hit_normal.x) + (hit_normal.z * hit_normal.z))
    } else {
        Vector3::new(0.0, -hit_normal.z, hit_normal.y)
            / Real::sqrt((hit_normal.y * hit_normal.y) + (hit_normal.z * hit_normal.z))
    };

    let nb = hit_normal % nt;

    Vector3::new(
        (vector_to_transform.x * nb.x) + (vector_to_transform.y * hit_normal.x) + (vector_to_transform.z * nt.x),
        (vector_to_transform.x * nb.y) + (vector_to_transform.y * hit_normal.y) + (vector_to_transform.z * nt.y),
        (vector_to_transform.x * nb.z) + (vector_to_transform.y * hit_normal.z) + (vector_to_transform.z * nt.z),
    )
}
