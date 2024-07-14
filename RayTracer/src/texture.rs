use crate::raytracer::*;
use crate::vec3::Vec3;
use std::rc::Rc;

pub trait texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct solid_color {
    albedo: Vec3,
}

impl solid_color {
    pub fn new(c: Vec3) -> Self {
        Self { albedo: c }
    }
    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { albedo: Vec3::new(r, g, b) }
    }
}

impl texture for solid_color {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct checker_texture {
    inv_scale: f64,
    even: Rc<dyn texture>,
    odd: Rc<dyn texture>,
}

impl checker_texture {
    pub fn new(scale: f64, even: Rc<dyn texture>, odd: Rc<dyn texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd, 
        }
    }
    pub fn new_from_colors(scale: f64, c1: Vec3, c2: Vec3) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(solid_color::new(c1)),
            odd: Rc::new(solid_color::new(c2)),
            // odd: Rc::new(solid_color::new(c1)),
            // even: Rc::new(solid_color::new(c2)),
        }
    }
}

impl texture for checker_texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        // let xinteger = (self.inv_scale * p.x).floor() as usize;
        // let yinteger = (self.inv_scale * p.y).floor() as usize;
        // let zinteger = (self.inv_scale * p.z).floor() as usize;
        // let xinteger = (self.inv_scale * p.x).floor() as usize;
        // let yinteger = (self.inv_scale * p.y).floor() as usize;
        // let zinteger = (self.inv_scale * p.z).floor() as usize;
        let xinteger = (self.inv_scale * p.x).floor() as i32;
        let yinteger = (self.inv_scale * p.y).floor() as i32;
        let zinteger = (self.inv_scale * p.z).floor() as i32;

        let iseven = (xinteger + yinteger + zinteger) % 2 == 0;
        if iseven {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}