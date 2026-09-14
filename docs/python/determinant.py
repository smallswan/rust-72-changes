import numpy as np
import sympy as sp
from sympy import Rational, simplify

matrix = np.array([[1, 1, -2], [5, -2, 7], [2, -5, 4]])
# 计算行列式
print(np.linalg.det(matrix))

matrix = sp.Matrix([[246, 427, 327], [1014, 543, 443], [-342, 721, 621]])
print(matrix.det())

# 定义符号变量
a, b = sp.symbols("a b")

# 定义矩阵
A = sp.Matrix([[a, b, b], [b, a, b], [b, b, a]])

# 计算行列式
det_A = A.det()
det_A
# 定义符号变量
x, y = sp.symbols("x y")

# 符号运算
expr1 = x**2 + 2 * x + 1
expr2 = (x + 1) ** 2

print(expr1)  # x**2 + 2*x + 1
print(sp.simplify(expr1 - expr2))  # 0

# 方程求解
equation = x**2 - 4 * x + 4
solutions = sp.solve(equation, x)
print(solutions)  # [2]

# 微积分
# 求导
derivative = sp.diff(x**3 + 2 * x**2 + x, x)
print(derivative)  # 3*x**2 + 4*x + 1
# 积分
integral = sp.integrate(x**2, x)
print(integral)  # x**3/3

# 极限
limit_expr = sp.limit(sp.sin(x) / x, x, 0)
print(limit_expr)  # 1

# 矩阵运算
A = sp.Matrix([[1 / 2, 2 / 3], [3 / 4, 4 / 5]])
B = sp.Matrix([[2, 0], [1, 2]])

print("A + B =", A + B)
print("A * B =", A * B)
print("A的行列式 =", A.det())
print("A的逆 =", A.inv())


A = sp.Matrix(
    [
        [Rational(1, 2), Rational(1, 3), Rational(1, 4)],
        [Rational(1, 3), Rational(1, 4), Rational(1, 5)],
        [Rational(1, 4), Rational(1, 5), Rational(1, 6)],
    ]
)

B = sp.Matrix(
    [
        [Rational(1, 2), Rational(1, 3), Rational(1, 4)],
        [Rational(2, 3), Rational(1, 4), Rational(1, 5)],
        [Rational(3, 4), Rational(3, 5), Rational(5, 6)],
    ]
)

print("A + B =", A + B)
print("A * B =", A * B)
print("A的行列式 =", A.det())
print("A的逆 =", A.inv())

A = sp.Matrix([[2, -1], [-1, 2]])
print(A**5)
# 已知对称矩阵A，求正交矩阵P, 使得$$P^{−1}AP=Λ$$
A = sp.Matrix([[2, -1], [-1, 2]])
# 1. 求特征值
eigenvals = A.eigenvals()  # 返回 {特征值: 重数}
print("特征值：", eigenvals)  # 输出 {1: 1, 3: 1}

# 2. 求特征向量（已正交）
eigenvects = A.eigenvects()  # 返回 [(特征值, 重数, [特征向量]), ...]
alpha1 = eigenvects[0][2][0]  # λ=1对应的特征向量 [1,1]
alpha2 = eigenvects[1][2][0]  # λ=3对应的特征向量 [1,-1]

# 3. 特征向量单位化
gamma1 = alpha1 / alpha1.norm()
gamma2 = alpha2 / alpha2.norm()

# 4. 构造正交矩阵P
P = sp.Matrix.hstack(gamma1, gamma2)
print("正交矩阵P：")
sp.pprint(P)

# 验证 P^TAP = Λ
Lambda = P.T @ A @ P
print("验证 P^TAP = Λ：")
sp.pprint(Lambda)

A = sp.Matrix([[0, -1, 1], [-1, 0, 1], [1, 1, 0]])
# 1. 求特征值
eigenvals = A.eigenvals()  # 返回 {特征值: 重数}
print("特征值：", eigenvals)
# 2. 求特征向量（已正交）
eigenvects = A.eigenvects()
print("特征向量：", eigenvects)
# 如果特征值重数大于1，求出全部特征向量
# 3. 遍历结果并解析
for eig_val, multiplicity, eig_vecs in eigenvects:
    print(f"特征值: {simplify(eig_val)}")
    print(f"代数重数: {multiplicity}")
    print("对应的特征向量:")
    for vec in eig_vecs:
        # 简化特征向量并输出
        print(f"  {simplify(vec)}")
        print("  对应的单位向量:", simplify(vec / vec.norm()))


# 1. 定义对称矩阵A
A = sp.Matrix([[0, -1, 1], [-1, 0, 1], [1, 1, 0]])

# 2. 计算特征值和特征向量
eigenvects = A.eigenvects()
# 整理特征值和对应的特征向量列表
eig_info = []
for val, multi, vecs in eigenvects:
    eig_info.append((val, vecs))

# 3. 对特征向量进行正交化 + 单位化（实对称矩阵不同特征值的特征向量已正交）
orthonormal_vecs = []
for eig_val, eig_vecs in eig_info:
    # 对每个特征值对应的特征向量组做施密特正交化
    orthogonal_vecs = sp.GramSchmidt(
        eig_vecs, orthonormal=True
    )  # normalize=True 直接单位化
    orthonormal_vecs.extend(orthogonal_vecs)

# 4. 构造正交矩阵P（按列排列正交单位化的特征向量）
P = sp.Matrix.hstack(*orthonormal_vecs)

# 5. 验证：计算P^T * A * P（正交矩阵P^{-1}=P^T），应得到对角矩阵Λ
Lambda = P.T @ A @ P
# 简化对角矩阵（消除符号计算的微小误差）
Lambda = Lambda.applyfunc(sp.simplify)

# 输出结果
print("=== 正交矩阵 P ===")
sp.pprint(P)
print("\n=== 对角矩阵 Λ (P^TAP 的结果) ===")
sp.pprint(Lambda)
print("\n=== 验证 P^T * P = 单位矩阵（正交性）===")
sp.pprint(sp.simplify(P.T @ P))


# 特征变量含虚数
A = sp.Matrix([[0, 2], [-2, 0]])
eigenvects = A.eigenvects()
print("特征值和特征向量：", eigenvects)

## 线性方程组求解
# 1. 系数矩阵 A
A = np.array([[-9, -6, 8], [-6, 3, 4], [-8, 10, -6]], dtype=float)

# 2. 结果向量 b（右边的值）
b = np.array([67, 35, 18], dtype=float)

# 3. 直接求解 Ax = b
x = np.linalg.solve(A, b)

# 输出结果
print("解 x =", np.round(x, 4))

A = sp.Matrix([[1, 3, 5], [2, 5, 1], [2, 3, 8]])
b = sp.Matrix([[10, 8, 3]])
x = sp.linsolve((A, b))
print("解 x =", x)

# 矩阵幂运算
A = sp.Matrix([[1, 1, 2], [2, 2, 4], [3, 3, 6]])
print("A^10 =", A**10)
