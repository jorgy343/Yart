use image::Rgb;

use crate::{common::Real, math::color3::Color3, scene::Scene};

pub fn render(scene: &Scene) -> Vec<Rgb<u8>> {
    let mut rng = rand::thread_rng();

    let width = scene.camera.screen_size().0;
    let height = scene.camera.screen_size().1;

    let total_subpixels = scene.camera.subpixel_count() * scene.camera.subpixel_count();

    let mut pixels = vec![Color3::default(); (width * height) as usize];

    for _ in 0..scene.config.iterations() {
        for y in 0..height {
            for x in 0..width {
                let pixel_index = (y * width + x) as usize;

                for subpixel_y in 0..scene.camera.subpixel_count() {
                    for subpixel_x in 0..scene.camera.subpixel_count() {
                        let ray = scene.camera.create_ray(&mut rng, (x, y), (subpixel_x, subpixel_y));
                        let color = scene.cast_ray_color(&mut rng, &ray, 1);

                        pixels[pixel_index] += color;
                    }
                }

                pixels[pixel_index] /= total_subpixels as Real;
            }
        }
    }

    pixels
        .iter()
        .map(|pixel| {
            Rgb::from([
                (pixel.r / (scene.config.iterations() as Real) * 255.0) as u8,
                (pixel.g / (scene.config.iterations() as Real) * 255.0) as u8,
                (pixel.b / (scene.config.iterations() as Real) * 255.0) as u8,
            ])
        })
        .collect()
}
