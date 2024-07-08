use image::RgbImage;
use crate::interval::Interval;
use crate::vec3::Vec3;

/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    // let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    // *pixel = image::Rgb(pixel_color);

    // Write the translated [0,255] value of each color component.

    let intensity = Interval::new(0.0, 0.999);
    let r = (256.0 * intensity.clamp(pixel_color.x)).floor() as u8;
    let g = (256.0 * intensity.clamp(pixel_color.y)).floor() as u8;
    let b = (256.0 * intensity.clamp(pixel_color.z)).floor() as u8;
    // let r = (256.0 * intensity.clamp(pixel_color[0] as f64)).floor() as u8;
    // let g = (256.0 * intensity.clamp(pixel_color[1] as f64)).floor() as u8;
    // let b = (256.0 * intensity.clamp(pixel_color[2] as f64)).floor() as u8;

    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb([r, g, b]);

}
