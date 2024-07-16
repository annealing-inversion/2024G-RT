// use crate::hittable::{Hittable, HittableList};
use crate::hittable::{Hittable, hit_record};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::aabb::aabb;
use crate::interval::*;
use crate::vec3::Vec3;
// use crate::raytracer:
use crate::raytracer::random_int_range;
use std::rc::Rc;
use std::sync::Arc;
use std::cmp::Ordering;

pub struct bvh_node {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: aabb,
}
impl bvh_node {
    // pub fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, time0: f64, time1: f64) -> Self {
    //     let box_left = left.bounding_box(time0, time1);
    //     let box_right = right.bounding_box(time0, time1);
    //     let bounding_box = aabb::new_from_aabbs(&box_left, &box_right);
    //     Self {left, right, bounding_box}
    // }
    pub fn new (list: HittableList) -> Self {   
        let len = list.objects.len();
        Self::new_with_span(list.objects, 0, len)
    }
    fn new_with_span(mut objects: Vec<Arc<dyn Hittable + Send + Sync>>, start: usize, end: usize) -> Self {
        let mut tmp = Self {left: Arc::new(HittableList::new()), right: Arc::new(HittableList::new()), bbox: aabb::empty};
        // tmp.bbox = aabb::empty;
        for object_index in start..end {
            tmp.bbox = aabb::new_from_aabbs(&tmp.bbox, &objects[object_index].bounding_box());
        }

        let axis = tmp.bbox.longest_axis();
        // let axis = raytracer::random_int_range(0, 2);
        let comparator = match axis {
            0 => |a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>| Self::box_x_compare(a.clone(), b.clone()),
            1 => |a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>| Self::box_y_compare(a.clone(), b.clone()),
            2 => |a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>| Self::box_z_compare(a.clone(), b.clone()),
            _ => panic!("Invalid axis index"),
        };
        let object_span = end - start;
        if object_span == 1 {
            tmp.left = objects[start].clone();
            tmp.right = objects[start].clone();
        } else if object_span == 2 {
            tmp.left = objects[start].clone();
            tmp.right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(comparator); //???
            
            let mid = start + object_span / 2;
            // let objects_clone = objects.clone();
            tmp.left = Arc::new(bvh_node::new_with_span(objects.clone(), start, mid));
            tmp.right = Arc::new(bvh_node::new_with_span(objects.clone(), mid, end));
        }
        // tmp.bbox = aabb::new_from_aabbs(&tmp.left.bounding_box(), &tmp.right.bounding_box());
        tmp
    }
    pub fn box_compare(a: Arc<dyn Hittable + Send + Sync>, b: Arc<dyn Hittable + Send + Sync>, axis: usize) -> bool {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);
        a_axis_interval.min < b_axis_interval.min
    }
    // pub fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    //     // box_compare(a, b, 0)
    //     Self::box_compare(a, b, 0)
    // }
    pub fn box_x_compare(a: Arc<dyn Hittable + Send + Sync>, b: Arc<dyn Hittable + Send + Sync>) -> std::cmp::Ordering {
        if Self::box_compare(a.clone(), b.clone(), 0) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
    // pub fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    //     // box_compare(a, b, 1)
    //     Self::box_compare(a, b, 1)
    // }
    // pub fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    //     // box_compare(a, b, 2)
    //     Self::box_compare(a, b, 2)
    // }
    pub fn box_y_compare(a: Arc<dyn Hittable + Send + Sync>, b: Arc<dyn Hittable + Send + Sync>) -> std::cmp::Ordering {
        if Self::box_compare(a.clone(), b.clone(), 1) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
    pub fn box_z_compare(a: Arc<dyn Hittable + Send + Sync>, b: Arc<dyn Hittable + Send + Sync>) -> std::cmp::Ordering {
        if Self::box_compare(a.clone(), b.clone(), 2) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }

}
impl Hittable for bvh_node {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);
        hit_left || hit_right
    }
    fn bounding_box(&self) -> aabb {
        return self.bbox;
    }
}