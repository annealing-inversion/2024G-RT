use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{hit_record, Hittable};
use crate::texture::*;
use crate::raytracer::random_double;
use std::rc::Rc;



pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
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
        // println!("rec.u: {}, rec.v: {}", rec.u, rec.v);
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
        let cos_theta = (unit_direction * -1.0).dot(rec.normal);
        let cos_theta = cos_theta.min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let mut direction = Vec3::zero();
        if cannot_refract || dielectric::reflectance(cos_theta, ri) > random_double() {
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

pub struct diffuse_light {
    pub tex: Rc<dyn texture>,
}

impl diffuse_light {
    pub fn new(t: Rc<dyn texture>) -> Self {
        Self { tex: t }
    }
    pub fn new_from_emit_color(c: Vec3) -> Self {
        Self { tex: Rc::new(solid_color::new(c)) }
    }
}

impl Material for diffuse_light {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    pub tex: Rc<dyn texture>,
}
impl Isotropic {
    pub fn new(t: Rc<dyn texture>) -> Self {
        Self { tex: t }
    }
    pub fn new_from_color(c: Vec3) -> Self {
        Self { tex: Rc::new(solid_color::new(c)) }
    }
}
impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *scattered = Ray::new_with_time(rec.p, Vec3::random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        return true;
    }
}