//! # 安全模块
//!
//! 对应第五十九/六十一章，涵盖数据脱敏、路径遍历防护、安全运算、哈希等。
//! 使用纯 Rust 实现，不依赖外部密码学库。

// ============================================================
// 数据脱敏
// ============================================================

/// 手机号脱敏：13812345678 → 138****5678
pub fn mask_phone(phone: &str) -> String {
    let chars: Vec<char> = phone.chars().collect();
    if chars.len() < 7 {
        return "*".repeat(chars.len());
    }
    let n = chars.len();
    let mut result = String::with_capacity(n);
    for (i, &c) in chars.iter().enumerate() {
        if i < 3 || i >= n - 4 {
            result.push(c);
        } else {
            result.push('*');
        }
    }
    result
}

/// 邮箱脱敏：user@example.com → u***@example.com
pub fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let (local, domain) = email.split_at(at_pos);
        let local_chars: Vec<char> = local.chars().collect();
        if local_chars.is_empty() {
            return email.to_string();
        }
        let mut result = String::new();
        result.push(local_chars[0]);
        result.push_str(&"*".repeat(local_chars.len().saturating_sub(1)));
        result.push_str(domain);
        result
    } else {
        email.to_string()
    }
}

/// 身份证号脱敏：110101199001011234 → 110***********1234
pub fn mask_id_card(id: &str) -> String {
    let chars: Vec<char> = id.chars().collect();
    if chars.len() < 8 {
        return "*".repeat(chars.len());
    }
    let n = chars.len();
    let mut result = String::with_capacity(n);
    for (i, &c) in chars.iter().enumerate() {
        if i < 3 || i >= n - 4 {
            result.push(c);
        } else {
            result.push('*');
        }
    }
    result
}

// ============================================================
// 路径遍历防护
// ============================================================

/// 检查路径是否在允许的基础目录内，防止路径遍历攻击
pub fn is_path_safe(base: &std::path::Path, user_input: &str) -> bool {
    let full = base.join(user_input);

    // 检查是否包含 .. 或绝对路径
    if user_input.contains("..") {
        return false;
    }
    if user_input.starts_with('/') || user_input.starts_with('\\') {
        return false;
    }
    // Windows 盘符检查
    if user_input.len() >= 2 {
        let bytes = user_input.as_bytes();
        if bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            return false;
        }
    }

    // 尝试规范化并验证
    match (base.canonicalize(), full.canonicalize()) {
        (Ok(base_canon), Ok(full_canon)) => full_canon.starts_with(base_canon),
        _ => {
            // 无法规范化时，仅检查 .. 和绝对路径
            true
        }
    }
}

// ============================================================
// 安全运算
// ============================================================

/// 安全加法：溢出返回 None
pub fn safe_add(a: u64, b: u64) -> Option<u64> {
    a.checked_add(b)
}

/// 安全乘法：溢出返回 None
pub fn safe_mul(a: u64, b: u64) -> Option<u64> {
    a.checked_mul(b)
}

/// 饱和加法：溢出返回最大值
pub fn saturating_add(a: u64, b: u64) -> u64 {
    a.saturating_add(b)
}

// ============================================================
// 常量时间比较（防止时序攻击）
// ============================================================

/// 常量时间比较两个字节切片，防止时序侧信道攻击
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

// ============================================================
// 简单哈希（教学用途，非密码学安全）
// ============================================================

/// FNV-1a 哈希（64位），用于数据完整性校验演示
pub fn fnv1a_hash(data: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// 简单 SHA-256 风格的循环哈希（教学演示，非真正 SHA-256）
pub fn simple_hash(data: &[u8]) -> [u8; 32] {
    let mut state = [0u8; 32];
    // 用 FNV-1a 作为种子
    let seed = fnv1a_hash(data);

    // 初始化状态
    for (i, byte) in state.iter_mut().enumerate() {
        *byte = (seed.wrapping_mul((i as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15))
            >> ((i * 4) % 56)) as u8;
    }

    // 混合输入数据
    for (i, &byte) in data.iter().enumerate() {
        let idx = i % 32;
        state[idx] = state[idx].wrapping_add(byte);
        state[(idx + 1) % 32] ^= state[idx].rotate_left(3);
        state[(idx + 7) % 32] = state[(idx + 7) % 32].wrapping_add(state[idx].rotate_right(5));
    }

    // 最终混淆
    for round in 0..64 {
        for i in 0..32 {
            let j = (i + round + 1) % 32;
            state[i] = state[i]
                .wrapping_add(state[j])
                .rotate_left(((round + i) % 7 + 1) as u32);
        }
    }

    state
}

// ============================================================
// 密码强度检查
// ============================================================

/// 密码强度评估，返回 0-4 的评分
pub fn password_strength(password: &str) -> u8 {
    let mut score = 0u8;

    // 长度检查
    if password.len() >= 8 {
        score += 1;
    }
    if password.len() >= 12 {
        score += 1;
    }

    // 复杂度检查
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    let variety = [has_lower, has_upper, has_digit, has_special]
        .iter()
        .filter(|&&b| b)
        .count();
    if variety >= 3 {
        score += 1;
    }
    if variety == 4 {
        score += 1;
    }

    score.min(4)
}

/// 密码强度描述
pub fn password_strength_label(password: &str) -> &'static str {
    match password_strength(password) {
        0 | 1 => "弱",
        2 => "中",
        3 => "强",
        _ => "极强",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_phone() {
        assert_eq!(mask_phone("13812345678"), "138****5678");
        assert_eq!(mask_phone("123"), "***");
    }

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("user@example.com"), "u***@example.com");
        assert_eq!(mask_email("a@test.com"), "a@test.com");
        assert_eq!(mask_email("notanemail"), "notanemail");
    }

    #[test]
    fn test_mask_id_card() {
        let masked = mask_id_card("110101199001011234");
        assert_eq!(masked, "110***********1234");
    }

    #[test]
    fn test_path_traversal() {
        assert!(!is_path_safe(
            std::path::Path::new("/app/data"),
            "../../../etc/passwd"
        ));
        assert!(!is_path_safe(
            std::path::Path::new("/app/data"),
            "/etc/passwd"
        ));
        assert!(is_path_safe(std::path::Path::new("/app/data"), "file.txt"));
    }

    #[test]
    fn test_safe_arithmetic() {
        assert_eq!(safe_add(1, 2), Some(3));
        assert_eq!(safe_add(u64::MAX, 1), None);
        assert_eq!(safe_mul(u64::MAX, 2), None);
        assert_eq!(saturating_add(u64::MAX, 1), u64::MAX);
    }

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare(b"hello", b"hello"));
        assert!(!constant_time_compare(b"hello", b"world"));
        assert!(!constant_time_compare(b"hello", b"hi"));
    }

    #[test]
    fn test_fnv1a() {
        let h1 = fnv1a_hash(b"hello");
        let h2 = fnv1a_hash(b"hello");
        let h3 = fnv1a_hash(b"world");
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_simple_hash() {
        let h1 = simple_hash(b"hello");
        let h2 = simple_hash(b"hello");
        let h3 = simple_hash(b"world");
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert_eq!(h1.len(), 32);
    }

    #[test]
    fn test_password_strength() {
        assert_eq!(password_strength("123"), 0);
        assert_eq!(password_strength("abcdefgh"), 1); // 长度8+全小写=1
        assert_eq!(password_strength("Abcdefgh1"), 2); // 长度9+大小写+数字=2
        assert_eq!(password_strength("Abcdefgh1!@"), 3); // 长度11+四种字符=3

        assert_eq!(password_strength_label("123"), "弱");
        assert_eq!(password_strength_label("Abcdefgh1!@"), "强");
        assert_eq!(password_strength_label("Abcdefghijk1!@"), "极强"); // 长度15+四种字符=4
    }
}
