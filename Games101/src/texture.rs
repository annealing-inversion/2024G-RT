#![allow(warnings)]
use nalgebra::{Vector3};

use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct Texture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new(name: &str) -> Self {
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    }

    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        // 在此实现双线性插值函数, 并替换掉get_color
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let u_img_floor = u_img.floor() as i32;
        let v_img_floor = v_img.floor() as i32;
        let u_img_ceil = u_img.ceil() as i32;
        let v_img_ceil = v_img.ceil() as i32;

        let u_ratio = u_img - u_img_floor as f64;
        let v_ratio = v_img - v_img_floor as f64;

        let color1: &VecN<u8, 3> = self.img_data.at_2d(v_img_floor, u_img_floor).unwrap();
        let color2: &VecN<u8, 3> = self.img_data.at_2d(v_img_floor, u_img_ceil).unwrap();
        let color3: &VecN<u8, 3> = self.img_data.at_2d(v_img_ceil, u_img_floor).unwrap();
        let color4: &VecN<u8, 3> = self.img_data.at_2d(v_img_ceil, u_img_ceil).unwrap();
        
        let clr1 = Vector3::new(color1[2] as f64, color1[1] as f64, color1[0] as f64) * (1.0 - u_ratio) + Vector3::new(color2[2] as f64, color2[1] as f64, color2[0] as f64) * u_ratio;
        let clr2 = Vector3::new(color3[2] as f64, color3[1] as f64, color3[0] as f64) * (1.0 - u_ratio) + Vector3::new(color4[2] as f64, color4[1] as f64, color4[0] as f64) * u_ratio;

        clr1 * (1.0 - v_ratio) + clr2 * v_ratio
    }
}