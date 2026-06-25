use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建一个PNG位图后端，并获取绘图区域
    let root = BitMapBackend::new("/tmp/image/plot.png", (640, 480)).into_drawing_area();
    // 2. 用白色填充背景
    root.fill(&WHITE)?;

    // 3. 构建图表上下文
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font()) // 图表标题
        .margin(5)
        .x_label_area_size(30) // X轴标签区域大小
        .y_label_area_size(30) // Y轴标签区域大小
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?; // 构建2D坐标系

    // 4. 绘制网格和坐标轴
    chart.configure_mesh().draw()?;

    // 5. 绘制数据系列 (LineSeries)
    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)), // 使用迭代器生成数据点
            &RED,                                                    // 线条颜色
        ))?
        .label("y = x^2") // 为系列添加标签，用于生成图例
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED)); // 图例样式

    // 6. 绘制图例
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    // 7. 将结果呈现到文件中
    root.present()?;
    Ok(())
}
