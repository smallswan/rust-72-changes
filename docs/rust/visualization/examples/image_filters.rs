//! 图像滤镜、色彩调整与统计分析示例
//!
//! 演示内容：
//! - 锐化（sharpen3x3 / sharpen_gaussian）
//! - 中值滤波（median_filter）
//! - 双边滤波（bilateral_filter）
//! - 拉普拉斯滤波（laplacian_filter）
//! - 对比度拉伸（stretch_contrast）
//! - 直方图均衡化（equalize_histogram）
//! - Otsu 二值化
//! - 噪声模拟（高斯噪声 / 椒盐噪声）
//! - 直方图统计

use image::Luma;
use imageproc::contrast::{equalize_histogram, otsu_level, stretch_contrast, threshold, ThresholdType};
use imageproc::filter::{
    bilateral::GaussianEuclideanColorDistance, bilateral_filter, laplacian_filter, median_filter,
    sharpen3x3, sharpen_gaussian,
};
use imageproc::map::map_pixels;
use imageproc::noise::{gaussian_noise, salt_and_pepper_noise};
use imageproc::stats::histogram;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open("examples/image/romanesco.jpg")?;
    let gray = img.to_luma8();
    println!("加载图像: {}x{}", gray.width(), gray.height());

    let out_dir = "examples/image";

    // ===== 1. 锐化 =====
    println!("\n===== 锐化 =====");

    let sharpened_3x3 = sharpen3x3(&gray);
    sharpened_3x3.save(format!("{out_dir}/filter_sharpen3x3.png"))?;
    println!("3x3 锐化已保存");

    let sharpened_gauss = sharpen_gaussian(&gray, 1.0, 0.7);
    sharpened_gauss.save(format!("{out_dir}/filter_sharpen_gaussian.png"))?;
    println!("高斯锐化已保存 (sigma=1.0, strength=0.7)");

    // ===== 2. 中值滤波 =====
    println!("\n===== 中值滤波 =====");

    // 中值滤波常用于去噪（尤其是椒盐噪声）
    let noisy = salt_and_pepper_noise(&gray, 0.05, 42);
    noisy.save(format!("{out_dir}/filter_noisy_salt.png"))?;

    let median_3 = median_filter(&noisy, 1, 1); // 3x3 窗口
    median_3.save(format!("{out_dir}/filter_median3x3.png"))?;
    println!("3x3 中值滤波已保存");

    let median_5 = median_filter(&noisy, 2, 2); // 5x5 窗口
    median_5.save(format!("{out_dir}/filter_median5x5.png"))?;
    println!("5x5 中值滤波已保存");

    // ===== 3. 双边滤波 =====
    println!("\n===== 双边滤波 =====");

    // 双边滤波在去噪的同时保留边缘
    let bilateral = bilateral_filter(
        &gray,
        2,    // 窗口半径 (2 -> 5x5)
        3.0,  // spatial_sigma
        GaussianEuclideanColorDistance::new(10.0), // color_sigma
    );
    bilateral.save(format!("{out_dir}/filter_bilateral.png"))?;
    println!("双边滤波已保存 (window=5, sigma_color=10, sigma_spatial=10)");

    // ===== 4. 拉普拉斯滤波 =====
    println!("\n===== 拉普拉斯滤波 =====");

    // laplacian_filter 返回 i16 类型，需要转换回 u8 才能保存
    let laplacian_i16 = laplacian_filter(&gray);
    let laplacian = map_pixels(&laplacian_i16, |p| {
        // 将 i16 值映射到 u8：取绝对值并 clamp
        let v = (p[0].unsigned_abs() as u32).min(255) as u8;
        Luma([v])
    });
    laplacian.save(format!("{out_dir}/filter_laplacian.png"))?;
    println!("拉普拉斯滤波已保存（边缘/二阶导数检测）");

    // ===== 5. 对比度拉伸 =====
    println!("\n===== 对比度拉伸 =====");

    // 将灰度值 [50, 200] 映射到 [0, 255]
    let stretched = stretch_contrast(&gray, 50, 200, 0, 255);
    stretched.save(format!("{out_dir}/filter_stretch_contrast.png"))?;
    println!("对比度拉伸 [50,200] -> [0,255] 已保存");

    // ===== 6. 直方图均衡化 =====
    println!("\n===== 直方图均衡化 =====");

    let equalized = equalize_histogram(&gray);
    equalized.save(format!("{out_dir}/filter_equalized.png"))?;
    println!("直方图均衡化已保存");

    // ===== 7. Otsu 二值化 =====
    println!("\n===== Otsu 二值化 =====");

    let otsu_thresh = otsu_level(&gray);
    println!("Otsu 最优阈值: {}", otsu_thresh);

    let binary = threshold(&gray, otsu_thresh, ThresholdType::Binary);
    binary.save(format!("{out_dir}/filter_otsu_binary.png"))?;
    println!("Otsu 二值化已保存");

    // 反二值化
    let binary_inv = threshold(&gray, otsu_thresh, ThresholdType::BinaryInverted);
    binary_inv.save(format!("{out_dir}/filter_otsu_inverted.png"))?;
    println!("Otsu 反二值化已保存");

    // ===== 8. 噪声模拟 =====
    println!("\n===== 噪声模拟 =====");

    let gauss_noisy = gaussian_noise(&gray, 0.0, 20.0, 42);
    gauss_noisy.save(format!("{out_dir}/filter_gaussian_noise.png"))?;
    println!("高斯噪声 (mean=0, sigma=20) 已保存");

    let sp_noisy = salt_and_pepper_noise(&gray, 0.02, 42);
    sp_noisy.save(format!("{out_dir}/filter_salt_pepper.png"))?;
    println!("椒盐噪声 (rate=0.02) 已保存");

    // ===== 9. 直方图统计 =====
    println!("\n===== 直方图统计 =====");

    let hist = histogram(&gray);
    // 打印前 16 个灰度级别的像素数量
    println!("灰度直方图（前 16 级）:");
    for (i, channel_hist) in hist.channels.iter().enumerate() {
        if i > 0 {
            break; // 灰度图只有一个通道
        }
        for bin in 0..16 {
            println!("  灰度 {:>3}: {:>5} 像素", bin, channel_hist[bin]);
        }
        // 统计摘要
        let total: u32 = channel_hist.iter().sum();
        let max_bin = channel_hist
            .iter()
            .enumerate()
            .max_by_key(|&(_, v)| v)
            .unwrap();
        println!(
            "  ... 总像素: {}, 众数灰度值: {} ({} 像素)",
            total, max_bin.0, max_bin.1
        );
    }

    println!("\n所有滤镜与统计示例运行完毕！");
    Ok(())
}
