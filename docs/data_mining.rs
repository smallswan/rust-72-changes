//! # 数据挖掘模块
//!
//! 对应第五十五章，涵盖聚类、异常检测、数据预处理等。

// ============================================================
// 统计基础
// ============================================================

/// 计算均值
pub fn mean(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

/// 计算标准差
pub fn std_dev(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let m = mean(data);
    let variance = data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

// ============================================================
// 数据预处理
// ============================================================

/// Min-Max 归一化：映射到 [0, 1] 区间
pub fn min_max_normalize(data: &[f64]) -> Vec<f64> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    if range == 0.0 {
        return vec![0.0; data.len()];
    }
    data.iter().map(|x| (x - min) / range).collect()
}

/// Z-Score 标准化：均值为 0，标准差为 1
pub fn zscore_standardize(data: &[f64]) -> Vec<f64> {
    let m = mean(data);
    let s = std_dev(data);
    if s == 0.0 {
        return vec![0.0; data.len()];
    }
    data.iter().map(|x| (x - m) / s).collect()
}

// ============================================================
// 异常检测
// ============================================================

/// Z-Score 异常检测：返回 (索引, 值, Z分数) 列表
pub fn detect_anomalies_zscore(data: &[f64], threshold: f64) -> Vec<(usize, f64, f64)> {
    let m = mean(data);
    let s = std_dev(data);
    let mut anomalies = Vec::new();

    if s == 0.0 {
        return anomalies;
    }

    for (i, &value) in data.iter().enumerate() {
        let z = (value - m) / s;
        if z.abs() > threshold {
            anomalies.push((i, value, z));
        }
    }
    anomalies
}

/// IQR（四分位距）异常检测
pub fn detect_anomalies_iqr(data: &mut Vec<f64>) -> Vec<(usize, f64)> {
    let mut sorted = data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted.len();
    if n < 4 {
        return Vec::new();
    }

    let q1 = sorted[n / 4];
    let q3 = sorted[3 * n / 4];
    let iqr = q3 - q1;
    let lower = q1 - 1.5 * iqr;
    let upper = q3 + 1.5 * iqr;

    data.iter()
        .enumerate()
        .filter(|&(_, &v)| v < lower || v > upper)
        .map(|(i, &v)| (i, v))
        .collect()
}

// ============================================================
// K-Means 聚类
// ============================================================

/// 二维数据点
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// K-Means 聚类结果
#[derive(Debug)]
pub struct KMeansResult {
    pub centroids: Vec<Point>,
    pub assignments: Vec<usize>,
}

/// K-Means 聚类算法
pub fn kmeans(data: &[Point], k: usize, max_iters: usize) -> KMeansResult {
    let n = data.len();
    if n == 0 || k == 0 {
        return KMeansResult {
            centroids: Vec::new(),
            assignments: Vec::new(),
        };
    }

    // 初始化：均匀选取 k 个点作为初始中心
    let mut centroids: Vec<Point> = Vec::with_capacity(k);
    for i in 0..k {
        let idx = n * i / k;
        centroids.push(data[idx]);
    }

    let mut assignments = vec![0usize; n];

    for _ in 0..max_iters {
        let mut changed = false;

        // 分配步骤
        for (i, point) in data.iter().enumerate() {
            let nearest = centroids
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.distance(point).partial_cmp(&b.distance(point)).unwrap())
                .map(|(idx, _)| idx)
                .unwrap_or(0);
            if assignments[i] != nearest {
                assignments[i] = nearest;
                changed = true;
            }
        }

        if !changed {
            break;
        }

        // 更新步骤
        let mut sums = vec![(0.0f64, 0.0f64); k];
        let mut counts = vec![0usize; k];
        for (i, point) in data.iter().enumerate() {
            let c = assignments[i];
            sums[c].0 += point.x;
            sums[c].1 += point.y;
            counts[c] += 1;
        }
        for (j, centroid) in centroids.iter_mut().enumerate() {
            if counts[j] > 0 {
                centroid.x = sums[j].0 / counts[j] as f64;
                centroid.y = sums[j].1 / counts[j] as f64;
            }
        }
    }

    KMeansResult {
        centroids,
        assignments,
    }
}

// ============================================================
// 模型评估
// ============================================================

/// k 折交叉验证的索引划分
pub fn k_fold_indices(n: usize, k: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    let fold_size = n / k;
    let mut folds = Vec::new();

    for i in 0..k {
        let start = i * fold_size;
        let end = if i == k - 1 { n } else { (i + 1) * fold_size };
        let val_indices: Vec<usize> = (start..end).collect();
        let train_indices: Vec<usize> = (0..start).chain(end..n).collect();
        folds.push((train_indices, val_indices));
    }
    folds
}

/// 计算准确率
pub fn accuracy(tp: usize, tn: usize, fp: usize, fn_: usize) -> f64 {
    let total = tp + tn + fp + fn_;
    if total == 0 {
        0.0
    } else {
        (tp + tn) as f64 / total as f64
    }
}

/// 计算精确率
pub fn precision(tp: usize, fp: usize) -> f64 {
    let denom = tp + fp;
    if denom == 0 {
        0.0
    } else {
        tp as f64 / denom as f64
    }
}

/// 计算召回率
pub fn recall(tp: usize, fn_: usize) -> f64 {
    let denom = tp + fn_;
    if denom == 0 {
        0.0
    } else {
        tp as f64 / denom as f64
    }
}

/// 计算 F1 分数
pub fn f1_score(tp: usize, fp: usize, fn_: usize) -> f64 {
    let p = precision(tp, fp);
    let r = recall(tp, fn_);
    if p + r == 0.0 {
        0.0
    } else {
        2.0 * p * r / (p + r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_std() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&data), 3.0);
        assert!((std_dev(&data) - 1.4142135).abs() < 1e-5);
    }

    #[test]
    fn test_min_max_normalize() {
        let data = vec![1.0, 5.0, 10.0];
        let normalized = min_max_normalize(&data);
        assert!((normalized[0] - 0.0).abs() < 1e-10);
        assert!((normalized[1] - 0.4444).abs() < 1e-3);
        assert!((normalized[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_zscore() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let standardized = zscore_standardize(&data);
        assert!((mean(&standardized) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_anomaly_detection() {
        let data = vec![10.0, 12.0, 11.0, 10.0, 100.0, 11.0, 9.0];
        let anomalies = detect_anomalies_zscore(&data, 2.0);
        assert!(!anomalies.is_empty());
        assert_eq!(anomalies[0].0, 4); // 100.0 是异常
    }

    #[test]
    fn test_kmeans() {
        let data = vec![
            Point::new(1.0, 2.0),
            Point::new(1.5, 1.8),
            Point::new(5.0, 8.0),
            Point::new(8.0, 8.0),
            Point::new(1.0, 0.6),
            Point::new(9.0, 11.0),
        ];
        let result = kmeans(&data, 2, 100);
        // 前三个点应属于同一簇，后三个点属于另一簇
        assert_eq!(result.assignments[0], result.assignments[1]);
        assert_eq!(result.assignments[0], result.assignments[4]);
        assert_eq!(result.assignments[2], result.assignments[3]);
        assert_eq!(result.assignments[3], result.assignments[5]);
        assert_ne!(result.assignments[0], result.assignments[3]);
    }

    #[test]
    fn test_k_fold() {
        let folds = k_fold_indices(10, 5);
        assert_eq!(folds.len(), 5);
        for (train, val) in &folds {
            assert_eq!(train.len() + val.len(), 10);
            assert_eq!(val.len(), 2);
        }
    }

    #[test]
    fn test_metrics() {
        // TP=8, FP=2, TN=85, FN=5
        assert!((accuracy(8, 85, 2, 5) - 0.93).abs() < 1e-5);
        assert!((precision(8, 2) - 0.8).abs() < 1e-5);
        assert!((recall(8, 5) - 0.6153).abs() < 1e-3);
        assert!((f1_score(8, 2, 5) - 0.6956).abs() < 1e-3);
    }
}
