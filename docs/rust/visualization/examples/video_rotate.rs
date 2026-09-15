//! 视频旋转/翻转示例（使用 ffmpeg-next）
//!
//! 1. transpose 旋转 90°/180°
//! 2. hflip 水平翻转
//! 3. vflip 垂直翻转
//!
//! 运行: cargo run --example video_rotate

#[path = "video_common/mod.rs"]
mod video_common;

use std::path::Path;

fn main() -> Result<(), ffmpeg_next::Error> {
    let input = Path::new("examples/video/Rotation3D.mp4");

    // ===== 1. 顺时针旋转 90° =====
    // transpose: 0=逆时针90°+垂直翻转 1=顺时针90° 2=逆时针90° 3=顺时针90°+垂直翻转
    println!("===== 顺时针旋转 90° (transpose=1) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_rotate90.mp4"),
        "transpose=1,format=yuv420p",
        |w, h| (h, w), // 旋转 90° 后宽高互换
    )?;

    // ===== 2. 旋转 180°（两次 transpose） =====
    println!("\n===== 旋转 180° =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_rotate180.mp4"),
        "transpose=1,transpose=1,format=yuv420p",
        |w, h| (w, h),
    )?;

    // ===== 3. 水平翻转（镜像） =====
    println!("\n===== 水平翻转 (hflip) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_hflip.mp4"),
        "hflip,format=yuv420p",
        |w, h| (w, h),
    )?;

    // ===== 4. 垂直翻转（上下颠倒） =====
    println!("\n===== 垂直翻转 (vflip) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_vflip.mp4"),
        "vflip,format=yuv420p",
        |w, h| (w, h),
    )?;

    // ===== 5. 任意角度旋转（rotate 滤镜，45°） =====
    println!("\n===== 任意角度旋转 45° (rotate) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_rotate45.mp4"),
        // rotate 接受弧度，PI/4 = 45°，fillcolor 指定空白区颜色
        "rotate=PI/4:fillcolor=black,format=yuv420p",
        |w, h| (w, h),
    )?;

    println!("\n所有旋转/翻转示例运行完毕！");
    Ok(())
}
