use std::rc::Rc;
use crate::hittable::{Hittable, hit_record};
use crate::material::Material;
// use crate::texture::Texture;
use crate::texture::texture;
use crate::vec3::Vec3;
use crate::aabb::aabb;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Isotropic;
use crate::raytracer;

pub struct constant_medium {
    pub boundary: Rc<dyn Hittable>,
    pub neg_inv_density: f64,
    pub phase_function: Rc<dyn Material>,
}

impl constant_medium {
    pub fn new(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn texture>) -> Self {
        constant_medium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::new(a)),
        }
    }
    pub fn new_from_color(b: Rc<dyn Hittable>, d: f64, c: Vec3) -> Self {
        println!("constant_medium::new_from_color");
        constant_medium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::new_from_color(c)),
        }
        
    }
}
impl Hittable for constant_medium {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        // println!("constant_medium::hit");
        // let rec1 = hit_record::new();
        let mut rec1 = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Rc::new(crate::material::lambertian::new(Vec3::zero())),
            u: 0.0,
            v: 0.0,
        };
        // let rec2 = hit_record::new();
        let mut rec2 = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Rc::new(crate::material::lambertian::new(Vec3::zero())),
            u: 0.0,
            v: 0.0,
        };

        if !self.boundary.hit(r, Interval::universe, &mut rec1) {
            return false;
        }
        // println!("test1 rec1.t: {} rec2.t: {}", rec1.t, rec2.t);
        // println!("rec1.t modified: {}", rec1.t + 0.0001);
        if !self.boundary.hit(r, Interval::new(rec1.t + 0.0001, f64::INFINITY), &mut rec2) {
            return false;
        }
        // println!("rec1.t: {}, rec2.t: {}", rec1.t, rec2.t);
        // println!("test2 rec1.t: {} rec2.t: {}", rec1.t, rec2.t);
        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        // println!("rec1.t: {}, rec2.t: {}", rec1.t, rec2.t);

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * raytracer::random_double().ln();
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);  // arbitrary
        rec.front_face = true;     // also arbitrary
        rec.mat = self.phase_function.clone();
        // println!("constant_medium::hit returning true");
        true
    }
    fn bounding_box(&self) -> aabb {
        self.boundary.bounding_box()
    }
}