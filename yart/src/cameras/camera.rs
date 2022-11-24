use crate::geometries::ray::Ray;
use rand::RngCore;

pub trait Camera {
    fn screen_size(&self) -> (u32, u32);
    fn subpixel_count(&self) -> u32;

    fn create_ray(&self, rng: &mut dyn RngCore, pixel: (u32, u32), subpixel: (u32, u32)) -> Ray;
}
