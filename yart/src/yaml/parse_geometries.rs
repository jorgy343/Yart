use super::parse_math::parse_vector3;
use crate::{
    common::Real,
    geometries::{
        area_light::AreaLight, geometry::Geometry, intersectable::Intersectable,
        intersectable_collection::IntersectableCollection, parallelogram::Parallelogram, plane::Plane, sphere::Sphere,
        triangle::Triangle,
    },
    materials::material::MaterialIndex,
};
use std::{collections::HashMap, rc::Rc};
use yaml_rust::Yaml;

enum GeometryEnum {
    Intersectable(Rc<dyn Intersectable>),
    Geometry(Rc<dyn Geometry>),
}

fn create_intersectable_function_map() -> Vec<(
    &'static str,
    fn(&Yaml, &HashMap<String, MaterialIndex>, &mut Vec<Rc<dyn AreaLight>>) -> Option<GeometryEnum>,
)> {
    let mut map: Vec<(
        &'static str,
        fn(&Yaml, &HashMap<String, MaterialIndex>, &mut Vec<Rc<dyn AreaLight>>) -> Option<GeometryEnum>,
    )> = Vec::new();

    map.push(("sphere", parse_sphere));
    map.push(("plane", parse_plane));
    map.push(("triangle", parse_triangle));
    map.push(("collection", parse_intersectable_collection));

    map
}

fn create_geometry_function_map() -> Vec<(
    &'static str,
    fn(&Yaml, &HashMap<String, MaterialIndex>, &mut Vec<Rc<dyn AreaLight>>) -> Option<GeometryEnum>,
)> {
    let mut map: Vec<(
        &'static str,
        fn(&Yaml, &HashMap<String, MaterialIndex>, &mut Vec<Rc<dyn AreaLight>>) -> Option<GeometryEnum>,
    )> = Vec::new();

    map.push(("sphere", parse_sphere));
    map.push(("plane", parse_plane));
    map.push(("triangle", parse_triangle));

    map
}

pub fn parse_intersectable(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<Rc<dyn Intersectable>> {
    let mut found_geometry_enum: Option<GeometryEnum> = None;

    for (name, function) in create_intersectable_function_map() {
        let child_node = &node[name];

        if !child_node.is_badvalue() {
            found_geometry_enum = function(child_node, material_name_to_index_map, area_lights);
        }
    }

    match found_geometry_enum {
        Some(geometry_enum) => match geometry_enum {
            GeometryEnum::Intersectable(intersectable) => Some(intersectable),
            GeometryEnum::Geometry(geometry) => Some(geometry as Rc<dyn Intersectable>),
        },
        None => None,
    }
}

fn parse_intersectables(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<Vec<Rc<dyn Intersectable>>> {
    let mut intersectables = Vec::new();

    if !node.is_badvalue() && node.is_array() {
        for child_node in node.as_vec()? {
            let maybe_intersectable = parse_intersectable(child_node, material_name_to_index_map, area_lights);

            if maybe_intersectable.is_some() {
                intersectables.push(maybe_intersectable.unwrap());
            }
        }
    }

    Some(intersectables)
}

fn parse_geometry(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<Rc<dyn Geometry>> {
    let mut found_geometry_enum: Option<GeometryEnum> = None;

    for (name, function) in create_geometry_function_map() {
        let child_node = &node[name];

        if !child_node.is_badvalue() {
            found_geometry_enum = function(child_node, material_name_to_index_map, area_lights);
        }
    }

    match found_geometry_enum {
        Some(geometry_enum) => match geometry_enum {
            GeometryEnum::Intersectable(_) => None,
            GeometryEnum::Geometry(geometry) => Some(geometry),
        },
        None => None,
    }
}

fn parse_sphere(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<GeometryEnum> {
    let material_name = node["material"].as_str()?;

    let position = parse_vector3(&node["position"])?;
    let radius = node["radius"].as_f64()? as Real;

    let material_index = match material_name_to_index_map.get(material_name) {
        Some(x) => *x,
        None => 0 as MaterialIndex,
    };

    Some(GeometryEnum::Geometry(Rc::new(Sphere::new(
        &position,
        radius,
        material_index,
    ))))
}

fn parse_plane(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<GeometryEnum> {
    let material_name = node["material"].as_str()?;

    let normal = parse_vector3(&node["normal"])?;
    let distance = node["distance"].as_f64()? as Real;

    let material_index = match material_name_to_index_map.get(material_name) {
        Some(x) => *x,
        None => 0 as MaterialIndex,
    };

    Some(GeometryEnum::Geometry(Rc::new(Plane::new(
        normal,
        distance,
        material_index,
    ))))
}

fn parse_triangle(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<GeometryEnum> {
    let material_name = node["material"].as_str()?;

    let vertex0 = parse_vector3(&node["vertex0"])?;
    let vertex1 = parse_vector3(&node["vertex1"])?;
    let vertex2 = parse_vector3(&node["vertex2"])?;

    let maybe_normal0 = parse_vector3(&node["normal0"]);
    let maybe_normal1 = parse_vector3(&node["normal1"]);
    let maybe_normal2 = parse_vector3(&node["normal2"]);

    let material_index = match material_name_to_index_map.get(material_name) {
        Some(x) => *x,
        None => 0 as MaterialIndex,
    };

    if maybe_normal0.is_some() && maybe_normal1.is_some() && maybe_normal2.is_some() {
        Some(GeometryEnum::Geometry(Rc::new(Triangle::new(
            &vertex0,
            &vertex1,
            &vertex2,
            &maybe_normal0.unwrap(),
            &maybe_normal1.unwrap(),
            &maybe_normal2.unwrap(),
            material_index,
        ))))
    } else {
        let normal = Triangle::calculate_face_normal(&vertex0, &vertex1);

        Some(GeometryEnum::Geometry(Rc::new(Triangle::new(
            &vertex0,
            &vertex1,
            &vertex2,
            &normal,
            &normal,
            &normal,
            material_index,
        ))))
    }
}

fn parse_parallelogram(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<GeometryEnum> {
    let material_name = node["material"].as_str()?;
    let area_light = node["areaLight"].as_bool().unwrap_or(false);

    let position = parse_vector3(&node["position"])?;
    let edge1 = parse_vector3(&node["edge1"])?;
    let edge2 = parse_vector3(&node["edge2"])?;

    let material_index = match material_name_to_index_map.get(material_name) {
        Some(x) => *x,
        None => 0 as MaterialIndex,
    };

    let parallelogram = Rc::new(Parallelogram::new(&position, &edge1, &edge2, material_index));

    if area_light {
        area_lights.push(parallelogram.clone());
    }

    Some(GeometryEnum::Geometry(parallelogram))
}

fn parse_intersectable_collection(
    node: &Yaml,
    material_name_to_index_map: &HashMap<String, MaterialIndex>,
    area_lights: &mut Vec<Rc<dyn AreaLight>>,
) -> Option<GeometryEnum> {
    let children = parse_intersectables(&node["children"], material_name_to_index_map, area_lights)?;

    Some(GeometryEnum::Intersectable(Rc::new(IntersectableCollection::new(
        children,
    ))))
}
