use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::interval::Interval;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}
impl aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut tmp = Self::empty;
        tmp.x = x;
        tmp.y = y;
        tmp.z = z;
        tmp.pad_to_minimums();
        // Self { x, y, z }
        tmp
    }
    pub const empty: aabb = aabb {
        x: Interval::empty,
        y: Interval::empty,
        z: Interval::empty,
    };
    pub const universe: aabb = aabb {
        x: Interval::universe,
        y: Interval::universe,
        z: Interval::universe,
    };
    pub fn new_from_points(p0: Vec3, p1: Vec3) -> Self {
        // let x = if p0.x < p1.x { Interval::new(p0.x, p1.x) } else { Interval::new(p1.x, p0.x) };
        // let y = if p0.y < p1.y { Interval::new(p0.y, p1.y) } else { Interval::new(p1.y, p0.y) };
        // let z = if p0.z < p1.z { Interval::new(p0.z, p1.z) } else { Interval::new(p1.z, p0.z) };
        // Self::pad_to_minimums();
        // Self { x, y, z }
        let mut tmp = Self::empty;
        tmp.x = if p0.x < p1.x { Interval::new(p0.x, p1.x) } else { Interval::new(p1.x, p0.x) };
        tmp.y = if p0.y < p1.y { Interval::new(p0.y, p1.y) } else { Interval::new(p1.y, p0.y) };
        tmp.z = if p0.z < p1.z { Interval::new(p0.z, p1.z) } else { Interval::new(p1.z, p0.z) };
        tmp.pad_to_minimums();
        tmp
    }
    pub fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta { self.x = self.x.expand(delta); }
        if self.y.size() < delta { self.y = self.y.expand(delta); }
        if self.z.size() < delta { self.z = self.z.expand(delta); }
        // if x.size() < delta x = x.expand(delta);
        // if y.size() < delta y = y.expand(delta);
        // if z.size() < delta z = z.expand(delta);
    }
    pub fn new_from_aabbs(box0: &aabb, box1: &aabb) -> Self {
        let x = Interval::new_from_intervals(&box0.x, &box1.x);
        let y = Interval::new_from_intervals(&box0.y, &box1.y);
        let z = Interval::new_from_intervals(&box0.z, &box1.z);
        Self { x, y, z }
    }
    pub fn axis_interval(&self, n: usize) -> Interval {
        if n == 1 {
            return self.y;
        } else if n == 2 {
            return self.z;
        } else {
            return self.x;
        }
    }
    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            } else {
                return 2;
            }
        } else {
            if self.y.size() > self.z.size() {
                return 1;
            } else {
                return 2;
            }
        }
    }
    pub fn hit(&self, r: &Ray, mut t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;
            if t0 < t1 {
                if t0 > t.min { t.min = t0; }
                if t1 < t.max { t.max = t1; }
            } else {
                if t1 > t.min { t.min = t1; }
                if t0 < t.max { t.max = t0; }
            }
        }
        if t.min >= t.max { return false; }
        return true;
    }
}

impl Add<Vec3> for aabb {
    type Output = Self;
    fn add(self, offset: Vec3) -> Self {
        Self {
            // self.x + offset.x,
            // self.y + offset.y,
            // self.z + offset.z,
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}