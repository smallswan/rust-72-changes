//! candle-core 矩阵运算与线性代数示例
//!
//! 演示内容：
//! - 矩阵乘法（matmul）
//! - 手动实现 Softmax
//! - 手动实现简单线性层前向传播
//! - 张量拼接（cat）

use candle_core::{DType, Device, Result, Tensor, D};

fn main() -> Result<()> {
    let device = Device::Cpu;

    // ===== 1. 矩阵乘法 =====
    println!("===== 矩阵乘法 =====");

    let a = Tensor::arange(0f32, 6f32, &device)?.reshape((2, 3))?;
    let b = Tensor::arange(0f32, 12f32, &device)?.reshape((3, 4))?;
    let c = a.matmul(&b)?;

    println!("A (2x3): {:?}", a.to_vec2::<f32>()?);
    println!("B (3x4): {:?}", b.to_vec2::<f32>()?);
    println!("A @ B (2x4): {:?}", c.to_vec2::<f32>()?);

    // 验证：C[0,0] = 0*0 + 1*4 + 2*8 = 20
    //        C[0,1] = 0*1 + 1*5 + 2*9 = 23
    //        C[1,0] = 3*0 + 4*4 + 5*8 = 56

    // ===== 2. 批量矩阵乘法 =====
    println!("\n===== 批量矩阵乘法 =====");

    // (batch=2, seq=3, dim=4)
    let batch_a = Tensor::randn(0f32, 1f32, (2, 3, 4), &device)?;
    // (batch=2, dim=4, out=2)
    let batch_b = Tensor::randn(0f32, 1f32, (2, 4, 2), &device)?;
    let batch_c = batch_a.matmul(&batch_b)?;

    println!("batch_a shape: {:?}", batch_a.shape().dims());
    println!("batch_b shape: {:?}", batch_b.shape().dims());
    println!("batch_c shape: {:?}", batch_c.shape().dims());

    // ===== 3. 手动实现 Softmax =====
    println!("\n===== 手动实现 Softmax =====");

    let logits = Tensor::new(&[[1.0f32, 2.0, 3.0], [1.0, 1.0, 1.0]], &device)?;
    println!("logits: {:?}", logits.to_vec2::<f32>()?);

    // softmax(x) = exp(x - max(x)) / sum(exp(x - max(x)))
    // 沿最后一维计算 max，保持维度以便广播
    let max_vals = logits.max_keepdim(D::Minus1)?;
    let shifted = logits.broadcast_sub(&max_vals)?;
    let exp_shifted = shifted.exp()?;
    let sum_exp = exp_shifted.sum_keepdim(D::Minus1)?;
    let softmax = exp_shifted.broadcast_div(&sum_exp)?;

    println!("softmax: {:?}", softmax.to_vec2::<f32>()?);
    // 验证每行之和为 1
    let row_sums = softmax.sum(1)?;
    println!("softmax 行和: {:?}", row_sums.to_vec1::<f32>()?);

    // ===== 4. 线性层前向传播 =====
    println!("\n===== 线性层前向传播 =====");

    // 模拟 y = x @ W^T + b
    let batch_size = 3;
    let in_features = 4;
    let out_features = 2;

    // 输入: (batch_size, in_features)
    let x = Tensor::randn(0f32, 1f32, (batch_size, in_features), &device)?;
    // 权重: (out_features, in_features)
    let w = Tensor::randn(0f32, 1f32, (out_features, in_features), &device)?;
    // 偏置: (out_features,)
    let b = Tensor::ones(out_features, DType::F32, &device)?;

    // 前向传播：y = x @ W^T + b
    let w_t = w.transpose(0, 1)?; // (in_features, out_features)
    let linear_out = x.matmul(&w_t)?.broadcast_add(&b)?; // (batch_size, out_features)

    println!("输入 x shape: {:?}", x.shape().dims());
    println!("权重 W shape: {:?}", w.shape().dims());
    println!("输出 y shape: {:?}", linear_out.shape().dims());
    println!("输出 y: {:?}", linear_out.to_vec2::<f32>()?);

    // ===== 5. 张量拼接（cat） =====
    println!("\n===== 张量拼接 =====");

    let t1 = Tensor::new(&[[1.0f32, 2.0], [3.0, 4.0]], &device)?;
    let t2 = Tensor::new(&[[5.0f32, 6.0], [7.0, 8.0]], &device)?;

    // 沿第 0 维拼接（纵向拼接）
    let cat_dim0 = Tensor::cat(&[&t1, &t2], 0)?;
    println!("cat(dim=0):\n{:?}", cat_dim0.to_vec2::<f32>()?);

    // 沿第 1 维拼接（横向拼接）
    let cat_dim1 = Tensor::cat(&[&t1, &t2], 1)?;
    println!("cat(dim=1):\n{:?}", cat_dim1.to_vec2::<f32>()?);

    // ===== 6. 余弦相似度 =====
    println!("\n===== 余弦相似度 =====");

    let v1 = Tensor::new(&[1.0f32, 2.0, 3.0], &device)?;
    let v2 = Tensor::new(&[4.0f32, 5.0, 6.0], &device)?;

    // cosine_sim(a, b) = (a · b) / (||a|| * ||b||)
    let dot = v1.mul(&v2)?.sum_all()?;
    let norm1 = v1.sqr()?.sum_all()?.sqrt()?;
    let norm2 = v2.sqr()?.sum_all()?.sqrt()?;
    let cos_sim = dot.div(&norm1)?.div(&norm2)?;

    println!("v1 = {:?}", v1.to_vec1::<f32>()?);
    println!("v2 = {:?}", v2.to_vec1::<f32>()?);
    println!("cosine_similarity = {}", cos_sim.to_scalar::<f32>()?);

    println!("\n所有示例运行完毕！");
    Ok(())
}
