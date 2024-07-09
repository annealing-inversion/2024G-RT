use crate::ray::Ray;
use crate::vec3::Vec3;
use indicatif::ProgressBar;
use image::{Rgb, RgbImage, ImageBuffer};
use crate::color::write_color;
use crate::interval::Interval;
use crate::hittable::{Hittable, hit_record};
use crate::material::Material;
use std::fs::File;
use std::rc::Rc;

pub struct Camera {
    pub aspect_ratio: f64,
    pub width: usize,
    pub height: usize,
    pub camera_center: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_loc: Vec3,
    pub samples_per_pixel: usize,
    pub pixel_samples_scale: f64,
    pub max_depth: usize,  
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            width: 100,
            height: 100,
            samples_per_pixel: 10,
            max_depth: 50,
            pixel_samples_scale: 0.1,
            camera_center: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel00_loc: Vec3::zero(),
        }
    }
    pub fn sample_square(&self) -> Vec3 {
        // return vec3::Vec3::new(random_double()-0.5, random_double()-0.5, 0.0);
        return Vec3::new(rand::random::<f64>()-0.5, rand::random::<f64>()-0.5, 0.0);
    }
    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let mut offset = self.sample_square();
        let mut pixel_sample = self.pixel00_loc + self.pixel_delta_u * (i as f64 + offset.x) + self.pixel_delta_v * (j as f64 + offset.y);
        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - self.camera_center;
        return Ray::new(ray_origin, ray_direction);
    }
    // pub fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> [f64; 3] {
    //     let mut rec = hit_record {
    //         p: Vec3::zero(),
    //         normal: Vec3::zero(),
    //         t: 0.0,
    //         front_face: false,
    //     };
    //     if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
    //         return [0.5*(rec.normal.x + 1.0), 0.5*(rec.normal.y + 1.0), 0.5*(rec.normal.z + 1.0)];
    //     }
    //     let unit_direction = r.direction().normalize();
    //     let a = 0.5 * (unit_direction.y + 1.0);
    //     let color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
    //     return [color.x, color.y, color.z];
    // }
    pub fn ray_color(&self, r: &Ray, depth: usize, world: &dyn Hittable) -> [f64; 3] {
        if depth <= 0 {
            return [0.0, 0.0, 0.0];
        }
        let mut rec = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Rc::new(crate::material::lambertian::new(Vec3::zero())),
        };
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
            let mut attenuation = Vec3::zero();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                // let tmp = attenuation * Vec3::from(self.ray_color(&scattered, depth-1, world));
                let tmp = attenuation * Vec3::from(self.ray_color(&scattered, depth-1, world));
                return [tmp.x, tmp.y, tmp.z];
            }
            else {
                return [0.0, 0.0, 0.0];
            }
        }
        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        let color = Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
        return [color.x, color.y, color.z];
    }

    pub fn initialize(&mut self) -> RgbImage {
        self.width = 800;
        self.height = 800;
        
        // let bar: ProgressBar = if is_ci() {
        // ProgressBar::hidden()
        // } else {
        //     ProgressBar::new((height * width) as u64)
        // };
        self.samples_per_pixel = 10;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        let mut img: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        self.aspect_ratio = self.width as f64 / self.height as f64;
        let mut focal_length = 1.0;
        let viewport_height = 2.0; 
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);
        self.camera_center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // pixel_delta_u = viewport_u / width as f64;
        self.pixel_delta_u = viewport_u / self.width as f64;
        // pixel_delta_v = viewport_v / height as f64;
        self.pixel_delta_v = viewport_v / self.height as f64;

        let viewport_upper_left = self.camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        // pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;
        self.pixel00_loc = viewport_upper_left + self.pixel_delta_u / 2.0 + self.pixel_delta_v / 2.0;
        return img;
    }
    pub fn render(&mut self, world: &dyn Hittable) -> () {
        let path = "output/test.jpg";
        let AUTHOR = "name";

        let mut img = self.initialize(); 
        for j in 0..self.height {
            // println!("j: {}", j);
            for i in 0..self.width {
                // println!("i: {}", i);
                let mut pixel_color = Vec3::zero();
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    // pixel_color += Vec3::from(self.ray_color(&r, world));
                    pixel_color += Vec3::from(self.ray_color(&r, self.max_depth, world));
                } 
                write_color(pixel_color * self.pixel_samples_scale, &mut img, i as usize, j as usize);
                // bar.inc(1);
            }
        }
        // bar.finish();
        let quality = 60;
        println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
        let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
        let mut output_file: File = File::create(path).unwrap();
        match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
            Ok(_) => {}
            Err(_) => println!("Outputting image fails."),
        }
    }
}