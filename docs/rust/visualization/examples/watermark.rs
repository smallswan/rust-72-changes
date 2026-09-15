use image::{DynamicImage, GenericImageView, imageops::FilterType};

fn add_watermark(base: &DynamicImage, watermark: &DynamicImage, opacity: f32) -> DynamicImage {
    let mut base = base.clone();
    let (bw, bh) = GenericImageView::dimensions(&base);
    // 按比例缩放，使水印宽度或高度为原图的1/4（保持宽高比）
    let (ww, wh) = GenericImageView::dimensions(watermark);
    let scale = 0.25_f32.min((bw as f32 / ww as f32).min(bh as f32 / wh as f32));
    let new_w = (ww as f32 * scale) as u32;
    let new_h = (wh as f32 * scale) as u32;
    let mut wm = watermark.resize(new_w, new_h, FilterType::Lanczos3);

    // 应用全局透明度（如果水印有alpha通道）
    if let DynamicImage::ImageRgba8(ref mut rgba) = wm {
        for pixel in rgba.pixels_mut() {
            let alpha = (pixel[3] as f32 * opacity).clamp(0.0, 255.0) as u8;
            pixel[3] = alpha;
        }
    } else {
        // 如果没有alpha通道，将其转换为RGBA并添加alpha
        let mut rgba = wm.to_rgba8();
        for pixel in rgba.pixels_mut() {
            pixel[3] = (255.0 * opacity) as u8;
        }
        wm = DynamicImage::ImageRgba8(rgba);
    }

    let (ww, wh) = GenericImageView::dimensions(&wm);
    let x = bw.saturating_sub(ww + 20);
    let y = bh.saturating_sub(wh + 20);

    // 使用 overlay 混合（水印的alpha通道会起作用）
    image::imageops::overlay(&mut base, &wm, x as i64, y as i64);
    base
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_img = image::open("examples/image/romanesco.jpg")?;
    let watermark = image::open("examples/image/rust-logo.png")?;
    let result = add_watermark(&base_img, &watermark, 0.5);
    result.save("examples/image/romanesco-watermarked.jpg")?;
    Ok(())
}
