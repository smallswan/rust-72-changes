//! # 算法模块
//!
//! 对应第二十六章算法、第七十三章经典问题，涵盖排序、搜索、动态规划、回溯等。

// ============================================================
// 斐波那契数列
// ============================================================

/// 迭代法求第 n 个斐波那契数
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

/// 矩阵快速幂法求斐波那契数（O(log n)）
pub fn fibonacci_fast(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    fn mat_mul(a: [[u128; 2]; 2], b: [[u128; 2]; 2]) -> [[u128; 2]; 2] {
        [
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1],
            ],
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1],
            ],
        ]
    }
    fn mat_pow(mut m: [[u128; 2]; 2], mut n: u64) -> [[u128; 2]; 2] {
        let mut result = [[1u128, 0], [0, 1]]; // 单位矩阵
        while n > 0 {
            if n & 1 == 1 {
                result = mat_mul(result, m);
            }
            m = mat_mul(m, m);
            n >>= 1;
        }
        result
    }
    let base = [[1u128, 1], [1, 0]];
    let result = mat_pow(base, n - 1);
    result[0][0] as u64
}

// ============================================================
// 汉诺塔
// ============================================================

/// 汉诺塔：返回移动步骤列表
pub fn hanoi(n: u32, from: &str, to: &str, aux: &str) -> Vec<String> {
    let mut steps = Vec::new();
    hanoi_helper(n, from, to, aux, &mut steps);
    steps
}

fn hanoi_helper(n: u32, from: &str, to: &str, aux: &str, steps: &mut Vec<String>) {
    if n == 0 {
        return;
    }
    hanoi_helper(n - 1, from, aux, to, steps);
    steps.push(format!("移动圆盘 {} 从 {} 到 {}", n, from, to));
    hanoi_helper(n - 1, aux, to, from, steps);
}

// ============================================================
// 动态规划
// ============================================================

/// 0-1 背包问题
pub fn knapsack_01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![0usize; capacity + 1];
    for i in 0..n {
        for w in (weights[i]..=capacity).rev() {
            dp[w] = dp[w].max(dp[w - weights[i]] + values[i]);
        }
    }
    dp[capacity]
}

/// 最长公共子序列长度
pub fn lcs_length(a: &[char], b: &[char]) -> usize {
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    dp[m][n]
}

/// 编辑距离（Levenshtein Distance）
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[m][n]
}

/// 最长递增子序列长度（O(n log n)）
pub fn lis_length(nums: &[i64]) -> usize {
    let mut tails: Vec<i64> = Vec::new();
    for &num in nums {
        match tails.binary_search(&num) {
            Ok(_) => {}
            Err(idx) => {
                if idx == tails.len() {
                    tails.push(num);
                } else {
                    tails[idx] = num;
                }
            }
        }
    }
    tails.len()
}

// ============================================================
// 约瑟夫环
// ============================================================

/// 约瑟夫环：n 人围圈，每 m 人出局，返回最后幸存者编号
pub fn josephus(n: usize, m: usize) -> usize {
    let mut pos = 0usize;
    for i in 2..=n {
        pos = (pos + m) % i;
    }
    pos
}

// ============================================================
// 排序算法
// ============================================================

/// 快速排序
pub fn quicksort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let pivot = &arr[arr.len() / 2];
    let left: Vec<T> = arr.iter().filter(|x| *x < pivot).cloned().collect();
    let mid: Vec<T> = arr.iter().filter(|x| *x == pivot).cloned().collect();
    let right: Vec<T> = arr.iter().filter(|x| *x > pivot).cloned().collect();
    let mut result = quicksort(&left);
    result.extend(mid);
    result.extend(quicksort(&right));
    result
}

/// 归并排序
pub fn merge_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mid = arr.len() / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);
    merge(&left, &right)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

// ============================================================
// 搜索
// ============================================================

/// 二分查找（返回索引，未找到返回 None）
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = arr.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

// ============================================================
// N 皇后问题
// ============================================================

/// N 皇后问题：返回所有解的数量
pub fn n_queens_count(n: usize) -> usize {
    let mut count = 0;
    let mut cols = vec![0usize; n];
    n_queens_backtrack(0, n, &mut cols, &mut count);
    count
}

fn n_queens_backtrack(row: usize, n: usize, cols: &mut Vec<usize>, count: &mut usize) {
    if row == n {
        *count += 1;
        return;
    }
    for col in 0..n {
        if n_queens_is_safe(row, col, cols) {
            cols[row] = col;
            n_queens_backtrack(row + 1, n, cols, count);
        }
    }
}

fn n_queens_is_safe(row: usize, col: usize, cols: &[usize]) -> bool {
    for i in 0..row {
        if cols[i] == col || (cols[i] as isize - col as isize).abs() == (row as isize - i as isize)
        {
            return false;
        }
    }
    true
}

// ============================================================
// 逆波兰表达式求值
// ============================================================

/// 逆波兰表示法（后缀表达式）求值
pub fn eval_rpn(tokens: &[&str]) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();
    for &token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return None;
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => stack.push(token.parse().ok()?),
        }
    }
    stack.pop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_fibonacci_fast() {
        assert_eq!(fibonacci_fast(0), 0);
        assert_eq!(fibonacci_fast(1), 1);
        assert_eq!(fibonacci_fast(10), 55);
        assert_eq!(fibonacci_fast(50), 12586269025);
    }

    #[test]
    fn test_hanoi() {
        let steps = hanoi(3, "A", "C", "B");
        assert_eq!(steps.len(), 7); // 2^3 - 1 = 7
        assert_eq!(steps[0], "移动圆盘 1 从 A 到 C");
    }

    #[test]
    fn test_knapsack() {
        let weights = vec![2, 3, 4, 5];
        let values = vec![3, 4, 5, 6];
        assert_eq!(knapsack_01(&weights, &values, 8), 10); // 物品1+物品3 或 物品2+物品3
    }

    #[test]
    fn test_lcs() {
        let a: Vec<char> = "ABCBDAB".chars().collect();
        let b: Vec<char> = "BDCABA".chars().collect();
        assert_eq!(lcs_length(&a, &b), 4);
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("", "abc"), 3);
    }

    #[test]
    fn test_lis() {
        assert_eq!(lis_length(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(lis_length(&[1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_josephus() {
        assert_eq!(josephus(5, 3), 3);
        assert_eq!(josephus(1, 1), 0);
    }

    #[test]
    fn test_quicksort() {
        let arr = vec![5, 2, 8, 1, 9, 3];
        assert_eq!(quicksort(&arr), vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let arr = vec![5, 2, 8, 1, 9, 3];
        assert_eq!(merge_sort(&arr), vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 5, 7, 9, 11];
        assert_eq!(binary_search(&arr, &7), Some(3));
        assert_eq!(binary_search(&arr, &4), None);
        assert_eq!(binary_search(&arr, &1), Some(0));
    }

    #[test]
    fn test_n_queens() {
        assert_eq!(n_queens_count(4), 2);
        assert_eq!(n_queens_count(8), 92);
    }

    #[test]
    fn test_rpn() {
        // "3 + 4 * 2" → "3 4 2 * +"
        assert_eq!(eval_rpn(&["3", "4", "2", "*", "+"]), Some(11.0));
        // "(1 + 2) * 3" → "1 2 + 3 *"
        assert_eq!(eval_rpn(&["1", "2", "+", "3", "*"]), Some(9.0));
    }
}
