pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
pub use crate::hittable::{hit_record, Hittable};
pub use crate::interval::Interval;
pub use crate::material::Material;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}   
impl Sphere {
    // pub fn new(center: Vec3, radius: f64) -> Self {
    //     // let default_mat = Rc::new(crate::material::lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    //     Self { center, radius }
    // }
    pub fn new(center: Vec3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().dot(r.direction());
        let h = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Rc::clone(&self.mat);

        // println!("{}",*rec.mat.as_ref());
        //println!("rec.mat: {:?}", rec.mat); 

        return true;
    }
}