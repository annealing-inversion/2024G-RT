pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
pub use crate::hittable::{hit_record, Hittable};
pub use crate::interval::Interval;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,

}   
impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    // fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut hit_record) -> bool {
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
        // if root <= ray_tmin || root >= ray_tmax {
        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            // if root <= ray_tmin || root >= ray_tmax {
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return true;
    }
}