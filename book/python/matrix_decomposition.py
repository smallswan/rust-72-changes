import numpy as np
from numpy.linalg import svd
from scipy.linalg import lu

# 定义矩阵
A = np.array([[-3, 8, -6], [-6, 2, -2], [-4, -10, -9], [-7, 3, -3]])

# NumPy 直接 QR 分解
Q, R = np.linalg.qr(A)

print("==== NumPy 内置 QR 分解 ====")
print("Q：")
print(np.round(Q, 4))
print("\nR：")
print(np.round(R, 4))
print("\n验证 Q^T Q = 单位矩阵：")
print(np.round(Q.T @ Q, 4))

# LU 分解
# 你的矩阵
A = np.array([[-5, 2, -6], [5, 5, 10], [5, -6, 5]], dtype=float)

# 分解：P 是置换矩阵，A = P^T L U
P, L, U = lu(A)

print("==== SciPy LU 分解 ====")
print("P 置换矩阵：")
print(P)
print("\nL 下三角：")
print(np.round(L, 4))
print("\nU 上三角：")
print(np.round(U, 4))
print("\n验证 A = P.T @ L @ U：")
print(np.round(P.T @ L @ U, 4))

# SVD 分解
A = np.array([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
U, s, V = svd(A)
print("U:", U)
print("S:", np.diag(s))
print("V^T:", V)
print("A:", A)
print("A = USV^T:", np.dot(U, np.dot(np.diag(s), V)))
