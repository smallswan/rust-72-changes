//! 图像绘图与形态学操作示例
//!
//! 演示内容：
//! - 绘制基本形状（矩形、圆、椭圆、线段）
//! - 贝塞尔曲线绘制
//! - 洪水填充（flood fill）
//! - 形态学操作（膨胀 / 腐蚀 / 开运算 / 闭运算）
//! - Hough 直线检测

use image::{Luma, Rgb, RgbImage};
use imageproc::distance_transform::Norm;
use imageproc::drawing::{
    draw_cubic_bezier_curve_mut, draw_filled_circle_mut, draw_filled_ellipse_mut,
    draw_filled_rect_mut, draw_hollow_circle_mut, draw_hollow_rect_mut, draw_line_segment_mut,
    flood_fill_mut,
};
use imageproc::hough::{detect_lines, draw_polar_lines, LineDetectionOptions, PolarLine};
use imageproc::map::map_pixels;
use imageproc::morphology::{close, dilate, erode, open};
use imageproc::rect::Rect;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = "examples/image";

    // ===== 1. 绘制基本形状 =====
    println!("===== 绘制基本形状 =====");

    // 创建一个 400x300 的白色画布
    let mut canvas = RgbImage::from_pixel(400, 300, Rgb([255u8, 255, 255]));

    // 填充矩形 (红色)
    draw_filled_rect_mut(
        &mut canvas,
        Rect::at(20, 20).of_size(120, 80),
        Rgb([255u8, 50, 50]),
    );

    // 空心矩形 (蓝色，线宽 3)
    draw_hollow_rect_mut(
        &mut canvas,
        Rect::at(160, 20).of_size(120, 80),
        Rgb([50u8, 50, 255]),
    );

    // 填充圆 (绿色)
    draw_filled_circle_mut(&mut canvas, (80, 180), 50, Rgb([50u8, 200, 50]));

    // 空心圆 (紫色)
    draw_hollow_circle_mut(&mut canvas, (220, 180), 50, Rgb([180u8, 50, 255]));

    // 填充椭圆 (橙色)
    draw_filled_ellipse_mut(&mut canvas, (340, 80), 50, 30, Rgb([255u8, 165, 0]));

    // 线段 (黑色)
    draw_line_segment_mut(
        &mut canvas,
        (280.0, 140.0),
        (390.0, 280.0),
        Rgb([0u8, 0, 0]),
    );

    // 三次贝塞尔曲线 (青色)
    draw_cubic_bezier_curve_mut(
        &mut canvas,
        (280.0, 200.0), // 起点
        (320.0, 150.0), // 控制点 1
        (360.0, 280.0), // 控制点 2
        (390.0, 200.0), // 终点
        Rgb([0u8, 200, 200]),
    );

    canvas.save(format!("{out_dir}/draw_shapes.png"))?;
    println!("基本形状绘图已保存");

    // ===== 2. 洪水填充（Flood Fill） =====
    println!("\n===== 洪水填充 =====");

    let mut flood_canvas = RgbImage::from_pixel(300, 200, Rgb([255u8, 255, 255]));

    // 先画一个封闭区域
    draw_hollow_rect_mut(
        &mut flood_canvas,
        Rect::at(50, 30).of_size(100, 100),
        Rgb([0u8, 0, 0]),
    );
    draw_hollow_circle_mut(&mut flood_canvas, (220, 100), 60, Rgb([0u8, 0, 0]));

    // 在封闭区域内进行洪水填充
    flood_fill_mut(
        &mut flood_canvas,
        100, // 起始 x
        80,  // 起始 y
        Rgb([255u8, 200, 200]), // 填充颜色（浅红）
    );
    flood_fill_mut(
        &mut flood_canvas,
        220, // 圆心 x
        100, // 圆心 y
        Rgb([200u8, 200, 255]), // 填充颜色（浅蓝）
    );

    flood_canvas.save(format!("{out_dir}/draw_flood_fill.png"))?;
    println!("洪水填充已保存");

    // ===== 3. 形态学操作 =====
    println!("\n===== 形态学操作 =====");

    // 加载图像并转为二值图
    let img = image::open("examples/image/romanesco.jpg")?;
    let gray = img.to_luma8();

    // 简单二值化
    let binary = map_pixels(&gray, |p| {
        if p[0] > 128 {
            Luma([255u8])
        } else {
            Luma([0u8])
        }
    });
    binary.save(format!("{out_dir}/morph_binary.png"))?;
    println!("二值化原图已保存");

    // 膨胀（扩大白色区域），使用 LInf 范数（棋盘距离）
    let dilated = dilate(&binary, Norm::LInf, 3);
    dilated.save(format!("{out_dir}/morph_dilate.png"))?;
    println!("膨胀 (LInf, k=3) 已保存");

    // 腐蚀（缩小白色区域）
    let eroded = erode(&binary, Norm::LInf, 3);
    eroded.save(format!("{out_dir}/morph_erode.png"))?;
    println!("腐蚀 (LInf, k=3) 已保存");

    // 开运算 = 先腐蚀后膨胀（去除小的白色噪点）
    let opened = open(&binary, Norm::LInf, 3);
    opened.save(format!("{out_dir}/morph_open.png"))?;
    println!("开运算 (LInf, k=3) 已保存");

    // 闭运算 = 先膨胀后腐蚀（填充小的黑色孔洞）
    let closed = close(&binary, Norm::LInf, 3);
    closed.save(format!("{out_dir}/morph_close.png"))?;
    println!("闭运算 (LInf, k=3) 已保存");

    // ===== 4. Hough 直线检测 =====
    println!("\n===== Hough 直线检测 =====");

    // 使用 Canny 边缘检测的结果作为 Hough 变换的输入
    let edges = imageproc::edges::canny(&gray, 20.0, 50.0);

    // 检测直线
    let options = LineDetectionOptions {
        vote_threshold: 100,    // 至少 100 票才被认为是直线
        suppression_radius: 1,  // 非极大值抑制半径
    };
    let lines: Vec<PolarLine> = detect_lines(&edges, options);
    println!("检测到 {} 条直线", lines.len());

    // 在原始彩色图上绘制检测到的直线
    let mut img_rgb = img.to_rgb8();
    draw_polar_lines(
        &mut img_rgb,
        &lines,
        Rgb([255u8, 0, 0]), // 红色线条
    );
    img_rgb.save(format!("{out_dir}/morph_hough_lines.png"))?;
    println!("Hough 直线检测结果已保存");

    // ===== 5. 组合示例：标注检测区域 =====
    println!("\n===== 组合示例：标注检测区域 =====");

    let mut annotated = img.to_rgb8();
    // 在图像上绘制一个半透明矩形标注区域
    let (w, h) = (annotated.width(), annotated.height());
    let roi = Rect::at((w / 4) as i32, (h / 4) as i32).of_size(w / 2, h / 2);

    // 绘制空心矩形作为 ROI 标注
    draw_hollow_rect_mut(&mut annotated, roi, Rgb([0u8, 255, 0]));

    // 在 ROI 四角绘制小圆点
    let corners = [
        (roi.left(), roi.top()),
        (roi.right() - 1, roi.top()),
        (roi.right() - 1, roi.bottom() - 1),
        (roi.left(), roi.bottom() - 1),
    ];
    for (cx, cy) in corners {
        draw_filled_circle_mut(&mut annotated, (cx, cy), 5, Rgb([255u8, 0, 0]));
    }

    annotated.save(format!("{out_dir}/draw_annotated.png"))?;
    println!("标注图像已保存");

    println!("\n所有绘图与形态学示例运行完毕！");
    Ok(())
}
