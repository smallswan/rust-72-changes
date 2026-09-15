use base64::{Engine as _, engine::general_purpose};
use data_encoding;
use std::fs::File;
use std::io::Read;

/// 读取一个文件并将其内容编码为标准 Base64 字符串
fn encode_file_to_base64(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 1. 打开文件
    let mut file = File::open(file_path)?;
    // 2. 将文件内容读取到字节向量中
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    // 3. 使用标准 Base64 编码
    let encoded = general_purpose::STANDARD.encode(&buffer);
    Ok(encoded)
}

#[test]
fn test_enc() {
    let data = b"hello123456";
    println!("test_enc:{}", data_encoding::BASE64.encode(data));

    let encoded = encode_file_to_base64("/tmp/image/romanesco.jpg").unwrap();
    println!("base64 len:{}", encoded.len());
}

use image::{ImageBuffer, Rgb, open};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

fn mosaic_region(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    block_size: u32,
) {
    // 确保区域不超出图片边界
    let (img_width, img_height) = img.dimensions();
    let x = x.min(img_width);
    let y = y.min(img_height);
    let width = width.min(img_width - x);
    let height = height.min(img_height - y);

    // 只遍历指定区域
    for row in (y..y + height).step_by(block_size as usize) {
        for col in (x..x + width).step_by(block_size as usize) {
            // 计算当前网格的实际大小（边界处理）
            let block_w = (col + block_size).min(x + width) - col;
            let block_h = (row + block_size).min(y + height) - row;

            // 计算网格内平均颜色
            let mut sum_r = 0u64;
            let mut sum_g = 0u64;
            let mut sum_b = 0u64;
            let total = (block_w * block_h) as u64;

            for dy in 0..block_h {
                for dx in 0..block_w {
                    let pixel = img.get_pixel(col + dx, row + dy);
                    sum_r += pixel[0] as u64;
                    sum_g += pixel[1] as u64;
                    sum_b += pixel[2] as u64;
                }
            }

            let avg_color = Rgb([
                (sum_r / total) as u8,
                (sum_g / total) as u8,
                (sum_b / total) as u8,
            ]);

            // 用平均色块覆盖该网格
            let rect = Rect::at(col as i32, row as i32).of_size(block_w, block_h);
            draw_filled_rect_mut(img, rect, avg_color);
        }
    }
}

#[test]
fn test_mosaic() -> Result<(), Box<dyn std::error::Error>> {
    let mut img = open("/tmp/image/romanesco.jpg")?.to_rgb8();

    // 对 (100, 100) 到 (300, 300) 区域打马赛克，块大小 15
    mosaic_region(&mut img, 150, 200, 200, 200, 15);

    img.save("/tmp/image/romanesco_mosaic.jpg")?;
    Ok(())
}
