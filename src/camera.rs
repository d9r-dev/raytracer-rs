use crate::{color::{write_color}, hittable::Hittable, interval::Interval, ray::Ray, util::random_f64, vec::Vec3, Color, Point3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    pixel_sample_scale: f64,
    max_depth: u32,
}

impl Camera {
    fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            let direction = Vec3::random_on_hemisphere(&rec.normal);
            return 0.5 * Self::ray_color(&Ray::new(&rec.point, &direction), world, depth - 1);
        }
        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn new(aspect_ratio: f64, image_width: u32) -> Camera {
        //Image
        let temp_image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height: u32 = if temp_image_height < 1 {
            1
        } else {
            temp_image_height
        };

        const VIEWPORT_HEIGHT: f64 = 2.0;
        const FOCAL_LENGTH: f64 = 1.0;

        let viewport_width: f64 =
            VIEWPORT_HEIGHT * (image_width as f64 / image_height as f64);

        //Camera
        let center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        let viewport_upper_left = center
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
            - &viewport_u / 2.0
            - &viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let samples_per_pixel = 100;
        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;
        let max_depth = 10;
        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_sample_scale,
            max_depth
        }
    }

    pub fn render(&mut self, world: &Vec<Box<dyn Hittable>>) {

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color : Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray : Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + Camera::ray_color(&ray, world, self.max_depth);
                } 
                pixel_color = pixel_color * self.pixel_sample_scale;
                write_color(&pixel_color);
            }
        }

        eprint!("\r Done. \n");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x) * self.pixel_delta_u) + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}
