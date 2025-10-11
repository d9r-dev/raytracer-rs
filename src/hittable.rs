use crate::{Point3, ray::Ray, vec::Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: Option<bool>,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            point,
            normal,
            t,
            front_face: None,
        }
    }

    pub fn set_face_normal(self: &mut Self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Some(ray.direction.dot(outward_normal) < 0.0);
        self.normal = if let Some(is) = self.front_face {
            if is { outward_normal } else { -outward_normal }
        } else {
            outward_normal
        }
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let dis = h * h - a * c;
        if dis < 0.0 {
            return None;
        }

        let sqrtd = dis.sqrt();
        let root = (h - sqrtd) / a;
        if root <= t_min || root >= t_max {
            return None;
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let mut record = HitRecord::new(p, normal, root);
        record.set_face_normal(ray, normal);

        Some(record)
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, ray_min: f64, ray_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_max;
        let mut temp_rec = None;

        for object in self {
            if let Some(rec) = object.hit(ray, ray_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
