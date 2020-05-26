extern crate image;
use std::env;



fn main() {
    let mut args = env::args();
    args.next();

    let img_path = match args.next() {
        None => {
            eprintln!("Error: Input file path is not specified. ");
            eprintln!("Usage: cargo run /path/to/input/image");
            return;
        }
        Some(s) => s,
    };

    //Load Image
    let img = image::open(&img_path).unwrap();
    let img = img.to_rgb();
    println!("Name: {}", img_path.rsplit("/").next().unwrap());
    println!("Size: {}x{}", img.width(), img.height());

    // 1. Use Iterator
    let res_img = //
        image::GrayImage::from_vec(img.width(), img.height(),
        img.pixels()
        .map(|&p| ((p.data[0] as u32 + p.data[1] as u32 + p.data[2] as u32) / 3) as u8)
        .collect::<_>()).unwrap();

    res_img.save("./result_iter.png").unwrap();

    // 2. Interpret function as Image
    let grayscale_filter = |u, v| {
        let mut val = [0; 1];
        if u < img.width() && v < img.height() {
            let pix = img.get_pixel(u, v); // panics if out of bounds
            val[0] = ((pix.data[0] as u32 + pix.data[1] as u32 + pix.data[2] as u32) / 3) as u8;

        }
        return image::Luma(val);
    };
    let res_img = image::GrayImage::from_fn(img.width(), img.height(), grayscale_filter);
    res_img.save("./result_fn.png").unwrap();

    // 3. As usual
    let mut res_img = image::GrayImage::new(img.width(), img.height());
    for v in 0..img.height() {
        for u in 0..img.width() {
            let pix = img.get_pixel(u, v); 
            let val = [((pix.data[0] as u32 + pix.data[1] as u32 + pix.data[2] as u32) / 3) as u8; 1];
            let gray = image::Luma(val);
            res_img.put_pixel(u, v, gray);
        }
    }
    res_img.save("./result_usual.png").unwrap();
    
    // 4. Simply interpret as GrayImage
    let img = image::open(&img_path).unwrap();
    let res_img = img.to_luma();
    res_img.save("./result_grayimage.png").unwrap();
}
