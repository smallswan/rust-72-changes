//! candle-core 自动求导（Autograd）示例
//!
//! 演示内容：
//! - Var 变量与梯度追踪
//! - 反向传播（backward）
//! - 手动实现梯度下降线性回归
//! - MSE 损失函数

use candle_core::{Device, Result, Tensor, Var};

/// 辅助函数：从 shape=[1] 的张量中提取标量值
fn scalar(t: &Tensor) -> Result<f32> {
    t.to_vec1::<f32>().map(|v| v[0])
}

fn main() -> Result<()> {
    let device = Device::Cpu;

    // ===== 1. 基础自动求导 =====
    println!("===== 基础自动求导 =====");
    println!("计算 f(x) = x^2 + 2x + 1 在 x=3 处的梯度");
    println!("解析梯度: f'(x) = 2x + 2, f'(3) = 8.0");

    let x = Var::new(&[3.0f32], &device)?;

    // f(x) = x^2 + 2x + 1
    let x_sq = x.sqr()?;                              // x^2
    let two_x = x.as_tensor().affine(2.0, 0.0)?;       // 2x
    let one = Tensor::new(&[1.0f32], &device)?;
    let f = x_sq.add(&two_x)?.add(&one)?;              // x^2 + 2x + 1

    println!("f(3) = {}", scalar(&f)?); // 9 + 6 + 1 = 16

    // 反向传播
    let grads = f.backward()?;
    let grad_x = grads.get(&x).unwrap();
    println!("df/dx at x=3 = {}", scalar(grad_x)?); // 2*3 + 2 = 8

    // ===== 2. 多元函数梯度 =====
    println!("\n===== 多元函数梯度 =====");
    println!("计算 f(x, y) = x^2 + x*y + y^2 在 (x=1, y=2) 处的梯度");
    println!("解析梯度: df/dx = 2x + y = 4, df/dy = x + 2y = 5");

    let xv = Var::new(&[1.0f32], &device)?;
    let yv = Var::new(&[2.0f32], &device)?;

    let x2 = xv.sqr()?;
    let y2 = yv.sqr()?;
    let xy = xv.as_tensor().mul(yv.as_tensor())?;
    let f2 = x2.add(&xy)?.add(&y2)?;

    println!("f(1, 2) = {}", scalar(&f2)?); // 1 + 2 + 4 = 7

    let grads2 = f2.backward()?;
    let grad_x2 = grads2.get(&xv).unwrap();
    let grad_y2 = grads2.get(&yv).unwrap();
    println!("df/dx = {}", scalar(grad_x2)?); // 2*1 + 2 = 4
    println!("df/dy = {}", scalar(grad_y2)?); // 1 + 2*2 = 5

    // ===== 3. 梯度下降线性回归 =====
    println!("\n===== 梯度下降线性回归 =====");
    println!("目标: 拟合 y = 2x + 1，使用梯度下降优化 w 和 b");

    // 生成训练数据：y = 2x + 1
    let x_data = Tensor::new(&[1.0f32, 2.0, 3.0, 4.0, 5.0], &device)?;
    let y_data = Tensor::new(&[3.0f32, 5.0, 7.0, 9.0, 11.0], &device)?; // 2x + 1

    // 初始化参数
    let w = Var::new(&[0.0f32], &device)?;
    let b = Var::new(&[0.0f32], &device)?;

    let learning_rate = 0.01f64;
    let n_samples = 5.0f64;

    println!(
        "初始参数: w={:.4}, b={:.4}",
        scalar(w.as_tensor())?,
        scalar(b.as_tensor())?
    );

    // 训练循环
    for epoch in 0..500 {
        // 前向传播: y_pred = w * x + b（需要广播，w 和 b 形状为 [1]）
        let y_pred = x_data
            .broadcast_mul(w.as_tensor())?
            .broadcast_add(b.as_tensor())?;

        // MSE 损失: L = mean((y_pred - y_data)^2)
        let diff = y_pred.sub(&y_data)?;
        let loss = (diff.sqr()?.sum_all()? / n_samples)?; // sum_all 返回标量张量

        // 每 100 轮打印一次
        if epoch % 100 == 0 {
            let loss_val = loss.to_scalar::<f32>()?; // sum_all 结果是 0 维，可用 to_scalar
            println!(
                "Epoch {:>3}: loss={:.6}, w={:.4}, b={:.4}",
                epoch,
                loss_val,
                scalar(w.as_tensor())?,
                scalar(b.as_tensor())?
            );
        }

        // 反向传播
        let grads = loss.backward()?;

        // 获取梯度
        let grad_w = grads.get(&w).unwrap();
        let grad_b = grads.get(&b).unwrap();

        // 参数更新: param = param - lr * grad
        // 使用 affine(mul, add) 实现标量乘法: grad * lr = grad.affine(lr, 0)
        let w_new = w.as_tensor().sub(&grad_w.affine(learning_rate, 0.0)?)?;
        let b_new = b.as_tensor().sub(&grad_b.affine(learning_rate, 0.0)?)?;

        w.set(&w_new)?;
        b.set(&b_new)?;
    }

    println!(
        "\n最终参数: w={:.4}, b={:.4}",
        scalar(w.as_tensor())?,
        scalar(b.as_tensor())?
    );
    println!("期望参数: w=2.0000, b=1.0000");

    // 验证预测
    let test_x = Tensor::new(&[6.0f32, 7.0, 8.0], &device)?;
    let predictions = test_x
        .broadcast_mul(w.as_tensor())?
        .broadcast_add(b.as_tensor())?;
    let expected = Tensor::new(&[13.0f32, 15.0, 17.0], &device)?;

    println!("\n预测验证:");
    println!("输入:     {:?}", test_x.to_vec1::<f32>()?);
    println!("预测值:   {:?}", predictions.to_vec1::<f32>()?);
    println!("期望值:   {:?}", expected.to_vec1::<f32>()?);

    println!("\n所有示例运行完毕！");
    Ok(())
}
