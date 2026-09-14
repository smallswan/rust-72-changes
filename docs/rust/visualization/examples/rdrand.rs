// 使用 rdrand crate 访问硬件随机数生成器
use rand_core::TryRng;
use rdrand::RdRand;

fn main() {
    if let Ok(mut rng) = RdRand::new() {
        if let Ok(val) = rng.try_next_u64() {
            println!("Hardware random: {}", val);
        }
    }
}
