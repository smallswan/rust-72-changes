//! 视频水印/字幕示例（使用 ffmpeg-next）
//!
//! 1. movie + overlay：叠加图片水印（使用项目里的 rust-logo.png）
//! 2. drawtext：绘制文字字幕
//! 3. drawbox + drawtext：底部半透明字幕条
//!
//! 运行: cargo run --example video_watermark
//! 注意: 需在项目根目录运行（滤镜里使用了相对路径的水印图片）

#[path = "video_common/mod.rs"]
mod video_common;

use std::path::Path;

fn main() -> Result<(), ffmpeg_next::Error> {
    let input = Path::new("examples/video/Rotation3D.mp4");

    // ===== 1. 图片水印：右下角叠加 Rust logo =====
    println!("===== 图片水印 (movie + overlay) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_watermark.mp4"),
        // movie 滤镜加载水印图片，scale 缩小后 overlay 到右下角（留 10px 边距）
        "movie=examples/image/rust-logo.png,scale=96:-1[wm];\
         [in][wm]overlay=x=W-w-10:y=H-h-10,format=yuv420p",
        |w, h| (w, h),
    )?;

    // ===== 2. 文字水印：左上角半透明文字 =====
    println!("\n===== 文字水印 (drawtext) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_text_watermark.mp4"),
        // fontfile 使用 Windows 自带字体，冒号需要转义
        "drawtext=fontfile='C\\:/Windows/Fonts/arial.ttf':\
         text='ffmpeg-next demo':fontsize=28:fontcolor=white@0.6:x=10:y=10,\
         format=yuv420p",
        |w, h| (w, h),
    )?;

    // ===== 3. 底部字幕条：半透明黑底 + 居中文字 =====
    println!("\n===== 底部字幕 (drawbox + drawtext) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_subtitle.mp4"),
        // 先画半透明黑色矩形当字幕背景，再叠加居中字幕文字
        "drawbox=x=0:y=ih-50:w=iw:h=50:color=black@0.5:t=fill,\
         drawtext=fontfile='C\\:/Windows/Fonts/arial.ttf':\
         text='This is a subtitle line':fontsize=24:fontcolor=white:\
         x=(w-text_w)/2:y=h-38,\
         format=yuv420p",
        |w, h| (w, h),
    )?;

    println!("\n所有水印/字幕示例运行完毕！");
    Ok(())
}
