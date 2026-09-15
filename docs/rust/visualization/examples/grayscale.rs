use image::imageops::fast_blur;
use image::{DynamicImage, Luma, Rgb};
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;

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
    let img = image::open("examples/image/romanesco.jpg").unwrap();
    let gray = to_grayscale(&img);
    gray.save("examples/image/grayscale.png").unwrap();
    println!("灰度图像已保存");

    // image crate 也内置了灰度转换
    let gray2 = img.to_luma8();
    gray2.save("examples/image/grayscale_builtin.png").unwrap();

    // 高斯模糊
    let blurred = gaussian_blur_f32(&gray, 5.0);
    blurred.save("examples/image/romanesco_blurred.png").unwrap();
    println!("高斯模糊图像已保存");

    // 快速模糊
    let fast_blurred = fast_blur(&gray, 5.0);
    fast_blurred
        .save("examples/image/romanesco_fast_blurred.png")
        .unwrap();
    println!("快速模糊图像已保存");

    // 边缘检测
    let edges = canny(&gray, 10.0, 30.0);
    edges.save("examples/image/romanesco_edges.png").unwrap();
    println!("边缘检测图像已保存");


     // 边缘检测
    let img = image::open("examples/image/fern.jpg").unwrap();
    let gray3 = img.to_luma8();
    let edges = canny(&gray3, 10.0, 30.0);
    edges.save("examples/image/fern_edges.png").unwrap();
    println!("边缘检测图像已保存");
}
