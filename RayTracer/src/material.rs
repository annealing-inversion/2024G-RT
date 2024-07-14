use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{hit_record, Hittable};
use crate::texture::*;
use std::rc::Rc;


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        false
    }
}

pub struct lambertian {
    pub albedo: Vec3,
    pub tex: Rc<dyn texture>,

} 
impl lambertian {
    pub fn new (a: Vec3) -> Self { //??
        Self { albedo: a, tex: Rc::new(solid_color::new(a)) }
    }
    pub fn new_with_texture (t: Rc<dyn texture>) -> Self {
        Self { albedo: Vec3::zero(), tex: t }
    }
}
impl Material for lambertian {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        // *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
        // *scattered = Ray::new(rec.p, scatter_direction);
        // *attenuation = self.albedo;
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        return true;
    }
}

pub struct metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl metal {
    pub fn new(a: Vec3, f: f64) -> Self {
        Self { albedo: a, fuzz: f.min(1.0) }    
    }
}
impl Material for metal {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        // let reflected = r_in.direction().reflect(rec.normal);
        // let reflected = reflect(r_in.direction(), rec.normal);
        let mut reflected = r_in.direction().reflect(rec.normal);
        reflected = reflected.normalize() + (Vec3::random_unit_vector() * self.fuzz);
        // *scattered = Ray::new(rec.p, reflected);
        // *scattered = Ray::new(rec.p, reflected, r_in.time());
        *scattered = Ray::new_with_time(rec.p, reflected, r_in.time());
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}

pub struct dielectric {
    pub refraction_index: f64,
}
impl dielectric {
    pub fn new(ri: f64) -> Self {
        Self { refraction_index: ri }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0-ref_idx) / (1.0+ref_idx);
        let r0 = r0*r0;
        return r0 + (1.0-r0)*((1.0-cosine).powi(5));
    }
}
impl Material for dielectric {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        // *attenuation = Vec3::new(0.9, 0.9, 0.9);
        let ri = if rec.front_face {1.0 / self.refraction_index} else {self.refraction_index};
        let unit_direction = r_in.direction().normalize();
        let cos_theta = (unit_direction * -1.0).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let mut direction = Vec3::zero();
        if ri * sin_theta > 1.0 || dielectric::reflectance(cos_theta, ri) > rand::random::<f64>() {
            direction = unit_direction.reflect(rec.normal);
        }
        else {
            direction = Vec3::refract(unit_direction, rec.normal, ri);
        }

        // *scattered = Ray::new(rec.p, direction);
        // *scattered = Ray::new(rec.p, direction, r_in.time());
        *scattered = Ray::new_with_time(rec.p, direction, r_in.time());

        return true;
    }
}