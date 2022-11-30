use crate::{common::Real, math::color3::Color3, scene::Scene};
use rand::RngCore;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn render(scene: &Scene) -> Vec<Color3> {
    let width = scene.camera.screen_size().0;
    let height = scene.camera.screen_size().1;

    let patches = create_patches(width, height, 8);

    let patch_results = patches
        .iter()
        .map(|patch| {
            let mut rng = rand::thread_rng();
            render_patch(&mut rng, scene, patch)
        })
        .collect::<Vec<_>>();

    let mut pixels = vec![Color3::default(); (width * height) as usize];

    for patch_result in &patch_results {}

    pixels

    // let total_subpixels = scene.camera.subpixel_count() * scene.camera.subpixel_count();

    // let mut pixels = vec![Color3::default(); (width * height) as usize];

    // for _ in 0..scene.config.iterations() {
    //     for y in 0..height {
    //         for x in 0..width {
    //             let pixel_index = (y * width + x) as usize;

    //             for subpixel_y in 0..scene.camera.subpixel_count() {
    //                 for subpixel_x in 0..scene.camera.subpixel_count() {
    //                     let ray = scene.camera.create_ray(&mut rng, (x, y), (subpixel_x, subpixel_y));
    //                     let color = scene.cast_ray_color(&mut rng, &ray, 1);

    //                     pixels[pixel_index] += color;
    //                 }
    //             }

    //             pixels[pixel_index] /= total_subpixels as Real;
    //         }
    //     }
    // }

    // pixels
    //     .iter()
    //     .map(|pixel| {
    //         Rgb::from([
    //             (pixel.r / (scene.config.iterations() as Real) * 255.0) as u8,
    //             (pixel.g / (scene.config.iterations() as Real) * 255.0) as u8,
    //             (pixel.b / (scene.config.iterations() as Real) * 255.0) as u8,
    //         ])
    //     })
    //     .collect()
}

#[derive(Debug, Copy, Clone)]
struct Patch {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

impl Patch {
    fn new(start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
}

#[derive(Debug)]
struct PatchResults {
    patch: Patch,
    patch_pixels: Vec<Color3>,
}

impl PatchResults {
    fn new(patch: Patch, patch_pixels: Vec<Color3>) -> Self {
        Self { patch, patch_pixels }
    }
}

fn create_patches(width: u32, height: u32, patch_size: u32) -> Vec<Patch> {
    let mut patches = Vec::new();

    let mut start_y = 0u32;
    let mut start_x = 0u32;

    loop {
        if start_y >= height {
            break;
        }

        let end_y = if start_y + patch_size >= height {
            height - 1
        } else {
            start_y + patch_size - 1
        };

        loop {
            if start_x >= width {
                break;
            }

            let end_x = if start_x + patch_size >= width {
                width - 1
            } else {
                start_x + patch_size - 1
            };

            patches.push(Patch::new(start_x, start_y, end_x, end_y));
            start_x += patch_size;
        }

        start_y += patch_size;
    }

    patches
}

fn render_patch(rng: &mut dyn RngCore, scene: &Scene, patch: &Patch) -> PatchResults {
    let width = scene.camera.screen_size().0;
    let height = scene.camera.screen_size().1;

    let total_subpixels = scene.camera.subpixel_count() * scene.camera.subpixel_count();

    let mut pixels = vec![Color3::default(); (width * height) as usize];

    for _ in 0..scene.config.iterations() {
        for y in patch.start_y..=patch.end_y {
            for x in patch.start_x..=patch.end_x {
                let pixel_index = (y * width + x) as usize;

                for subpixel_y in 0..scene.camera.subpixel_count() {
                    for subpixel_x in 0..scene.camera.subpixel_count() {
                        let ray = scene.camera.create_ray(rng, (x, y), (subpixel_x, subpixel_y));
                        let color = scene.cast_ray_color(rng, &ray, 1);

                        pixels[pixel_index] += color;
                    }
                }

                pixels[pixel_index] /= total_subpixels as Real;
            }
        }
    }

    PatchResults::new(*patch, pixels)
}
