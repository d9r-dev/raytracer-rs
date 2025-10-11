use crate::Color;

pub fn write_color(pixel_color: Color) {
    let ir = (pixel_color.x * 255.999) as u32;
    let ig = (pixel_color.y * 255.999) as u32;
    let ib = (pixel_color.z * 255.999) as u32;

    println!("{} {} {} ", ir, ig, ib);
}