use crate::vec3::Color;

pub fn write_color(pixel_color : Color){
    // Write the translated [0,255] value of each color component
    let x_col : i32 = (255.999 * pixel_color.x()) as i32;
    let y_col : i32 = (255.999 * pixel_color.y()) as i32;
    let z_col : i32 = (255.999 * pixel_color.z()) as i32;

    println!("{} {} {}", x_col, y_col, z_col);
}