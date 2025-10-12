use crate::{Color, Point3, color, hittable::Hittable, interval::Interval, ray::Ray, vec::Vec3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Color {
        if let Some(rec) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
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

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&mut self, world: &Vec<Box<dyn Hittable>>) {

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(&self.center, &ray_direction);
                let pixel_color = Self::ray_color(&ray, &world);
                color::write_color(pixel_color);
            }
        }

        eprint!("\r Done. \n");
    }
}
