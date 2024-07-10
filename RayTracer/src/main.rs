mod vec3;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod color;
mod raytracer;
mod interval;
mod camera;
mod material;

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
    let ground_material = Rc::new(crate::material::lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Rc::new(crate::material::lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_with_range(0.5, 1.0);
                    let fuzz = raytracer::random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(crate::material::metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(crate::material::dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(crate::material::dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));
    let material2 = Rc::new(crate::material::lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));
    let material3 = Rc::new(crate::material::metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));


    //here
    // let material_ground = Rc::new(crate::material::lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(crate::material::lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(crate::material::dielectric::new(1.5));
    // // let material_bubble = Rc::new(crate::material::dielectric::new(1.00/1.50));
    // let material_right = Rc::new(crate::material::metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    // world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    // world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    // // world.add(Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    // world.add(Rc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // let R = (pi / 4.0).cos();    
    // let material_left = Rc::new(material::lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    // let material_right = Rc::new(material::lambertian::new(Vec3::new(1.0, 0.0, 0.0)));
    // world.add(Rc::new(Sphere::new(Vec3::new(-R, 0.0, -1.0), R, material_left)));
    // world.add(Rc::new(Sphere::new(Vec3::new(R, 0.0, -1.0), R, material_right)));

    let mut cam = Camera::new();
    // cam.aspect_ratio
    cam.width = 1200;
    cam.height = 800;
    cam.samples_per_pixel = 30;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    // cam.lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    // cam.lookat = Vec3::new(0.0, 0.0, -1.0);
    // cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;   

    // println!("P3\n{} {}\n255", cam.width, cam.height);


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
