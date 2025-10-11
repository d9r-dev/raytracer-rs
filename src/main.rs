mod color;
mod hittable;
mod ray;
mod vec;

use std::f64;

use ray::Ray;
use vec::Vec3;

use crate::hittable::{HitRecord, Hittable, Sphere};

type Point3 = vec::Vec3;
type Color = vec::Vec3;

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;

    let temp_image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let image_height: u32 = if temp_image_height < 1 {
        (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32
    } else {
        temp_image_height
    };
    let image_height_final: u32 = image_height;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    let viewport_width: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / image_height_final as f64);

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let focal_length = 1.0;
    let viewport_height = VIEWPORT_HEIGHT;
    let viewport_width = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / image_height_final as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = &viewport_v / image_height_final as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, image_height_final);

    for j in 0..image_height_final {
        eprint!("\rScanlines remaining: {} ", image_height_final - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = ray::Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(&ray, &world);
            color::write_color(pixel_color);
        }
    }
    eprint!("\r Done. \n");
}

fn ray_color(ray: &ray::Ray, world: &Vec<Box<dyn Hittable>>) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction.unit();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
