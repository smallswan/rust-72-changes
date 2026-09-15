use exif::{In, Reader, Tag};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "examples/image/IMG_20190119_120119.jpg";
    let file = File::open(path)?;
    let exif = Reader::new().read_from_container(&mut BufReader::new(&file))?;

    // 定义要展示的常用 EXIF 标签及其可读名称
    let known_tags: Vec<(Tag, &str)> = vec![
        (Tag::Make, "相机制造商"),
        (Tag::Model, "相机型号"),
        (Tag::Orientation, "方向"),
        (Tag::XResolution, "X 分辨率"),
        (Tag::YResolution, "Y 分辨率"),
        (Tag::ResolutionUnit, "分辨率单位"),
        (Tag::Software, "软件"),
        (Tag::DateTime, "日期时间"),
        (Tag::DateTimeOriginal, "拍摄时间"),
        (Tag::DateTimeDigitized, "数字化时间"),
        (Tag::ExposureTime, "曝光时间"),
        (Tag::FNumber, "光圈值 (F)"),
        (Tag::ISOSpeed, "ISO 感光度"),
        (Tag::PhotographicSensitivity, "摄影感光度"),
        (Tag::FocalLength, "焦距"),
        (Tag::Flash, "闪光灯"),
        (Tag::WhiteBalance, "白平衡"),
        (Tag::ExposureProgram, "曝光程序"),
        (Tag::MeteringMode, "测光模式"),
        (Tag::LensModel, "镜头型号"),
        (Tag::LensMake, "镜头制造商"),
        (Tag::ShutterSpeedValue, "快门速度值"),
        (Tag::ApertureValue, "光圈值"),
        (Tag::BrightnessValue, "亮度值"),
        (Tag::MakerNote, "厂商注释"),
        (Tag::ExifVersion, "Exif 版本"),
        (Tag::ComponentsConfiguration, "组件配置"),
        (Tag::FlashpixVersion, "Flashpix 版本"),
        (Tag::ColorSpace, "色彩空间"),
        (Tag::PixelXDimension, "像素宽度"),
        (Tag::PixelYDimension, "像素高度"),
        (Tag::SensingMethod, "感光方式"),
        (Tag::FileSource, "文件来源"),
        (Tag::SceneType, "场景类型"),
        (Tag::SubSecTime, "亚秒时间"),
        (Tag::SubSecTimeOriginal, "亚秒拍摄时间"),
        (Tag::SubSecTimeDigitized, "亚秒数字化时间"),
        (Tag::ExposureMode, "曝光模式"),
        (Tag::FocalLengthIn35mmFilm, "35mm 等效焦距"),
        (Tag::SceneCaptureType, "场景拍摄类型"),
        (Tag::YCbCrPositioning, "YCbCr 定位"),
        (Tag::Compression, "压缩方式"),
        (Tag::JPEGInterchangeFormat, "JPEG 偏移量"),
        (Tag::JPEGInterchangeFormatLength, "JPEG 数据长度"),
        (Tag::InteroperabilityIndex, "互操作索引"),
        (Tag::InteroperabilityVersion, "互操作版本"),
        (Tag::GPSLatitude, "GPS 纬度"),
        (Tag::GPSLongitude, "GPS 经度"),
        (Tag::GPSLatitudeRef, "GPS 纬度参考"),
        (Tag::GPSLongitudeRef, "GPS 经度参考"),
        (Tag::GPSAltitude, "GPS 海拔"),
        (Tag::GPSAltitudeRef, "GPS 海拔参考"),
        (Tag::GPSTimeStamp, "GPS 时间戳"),
        (Tag::GPSDateStamp, "GPS 日期"),
        (Tag::ImageWidth, "图像宽度"),
        (Tag::ImageLength, "图像高度"),
    ];

    println!("=== EXIF 信息: {} ===\n", path);

    // 分别输出 Primary IFD 和 Thumbnail IFD 的字段
    for ifd in &[In::PRIMARY, In::THUMBNAIL] {
        let ifd_name = if *ifd == In::PRIMARY {
            "Primary Image"
        } else {
            "Thumbnail"
        };

        let fields: Vec<_> = exif.fields().filter(|f| f.ifd_num == *ifd).collect();
        if fields.is_empty() {
            continue;
        }

        println!("--- {} ---", ifd_name);
        for field in &fields {
            let display_name = known_tags
                .iter()
                .find(|(t, _)| *t == field.tag)
                .map(|(_, name)| *name)
                .unwrap_or_else(|| {
                    // 使用 leak 来获取 &'static str，仅在示例中使用
                    Box::leak(format!("{:?}", field.tag).into_boxed_str()) as &str
                });

            let value = field.display_value().with_unit(&exif).to_string();
            println!("  {:<24} : {}", display_name, value);
        }
        println!();
    }

    let total = exif.fields().count();
    println!("共读取到 {} 个 EXIF 字段", total);

    // 去除EXIF信息，并保存到新文件
    let output_path = "examples/image/IMG_20190119_120119_no_exif.jpg";
    let img = image::open(path)?;
    img.save(output_path)?;
    println!("\n已去除 EXIF 信息并保存到: {}", output_path);

    // 验证新文件是否还有 EXIF 信息
    let verify_file = File::open(output_path)?;
    match Reader::new().read_from_container(&mut BufReader::new(&verify_file)) {
        Ok(new_exif) => println!("警告: 新文件仍包含 {} 个 EXIF 字段", new_exif.fields().count()),
        Err(_) => println!("验证通过: 新文件不包含 EXIF 信息"),
    }

    Ok(())
}
