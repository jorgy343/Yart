use crate::{
    common::{Real, ONE_OVER_PI},
    geometries::{geometry::Geometry, ray::Ray},
    materials::material::{generate_cosine_weighted_hemisphere_sample, Material},
    math::{color3::Color3, vector3::Vector3},
    scene::Scene,
};
use rand::{seq::SliceRandom, Rng, RngCore};

#[derive(Debug)]
pub struct LambertianMaterial {
    pub diffuse_color: Color3,
}

impl LambertianMaterial {
    pub fn new(diffuse_color: &Color3) -> Self {
        Self {
            diffuse_color: *diffuse_color,
        }
    }
}

impl Material for LambertianMaterial {
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
        {
            let area_light = scene.area_lights.choose(rng).unwrap();

            // Direct light sample to a random light.
            let outgoing_direction = area_light.get_direction_towards_light(rng, hit_position, hit_normal);
            let outgoing_ray = Ray::new(hit_position, &outgoing_direction);

            let color_sample = scene.cast_ray_color(rng, &outgoing_ray, current_depth + 1);

            let brdf = ONE_OVER_PI;
            let inverse_pdf = area_light.calculate_inverse_pdf(
                rng,
                hit_position,
                hit_normal,
                incoming_direction,
                &outgoing_direction,
            );

            let cosine_theta = Real::max(0.0, hit_normal ^ outgoing_direction);

            let output_color = brdf * self.diffuse_color * color_sample * inverse_pdf * cosine_theta;

            output_color
        }
    }
}
