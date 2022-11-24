use super::{
    parse_cameras::parse_camera, parse_geometries::parse_intersectable, parse_lights::parse_lights,
    parse_materials::parse_materials, parse_miss_shaders::parse_miss_shader,
};
use crate::{errors::Result, scene::Scene, yaml::parse_config::parse_config};
use std::{
    fs::{self},
    path::Path,
};
use yaml_rust::{Yaml, YamlLoader};

pub fn load_scene(path: &Path) -> Result<Scene> {
    let yaml_data = fs::read_to_string(path).unwrap();
    let doc = YamlLoader::load_from_str(yaml_data.as_str()).unwrap();

    let scene = parse_scene(&doc[0]);
    scene
}

fn parse_scene(node: &Yaml) -> Result<Scene> {
    let config = parse_config(&node["config"])?;

    let camera = parse_camera(&node["camera"]).unwrap();
    let miss_shader = parse_miss_shader(&node["missShader"]).unwrap();
    let lights = parse_lights(&node["lights"]).unwrap();
    let (materials, material_name_to_index_map) = parse_materials(&node["materials"]).unwrap();

    let mut area_lights = Vec::new();
    let root_geometry = parse_intersectable(&node["geometry"], &material_name_to_index_map, &mut area_lights).unwrap();

    Ok(Scene::new(
        config,
        camera,
        materials,
        lights,
        area_lights,
        miss_shader,
        root_geometry,
    ))
}
