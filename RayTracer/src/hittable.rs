use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::aabb;
// use std::rc::Rc;
use std::sync::Arc;


pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat:Arc<dyn Material + Send + Sync>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}
impl hit_record {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {outward_normal * -1.0};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool;
    fn bounding_box(&self) -> aabb;
}

pub struct translate {
    pub object: Arc<dyn Hittable + Send + Sync>,
    pub offset: Vec3,
    pub bbox: aabb,
}
impl translate {
    pub fn new (object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self { object, offset, bbox }
    }
}

impl Hittable for translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }
        rec.p += self.offset;
        return true;
    }
    fn bounding_box(&self) -> aabb {
        self.bbox
    }
}

pub struct rotate_y {
    pub object: Arc<dyn Hittable + Send + Sync>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: aabb,
}
impl rotate_y {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {

        let mut bbox = object.bounding_box();
        let mut new_rot = rotate_y { object, sin_theta: 0.0, cos_theta: 0.0, bbox: aabb::empty };
        let radians = angle.to_radians();
        new_rot.sin_theta = radians.sin();
        new_rot.cos_theta = radians.cos();
        // let mut bbox = object.bounding_box();
        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;    
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
                    let newx = new_rot.cos_theta * x + new_rot.sin_theta * z;
                    let newz = -new_rot.sin_theta * x + new_rot.cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3{
                        if tester[c] < min[c] { min[c] = tester[c]; }
                        if tester[c] > max[c] { max[c] = tester[c]; }
                    }
                }
            }
        }
        bbox = aabb::new_from_points(min, max);
        new_rot.bbox = bbox;
        new_rot
    }

}

impl Hittable for rotate_y {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin.x = self.cos_theta * r.origin().x - self.sin_theta * r.origin().z;
        origin.z = self.sin_theta * r.origin().x + self.cos_theta * r.origin().z;
        direction.x = self.cos_theta * r.direction().x - self.sin_theta * r.direction().z;
        direction.z = self.sin_theta * r.direction().x + self.cos_theta * r.direction().z;
        let rotated_r = Ray::new_with_time(origin, direction, r.time());
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }
        let mut p = rec.p;
        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
        
        let mut normal = rec.normal;
        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
        rec.p = p;
        rec.normal = normal;

        return true;
    }
    fn bounding_box(&self) -> aabb {
        self.bbox
    }
}