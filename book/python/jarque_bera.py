import numpy as np
from statsmodels.stats.stattools import jarque_bera

# 生成数据：正态vs尖峰厚尾
np.random.seed(0)
normal = np.random.normal(loc=0, scale=1, size=1000)
non_normal = np.random.standard_t(df=5, size=1000)  # t分布（尖峰厚尾）

# JB检验
jb_norm, p_norm, skew_norm, kurt_norm = jarque_bera(normal)
jb_non, p_non, skew_non, kurt_non = jarque_bera(non_normal)

print("=== 正态数据 ===")
print(f"JB={jb_norm:.2f}, p={p_norm:.3f}, 偏度={skew_norm:.2f}, 峰度={kurt_norm:.2f}")
# → p>0.05，不拒绝正态

print("\n=== 非正态（t分布）===")
print(f"JB={jb_non:.2f}, p={p_non:.3f}, 偏度={skew_non:.2f}, 峰度={kurt_non:.2f}")
# → p≈0，强烈拒绝正态