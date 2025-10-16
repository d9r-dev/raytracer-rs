mod color;
mod hittable;
mod ray;
mod vec;
mod interval;
mod camera;
mod util;

use vec::Vec3;

use crate::{camera::Camera, hittable::{Hittable, Sphere}};

type Point3 = vec::Vec3;
type Color = vec::Vec3;

fn main() {
   let mut world: Vec<Box<dyn Hittable>> = Vec::new();

   world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
   world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    
    let mut camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}

