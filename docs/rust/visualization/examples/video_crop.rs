//! 视频裁剪示例（使用 ffmpeg-next）
//!
//! 1. 画面裁剪 (crop)：截取画面中的某个区域
//! 2. 时间裁剪 (trim)：截取视频中的某个时间段
//!
//! 运行: cargo run --example video_crop

#[path = "video_common/mod.rs"]
mod video_common;

use std::path::Path;

fn main() -> Result<(), ffmpeg_next::Error> {
    let input = Path::new("examples/video/Rotation3D.mp4");

    // ===== 1. 画面裁剪：取画面中央 1/2 宽、1/2 高的区域 =====
    println!("===== 画面裁剪 (crop) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_crop_center.mp4"),
        // crop=输出宽:输出高:x偏移:y偏移，iw/ih 为输入宽高
        "crop=iw/2:ih/2:iw/4:ih/4,format=yuv420p",
        |w, h| (w / 2, h / 2),
    )?;

    // ===== 2. 画面裁剪：取左上角 1/3 区域 =====
    println!("\n===== 画面裁剪 (左上角) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_crop_topleft.mp4"),
        "crop=iw/3:ih/3:0:0,format=yuv420p",
        |w, h| (w / 3, h / 3),
    )?;

    // ===== 3. 时间裁剪：截取第 1~3 秒 =====
    println!("\n===== 时间裁剪 (trim 1s~3s) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_trim_1_3.mp4"),
        // trim 截取时间段，setpts 让时间戳从 0 重新开始
        "trim=start=1:end=3,setpts=PTS-STARTPTS,format=yuv420p",
        |w, h| (w, h),
    )?;

    println!("\n所有裁剪示例运行完毕！");
    Ok(())
}
