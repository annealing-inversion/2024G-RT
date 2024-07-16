use crate::ray::Ray;
use crate::vec3::Vec3;
use indicatif::ProgressBar;
use image::{Rgb, RgbImage, ImageBuffer};
use crate::color::write_color;
use crate::interval::Interval;
use crate::hittable::{Hittable, hit_record};
use crate::material::Material;
use crate::raytracer::random_double;
use std::fs::File;
use std::rc::Rc;
use std::sync::Arc;
use crossbeam::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, Condvar};
use indicatif::{ProgressStyle};

const HEIGHT_PARTITION: usize = 20;
const WIDTH_PARTITION: usize = 20;
const THREAD_LIMIT: usize = 16;


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
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
    pub background: Vec3,
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
            vfov: 90.0,
            lookfrom: Vec3::zero(),
            lookat: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
            background: Vec3::zero(),
        }
    }
    pub fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        return self.defocus_disk_u * p.x + self.defocus_disk_v * p.y + self.camera_center;
    }
    pub fn sample_square(&self) -> Vec3 {
        return Vec3::new(rand::random::<f64>()-0.5, rand::random::<f64>()-0.5, 0.0);
    }
    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let mut offset = self.sample_square();
        let mut pixel_sample = self.pixel00_loc + self.pixel_delta_u * (i as f64 + offset.x) + self.pixel_delta_v * (j as f64 + offset.y);
        let ray_origin = if self.defocus_angle > 0.0 {self.defocus_disk_sample()} else {self.camera_center};
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();
        return Ray::new_with_time(ray_origin, ray_direction, ray_time);
    }
    pub fn ray_color(&self, r: &Ray, depth: usize, world: Arc<dyn Hittable + Send + Sync>) -> [f64; 3] {
        if depth <= 0 {
            return [0.0, 0.0, 0.0];
        }
        let mut rec = hit_record {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(crate::material::lambertian::new(Vec3::zero())),
            u: 0.0,
            v: 0.0,
        };
        if !world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            return [self.background.x, self.background.y, self.background.z];
        }
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero(); 
        let mut color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);
        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return [color_from_emission.x, color_from_emission.y, color_from_emission.z];
        }
        let color_from_scatter = Vec3::from(self.ray_color(&scattered, depth-1, world)) * attenuation;
        return [color_from_emission.x + color_from_scatter.x, color_from_emission.y + color_from_scatter.y, color_from_emission.z + color_from_scatter.z];
    }

    pub fn initialize(&mut self) -> RgbImage {
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        let mut img: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);

        self.aspect_ratio = self.width as f64 / self.height as f64;
        
        self.camera_center = self.lookfrom;


        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = Vec3::cross(self.vup, self.w).normalize();
        self.v = Vec3::cross(self.w, self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * viewport_height * -1.0;

        self.pixel_delta_u = viewport_u / self.width as f64;
        self.pixel_delta_v = viewport_v / self.height as f64;

        let viewport_upper_left = self.camera_center - viewport_u / 2.0 - viewport_v / 2.0 - self.w * self.focus_dist;
        self.pixel00_loc = viewport_upper_left + self.pixel_delta_u / 2.0 + self.pixel_delta_v / 2.0;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
        return img;
    }
    // pub fn render(&mut self, world: &dyn Hittable) -> () {
    //     let bar: ProgressBar = if option_env!("CI").unwrap_or_default() == "true" {
    //         ProgressBar::hidden()
    //     } else {
    //         ProgressBar::new((self.height * self.width) as u64)
    //     };

    //     let path = "output2/test.jpg";
    //     let AUTHOR = "name";

    //     let mut img = self.initialize(); 
    //     for j in 0..self.height {
    //         for i in 0..self.width {
    //             let mut pixel_color = Vec3::zero();
    //             for sample in 0..self.samples_per_pixel {
    //                 let r = self.get_ray(i, j);
    //                 pixel_color += Vec3::from(self.ray_color(&r, self.max_depth, world));
    //             } 
    //             write_color(pixel_color * self.pixel_samples_scale, &mut img, i as usize, j as usize);
    //             bar.inc(1);
    //         }
    //     }
    //     bar.finish();
    //     let quality = 60;
    //     println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    //     let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    //     let mut output_file: File = File::create(path).unwrap();
    //     match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
    //         Ok(_) => {}
    //         Err(_) => println!("Outputting image fails."),
    //     }
    // }
    pub fn get_ProgressBar(height: usize, width: usize) -> ProgressBar {
        let bar: ProgressBar = if option_env!("CI").unwrap_or_default() == "true" {
         ProgressBar::hidden()
         } else {
         ProgressBar::new((height * width) as u64)
         };
        
         bar.set_style(ProgressStyle::default_bar()
         .template("{spinner:.green} Elapsed {elapsed_precise} [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {msg}")
         .progress_chars("●▸▹⋅"));
        
         bar
    }
    pub fn render(&mut self, world: Arc<dyn Hittable + Send + Sync>) -> () {
        // let bar: ProgressBar = if option_env!("CI").unwrap_or_default() == "true" {
        //     ProgressBar::hidden()
        // } else {
        //     ProgressBar::new((self.height * self.width) as u64)
        // };
        let bar = Self::get_ProgressBar(self.height, self.width);


        let path = "output2/test.jpg";
        let AUTHOR = "name";

        let mut img = self.initialize(); 
        let img_mtx = Arc::new(Mutex::new(&mut img));
        let camera_wrapper = Arc::new(self);
        let bar = Arc::new(bar);
        let bar_wrapper = Arc::clone(&bar);
        
        thread::scope(move |thd_spawner|{
            let thread_count = Arc::new(AtomicUsize::new(0));
            let thread_number_controller = Arc::new(Condvar::new());
      
            let chunk_width = (camera_wrapper.width + WIDTH_PARTITION - 1) / WIDTH_PARTITION;
            let chunk_height = (camera_wrapper.height + HEIGHT_PARTITION - 1) / HEIGHT_PARTITION;
            
            for j in 0..HEIGHT_PARTITION {
              for i in 0..WIDTH_PARTITION {
                // WAIT
                let lock_for_condv = Mutex::new(false);
                let mut thread_number_controller = thread_number_controller.clone();
                let mut thread_count = thread_count.clone();

                while !(thread_count.load(Ordering::SeqCst) < THREAD_LIMIT) { // outstanding thread number control
                  thread_number_controller.wait(lock_for_condv.lock().unwrap()).unwrap();
                }
                // ... // some Arc::clone(..._wrapper)        
                let camera = Arc::clone(&camera_wrapper);
                let world = Arc::clone(&world);
                let bar = Arc::clone(&bar_wrapper);
                let img_mtx = Arc::clone(&img_mtx);
                
                // move "thread_count++" out of child thread, so that it's sequential with thread number control code
                thread_count.fetch_add(1, Ordering::SeqCst);
                bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst))); // set "thread_count" information to progress bar
      
                let _ = thd_spawner.spawn(move |_| {
                  camera.render_sub(&world, &img_mtx, &bar, 
                    i * chunk_width, (i + 1) * chunk_width, 
                    j * chunk_height, (j + 1) * chunk_height);
      
                  thread_count.fetch_sub(1, Ordering::SeqCst); // subtract first, then notify.

                  let mut thread_number_controller = thread_number_controller.clone();
                //   let mut bar
                  bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst)));
                  // NOTIFY
                  thread_number_controller.notify_one();
                });
      
              }
            }
        }).unwrap();
        // let bar = Arc::clone(&bar);
        // bar_wrapper.finish();
        bar.finish();

        let quality = 60;
        println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
        let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
        let mut output_file: File = File::create(path).unwrap();
        match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
            Ok(_) => {}
            Err(_) => println!("Outputting image fails."),
        }
    }
    pub fn render_sub(&self, world: &Arc<dyn Hittable + Send + Sync>, img_mtx: &Arc<Mutex<&mut RgbImage>>, bar: &Arc<ProgressBar>, x_min: usize, x_max: usize, y_min: usize, y_max: usize) {
        let x_max = x_max.min(self.width);
        let y_max = y_max.min(self.height);

        let mut buff = Vec::new();

        for y in y_min..y_max {
            for x in x_min..x_max {
                let mut pixel_color = Vec3::zero();
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += Vec3::from(self.ray_color(&r, self.max_depth, world.clone()));
                } 
                // write_color(pixel_color * self.pixel_samples_scale, &mut buff, x as usize, y as usize);
                buff.push((x, y, pixel_color * self.pixel_samples_scale));
            }
            bar.inc((x_max - x_min) as u64);
        }
        let mut img = img_mtx.lock().unwrap();
        for (x, y, color) in buff {
            write_color(color, &mut img, x, y);
            // bar.inc(1);
        }


    }


}