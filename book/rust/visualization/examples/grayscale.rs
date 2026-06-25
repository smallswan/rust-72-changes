use image::{DynamicImage, Luma, Rgb};

fn to_grayscale(img: &DynamicImage) -> image::GrayImage {
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let mut gray = image::GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let Rgb([r, g, b]) = rgb.get_pixel(x, y);
            // ITU-R BT.601 加权灰度
            let gray_val = 0.299 * *r as f32 + 0.587 * *g as f32 + 0.114 * *b as f32;
            gray.put_pixel(x, y, Luma([gray_val as u8]));
        }
    }
    gray
}

fn main() {
    let img = image::open("/tmp/image/romanesco.jpg").unwrap();
    let gray = to_grayscale(&img);
    gray.save("/tmp/image/grayscale.png").unwrap();
    println!("灰度图像已保存");

    // image crate 也内置了灰度转换
    let gray2 = img.to_luma8();
    gray2.save("/tmp/image/grayscale_builtin.png").unwrap();
}
