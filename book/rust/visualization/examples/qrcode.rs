use image::{GenericImage, ImageBuffer, Rgba};
use qrcode::QrCode;

fn generate_qr_image(data: &str, size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // 1. 编码数据生成二维码
    let code = QrCode::new(data.as_bytes()).unwrap();

    // 2. 设置二维码的模块大小，并添加白色边框
    let border = 4;
    let module_size = (size as f32 / (code.width() as f32 + 2.0 * border as f32)).ceil() as u32;

    // 3. 将二维码渲染为图像
    code.render::<Rgba<u8>>()
        .min_dimensions(size, size) // 设置最小尺寸，确保清晰
        .quiet_zone(true) // 添加静区（白色边框）
        .module_dimensions(module_size, module_size)
        .build()
}

fn combine_qr_codes_horizontally(
    img1: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    img2: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    gap: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // 1. 计算新图片的尺寸
    let (w1, h1) = img1.dimensions();
    let (w2, h2) = img2.dimensions();
    let final_width = w1 + gap + w2;
    let final_height = h1.max(h2);

    // 2. 创建新画布，并填充白色背景
    let mut combined =
        ImageBuffer::from_pixel(final_width, final_height, Rgba([255, 255, 255, 255]));

    // 3. 将第一张图复制到左侧 (0, 0) 位置
    combined.copy_from(img1, 0, 0).unwrap();

    // 4. 将第二张图复制到右侧 (w1 + gap, 0) 位置
    combined.copy_from(img2, w1 + gap, 0).unwrap();

    combined
}

fn main() {
    // 1. 生成两个二维码图像，尺寸设为 200x200
    let qr1 = generate_qr_image("https://qr.alipay.com/fkx18203840eqbo648wdo95", 200);
    let qr2 = generate_qr_image("wxp://f2f0QuWokq-0iWOJrBOVHz7vVLoOhrChnMI28COpdSmFzRs", 200);

    // 2. 横向合并，间隔 20 像素
    let combined = combine_qr_codes_horizontally(&qr1, &qr2, 20);

    // 3. 保存最终图片
    combined.save("/tmp/combined_qr_codes.png").unwrap();
}
