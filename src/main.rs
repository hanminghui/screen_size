use std::{io, println};
use libm;

// ipad pro 10.5 2224*1668 10.5 264ppi
// MacbookPro 14 2021/2023 3024*1964 14.2 254ppi
// MacbookPro 16 2021/2023 3456*2234 16.2 254ppi
// MacbookAir 13 m2 2022 2560*1664 13.6 224.5ppi
// MacbookAir 15 m2 2022 2880*1864 15.2 225.7ppi
// ThinkPad x390 1920*1080 13.3 166ppi
// ThinkPad e40 1366*768 14.0 112ppi
// Hornor magicbook 14 2160*1440 14.0 185ppi

fn main() {
    println!("================================================================================");
    println!("                                  Screen Size");
    println!("--------------------------------------------------------------------------------");
    println!("Please input your screen resolution: (for example: 1920*1080)");
    
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

    println!("Now, please input your screen size in inches: (for example: 27)");

    let w: f32 = width as f32;
    let h: f32 = height as f32;

    let mut size = String::new();
    io::stdin().read_line(&mut size)
        .expect("Didn't Receive Input");
    let r:f32 = size.trim().parse()
        .expect("size wasn't assigned a number");
    println!("--------------------------------------------------------------------------------");
    println!("Your device is {} inches of resolution {} * {} px (of {:.2}ppi)", 
        r, width, height, get_ppi(w, h, r));
    //println!("height/width: {}", w/h);
    let result_w = get_width(w, h, r);
    let result_h: f32 = result_w / w * h;
    println!("and size is {:.2} * {:.2} cm, area is {:.2} cm²", 
        inch_to_cm(result_w), inch_to_cm(result_h), 
        get_area(inch_to_cm(result_w), inch_to_cm(result_h)));
    println!("================================================================================");
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

fn get_area(w: f32, h: f32) -> f32{
    return w * h
}

// fn aspect_ratio(width: u32, height: u32){

// }
