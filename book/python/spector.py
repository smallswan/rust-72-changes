import numpy as np
import matplotlib.pyplot as plt
import statsmodels.api as sm
from statsmodels.stats.stattools import jarque_bera
from scipy import stats

# 1. 加载 spector 数据集
data = sm.datasets.spector.load_pandas().data
print("=== Spector 数据集前5行 ===")
print(data.head())
print("\n=== 描述性统计 ===")
print(data.describe())

# 2. 提取连续变量：GPA（最适合正态性检验）
gpa = data['GPA'].values

# 3. Jarque-Bera 正态性检验（你之前用的方法）
jb_stat, jb_pval, skew, kurt = jarque_bera(gpa)

print("\n===== GPA 正态性检验（JB检验）=====")
print(f"JB统计量 = {jb_stat:.4f}")
print(f"p值      = {jb_pval:.4f}")
print(f"偏度     = {skew:.4f}")
print(f"峰度     = {kurt:.4f}")

# 4. 检验结论
alpha = 0.05
if jb_pval < alpha:
    print("\n✅ 结论：p < 0.05，拒绝原假设 → GPA **不服从正态分布**")
else:
    print("\n✅ 结论：p ≥ 0.05，不拒绝原假设 → GPA 近似正态分布")

# 5. 可视化：直方图 + 密度曲线 + QQ图
plt.rcParams['font.sans-serif'] = ['SimHei']
plt.rcParams['axes.unicode_minus'] = False

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

# 直方图+密度
ax1.hist(gpa, bins=10, density=True, alpha=0.7, label='GPA分布')
kde = stats.gaussian_kde(gpa)
x_range = np.linspace(gpa.min(), gpa.max(), 200)
ax1.plot(x_range, kde(x_range), 'r', linewidth=2, label='核密度')
ax1.set_title('GPA 分布直方图+密度')
ax1.legend()

# QQ图
stats.probplot(gpa, plot=ax2)
ax2.set_title('GPA QQ正态检验图')

plt.tight_layout()
plt.show()