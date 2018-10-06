extern crate image;

use image::{GenericImage, GenericImageView, ImageBuffer};

type rgba = image::Rgba<u8>;

fn process_pixel(pixel: &rgba,factor:f64) -> rgba {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);

    let r = ((factor * (r as f64) / 255f64).round() * (255f64 / factor)) as u8;
    let g = ((factor * (g as f64) / 255f64).round() * (255f64 / factor)) as u8;
    let b = ((factor * (b as f64) / 255f64).round() * (255f64 / factor)) as u8;
    
    let out = image::Rgba([r, g, b, 0xFFu8]);
    out
}
fn main() {
    let file_name = "picture.jpg";
    let img = image::open(&file_name).expect("Couldn't open picture.jpg");
    let (width, height) = img.dimensions();
    let mut out = image::RgbaImage::new(width, height);

    let args:Vec<_> = std::env::args().collect();
    
    
    println!("{} is {}x{}", &file_name, &width, &height);
    println!("Starting filtering...");

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let old_pixel = img.get_pixel(x, y);
            let new_pixel = process_pixel(&old_pixel,16f64);
            out.put_pixel(x, y, new_pixel);

            let mut loss_diference: [i16; 3] = [
                (old_pixel[0] as i16 - new_pixel[0] as i16).min(0),
                (old_pixel[1] as i16 - new_pixel[1] as i16).min(0),
                (old_pixel[2] as i16 - new_pixel[2] as i16).min(0),
            ];

            let dx: [i32; 4] = [1, -1, 0, 1];
            let dy: [i32; 4] = [0, 1, 1, 1];
            let quant: [f64; 4] = [7f64, 3f64, 5f64, 1f64];
            for di in 0..3 {
                let (this_x, this_y) = ((x as i32 + dx[di]) as u32, (y as i32 + dy[di]) as u32);
                let next_pixel = img.get_pixel(this_x, this_y);
                let mut modified_pixel: [u8; 4] = [0x00, 0x00, 0x00, 0xFFu8];
                for i in 0..2 {
                    modified_pixel[i] =
                        ((next_pixel[i] as f64) + ((loss_diference[i] as f64) * (quant[i] / 16f64))) as u8;
                }
                out.put_pixel(this_x, this_y, image::Rgba(modified_pixel));
            }
        }
    }

    println!("Saving...");
    out.save("output.png").expect("Couldn't save output.png");
    println!("Saved");
}
