#![allow(warnings)]
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
mod aabb;
mod bvh;
mod texture;
mod perlin;
mod quad;

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
use aabb::*;
use bvh::*;
use texture::*;
// use crate::texture::*;
use material::*;
use perlin::*;
use quad::Quad;
// use quad::quad;

use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;

const AUTHOR: &str = "name";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = Rc::new(checker_texture::new_from_colors(0.32,Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(lambertian::new_with_texture(checker)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Rc::new(crate::material::lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0,0.5), 0.0);
                    world.add(Rc::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
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

    // world = Rc::new(bvh_node::new(world));
    // world = HittableList::new_from_bvh(world);
    // world = HittableList::new_from_bvh(bvh_node::new(world));
    let bvh = Rc::new(bvh_node::new(world));
    let mut new_world = HittableList::new();
    new_world.add(bvh);
    world = new_world;


    let mut cam = Camera::new();
    // cam.width = 1200;
    // cam.height = 800;
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 30;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;   

    // println!("P3\n{} {}\n255", cam.width, cam.height);


    cam.render(&world);

}
fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Rc::new(checker_texture::new_from_colors(0.32,Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Rc::new(lambertian::new_with_texture(checker.clone())))));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Rc::new(lambertian::new_with_texture(checker.clone())))));
    let mut cam = Camera::new();
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 10;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.render(&world);
}
fn earth() {
    let earth_texture = Rc::new(image_texture::new("earthmap.jpg"));
    let earth_surface = Rc::new(lambertian::new_with_texture(earth_texture));
    let globe = Rc::new(Sphere::new(Vec3::zero(), 2.0, earth_surface));
    let mut cam = Camera::new();
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 30;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 12.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // cam.render(&HittableList::new_from_list(vec![globe]));
    let mut world = HittableList::new();
    world.add(globe);
    cam.render(&world);
}
fn perlin_spheres() {
    let mut world = HittableList::new();
    let pertext = Rc::new(noise_texture::new_with_scale(4.0));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(lambertian::new_with_texture(pertext.clone())))));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Rc::new(lambertian::new_with_texture(pertext.clone())))));

    let mut cam = Camera::new();
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 10;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    // println!("test");
    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();
    let left_red = Rc::new(lambertian::new(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(lambertian::new(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(lambertian::new(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(lambertian::new(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(lambertian::new(Vec3::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(Vec3::new(-3.0,-2.0,5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(-2.0,-2.0,0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(3.0,-2.0,1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(-2.0,3.0,1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(-2.0,-3.0,5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal.clone())));

    let mut cam = Camera::new();
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 10;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.7, 0.8, 1.0);
    cam.vfov = 80.0;
    cam.lookfrom = Vec3::new(0.0, 0.0, 9.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.render(&world);
}
fn simple_light() {
    let mut world = HittableList::new();
    let pertext = Rc::new(noise_texture::new_with_scale(4.0));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(lambertian::new_with_texture(pertext.clone())))));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Rc::new(lambertian::new_with_texture(pertext.clone()))))); 
    let difflight = Rc::new(diffuse_light::new_from_emit_color(Vec3::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));  
    world.add(Rc::new(Quad::new(Vec3::new(3.0,1.0,-2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), difflight.clone())));
    let mut cam = Camera::new();
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 10;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);
    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(26.0, 3.0, 6.0);
    cam.lookat = Vec3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.render(&world);
}
fn cornell_box() {
    let mut world = HittableList::new();
    let red = Rc::new(lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Rc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Rc::new(lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Rc::new(diffuse_light::new_from_emit_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0,0.0, 555.0), red.clone())));    
    world.add(Rc::new(Quad::new(Vec3::new(343.0,554.0,332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(555.0,555.0,555.0),Vec3::new(-555.0,0.0,0.0),Vec3::new(0.0,0.0,-555.0), white.clone())));
    world.add(Rc::new(Quad::new(Vec3::new(0.0,0.0,555.0),Vec3::new(555.0,0.0,0.0),Vec3::new(0.0,555.0,0.0), white.clone())));
    
    world.add(Quad::boxx(Vec3::new(130.0,0.0,65.0),Vec3::new(295.0,165.0,230.0),white.clone()));
    world.add(Quad::boxx(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone())); 

    let mut cam = Camera::new();

    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 50;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.render(&world);

}


fn main() {
    match 7 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => {}
    }
    

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
