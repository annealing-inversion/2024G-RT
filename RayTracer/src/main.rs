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
mod con_medium;

use std::rc::Rc;
use std::sync::Arc;
use vec3::*;
use ray::*;
use crate::hittable::*;
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
// use constant_medium::*;
use con_medium::*;
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

    let checker = Arc::new(checker_texture::new_from_colors(0.32,Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_with_texture(checker)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Arc::new(crate::material::lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0,0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_with_range(0.5, 1.0);
                    let fuzz = raytracer::random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(crate::material::metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(crate::material::dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(crate::material::dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));
    let material2 = Arc::new(crate::material::lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));
    let material3 = Arc::new(crate::material::metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // world = Arc::new(bvh_node::new(world));
    // world = HittableList::new_from_bvh(world);
    // world = HittableList::new_from_bvh(bvh_node::new(world));
    let bvh = Arc::new(bvh_node::new(world));
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
    cam.render(Arc::new(world));
    // cam.render(&world);

}
fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(checker_texture::new_from_colors(0.32,Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Arc::new(lambertian::new_with_texture(checker.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Arc::new(lambertian::new_with_texture(checker.clone())))));
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
    cam.render(Arc::new(world));
}
fn earth() {
    let earth_texture = Arc::new(image_texture::new("earthmap.jpg"));
    let earth_surface = Arc::new(lambertian::new_with_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Vec3::zero(), 2.0, earth_surface));
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
    cam.render(Arc::new(world));
}
fn perlin_spheres() {
    let mut world = HittableList::new();
    let pertext = Arc::new(noise_texture::new_with_scale(4.0));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_with_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(lambertian::new_with_texture(pertext.clone())))));

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
    cam.render(Arc::new(world));
}

fn quads() {
    let mut world = HittableList::new();
    let left_red = Arc::new(lambertian::new(Vec3::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(lambertian::new(Vec3::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(lambertian::new(Vec3::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(lambertian::new(Vec3::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(lambertian::new(Vec3::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(Vec3::new(-3.0,-2.0,5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(-2.0,-2.0,0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(3.0,-2.0,1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(-2.0,3.0,1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(-2.0,-3.0,5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal.clone())));

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
    cam.render(Arc::new(world));
}
fn simple_light() {
    let mut world = HittableList::new();
    let pertext = Arc::new(noise_texture::new_with_scale(4.0));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(lambertian::new_with_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(lambertian::new_with_texture(pertext.clone()))))); 
    let difflight = Arc::new(diffuse_light::new_from_emit_color(Vec3::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));  
    world.add(Arc::new(Quad::new(Vec3::new(3.0,1.0,-2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), difflight.clone())));
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
    cam.render(Arc::new(world));
}
fn cornell_box() {
    let mut world = HittableList::new();
    let red = Arc::new(lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(diffuse_light::new_from_emit_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0,0.0, 555.0), red.clone())));    
    world.add(Arc::new(Quad::new(Vec3::new(343.0,554.0,332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(555.0,555.0,555.0),Vec3::new(-555.0,0.0,0.0),Vec3::new(0.0,0.0,-555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0,0.0,555.0),Vec3::new(555.0,0.0,0.0),Vec3::new(0.0,555.0,0.0), white.clone())));
    
    // world.add(Quad::boxx(Vec3::new(130.0,0.0,65.0),Vec3::new(295.0,165.0,230.0),white.clone()));
    // world.add(Quad::boxx(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone())); 
    let mut box1 = Quad::boxx(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,330.0,165.0),white.clone());
    // box1 = Arc::new(rotate_y::new(box1, 15.0));
    let box1 = Arc::new(rotate_y::new(box1, 15.0));
    // box1 = Arc::new(translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    let box1 = Arc::new(translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    world.add(box1);
    // let mut box2 = Arc::new(Quad::boxx(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,165.0,165.0),white.clone()));
    let mut box2 = Quad::boxx(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,165.0,165.0),white.clone());
    // box2 = Arc::new(rotate_y::new(box2, -18.0));
    let box2 = Arc::new(rotate_y::new(box2, -18.0));
    // box2 = Arc::new(translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    let box2 = Arc::new(translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    world.add(box2);


    let mut cam = Camera::new();

    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 30;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.render(Arc::new(world));

}
fn cornell_smoke() {
    let mut world = HittableList::new();
    let red = Arc::new(lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(diffuse_light::new_from_emit_color(Vec3::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0,0.0, 555.0), red.clone())));    
    world.add(Arc::new(Quad::new(Vec3::new(113.0,554.0,127.0), Vec3::new(330.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 305.0), light.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 555.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0,0.0,0.0),Vec3::new(555.0,0.0,0.0),Vec3::new(0.0,0.0,555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0,0.0,555.0),Vec3::new(555.0,0.0,0.0),Vec3::new(0.0,555.0,0.0), white.clone())));
    
    let box1 = Quad::boxx(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,330.0,165.0),white.clone());
    let box1 = Arc::new(rotate_y::new(box1, 15.0));
    let box1 = Arc::new(translate::new(box1, Vec3::new(265.0,0.0,295.0)));
    // world.add(box1);
    world.add(Arc::new(constant_medium::new_from_color(box1, 0.01, Vec3::new(0.0, 0.0, 0.0))));
    let box2 = Quad::boxx(Vec3::new(0.0,0.0,0.0),Vec3::new(165.0,165.0,165.0),white.clone());
    let box2 = Arc::new(rotate_y::new(box2, -18.0));
    let box2 = Arc::new(translate::new(box2, Vec3::new(130.0,0.0,65.0)));
    world.add(Arc::new(constant_medium::new_from_color(box2, 0.01, Vec3::new(1.0, 1.0, 1.0))));



    let mut cam = Camera::new();
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 40;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(278.0, 278.0, -800.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.render(Arc::new(world));
}
fn final_scene() {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(lambertian::new(Vec3::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;

    for i in 0..boxes_per_side{
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = raytracer::random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Quad::boxx(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone()));
        }
    }
    let mut world = HittableList::new();
    world.add(Arc::new(bvh_node::new(boxes1)));

    let light = Arc::new(diffuse_light::new_from_emit_color(Vec3::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(Vec3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), light.clone())));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(center1, center2, 50.0, sphere_material.clone())));

    world.add(Arc::new(Sphere::new(Vec3::new(260.0,150.0,45.0),50.0,Arc::new(dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0,150.0,145.0),50.0,Arc::new(metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)))));

    let boundary = Arc::new(Sphere::new(Vec3::new(360.0,150.0,145.0),70.0,Arc::new(dielectric::new(1.5))));
    world.add(boundary.clone());
    world.add(Arc::new(constant_medium::new_from_color(boundary.clone(), 0.2, Vec3::new(0.2, 0.4, 0.9))));
    let boundary = Arc::new(Sphere::new(Vec3::zero(), 5000.0, Arc::new(dielectric::new(1.5))));
    world.add(Arc::new(constant_medium::new_from_color(boundary.clone(), 0.0001, Vec3::new(1.0, 1.0, 1.0))));

    let emat = Arc::new(lambertian::new_with_texture(Arc::new(image_texture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(Vec3::new(400.0,200.0,400.0),100.0,emat)));
    let pertext = Arc::new(noise_texture::new_with_scale(0.2));
    world.add(Arc::new(Sphere::new(Vec3::new(220.0,280.0,300.0),80.0,Arc::new(lambertian::new_with_texture(pertext)))));


    let mut boxes2 = HittableList::new();
    let white = Arc::new(lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(Vec3::random_with_range(0.0, 165.0), 10.0, white.clone())));
    }
    world.add(Arc::new(translate::new(Arc::new(rotate_y::new(Arc::new(bvh_node::new(boxes2)), 15.0)), Vec3::new(-100.0, 270.0, 395.0))));


    let mut cam = Camera::new();
    cam.width = 800;
    cam.height = 800;
    cam.samples_per_pixel = 10000;
    cam.aspect_ratio = cam.width as f64 / cam.height as f64;
    cam.max_depth = 50;
    cam.background = Vec3::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Vec3::new(478.0, 278.0, -600.0);
    cam.lookat = Vec3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.render(Arc::new(world));
}

fn main() {
    match 9 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(),
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
