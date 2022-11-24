use crate::{
    common::Real,
    math::{color3::Color3, color4::Color4, vector2::Vector2, vector3::Vector3, vector4::Vector4},
};
use yaml_rust::Yaml;

#[inline(always)]
pub fn parse_u16(node: &Yaml) -> Option<u16> {
    node.as_i64().map(|x| x as u16)
}

#[inline(always)]
pub fn parse_u32(node: &Yaml) -> Option<u32> {
    node.as_i64().map(|x| x as u32)
}

#[inline(always)]
pub fn parse_real(node: &Yaml) -> Option<Real> {
    node.as_f64()
        .or_else(|| node.as_i64().map(|x| x as f64))
        .map(|x| x as Real)
}

pub fn parse_vector4(node: &Yaml) -> Option<Vector4> {
    if !node.is_array() {
        return None;
    }

    let vec = node.as_vec()?;

    match vec.len() {
        1 => Some(Vector4::new(
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
        )),
        4 => Some(Vector4::new(
            parse_real(&vec[0])?,
            parse_real(&vec[1])?,
            parse_real(&vec[2])?,
            parse_real(&vec[3])?,
        )),
        _ => None,
    }
}

pub fn parse_vector3(node: &Yaml) -> Option<Vector3> {
    if !node.is_array() {
        return None;
    }

    let vec = node.as_vec()?;

    match vec.len() {
        1 => Some(Vector3::new(
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
        )),
        3 => Some(Vector3::new(
            parse_real(&vec[0])?,
            parse_real(&vec[1])?,
            parse_real(&vec[2])?,
        )),
        _ => None,
    }
}

pub fn parse_vector2(node: &Yaml) -> Option<Vector2> {
    if !node.is_array() {
        return None;
    }

    let vec = node.as_vec()?;

    match vec.len() {
        1 => Some(Vector2::new(parse_real(&vec[0])?, parse_real(&vec[0])?)),
        2 => Some(Vector2::new(parse_real(&vec[0])?, parse_real(&vec[1])?)),
        _ => None,
    }
}

pub fn parse_vector2u32(node: &Yaml) -> Option<(u32, u32)> {
    if !node.is_array() {
        return None;
    }

    let vec = node.as_vec()?;

    match vec.len() {
        1 => Some((vec[0].as_i64()? as u32, vec[0].as_i64()? as u32)),
        2 => Some((vec[0].as_i64()? as u32, vec[1].as_i64()? as u32)),
        _ => None,
    }
}

pub fn parse_color4(node: &Yaml) -> Option<Color4> {
    if !node.is_array() {
        return None;
    }

    let vec = node.as_vec()?;

    match vec.len() {
        1 => Some(Color4::new(
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
        )),
        4 => Some(Color4::new(
            parse_real(&vec[0])?,
            parse_real(&vec[1])?,
            parse_real(&vec[2])?,
            parse_real(&vec[3])?,
        )),
        _ => None,
    }
}

pub fn parse_color3(node: &Yaml) -> Option<Color3> {
    if !node.is_array() {
        return None;
    }

    let vec = node.as_vec()?;

    match vec.len() {
        1 => Some(Color3::new(
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
            parse_real(&vec[0])?,
        )),
        3 => Some(Color3::new(
            parse_real(&vec[0])?,
            parse_real(&vec[1])?,
            parse_real(&vec[2])?,
        )),
        _ => None,
    }
}
