extern crate image;

use image::{GenericImageView};

type rgba = image::Rgba<u8>;

fn process_pixel(pixel: &rgba, factor: u8) -> rgba {
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    let factor:f64 = factor as f64;

    let r = ((factor * (r as f64) / 255f64).round() * (255f64 / factor)) as u8;
    let g = ((factor * (g as f64) / 255f64).round() * (255f64 / factor)) as u8;
    let b = ((factor * (b as f64) / 255f64).round() * (255f64 / factor)) as u8;

    let out = image::Rgba([r, g, b, 0xFFu8]);
    out
}
fn main() {
    let mut input_file_name = String::from("picture.jpg");
    let mut output_file_name = String::from("output.png");
    let mut factor = 4u8;
    
    let mut args_list: Vec<_> = std::env::args().rev().collect();
    args_list.pop();
    while args_list.len() > 0 {
        let arg: String = args_list.pop().unwrap();

        match arg.as_ref() {
            "--in" => {
                input_file_name = args_list.pop().expect("No more arguments passed");
            }
            "--out" => {
                output_file_name = args_list.pop().expect("No more arguments passed");
            }
            "--factor" => {
                factor = args_list.pop().expect("No more arguments passed").parse::<u8>().expect("Couldn't parse input for --factor. It must be an u8");
            }
            &_ => {
                println!("Meaningless {}", arg);
            }
        }
    }

    let img = image::open(&input_file_name).expect("Couldn't open picture.jpg");
    let (width, height) = img.dimensions();
    let mut out = image::RgbaImage::new(width, height);

    println!("{} is {}x{}", &input_file_name, &width, &height);
    println!("Starting filtering with {} as factor...",factor);

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let old_pixel = img.get_pixel(x, y);
            let new_pixel = process_pixel(&old_pixel, factor);
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
                    modified_pixel[i] = ((next_pixel[i] as f64)
                        + ((loss_diference[i] as f64) * (quant[i] / 16f64)))
                        as u8;
                }
                out.put_pixel(this_x, this_y, image::Rgba(modified_pixel));
            }
        }
    }

    println!("Saving in {}...",output_file_name);
    out.save(output_file_name).expect("Couldn't save output.png");
    println!("Saved");
}
