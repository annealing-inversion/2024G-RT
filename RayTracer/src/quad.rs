
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::{hit_record, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::aabb;
use std::rc::Rc;

pub struct Quad {
    pub q: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub mat: Rc<dyn Material>,
    pub bbox: aabb,
    pub normal: Vec3,
    pub d: f64,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let mut quad_instance = Quad { q, u, v, w: Vec3::zero(), mat, bbox: aabb::empty, normal: Vec3::zero(), d: 0.0 };
        quad_instance.set_bounding_box();
        // let n = quad_instance.u.cross(quad_instance.v);
        let n = Vec3::cross(quad_instance.u, quad_instance.v);
        quad_instance.normal = n.normalize();
        quad_instance.d = quad_instance.normal.dot(quad_instance.q);
        quad_instance.w = n / n.dot(n);
        quad_instance
    }
    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = aabb::new_from_points(self.q, self.q + self.u + self.v);
        let bbox_diagonal2 = aabb::new_from_points(self.q + self.u, self.q + self.v);
        self.bbox = aabb::new_from_aabbs(&bbox_diagonal1, &bbox_diagonal2);
    }
    pub fn is_interior(alpha: f64, beta: f64, rec: &mut hit_record) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }
        rec.u = alpha;
        rec.v = beta;
        return true;
    }

}
impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let denom = self.normal.dot(r.direction());
     
        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }
        let intersection = r.at(t); 
        let planar_hitpt_vector = intersection - self.q;
        // let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let alpha = self.w.dot(Vec3::cross(planar_hitpt_vector, self.v));
        // let beta = self.w.dot(self.u.cross(planar_hitpt_vector));
        let beta = self.w.dot(Vec3::cross(self.u, planar_hitpt_vector));
        // if !is_interior(alpha, beta, rec) {
        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }


        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);
        return true;
    }

    fn bounding_box(&self) -> aabb {
        return self.bbox;
    } 
}  