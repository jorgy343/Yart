use crate::geometries::ray::Ray;
use rand::RngCore;

pub trait Camera {
    fn create_ray(&self, rng: &mut dyn RngCore, pixel: (u32, u32), subpixel: (u32, u32)) -> Ray;
}
