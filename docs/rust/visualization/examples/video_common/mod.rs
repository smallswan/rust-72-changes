//! 视频转码公共辅助模块
//!
//! 提供一个通用的 `transcode` 函数：
//!   解封装 -> 解码 -> 滤镜图(filter graph) -> libx264 编码 -> 封装
//!
//! 各示例只需传入不同的滤镜描述字符串(filter_spec)即可实现
//! 裁剪、拼接、旋转/翻转、水印/字幕等操作。

use ffmpeg_next as ffmpeg;

use ffmpeg::{codec, encoder, filter, format, frame, media, picture, Dictionary, Packet};
use std::path::Path;

/// 对输入视频应用滤镜并重新编码输出
///
/// * `input_path`  - 输入视频路径
/// * `output_path` - 输出视频路径
/// * `filter_spec` - FFmpeg 滤镜描述，如 "crop=320:240:0:0"
/// * `out_dims`    - 根据输入宽高计算输出宽高的闭包（滤镜可能改变分辨率）
pub fn transcode(
    input_path: &Path,
    output_path: &Path,
    filter_spec: &str,
    out_dims: impl Fn(u32, u32) -> (u32, u32),
) -> Result<(), ffmpeg::Error> {
    ffmpeg::init()?;

    let mut ictx = format::input(input_path)?;
    let mut octx = format::output(output_path)?;

    // ---------- 输入流与解码器 ----------
    let ist = ictx
        .streams()
        .best(media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let ist_index = ist.index();
    let ist_time_base = ist.time_base();

    let mut decoder = codec::context::Context::from_parameters(ist.parameters())?
        .decoder()
        .video()?;

    let in_w = decoder.width();
    let in_h = decoder.height();
    // x264 要求宽高为偶数
    let (ow, oh) = out_dims(in_w, in_h);
    let out_w = ow & !1;
    let out_h = oh & !1;

    println!(
        "输入: {}x{} {:?}, 滤镜: {}",
        in_w,
        in_h,
        decoder.format(),
        filter_spec
    );

    // ---------- 滤镜图 ----------
    let mut graph = filter::Graph::new();
    let pix_fmt_name = decoder
        .format()
        .descriptor()
        .map(|d| d.name())
        .unwrap_or("yuv420p");
    let buffer_args = format!(
        "video_size={}x{}:pix_fmt={}:time_base={}/{}:pixel_aspect=1/1",
        in_w,
        in_h,
        pix_fmt_name,
        ist_time_base.numerator(),
        ist_time_base.denominator(),
    );
    graph.add(&filter::find("buffer").unwrap(), "in", &buffer_args)?;
    graph.add(&filter::find("buffersink").unwrap(), "out", "")?;
    graph.output("in", 0)?.input("out", 0)?.parse(filter_spec)?;
    graph.validate()?;

    // 滤镜（如 concat/reverse）可能改变输出端 time base，必须从 buffersink 查询
    let sink_time_base: ffmpeg::Rational = unsafe {
        ffmpeg::ffi::av_buffersink_get_time_base(graph.get("out").unwrap().as_ptr()).into()
    };

    // ---------- 编码器 (libx264) ----------
    let global_header = octx
        .format()
        .flags()
        .contains(format::Flags::GLOBAL_HEADER);
    let codec = encoder::find(codec::Id::H264).ok_or(ffmpeg::Error::EncoderNotFound)?;
    let ost_index = {
        let ost = octx.add_stream(codec)?;
        ost.index()
    };

    let mut enc = codec::context::Context::new_with_codec(codec)
        .encoder()
        .video()?;
    enc.set_width(out_w);
    enc.set_height(out_h);
    enc.set_format(format::Pixel::YUV420P);
    enc.set_time_base(sink_time_base);
    if let Some(fr) = decoder.frame_rate() {
        enc.set_frame_rate(Some(fr));
    }
    if global_header {
        enc.set_flags(codec::Flags::GLOBAL_HEADER);
    }

    let mut opts = Dictionary::new();
    opts.set("preset", "medium");
    opts.set("crf", "23");
    let mut video_encoder = enc.open_with(opts)?;

    octx.stream_mut(ost_index)
        .unwrap()
        .set_parameters(&video_encoder);

    octx.write_header()?;
    let ost_time_base = octx.stream(ost_index).unwrap().time_base();

    // ---------- 主循环 ----------
    // 返回 true 表示滤镜图已关闭输入（如 trim 截取完毕），可以提前结束
    let process_decoded = |decoder: &mut codec::decoder::Video,
                               graph: &mut filter::Graph,
                               video_encoder: &mut encoder::Video,
                               octx: &mut format::context::Output|
     -> Result<bool, ffmpeg::Error> {
        let mut decoded = frame::Video::empty();
        while decoder.receive_frame(&mut decoded).is_ok() {
            let ts = decoded.timestamp();
            decoded.set_pts(ts);
            // trim 等滤镜截取完成后 buffersrc 会返回 EOF，视为正常结束
            if graph.get("in").unwrap().source().add(&decoded).is_err() {
                return Ok(true);
            }
            pull_and_encode(graph, video_encoder, octx, ost_index, sink_time_base, ost_time_base)?;
        }
        Ok(false)
    };

    'demux: for (stream, packet) in ictx.packets() {
        if stream.index() != ist_index {
            continue;
        }
        decoder.send_packet(&packet)?;
        if process_decoded(&mut decoder, &mut graph, &mut video_encoder, &mut octx)? {
            break 'demux;
        }
    }

    // ---------- 冲刷解码器 / 滤镜 / 编码器 ----------
    decoder.send_eof()?;
    process_decoded(&mut decoder, &mut graph, &mut video_encoder, &mut octx)?;

    // 滤镜图可能已经因 trim 提前关闭，flush 报 EOF 属正常
    let _ = graph.get("in").unwrap().source().flush();
    pull_and_encode(
        &mut graph,
        &mut video_encoder,
        &mut octx,
        ost_index,
        sink_time_base,
        ost_time_base,
    )?;

    video_encoder.send_eof()?;
    write_encoded(
        &mut video_encoder,
        &mut octx,
        ost_index,
        sink_time_base,
        ost_time_base,
    )?;

    octx.write_trailer()?;
    println!("已输出: {} ({}x{})", output_path.display(), out_w, out_h);
    Ok(())
}

/// 从滤镜图中拉取处理后的帧并送入编码器
fn pull_and_encode(
    graph: &mut filter::Graph,
    video_encoder: &mut encoder::Video,
    octx: &mut format::context::Output,
    ost_index: usize,
    src_tb: ffmpeg::Rational,
    dst_tb: ffmpeg::Rational,
) -> Result<(), ffmpeg::Error> {
    let mut filtered = frame::Video::empty();
    while graph.get("out").unwrap().sink().frame(&mut filtered).is_ok() {
        filtered.set_kind(picture::Type::None);
        video_encoder.send_frame(&filtered)?;
        write_encoded(video_encoder, octx, ost_index, src_tb, dst_tb)?;
    }
    Ok(())
}

/// 从编码器取出压缩包并写入输出文件
fn write_encoded(
    video_encoder: &mut encoder::Video,
    octx: &mut format::context::Output,
    ost_index: usize,
    src_tb: ffmpeg::Rational,
    dst_tb: ffmpeg::Rational,
) -> Result<(), ffmpeg::Error> {
    let mut encoded = Packet::empty();
    while video_encoder.receive_packet(&mut encoded).is_ok() {
        encoded.set_stream(ost_index);
        encoded.rescale_ts(src_tb, dst_tb);
        encoded.write_interleaved(octx)?;
    }
    Ok(())
}
