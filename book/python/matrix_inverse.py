import numpy as np
def matrix_inverse(matrix):
    """
    计算矩阵的逆矩阵

    参数:
    matrix: 待求逆的方阵

    返回:
    矩阵的逆矩阵，如果矩阵不可逆则返回None
    """
    try:
        # 计算矩阵的行列式，判断是否可逆
        det = np.linalg.det(matrix)
        print("行列式:", det)
        if abs(det) < 1e-10:  # 考虑浮点数精度问题
            print("矩阵不可逆（行列式为零）")
            return None

        # 计算逆矩阵
        inv_matrix = np.linalg.inv(matrix)
        return inv_matrix
    except np.linalg.LinAlgError:
        print("计算逆矩阵时发生错误")
        return None
    except ValueError:
        print("输入必须是方阵")
        return None

def adjugate_matrix(matrix):
    """计算矩阵的伴随矩阵"""
    n = matrix.shape[0]
    if matrix.shape != (n, n):
        raise ValueError("输入必须是方阵")

    # 计算代数余子式矩阵
    cofactor = np.zeros_like(matrix, dtype=np.float64)
    for i in range(n):
        for j in range(n):
            # 计算余子式：去除第i行第j列后的子矩阵的行列式
            minor = np.delete(np.delete(matrix, i, axis=0), j, axis=1)
            minor_det = np.linalg.det(minor)
            # 计算代数余子式：(-1)^(i+j) * 余子式
            cofactor[i, j] = ((-1) ** (i + j)) * minor_det

    # 伴随矩阵是代数余子式矩阵的转置
    adj = cofactor.T
    return adj
# 示例用法
if __name__ == "__main__":
    # 定义一个3x3的方阵
    A = np.array([[1, 0, 1],
                  [2, 1, 0],
                  [-3, 2, -5]])

    # 计算逆矩阵
    A_inv = matrix_inverse(A)
    A_adj = adjugate_matrix(A)

    if A_inv is not None:
        print("原矩阵:")
        print(A)
        print("矩阵的秩：",np.linalg.matrix_rank(A))
        print("\n逆矩阵:")
        print(A_inv)
        print("\n伴随矩阵:")
        print(A_adj)

        # 验证：原矩阵与逆矩阵的乘积应该是单位矩阵
        print("\n验证（原矩阵 × 逆矩阵）:")
        print(np.dot(A, A_inv).round(6))  # 四舍五入以消除浮点数误差

    A = np.array([[1,0,0],[2,2,0],[3,4,5]])
    A_inv = matrix_inverse(A)
    A_adj = adjugate_matrix(A)

    A_inv_adj = adjugate_matrix(A_inv)
    print(A_inv_adj)
    A_adj_inv = matrix_inverse(A_adj)
    print(A_adj_inv)
    print(A_inv_adj == A_adj_inv)
    print(np.allclose(A_inv_adj, A_adj_inv))

    product = np.dot(A, A_inv)
    print("\n原矩阵与逆矩阵的乘积:")
    print(product)

    # 与单位矩阵比较
    I = np.eye(3)
    print("\n是否接近单位矩阵:", np.allclose(product, I, atol=1e-10))
    
    A = np.array([[1,1,1],[2,1,0],[1,-1,0]])
    A_inv = matrix_inverse(A)
    print(A_inv)
    
    
import numpy as np
import matplotlib.pyplot as plt

# 创建矩阵
matrix = np.array([
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12]
])

# 可视化矩阵
plt.figure(figsize=(8, 6))
# 方法1: 设置中文字体
plt.rcParams['font.sans-serif'] = ['SimHei', 'Microsoft YaHei', 'SimSun', 'FangSong', 'KaiTi']
plt.imshow(matrix, cmap='viridis', interpolation='nearest')
plt.colorbar(label='数值')
plt.title('矩阵可视化')
plt.xlabel('列索引')
plt.ylabel('行索引')

# 在每个单元格显示数值
for i in range(matrix.shape[0]):
    for j in range(matrix.shape[1]):
        plt.text(j, i, f'{matrix[i, j]}', 
                ha='center', va='center', 
                color='white' if matrix[i, j] > 6 else 'black')

plt.show()


matrix = np.array([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
det = np.linalg.det(matrix)
print("行列式:", det)


