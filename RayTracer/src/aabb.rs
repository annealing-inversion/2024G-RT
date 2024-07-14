use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::interval::Interval;

#[derive(Clone, Copy)]
pub struct aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}
impl aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
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
        let x = if p0.x < p1.x { Interval::new(p0.x, p1.x) } else { Interval::new(p1.x, p0.x) };
        let y = if p0.y < p1.y { Interval::new(p0.y, p1.y) } else { Interval::new(p1.y, p0.y) };
        let z = if p0.z < p1.z { Interval::new(p0.z, p1.z) } else { Interval::new(p1.z, p0.z) };
        Self { x, y, z }
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