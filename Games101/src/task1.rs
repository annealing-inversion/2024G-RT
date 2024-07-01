#![allow(warnings)]

use std::io;
pub use std::env;
pub use nalgebra::Vector3;
pub use opencv::{
    Result,
};
pub use opencv::core::Vector;
pub use crate::rasterizer1::{Buffer, Rasterizer,Primitive};
pub use crate::utils::*;
pub use crate::shader::FragmentShaderPayload;
pub use crate::texture::Texture;
use opencv::imgcodecs::imwrite;
use opencv::highgui::{imshow, wait_key};

pub fn t1()-> Result<()>{
    println!("选择任务1");
    let mut angle = 0.0;
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![Vector3::new(2.0, 0.0, -2.0),
                   Vector3::new(0.0, 2.0, -2.0),
                   Vector3::new(-2.0, 0.0, -2.0)];
    let ind = vec![Vector3::new(0, 1, 2)];

    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);

    let mut k = 0;
    let mut frame_count = 0;

    println!("按下'r'键来设置旋转轴");
    let mut inputr = String::new();
    io::stdin().read_line(&mut inputr).unwrap();
    if inputr.trim() == "r" {
        println!("请输入旋转轴(x,y,z) 三个数字用空格隔开");
        let mut input_axis = String::new();
        io::stdin().read_line(&mut input_axis).unwrap();
        //let axis: Vec<f64> = input_axis.split(",").map(|x| x.trim().parse().unwrap()).collect();
        let axis: Vec<f64> = input_axis.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let axis = Vector3::new(axis[0], axis[1], axis[2]);
        // println!("请输入旋转角度");
        // let mut input_angle = String::new();
        // io::stdin().read_line(&mut input_angle).unwrap();
        // let angle: f64 = input_angle.trim().parse().unwrap();
        // //r.arbitrary_rotation = Rasterizer::get_rotation(axis, angle);
        //r.get_rotation(axis, angle);

        while k != 27 {
            r.clear(Buffer::Both);
            r.set_model(get_model_matrix(angle,1.0,true));
            r.set_view(get_view_matrix(eye_pos));
            r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
            r.get_rotation(axis, angle);
            r.draw_triangle(pos_id, ind_id, Primitive::Triangle);
    
            let frame_buffer = r.frame_buffer();
            let image = frame_buffer2cv_mat(frame_buffer);
            imshow("image", &image).unwrap();
    
            k = wait_key(80).unwrap();
            println!("frame count: {}", frame_count);
            if k == 'a' as i32 {
                angle += 10.0;
            } else if k == 'd' as i32 {
                angle -= 10.0;
            } 
            frame_count += 1;
        }

    }



    while k != 27 {
        r.clear(Buffer::Both);
        r.set_model(get_model_matrix(angle,1.0,false));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw_triangle(pos_id, ind_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);
        imshow("image", &image).unwrap();

        k = wait_key(80).unwrap();
        println!("frame count: {}", frame_count);
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        } 
        frame_count += 1;
    }
    Ok(())
}