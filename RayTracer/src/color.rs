use image::RgbImage;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub fn linear_to_gamma_corrected(linear_component: f64) -> f64 {
    if linear_component < 0.0 {
        0.0
    } else {
        linear_component.sqrt()
    }
}

/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    // Write the translated [0,255] value of each color component.
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;
    
    r = linear_to_gamma_corrected(r);   
    g = linear_to_gamma_corrected(g);
    b = linear_to_gamma_corrected(b);
    
    // println!("r: {}, g: {}, b: {}", r, g, b);

    let intensity = Interval::new(0.0, 0.999);
    let r = (256.0 * intensity.clamp(r)).floor() as u8;
    let g = (256.0 * intensity.clamp(g)).floor() as u8;
    let b = (256.0 * intensity.clamp(b)).floor() as u8;
    // let r = (256.0 * intensity.clamp(pixel_color.x)).floor() as u8;
    // let g = (256.0 * intensity.clamp(pixel_color.y)).floor() as u8;
    // let b = (256.0 * intensity.clamp(pixel_color.z)).floor() as u8;


    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb([r, g, b]);
}
