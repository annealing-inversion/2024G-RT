
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::{hit_record, Hittable};
use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;
use crate::aabb::*;
use crate::bvh::*;  

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox: aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() , bbox: aabb::empty }
    }
    // pub fn new_from_bvh(node: bvh_node) -> Self {
    //     let mut list = HittableList::new();
    //     list.add(node);
    //     list
    // }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.bbox = aabb::new_from_aabbs(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        let mut temp_rec = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(crate::material::lambertian::new(Vec3::zero())),
            u: 0.0,
            v: 0.0,     
        };
        // bool hit_anything = false;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.u = temp_rec.u;
                rec.v = temp_rec.v;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;
                rec.mat = Arc::clone(&temp_rec.mat);

            }
        }
        return hit_anything;
    }
    fn bounding_box(&self) -> aabb {
        return self.bbox;
    }
}