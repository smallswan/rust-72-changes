//! 图像几何变换示例
//!
//! 演示内容：
//! - 旋转（90° / 180° / 270° / 任意角度）
//! - 平移（translate）
//! - 翻转（水平 / 垂直）
//! - 裁剪（crop）
//! - 投影变换（warp / Projection）

use image::imageops::{flip_horizontal, flip_vertical};
use imageproc::geometric_transformations::{
    rotate90, rotate180, rotate270, rotate_about_center, translate, warp, Border, Interpolation,
    Projection,
};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open("examples/image/romanesco.jpg")?;
    let rgb = img.to_rgb8();
    println!("原始图像尺寸: {}x{}", rgb.width(), rgb.height());

    let out_dir = "examples/image";

    // ===== 1. 固定角度旋转 =====
    println!("\n===== 固定角度旋转 =====");

    let r90 = rotate90(&rgb);
    r90.save(format!("{out_dir}/geo_rotate90.png"))?;
    println!("旋转 90°: {}x{}", r90.width(), r90.height());

    let r180 = rotate180(&rgb);
    r180.save(format!("{out_dir}/geo_rotate180.png"))?;
    println!("旋转 180°: {}x{}", r180.width(), r180.height());

    let r270 = rotate270(&rgb);
    r270.save(format!("{out_dir}/geo_rotate270.png"))?;
    println!("旋转 270°: {}x{}", r270.width(), r270.height());

    // ===== 2. 任意角度旋转（绕中心） =====
    println!("\n===== 任意角度旋转 =====");

    // 旋转 30°，超出边界用黑色填充
    let r30 = rotate_about_center(
        &rgb,
        30.0_f32 * PI / 180.0, // 30° 转为弧度
        Interpolation::Bilinear,
        Border::Constant(image::Rgb([0u8, 0, 0])),
    );
    r30.save(format!("{out_dir}/geo_rotate30.png"))?;
    println!("绕中心旋转 30° 已保存");

    // 旋转 45°，不裁剪（自动扩展画布以容纳完整图像）
    use imageproc::geometric_transformations::rotate_about_center_no_crop;
    let r45 = rotate_about_center_no_crop(
        &rgb,
        45.0_f32 * PI / 180.0,
        Interpolation::Bilinear,
        Border::Constant(image::Rgb([128u8, 128, 128])),
    );
    r45.save(format!("{out_dir}/geo_rotate45_nocrop.png"))?;
    println!("旋转 45° (不裁剪): {}x{}", r45.width(), r45.height());

    // ===== 3. 平移 =====
    println!("\n===== 平移 =====");

    // 向右平移 100px，向下平移 50px
    let translated = translate(&rgb, (100, 50), Border::Constant(image::Rgb([0u8, 0, 0])));
    translated.save(format!("{out_dir}/geo_translate.png"))?;
    println!("平移 (100, 50) 已保存");

    // ===== 4. 翻转 =====
    println!("\n===== 翻转 =====");

    let flipped_h = flip_horizontal(&rgb);
    flipped_h.save(format!("{out_dir}/geo_flip_h.png"))?;
    println!("水平翻转已保存");

    let flipped_v = flip_vertical(&rgb);
    flipped_v.save(format!("{out_dir}/geo_flip_v.png"))?;
    println!("垂直翻转已保存");

    // ===== 5. 裁剪 =====
    println!("\n===== 裁剪 =====");

    // 从中心裁剪一个正方形区域
    let (w, h) = (rgb.width(), rgb.height());
    let crop_size = w.min(h) / 2;
    let x = (w - crop_size) / 2;
    let y = (h - crop_size) / 2;
    let cropped = image::imageops::crop_imm(&rgb, x, y, crop_size, crop_size).to_image();
    cropped.save(format!("{out_dir}/geo_crop.png"))?;
    println!("中心裁剪 ({crop_size}x{crop_size}) 已保存");

    // ===== 6. 投影变换（透视变换） =====
    println!("\n===== 投影变换 =====");

    // 构造一个简单的透视投影
    let (w, h) = (rgb.width() as f32, rgb.height() as f32);
    let projection = Projection::from_control_points(
        // 源图像四角
        [(0.0, 0.0), (w - 1.0, 0.0), (w - 1.0, h - 1.0), (0.0, h - 1.0)],
        // 目标位置（向内收缩制造透视效果）
        [
            (w * 0.1, h * 0.1),
            (w * 0.8, h * 0.05),
            (w * 0.9, h * 0.85),
            (w * 0.05, h * 0.9),
        ],
    )
    .expect("无法构造投影变换");

    let warped = warp(
        &rgb,
        projection,
        Interpolation::Bilinear,
        Border::Constant(image::Rgb([0u8, 0, 0])),
    );
    warped.save(format!("{out_dir}/geo_warp.png"))?;
    println!("透视变换已保存");

    println!("\n所有几何变换示例运行完毕！");
    Ok(())
}
