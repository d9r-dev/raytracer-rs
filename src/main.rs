mod color;
mod hittable;
mod ray;
mod vec;
mod interval;
mod camera;
mod util;
mod material;

use std::rc::Rc;

use vec::Vec3;

use crate::{camera::Camera, hittable::{Hittable, Sphere}, material::{Lambertian, Metal}};

type Point3 = vec::Vec3;
pub type Color = vec::Vec3;

fn main() {
   let mut world: Vec<Box<dyn Hittable>> = Vec::new();

   let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
   let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
   let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
   let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

   world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
   world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
   world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
   world.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    
    let mut camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}

