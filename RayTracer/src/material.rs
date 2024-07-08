use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{hit_record, Hittable};


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        false
    }
}

pub struct lambertian {
    pub albedo: Vec3,
} 
impl lambertian {
    pub fn new(a: Vec3) -> Self {
        Self { albedo: a }
    }
}
impl Material for lambertian {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct metal {
    pub albedo: Vec3,
}
impl metal {
    pub fn new(a: Vec3) -> Self {
        Self { albedo: a }
    }
}
impl Material for metal {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        // let reflected = r_in.direction().reflect(rec.normal);
        // let reflected = reflect(r_in.direction(), rec.normal);
        let reflected = r_in.direction().reflect(rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}