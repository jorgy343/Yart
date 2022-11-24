#![feature(trait_upcasting)]

pub mod cameras;
pub mod common;
pub mod errors;
pub mod geometries;
pub mod lights;
pub mod materials;
pub mod math;
pub mod miss_shaders;
pub mod rendering;
pub mod scene;
pub mod yaml;

use image::{ImageBuffer, Rgb, RgbImage};
use rendering::render;
use std::{error::Error, path::Path};
use yaml::parse::load_scene;

fn main() -> Result<(), Box<dyn Error>> {
    let scene = load_scene(Path::new("../../../scenes/test.yaml"))?;
    println!("{:?}", scene.area_lights);

    let pixels = render(&scene);

    write_image_file(&pixels)?;
    Ok(())
}

fn write_image_file(pixels: &Vec<Rgb<u8>>) -> Result<(), Box<dyn Error>> {
    let mut image: RgbImage = ImageBuffer::new(1920, 1080);

    for y in 0..1080 {
        for x in 0..1920 {
            image.put_pixel(x, y, pixels[(y * 1920 + x) as usize]);
        }
    }

    image.save("test.png")?;
    Ok(())
}
