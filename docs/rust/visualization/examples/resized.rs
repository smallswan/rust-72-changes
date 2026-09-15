use image::imageops;

fn main() {
    // 加载图像
    let img = image::open("examples/image/cloud.jpg").expect("无法打开图像");
    println!("原始尺寸: {}x{}", img.width(), img.height());

    // 使用不同滤波器缩放
    let methods = [
        ("nearest", imageops::FilterType::Nearest),
        ("triangle", imageops::FilterType::Triangle),
        ("catmull", imageops::FilterType::CatmullRom),
        ("gaussian", imageops::FilterType::Gaussian),
        ("lanczos", imageops::FilterType::Lanczos3),
    ];

    for (name, filter) in &methods {
        let resized = img.resize(224, 224, *filter);
        resized
            .save(format!("examples/image/resized_{}.png", name))
            .unwrap();
        println!("已保存 resized_{}.png (224x224)", name);
    }
}
