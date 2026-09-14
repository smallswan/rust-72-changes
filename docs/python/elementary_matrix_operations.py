"""
矩阵初等变换
（1）交换两行（列）
（2）某一行（列）乘以一个非零常数k
（3）某一行（列）乘以一个非零常数k，加到另一行（列）上
"""
import sympy as sp
import numpy as np
from sympy import S,nsimplify,symbols


def parse_extended_pattern(text):
    """
    解析扩展格式：1/2r1, 2r1, 0.5r1, 2/3c2
    """
    pattern = r'(?:(?P<numerator>\d+)/(?P<denominator>\d+)|(?P<decimal>\d+\.\d+)|(?P<integer>\d+))(?P<letter>[rc])(?P<index>\d+)'

    matches = re.finditer(pattern, text)

    results = []
    for match in matches:
        groups = match.groupdict()

        if groups['numerator'] and groups['denominator']:
            # 分数格式
            value_type = 'fraction'
            value = f"{groups['numerator']}/{groups['denominator']}"
            numeric_value = int(groups['numerator']) / int(groups['denominator'])
        elif groups['decimal']:
            # 小数格式
            value_type = 'decimal'
            value = groups['decimal']
            numeric_value = float(groups['decimal'])
        else:
            # 整数格式
            value_type = 'integer'
            value = groups['integer']
            numeric_value = int(groups['integer'])

        results.append({
            'value': value,
            'value_type': value_type,
            'numeric_value': numeric_value,
            'letter': groups['letter'],
            'index': int(groups['index'])
        })

    return results


## r1<->r2 交换第一行和二行,c1<->c2 交换第一列和第二列
def swap_rows(matrix, r_i, r_j):
    if r_i == r_j:
        print("不能交换同一行")
        return None
    n_rows = matrix.shape[0]
    if r_i < 0 or r_j < 0 or r_i >= n_rows or r_j >= n_rows:
        print("行数超出范围")
        return None
    # 创建行的副本并交换
    row_i = matrix[r_i, :].copy()
    row_j = matrix[r_j, :].copy()

    # print(f"{row_i}")
    # print(f"{row_j}")
    # 交换行
    matrix[r_i, :] = row_j
    matrix[r_j, :] = row_i
    return matrix


def swap_cols(matrix, c_i, c_j):
    if c_i == c_j:
        print("不能交换同一列")
        return None
    n_cols = matrix.shape[1]
    if c_i < 0 or c_j < 0 or c_i >= n_cols or c_j >= n_cols:
        print("列数超出范围")
        return None
    # 创建列的副本并交换
    col_i = matrix[:, c_i].copy()
    col_j = matrix[:, c_j].copy()
    # 交换列
    matrix[:, c_i] = col_j
    matrix[:, c_j] = col_i
    return matrix


## kr1 第一行乘以k
def multiply_row(matrix, r_i, k):
    if k == 0:
        print("不能乘以零")
        return None
    n_rows = matrix.shape[0]
    if r_i < 0 or r_i >= n_rows:
        print("行数超出范围")
        return None
    k = nsimplify(k, tolerance=1e-10)

    matrix[r_i, :] *= S(str(k))
    return matrix


## kc1 第二列乘以k
def multiply_col(matrix, c_i, k):
    if k == 0:
        print("不能乘以零")
        return None
    n_cols = matrix.shape[1]
    if c_i < 0 or c_i >= n_cols:
        print("列数超出范围")
        return None
    k = nsimplify(k, tolerance=1e-10)
    matrix[:, c_i] *= S(k)
    return matrix


## r1+kr2
def add_row(matrix, r_i, k, r_j):
    n_rows = matrix.shape[0]
    if r_i < 0 or r_i >= n_rows or r_j < 0 or r_j >= n_rows:
        print("行数超出范围")
        return None
    k = nsimplify(k, tolerance=1e-10)
    matrix[r_i, :] += S(k) * matrix[r_j, :]
    return matrix


## c1+kc2
def add_col(matrix, c_i, k, c_j):
    n_cols = matrix.shape[1]
    if c_i < 0 or c_i >= n_cols or c_j < 0 or c_j >= n_cols:
        print("列数超出范围")
        return None
    k = nsimplify(k, tolerance=1e-10)
    matrix[:, c_i] += S(k) * matrix[:, c_j]
    return matrix


# sp.init_printing(precision=3, floatmode='fixed')
matrix = sp.Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
print(add_col(matrix, 0, 2, 1))

matrix = sp.Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
print(swap_rows(matrix, 0, 2))
print(swap_cols(matrix, 0, 2))

matrix = sp.Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
print(multiply_row(matrix, 0, 2))
matrix = sp.Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
print(multiply_col(matrix, 0, 2))
matrix = sp.Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
print(add_row(matrix, 0, 2, 1))

import re


## r1<->r2,c1<->c2
## kr1,kc1
## r1+kr2,c1+kc2
## 符号计算
def sign_calculate(matrix, sign):
    print(f"{sign}")
    n_rows = matrix.shape[0]
    n_cols = matrix.shape[1]
    # 解析符号sign
    ## r1<->r2,c1<->c2
    # 匹配行交换或列交换模式
    swap_pattern = r'^[rc](\d+)<->[rc](\d+)$'
    # multiply_pattern = r'^(\d*\.?\d+)([rc])(\d+)$'
    multiply_pattern = r'(?:(?P<numerator>\d+)/(?P<denominator>\d+)|(?P<decimal>\d+\.\d+)|(?P<integer>\d+))(?P<letter>[rc])(?P<index>\d+)'
    add_pattern = r'^([rc])(\d+)([+-])(\d*\.?\d*)([rc])(\d+)$'
    match = re.match(swap_pattern, sign)
    if match:
        r_i = int(match.group(1)) - 1
        r_j = int(match.group(2)) - 1
        if sign.startswith('r'):
            matrix = swap_rows(matrix, r_i, r_j)
        elif sign.startswith('c'):
            matrix = swap_cols(matrix, r_i, r_j)
        return matrix
    elif re.match(multiply_pattern, sign):
        print("multiply_pattern...")
        r_c = None
        index = None
        k = None
        parsed = parse_extended_pattern(sign)
        for item in parsed:
            print(f"值: {item['value']} ({item['value_type']}), "
                  f"字母: {item['letter']}, 索引: {item['index']}, "
                  f"数值: {item['numeric_value']}")
            k = str(item['numeric_value'])
            k = nsimplify(k, tolerance=1e-10)
            print(S(k))
            r_c = item['letter']
            index = item['index']

        # match = re.match(multiply_pattern, sign)
        # k = S(match.group(1))
        # print(f"k:{k},{match.group(1)}")

        if r_c.startswith('r'):
            matrix = multiply_row(matrix, index - 1, k)
            return matrix
        elif r_c.startswith('c'):
            matrix = multiply_col(matrix, index - 1, k)
            return matrix
    elif re.match(add_pattern, sign):
        # print("add_pattern...")
        match = re.match(add_pattern, sign)
        # print(f"{match.group(1)} {match.group(2)} {match.group(3)} {match.group(4)} {match.group(5)} {match.group(6)}")

        if match.group(4) == '':
            k = 1
        else:
            ## 整数、浮点数判断
            if re.match(r'^\d+$', match.group(4)):
                k = S(match.group(4))
            elif re.match(r'^\d*\.?\d+$', match.group(4)):
                k = S(match.group(4))
        if match.group(3) == '-':
            k = S(-k)
        if match.group(1).startswith('r'):
            matrix = add_row(matrix, int(match.group(2)) - 1, k, int(match.group(6)) - 1)
            return matrix
        elif match.group(1).startswith('c'):
            matrix = add_col(matrix, int(match.group(2)) - 1, k, int(match.group(6)) - 1)
            return matrix

    else:
        print("无法匹配符号")
    return None


matrix = sp.Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
print(sign_calculate(matrix, "r1<->r2"))
# print(sign_calculate(matrix, "r2<->r3"))
# print(sign_calculate(matrix, "c2<->c3"))
# print(sign_calculate(matrix, "c22<->c33"))
# print(sign_calculate(matrix, "mc22<->c33"))

matrix = sp.Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]])
print(sign_calculate(matrix, "10r1"))
print(sign_calculate(matrix, "0.5r1"))
print(sign_calculate(matrix, "1/3r1"))

# print(sign_calculate(matrix, "5c1"))
# print(sign_calculate(matrix, "4c2"))
#
# print(sign_calculate(matrix, "c2+c3"))
# print(sign_calculate(matrix, "c1+2c2"))
# print(sign_calculate(matrix, "c2-c3"))

matrix = sp.Matrix([[2, -1, -1, 1, 2], [1, 1, -2, 1, 4], [4, -6, 2, -2, 4], [3, 6, -9, 7, 9]])

sign_rows_str = """r1<->r2
0.5r3
r2-r3
r3-2r1
r4-3r1
0.5r2
r3+5r2
r4-3r2
r3<->r4
r4-2r3"""

sign_rows = sign_rows_str.split("\n")
for sign_row in sign_rows:
    print(sign_row)

sign_rows = ["r1<->r2", "0.5r3", "r2-r3", "r3-2r1", "r4-3r1", "0.5r2", "r3+5r2", "r4-3r2", "r3<->r4", "r4-2r3"]
for sign_row in sign_rows:
    matrix = sign_calculate(matrix, sign_row)

print(matrix)
# 转换为 NumPy 数组
numpy_array = np.array(matrix).astype(np.float64)
print(numpy_array)

matrix = sp.Matrix([[3, 1, -1, 2], [-5, 1, 3, -4], [2, 0, 1, -1], [1, -5, 3, -3]])
print(matrix.det())

sign_rows = ["c1<->c2", "r2-r1", "r4+5r1", "r2<->r3", "r3+4r2", "r4-8r2", "r4+0.5r3"]
for sign_row in sign_rows:
    matrix = sign_calculate(matrix, sign_row)

print(matrix)
# 转换为 NumPy 数组
numpy_array = np.array(matrix).astype(np.float64)
print(numpy_array)

matrix = sp.Matrix([[1, -2, 3, -1, 1], [3, -1, 5, -3, 2], [2, 1, 2, -2, 3]])
sign_rows_str = """r2-3r1
r3-2r1
r3-r2"""

sign_rows = sign_rows_str.split("\n")

numpy_array = np.array(matrix).astype(np.float64)
print(numpy_array)
for sign_row in sign_rows:
    matrix = sign_calculate(matrix, sign_row)
    numpy_array = np.array(matrix).astype(np.float64)
    print(numpy_array)

    M_simplified = nsimplify(matrix, tolerance=1e-10)
    print("\n简化后的矩阵:")
    print(M_simplified)

# 方法3：使用 S 函数（推荐）

M3 = sp.Matrix([
    [S('1/2'), S('1/3')],
    [S('3/4'), S('2/5')]
])
print("\n矩阵 M3:")
print(M3)
print(S('1/2') * M3)

matrix = sp.Matrix([[1, 2, 2, 1], [0, -3, -6, -4]])
print(S('1/3') * matrix)

matrix = sp.Matrix([[3, 1, -6, -4, 2], [2, 2, -3, -5, 3], [1, -5, -6, 8, -6]])
sign_rows = ["r1-r2", "r2-2r1", "r3-r1", "r3+r2", "0.25r2", "r1+r2"]
for sign_row in sign_rows:
    matrix = sign_calculate(matrix, sign_row)
    print(matrix)

# 测试 , 2r1, 0.5r1, 2/3c2, 5c3, 0.75r4
text = "r2+1/2r1"
parsed = parse_extended_pattern(text)

for item in parsed:
    print(f"值: {item['value']} ({item['value_type']}), "
          f"字母: {item['letter']}, 索引: {item['index']}, "
          f"数值: {item['numeric_value']}")
    k = str(item['numeric_value'])
    k = nsimplify(k, tolerance=1e-10)
    print(S(k))

matrix = sp.Matrix([[246, 427, 327], [1014, 543, 443], [-342, 721, 621]])
print(matrix.det())
sign_rows = ["c1+c2", "c1+c3", "c2-c3"]
for sign_row in sign_rows:
    matrix = sign_calculate(matrix, sign_row)
    print(matrix)

matrix = sp.Matrix([[246, 427, 327], [1014, 543, 443], [-342, 721, 621]])
# 第一列加上第二列
matrix_col_op = matrix.copy()
matrix_col_op.col_op(0, lambda x, c: x + matrix_col_op.col(1)[c])
print(matrix_col_op)
# 第一列加上第三列
matrix_col_op.col_op(0, lambda x, c: x + matrix_col_op.col(2)[c])
print(matrix_col_op)
# 第二列减去第三列
matrix_col_op.col_op(1, lambda x, c: x - matrix_col_op.col(2)[c])
print(matrix_col_op)
# 第一列除以1000，第二列除以100
matrix_col_op.col_op(0, lambda x, c: x / 1000)
print(matrix_col_op)
matrix_col_op.col_op(1, lambda x, c: x / 100)
print(matrix_col_op)

print(matrix_col_op.det())

matrix = sp.Matrix([[246, 427, 327], [1014, 543, 443], [-342, 721, 621]])
# 交换第一行和第二行
matrix.row_swap(0, 1)
print(matrix)

# 创建符号矩阵
a, b, c, d, e, f, g, h, i = symbols('a:i')
M_sym = sp.Matrix([[a, b, c],
                [d, e, f],
                [g, h, i]])

print("原始符号矩阵:")
print(M_sym)
print(M_sym.det())
# 交换行
M_sym_swapped = M_sym.copy()
M_sym_swapped.row_swap(0, 1)

print("\n交换第0行和第1行后:")
print(M_sym_swapped)

# 定义符号变量
a, b, c = sp.symbols('a b c')

# 创建2x2符号矩阵
A = sp.Matrix([[a,0, b,c], [0,a,c, b],[b,c,a,0],[c,b,0,a]])

# 计算行列式
det_A = A.det()
print("行列式为:", det_A)