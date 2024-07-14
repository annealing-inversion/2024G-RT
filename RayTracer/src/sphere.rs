pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
pub use crate::hittable::{hit_record, Hittable};
pub use crate::interval::Interval;
pub use crate::material::Material;
pub use crate::aabb::aabb;
use std::rc::Rc;

pub struct Sphere {
    // pub center: Vec3,
    pub center1: Vec3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
    pub is_moving: bool,
    pub center_vec: Vec3,
    pub bbox: aabb,
}   
impl Sphere {
    pub fn new (center: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        // self.bbox = aabb::new(center - rvec, center + rvec);
        Self {
            center1: center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::zero(),
            // bbox: aabb::new(center - rvec, center + rvec),
            bbox: aabb::new_from_points(center - rvec, center + rvec),
        }
    }
    pub fn new_moving (center1: Vec3, center2: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        // let bbox1 = aabb::new(center1 - rvec, center1 + rvec);
        let bbox1 = aabb::new_from_points(center1 - rvec, center1 + rvec);
        // let bbox2 = aabb::new(center2 - rvec, center2 + rvec);
        let bbox2 = aabb::new_from_points(center2 - rvec, center2 + rvec);
        Self {
            center1,
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: aabb::new_from_aabbs(&bbox1, &bbox2),
        }
    }
    // pub fn new (center1: Vec3, center2: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
    //     Self {
    //         center1,
    //         radius,
    //         mat,
    //         is_moving: true,
    //         center_vec: center2 - center1,
    //     }
    // }
    pub fn sphere_center(&self, time: f64) -> Vec3 {
        return self.center1 + self.center_vec * time;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let center = if self.is_moving {self.sphere_center(r.time())} else {self.center1};
        let oc = center - r.origin();
        let a = r.direction().dot(r.direction());
        let h = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if root <= 0.003 {
            return false;
        }
        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Rc::clone(&self.mat);

        // println!("{}",*rec.mat.as_ref());
        //println!("rec.mat: {:?}", rec.mat);

        return true;
    }
    fn bounding_box(&self) -> aabb {
        // return bbox;
        return self.bbox;
    }
}