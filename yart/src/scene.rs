use crate::{
    cameras::camera::Camera,
    common::{Real, NORMAL_BUMP},
    geometries::{area_light::AreaLight, intersectable::Intersectable, ray::Ray},
    lights::light::Light,
    materials::material::Material,
    math::color3::Color3,
    miss_shaders::miss_shader::MissShader,
    yaml::parse_config::Config,
};
use rand::RngCore;

pub struct Scene {
    pub config: Config,
    pub camera: Box<dyn Camera>,
    pub materials: Vec<Box<dyn Material>>,
    pub lights: Vec<Box<dyn Light>>,
    pub area_lights: Vec<Box<dyn AreaLight>>,
    pub miss_shader: Box<dyn MissShader>,
    pub root_geometry: Box<dyn Intersectable>,
}

impl Scene {
    pub fn new(
        config: Config,
        camera: Box<dyn Camera>,
        materials: Vec<Box<dyn Material>>,
        lights: Vec<Box<dyn Light>>,
        area_lights: Vec<Box<dyn AreaLight>>,
        miss_shader: Box<dyn MissShader>,
        root_geometry: Box<dyn Intersectable>,
    ) -> Self {
        Self {
            config,
            camera,
            materials,
            lights,
            area_lights,
            miss_shader,
            root_geometry,
        }
    }

    pub fn cast_ray_color(&self, rng: &mut dyn RngCore, ray: &Ray, depth: u16) -> Color3 {
        if depth > 7 {
            return Color3::default();
        }

        let maybe_intersection = self.root_geometry.intersect(ray);

        match maybe_intersection {
            Some(intersection) => {
                let material = if intersection.material_index_override > 0 {
                    self.materials.get(intersection.material_index_override)
                } else {
                    self.materials.get(intersection.hit_geometry.material_index())
                };

                let mut hit_position = ray.position_along(intersection.entrance_distance);
                let hit_normal = intersection.hit_geometry.calculate_normal(ray, &hit_position);

                hit_position += hit_normal * NORMAL_BUMP;

                match material {
                    Some(material_some) => material_some.calculate_rendering_equation(
                        rng,
                        self,
                        depth,
                        intersection.hit_geometry,
                        &hit_position,
                        &hit_normal,
                        ray.direction(),
                    ),
                    None => Color3::default(),
                }
            }
            None => self.miss_shader.calculate_color(ray),
        }
    }

    pub fn cast_ray_distance(&self, ray: &Ray) -> Option<Real> {
        let intersection = self.root_geometry.intersect(ray)?;
        Some(Real::max(0.0, intersection.entrance_distance))
    }
}
