//! # Rust 七十二变 —— 整合代码演示入口
//!
//! 运行 `cargo run` 查看各模块的演示输出。

use rust_72_changes::{algorithms, data_mining, graph_theory, number_theory, security};

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║          Rust 七十二变 —— 整合代码演示                    ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    demo_number_theory();
    demo_graph_theory();
    demo_algorithms();
    demo_data_mining();
    demo_security();

    println!("\n══════════════════════════════════════════════════════════");
    println!("  所有模块演示完毕！运行 `cargo test` 查看单元测试。");
    println!("══════════════════════════════════════════════════════════");
}

// ============================================================
// 第五十七章 数论
// ============================================================

fn demo_number_theory() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  第五十七章 数论");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // GCD 与 LCM
    let a = 48u64;
    let b = 18u64;
    println!("  GCD({}, {}) = {}", a, b, number_theory::gcd(a, b));
    println!("  LCM({}, {}) = {}", a, b, number_theory::lcm(a, b));

    // 扩展欧几里得
    let (g, x, y) = number_theory::ext_gcd(35, 15);
    println!("  ext_gcd(35, 15) = (gcd={}, x={}, y={})", g, x, y);

    // 模逆元
    if let Some(inv) = number_theory::mod_inverse(3, 11) {
        println!(
            "  3^(-1) mod 11 = {}  (验证: 3×{} mod 11 = {})",
            inv,
            inv,
            (3 * inv) % 11
        );
    }

    // 快速幂
    println!("  2^10 mod 1000 = {}", number_theory::mod_pow(2, 10, 1000));
    println!(
        "  3^100 mod 1000000007 = {}",
        number_theory::mod_pow(3, 100, 1_000_000_007)
    );

    // 素数判定
    let nums = vec![97, 100, 7919, 1000003];
    for n in &nums {
        println!(
            "  is_prime({}) = {} (Miller-Rabin: {})",
            n,
            number_theory::is_prime(*n),
            number_theory::miller_rabin(*n)
        );
    }

    // 埃氏筛法
    let primes = number_theory::sieve_of_eratosthenes(50);
    println!("  50 以内的素数 ({} 个): {:?}", primes.len(), primes);

    // 欧拉函数
    for n in &[1u64, 6, 10, 12, 36] {
        println!("  φ({}) = {}", n, number_theory::euler_totient(*n));
    }
    println!();
}

// ============================================================
// 第五十八章 图论
// ============================================================

fn demo_graph_theory() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  第五十八章 图论");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 构建图
    let mut g = graph_theory::Graph::new(6);
    g.add_edge(0, 1, 10);
    g.add_edge(0, 2, 3);
    g.add_edge(1, 3, 2);
    g.add_edge(2, 1, 4);
    g.add_edge(2, 3, 8);
    g.add_edge(3, 4, 9);
    g.add_edge(4, 5, 1);

    // BFS / DFS
    println!("  BFS 从 0 出发: {:?}", g.bfs(0));
    println!("  DFS 从 0 出发: {:?}", g.dfs(0));

    // BFS 最短路径
    let dist = g.bfs_shortest_path(0);
    println!("  BFS 最短距离 (无权): {:?}", dist);

    // Dijkstra 最短路径
    let dist = g.dijkstra(0);
    println!("  Dijkstra 最短距离: {:?}", dist);

    // 最小生成树 (Kruskal)
    let mut edges = vec![(0, 1, 4), (0, 2, 1), (1, 2, 2), (1, 3, 3), (2, 3, 5)];
    let mst_weight = graph_theory::kruskal_mst(4, &mut edges);
    println!("  Kruskal MST 总权重: {}", mst_weight);

    // 拓扑排序
    let dag = vec![vec![1, 2], vec![3], vec![3], vec![]];
    match graph_theory::topological_sort(4, &dag) {
        Some(order) => println!("  拓扑排序: {:?}", order),
        None => println!("  拓扑排序: 检测到环！"),
    }

    // Floyd-Warshall
    let fw_edges = vec![(0, 1, 3), (1, 2, 1), (0, 2, 10)];
    let dist = graph_theory::floyd_warshall(3, &fw_edges);
    println!("  Floyd-Warshall 全源最短距离:");
    for (i, row) in dist.iter().enumerate() {
        println!("    从 {}: {:?}", i, row);
    }
    println!();
}

// ============================================================
// 第二十六章 算法 & 第七十三章 经典问题
// ============================================================

fn demo_algorithms() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  第二十六章 算法 & 第七十三章 经典问题");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 斐波那契
    print!("  斐波那契前 15 项: ");
    for i in 0..15 {
        print!("{} ", algorithms::fibonacci(i));
    }
    println!();
    println!("  fibonacci_fast(50) = {}", algorithms::fibonacci_fast(50));

    // 汉诺塔
    let steps = algorithms::hanoi(3, "A", "C", "B");
    println!("  汉诺塔(3) 共 {} 步:", steps.len());
    for step in &steps {
        println!("    {}", step);
    }

    // 0-1 背包
    let weights = vec![2, 3, 4, 5];
    let values = vec![3, 4, 5, 6];
    let max_val = algorithms::knapsack_01(&weights, &values, 8);
    println!("  0-1 背包 (容量8): 最大价值 = {}", max_val);

    // LCS
    let s1: Vec<char> = "ABCBDAB".chars().collect();
    let s2: Vec<char> = "BDCABA".chars().collect();
    println!(
        "  LCS(\"ABCBDAB\", \"BDCABA\") = {}",
        algorithms::lcs_length(&s1, &s2)
    );

    // 编辑距离
    println!(
        "  edit_distance(\"kitten\", \"sitting\") = {}",
        algorithms::edit_distance("kitten", "sitting")
    );

    // 最长递增子序列
    let nums = vec![10i64, 9, 2, 5, 3, 7, 101, 18];
    println!("  LIS({:?}) = {}", nums, algorithms::lis_length(&nums));

    // 约瑟夫环
    println!(
        "  约瑟夫环(n=41, m=3): 幸存者编号 = {}",
        algorithms::josephus(41, 3)
    );

    // 排序
    let arr = vec![5, 2, 8, 1, 9, 3, 7, 6, 4];
    println!("  快速排序 {:?} → {:?}", arr, algorithms::quicksort(&arr));
    println!("  归并排序 {:?} → {:?}", arr, algorithms::merge_sort(&arr));

    // 二分查找
    let sorted = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!(
        "  二分查找 7 在 {:?} → 索引 {:?}",
        sorted,
        algorithms::binary_search(&sorted, &7)
    );

    // N 皇后
    for n in &[4, 5, 6, 7, 8] {
        println!("  N 皇后({}): {} 种解法", n, algorithms::n_queens_count(*n));
    }

    // 逆波兰表达式
    let rpn = ["3", "4", "2", "*", "+"];
    println!("  逆波兰 {:?} = {:?}", rpn, algorithms::eval_rpn(&rpn));
    println!();
}

// ============================================================
// 第五十五章 数据挖掘
// ============================================================

fn demo_data_mining() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  第五十五章 数据挖掘");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 统计基础
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    println!("  数据: {:?}", data);
    println!(
        "  均值 = {:.2}, 标准差 = {:.4}",
        data_mining::mean(&data),
        data_mining::std_dev(&data)
    );

    // 归一化
    let normalized = data_mining::min_max_normalize(&data);
    println!("  Min-Max 归一化: {:?}", normalized);
    let standardized = data_mining::zscore_standardize(&data);
    println!("  Z-Score 标准化: {:?}", standardized);

    // 异常检测
    let data_with_outlier = vec![10.0, 12.0, 11.0, 10.0, 100.0, 11.0, 9.0];
    println!("\n  含异常值的数据: {:?}", data_with_outlier);
    let anomalies = data_mining::detect_anomalies_zscore(&data_with_outlier, 2.0);
    println!("  Z-Score 异常检测 (阈值2.0):");
    for (idx, val, z) in &anomalies {
        println!("    索引 {}: 值={}, Z分数={:.2}", idx, val, z);
    }

    // K-Means 聚类
    let points = vec![
        data_mining::Point::new(1.0, 2.0),
        data_mining::Point::new(1.5, 1.8),
        data_mining::Point::new(1.0, 0.6),
        data_mining::Point::new(5.0, 8.0),
        data_mining::Point::new(8.0, 8.0),
        data_mining::Point::new(9.0, 11.0),
    ];
    let result = data_mining::kmeans(&points, 2, 100);
    println!("\n  K-Means 聚类 (k=2):");
    for (i, p) in points.iter().enumerate() {
        println!(
            "    点({:.1}, {:.1}) → 簇 {}",
            p.x, p.y, result.assignments[i]
        );
    }
    for (i, c) in result.centroids.iter().enumerate() {
        println!("    簇 {} 中心: ({:.2}, {:.2})", i, c.x, c.y);
    }

    // 交叉验证
    let folds = data_mining::k_fold_indices(10, 5);
    println!("\n  5 折交叉验证 (10 个样本):");
    for (i, (train, val)) in folds.iter().enumerate() {
        println!("    折 {}: 训练{}个, 验证{:?}", i + 1, train.len(), val);
    }

    // 评估指标
    println!("\n  混淆矩阵评估 (TP=8, FP=2, TN=85, FN=5):");
    println!("    准确率 = {:.4}", data_mining::accuracy(8, 85, 2, 5));
    println!("    精确率 = {:.4}", data_mining::precision(8, 2));
    println!("    召回率 = {:.4}", data_mining::recall(8, 5));
    println!("    F1 分数 = {:.4}", data_mining::f1_score(8, 2, 5));
    println!();
}

// ============================================================
// 第五十九/六十一章 安全
// ============================================================

fn demo_security() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  第五十九/六十一章 软件漏洞 & 数据安全");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 数据脱敏
    println!("  数据脱敏:");
    println!(
        "    手机号: 13812345678 → {}",
        security::mask_phone("13812345678")
    );
    println!(
        "    邮箱:   user@example.com → {}",
        security::mask_email("user@example.com")
    );
    println!(
        "    身份证: 110101199001011234 → {}",
        security::mask_id_card("110101199001011234")
    );

    // 路径遍历防护
    println!("\n  路径遍历防护:");
    let base = std::path::Path::new("/app/data");
    let inputs = ["file.txt", "../../../etc/passwd", "/etc/passwd"];
    for input in &inputs {
        println!(
            "    \"{}\" → 安全: {}",
            input,
            security::is_path_safe(base, input)
        );
    }

    // 安全运算
    println!("\n  安全运算 (整数溢出防护):");
    println!("    safe_add(1, 2) = {:?}", security::safe_add(1, 2));
    println!(
        "    safe_add(u64::MAX, 1) = {:?}",
        security::safe_add(u64::MAX, 1)
    );
    println!(
        "    saturating_add(u64::MAX, 1) = {}",
        security::saturating_add(u64::MAX, 1)
    );

    // 常量时间比较
    println!("\n  常量时间比较 (防时序攻击):");
    println!(
        "    compare(\"secret\", \"secret\") = {}",
        security::constant_time_compare(b"secret", b"secret")
    );
    println!(
        "    compare(\"secret\", \"attack\") = {}",
        security::constant_time_compare(b"secret", b"attack")
    );

    // 哈希
    println!("\n  哈希校验:");
    let data = b"Hello, Rust!";
    let fnv = security::fnv1a_hash(data);
    println!(
        "    FNV-1a(\"{}\") = {:#018x}",
        String::from_utf8_lossy(data),
        fnv
    );
    let hash = security::simple_hash(data);
    println!(
        "    simple_hash(\"{}\") = {}",
        String::from_utf8_lossy(data),
        hex_encode(&hash)
    );

    // 密码强度
    println!("\n  密码强度评估:");
    let passwords = ["123", "abcdefgh", "Abcdefgh1", "Abcdefgh1!@"];
    for pwd in &passwords {
        println!(
            "    \"{}\" → {} (分数 {}/4)",
            pwd,
            security::password_strength_label(pwd),
            security::password_strength(pwd)
        );
    }
    println!();
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
