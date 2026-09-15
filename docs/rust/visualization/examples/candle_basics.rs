//! candle-core 基础张量操作示例
//!
//! 演示内容：
//! - 张量创建（new / zeros / ones / arange / randn）
//! - 形状变换（reshape / transpose）
//! - 基本算术运算（add / sub / mul / div）
//! - 元素级数学函数（exp / log / sqrt / sqr / tanh）
//! - 规约操作（sum / mean）

use candle_core::{DType, Device, Result, Tensor};

fn main() -> Result<()> {
    let device = Device::Cpu;

    // ===== 1. 张量创建 =====
    println!("===== 张量创建 =====");

    // 从 Rust 数组创建
    let a = Tensor::new(&[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0], &device)?;
    println!("从数组创建: shape={:?}, dtype={:?}", a.shape().dims(), a.dtype());

    // 全零 / 全一张量
    let zeros = Tensor::zeros((2, 3), DType::F32, &device)?;
    let ones = Tensor::ones((2, 3), DType::F32, &device)?;
    println!("zeros(2x3): {:?}", zeros.to_vec2::<f32>()?);
    println!("ones(2x3):  {:?}", ones.to_vec2::<f32>()?);

    // 等差序列
    let arange = Tensor::arange(0f32, 12f32, &device)?;
    println!("arange(0..12): {:?}", arange.to_vec1::<f32>()?);

    // 正态分布随机张量
    let randn = Tensor::randn(0f32, 1f32, (3, 3), &device)?;
    println!("randn(3x3): shape={:?}", randn.shape().dims());

    // 用 full 创建常值张量
    let full = Tensor::full(3.14f32, (2, 2), &device)?;
    println!("full(3.14, 2x2): {:?}", full.to_vec2::<f32>()?);

    // ===== 2. 形状变换 =====
    println!("\n===== 形状变换 =====");

    // reshape：将 1D 变为 2D（使用 -1 表示自动推断维度）
    let reshaped = a.reshape((2, 3))?;
    println!("reshape(6) -> (2,3): {:?}", reshaped.to_vec2::<f32>()?);

    // 转置
    let transposed = reshaped.transpose(0, 1)?;
    println!("transpose(2,3) -> (3,2): {:?}", transposed.to_vec2::<f32>()?);

    // ===== 3. 基本算术运算 =====
    println!("\n===== 基本算术运算 =====");

    let x = Tensor::new(&[1.0f32, 2.0, 3.0], &device)?;
    let y = Tensor::new(&[4.0f32, 5.0, 6.0], &device)?;

    let sum = x.add(&y)?;
    let diff = y.sub(&x)?;
    let prod = x.mul(&y)?;
    let quot = y.div(&x)?;

    println!("x + y = {:?}", sum.to_vec1::<f32>()?);
    println!("y - x = {:?}", diff.to_vec1::<f32>()?);
    println!("x * y = {:?}", prod.to_vec1::<f32>()?);
    println!("y / x = {:?}", quot.to_vec1::<f32>()?);

    // 标量运算：通过 full 创建标量张量后运算
    let scalar = Tensor::full(10.0f32, (3,), &device)?;
    let scaled = x.mul(&scalar)?;
    println!("x * 10 = {:?}", scaled.to_vec1::<f32>()?);

    // ===== 4. 元素级数学函数 =====
    println!("\n===== 元素级数学函数 =====");

    let vals = Tensor::new(&[1.0f32, 2.0, 3.0, 4.0], &device)?;

    let exp_vals = vals.exp()?;
    let log_vals = exp_vals.log()?;
    let sqrt_vals = vals.sqrt()?;
    let sqr_vals = vals.sqr()?;
    let tanh_vals = vals.tanh()?;

    println!("exp({:?})  = {:?}", vals.to_vec1::<f32>()?, exp_vals.to_vec1::<f32>()?);
    println!("log(exp)  = {:?}", log_vals.to_vec1::<f32>()?);
    println!("sqrt      = {:?}", sqrt_vals.to_vec1::<f32>()?);
    println!("sqr       = {:?}", sqr_vals.to_vec1::<f32>()?);
    println!("tanh      = {:?}", tanh_vals.to_vec1::<f32>()?);

    // ===== 5. 规约操作 =====
    println!("\n===== 规约操作 =====");

    let matrix = Tensor::new(&[[1.0f32, 2.0, 3.0], [4.0, 5.0, 6.0]], &device)?;
    println!("矩阵: {:?}", matrix.to_vec2::<f32>()?);

    // 对所有元素求和
    let total_sum = matrix.sum_all()?;
    println!("sum_all = {}", total_sum.to_scalar::<f32>()?);

    // 沿指定维度求和（D(1) 表示第 1 维，即按行求和）
    let row_sum = matrix.sum(1)?;
    println!("sum(dim=1) = {:?}", row_sum.to_vec1::<f32>()?);

    // 沿指定维度求均值
    let col_mean = matrix.mean(0)?;
    println!("mean(dim=0) = {:?}", col_mean.to_vec1::<f32>()?);

    // ===== 6. 广播（Broadcasting） =====
    println!("\n===== 广播 =====");

    let big = Tensor::new(&[[1.0f32, 2.0, 3.0], [4.0, 5.0, 6.0]], &device)?;
    let small = Tensor::new(&[10.0f32, 20.0, 30.0], &device)?;

    // 使用 broadcast_add 进行广播加法：small (3,) 自动广播到 (2, 3)
    let broadcast_add = big.broadcast_add(&small)?;
    println!("broadcast add: {:?}", broadcast_add.to_vec2::<f32>()?);

    println!("\n所有示例运行完毕！");
    Ok(())
}
