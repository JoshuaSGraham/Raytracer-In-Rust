
fn main() {
    //image 
    let image_width = 256;
    let image_height = 256;

    //render
    print!("P3\n{} {} \n255\n", image_width, image_height);

    for height_index in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", height_index);
        for width_index in 0..image_width {
            
            let red = width_index as f64 / (image_width -1) as f64;
            let green = height_index as f64 / (image_height -1) as f64;
            let blue = 0.25; 

            let ir : i32 = (255.999 * red) as i32;
            let ig : i32 = (255.999 * green) as i32;
            let ib : i32 = (255.999 * blue) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprintln!("\nDone");
}