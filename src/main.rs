use std::{io, println};
use libm;

// ipad pro 10.5 2224*1668 10.5 264ppi
// MacbookPro 14 2021/2023 3024*1964 14.2 254ppi
// MacbookPro 16 2021/2023 3456*2234 16.2 254ppi
// MacbookAir m2 2022 2560*1664 13.6 224ppi

fn main() {
    println!("==================================================");
    println!("Screen Size");
    println!("--------------------------------------------------");
    println!("Please input your screen resolution:");
    println!("for example: 1920*1080");
    
    let mut resolution = String::new();
    io::stdin().read_line(&mut resolution)
        .expect("Didn't Receive Input");
    let pixels: Vec<_> = resolution
        .split('*')
        .map(|field| field.trim())
        .collect();

    let pixel_a = pixels[0];
    let pixel_b = pixels[1];
    let mut width: u32 = pixel_a.trim().parse()
        .expect("width wasn't assigned a number");
    let mut height: u32 = pixel_b.trim().parse()
        .expect("height wasn't assigned a number");

    if width < height {
        std::mem::swap(&mut width, &mut height);
    }

    println!("Now, please input your screen size in inches:");
    println!("for example: 27");

    let w: f32 = width as f32;
    let h: f32 = height as f32;

    let mut size = String::new();
    io::stdin().read_line(&mut size)
        .expect("Didn't Receive Input");
    let r:f32 = size.trim().parse()
        .expect("size wasn't assigned a number");
    println!("--------------------------------------------------");
    let result_w = get_width(w, h, r);
    println!("Your device infos:",);
    println!("width: {}; height: {}", width, height);
    println!("height/width: {}", w/h);
    println!("Your screen size is {}", r);
    println!("width is {} inches, which is {}cm", 
        result_w, inch_to_cm(result_w));
    let result_h: f32 = result_w / w * h;
    println!("height is {} inches, which is {}cm", 
        result_h, inch_to_cm(result_h));
    println!("and at a pixel density of {}ppi.", get_ppi(w, h, r));
}

fn get_width(w: f32, h: f32, r: f32) -> f32{ 
    return libm::sqrtf(libm::powf(r, 2.0) / (1.0 + libm::powf(h / w, 2.0)))
}

fn inch_to_cm(inch: f32) -> f32{
    return 2.54 * inch
}

fn get_ppi(w: f32, h: f32, r: f32) -> f32{
    //number of pixels in the diagonal of the device
    let d: f32 = libm::sqrtf(libm::powf(w, 2.0) + libm::powf(h, 2.0));
    //PPI = divide the number of pixels by the physical (diagonal) size of the screen in inches
    return d / r
}

// fn aspect_ratio(width: u32, height: u32){

// }
