//! 视频拼接示例（使用 ffmpeg-next）
//!
//! 利用 split + trim + concat 滤镜，把同一视频的多个片段拼接成新视频：
//!   片段1: 第 0~2 秒
//!   片段2: 第 3~5 秒（拼接在片段 1 后面）
//!
//! 运行: cargo run --example video_concat

#[path = "video_common/mod.rs"]
mod video_common;

use std::path::Path;

fn main() -> Result<(), ffmpeg_next::Error> {
    let input = Path::new("examples/video/Rotation3D.mp4");

    // ===== 1. 两个时间段拼接 =====
    println!("===== 拼接 [0s~2s] + [3s~5s] =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_concat_segments.mp4"),
        // split 复制成两路 -> 各自 trim 出时间段 -> concat 首尾相接
        "split[a][b];\
         [a]trim=start=0:end=2,setpts=PTS-STARTPTS[v1];\
         [b]trim=start=3:end=5,setpts=PTS-STARTPTS[v2];\
         [v1][v2]concat=n=2:v=1:a=0,format=yuv420p",
        |w, h| (w, h),
    )?;

    // ===== 2. 正放 + 倒放拼接（乒乓效果） =====
    println!("\n===== 拼接 正放[0s~2s] + 倒放[0s~2s] =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_pingpong.mp4"),
        "trim=start=0:end=2,setpts=PTS-STARTPTS,split[a][b];\
         [b]reverse[r];\
         [a][r]concat=n=2:v=1:a=0,format=yuv420p",
        |w, h| (w, h),
    )?;

    // ===== 3. 原始画面与镜像画面左右并排 (hstack) =====
    println!("\n===== 左右并排拼接 原始|镜像 (hstack) =====");
    video_common::transcode(
        input,
        Path::new("examples/video/Rotation3D_side_by_side.mp4"),
        "split[a][b];\
         [b]hflip[m];\
         [a][m]hstack=inputs=2,format=yuv420p",
        |w, h| (w * 2, h),
    )?;

    println!("\n所有拼接示例运行完毕！");
    Ok(())
}
