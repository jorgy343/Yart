use crate::{
    errors::Error,
    math::vector2::Vector2,
    yaml::parse_math::{parse_u32, parse_vector2},
};
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct Config {
    iterations: u32,
    color_clamp: Vector2,
}

impl Config {
    pub fn new(iterations: u32, color_clamp: &Vector2) -> Self {
        Self {
            iterations,
            color_clamp: *color_clamp,
        }
    }

    pub fn iterations(&self) -> u32 {
        self.iterations
    }

    pub fn color_clamp(&self) -> Vector2 {
        self.color_clamp
    }
}

pub fn parse_config(node: &Yaml) -> Result<Config, Error> {
    let iterations = parse_u32(&node["iterations"])
        .ok_or_else(|| Error::from_yaml_parser("iterations", Some(node.clone()), None))?;

    let color_clamp = parse_vector2(&node["colorClamp"])
        .ok_or_else(|| Error::from_yaml_parser("colorClamp", Some(node.clone()), None))?;

    Ok(Config::new(iterations, &color_clamp))
}
