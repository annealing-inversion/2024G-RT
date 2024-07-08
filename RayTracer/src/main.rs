mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod color;
mod raytracer;
mod interval;
mod camera;

use std::rc::Rc;
use vec3::*;
use ray::*;
use crate::hittable::{hit_record, Hittable};
use hittable_list::*;
use sphere::*;
use color::*;
use raytracer::*;
use interval::*;
use camera::*;


use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;

const AUTHOR: &str = "name";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new();
    // cam.aspect_ratio
    cam.width = 800;
    cam.height = 800;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;

    cam.render(&world);
    
    


    // 以下是write color和process bar的示例代码
    // let pixel_color = [155u8; 3];
    // for i in 0..100 {
    //     for j in 0..100 {
    //         write_color(pixel_color, &mut img, i, j);
    //         bar.inc(1);
    //     }
    // }
    // bar.finish();

    // println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    // let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    // let mut output_file: File = File::create(path).unwrap();
    // match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
    //     Ok(_) => {}
    //     Err(_) => println!("Outputting image fails."),
    // }
}
