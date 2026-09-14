import re
import sys

import numpy as np
import sympy as sp
from PyQt5.QtCore import Qt
from PyQt5.QtWidgets import (
    QApplication,
    QGridLayout,
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QMainWindow,
    QPushButton,
    QSpinBox,
    QTextEdit,
    QVBoxLayout,
    QWidget,
)


class MatrixInputWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.initUI()

    def initUI(self):
        self.setWindowTitle("矩阵计算器")
        self.setGeometry(100, 100, 600, 800)

        central_widget = QWidget()
        self.setCentralWidget(central_widget)

        layout = QVBoxLayout()

        # 矩阵A
        matrix_A = QLabel("矩阵A")
        matrix_A.setAlignment(Qt.AlignCenter)
        layout.addWidget(matrix_A)

        self.matrix_layout = QGridLayout()
        self.entries = []
        self.create_matrix_inputs("A", 3, 3)
        layout.addLayout(self.matrix_layout)

        button_layout = self.create_control_buttons("A")
        layout.addLayout(button_layout)

        # 矩阵B
        matrix_B = QLabel("矩阵B")
        matrix_B.setAlignment(Qt.AlignCenter)
        layout.addWidget(matrix_B)

        self.matrixB_layout = QGridLayout()
        self.entriesB = []
        self.create_matrix_inputs("B", 3, 3)
        layout.addLayout(self.matrixB_layout)

        B_button_layout = self.create_control_buttons("B")
        layout.addLayout(B_button_layout)

        # 矩阵运算
        self.calc_layout = QHBoxLayout()

        self.calc_add_button = QPushButton("A+B")
        self.calc_add_button.clicked.connect(self.add_matrix)
        self.calc_layout.addWidget(self.calc_add_button)

        self.calc_sub_button = QPushButton("A-B")
        self.calc_sub_button.clicked.connect(self.sub_matrix)
        self.calc_layout.addWidget(self.calc_sub_button)

        self.calc_BsubA_button = QPushButton("B-A")
        self.calc_BsubA_button.clicked.connect(self.sub_BA_matrix)
        self.calc_layout.addWidget(self.calc_BsubA_button)

        self.calc_mul_button = QPushButton("AB")
        self.calc_mul_button.clicked.connect(self.mul_matrix)
        self.calc_layout.addWidget(self.calc_mul_button)

        self.calc_mul_button2 = QPushButton("BA")
        self.calc_mul_button2.clicked.connect(self.mul_BA_matrix)
        self.calc_layout.addWidget(self.calc_mul_button2)

        self.calc_solve_button = QPushButton("Ax=b")
        self.calc_solve_button.clicked.connect(self.solve_linear_system)
        self.calc_layout.addWidget(self.calc_solve_button)

        layout.addLayout(self.calc_layout)

        # 结果显示
        self.result_text = QTextEdit()
        self.result_text.setReadOnly(True)
        layout.addWidget(QLabel("结果:"))
        layout.addWidget(self.result_text)

        central_widget.setLayout(layout)

    def create_matrix_inputs(self, matrix, rows, cols):
        if matrix == "A":
            layout = self.matrix_layout
            entries = self.entries
        else:
            layout = self.matrixB_layout
            entries = self.entriesB

        # 清除现有输入框
        while layout.count():
            child = layout.takeAt(0)
            if child.widget():
                child.widget().deleteLater()

        entries.clear()
        for i in range(rows):
            row_entries = []
            for j in range(cols):
                entry = QLineEdit()
                entry.setText("0")
                entry.setMaximumWidth(60)
                layout.addWidget(entry, i, j)
                row_entries.append(entry)
            entries.append(row_entries)

    def create_control_buttons(self, matrix):
        layout = QHBoxLayout()

        get_btn = QPushButton("矩阵信息")
        get_btn.clicked.connect(lambda: self.get_matrix(matrix))
        layout.addWidget(get_btn)

        identity_btn = QPushButton("单位矩阵")
        identity_btn.clicked.connect(lambda: self.identity_matrix(matrix))
        layout.addWidget(identity_btn)

        power_btn = QPushButton("幂")
        power_btn.clicked.connect(lambda: self.power_matrix(matrix))
        layout.addWidget(power_btn)

        power_spin = QSpinBox()
        power_spin.setFixedSize(50, 30)
        power_spin.setRange(2, 10)
        power_spin.setValue(2)
        setattr(self, f"power_spin_{matrix}", power_spin)

        power_spin.valueChanged.connect(lambda: self.power_matrix(matrix))
        layout.addWidget(power_spin)

        rows_label = QLabel("行数:")
        rows_label.setAlignment(Qt.AlignRight)
        layout.addWidget(rows_label)

        rows_spin = QSpinBox()
        rows_spin.setFixedSize(50, 30)
        rows_spin.setRange(1, 9)
        rows_spin.setValue(3)
        rows_spin.valueChanged.connect(lambda: self.update_matrix_size(matrix))
        setattr(self, f"rows_spin_{matrix}", rows_spin)
        layout.addWidget(rows_spin)

        cols_label = QLabel("列数:")
        cols_label.setAlignment(Qt.AlignRight)
        layout.addWidget(cols_label)

        cols_spin = QSpinBox()
        cols_spin.setFixedSize(50, 30)
        cols_spin.setRange(1, 9)
        cols_spin.setValue(3)
        cols_spin.valueChanged.connect(lambda: self.update_matrix_size(matrix))
        setattr(self, f"cols_spin_{matrix}", cols_spin)
        layout.addWidget(cols_spin)

        clear_btn = QPushButton("清空")
        clear_btn.clicked.connect(lambda: self.clear_matrix(matrix))
        layout.addWidget(clear_btn)

        return layout

    def update_matrix_size(self, matrix):
        rows_spin = getattr(self, f"rows_spin_{matrix}")
        cols_spin = getattr(self, f"cols_spin_{matrix}")
        rows = rows_spin.value()
        cols = cols_spin.value()
        self.create_matrix_inputs(matrix, rows, cols)

    def get_entries_and_shape(self, matrix):
        if matrix == "A":
            entries = self.entries
            rows = self.rows_spin_A.value()
            cols = self.cols_spin_A.value()
        else:
            entries = self.entriesB
            rows = self.rows_spin_B.value()
            cols = self.cols_spin_B.value()
        return entries, rows, cols

    # ── 统一辅助：从输入框读取矩阵，返回 sp.Matrix（精确有理数）──────────────
    def _parse_sp_matrix(self, matrix_name):
        """
        读取矩阵输入框，返回 (sp.Matrix, rows, cols)。
        支持整数、小数（自动转为精确有理数）、分数（如 1/3）。
        若输入非法，抛出 ValueError。
        """
        entries, rows, cols = self.get_entries_and_shape(matrix_name)
        data = []
        for i in range(rows):
            row = []
            for j in range(cols):
                val = entries[i][j].text().strip()
                if not SafeNumberConverter.is_number(val):
                    raise ValueError(f"矩阵{matrix_name}[{i+1}][{j+1}] 不是有效数字：{val}\n支持格式：整数、小数、分数（如 1/3）")
                row.append(SafeNumberConverter.to_rational(val))
            data.append(row)
        return sp.Matrix(data), rows, cols

    def get_matrix(self, matrix):
        try:
            sp_matrix, rows, cols = self._parse_sp_matrix(matrix)

            is_square = (rows == cols)
            rank = sp_matrix.rank()
            transpose_matrix = sp_matrix.T
            is_symmetric = (is_square and sp_matrix == transpose_matrix)

            det = sp_matrix.det() if is_square else None
            sp_matrix_adj = None
            sp_matrix_inv = None
            eigen_text = ""

            if is_square:
                sp_matrix_adj = sp_matrix.adjugate()
                if det != 0:
                    sp_matrix_inv = sp_matrix.inv()

                # 特征值 / 特征向量
                eigenvals = sp_matrix.eigenvals()
                eigenvects = sp_matrix.eigenvects()
                trace = sp_matrix.trace()

                lines = []
                idx = 1
                for eigenval, multiplicity, eigenvectors in eigenvects:
                    lines.append(f"  λ{idx} = {eigenval}  (代数重数={multiplicity}，几何重数={len(eigenvectors)})")
                    for k, vec in enumerate(eigenvectors):
                        lines.append(f"    特征向量{k+1}: {list(vec)}")
                    idx += 1
                eigen_text = "\n".join(lines) if lines else "  （无）"

                result_text = (
                    f"矩阵{matrix} ({rows}×{cols})：\n"
                    f"{self._format_sp_matrix(sp_matrix)}\n\n"
                    f"是否为方阵: {is_square}  | 是否为对称矩阵: {is_symmetric}\n"
                    f"秩: {rank}\n"
                    f"行列式: {det}  | 迹: {trace}  | 是否可逆: {det != 0}\n\n"
                    f"转置矩阵:\n{self._format_sp_matrix(transpose_matrix)}\n\n"
                    f"伴随矩阵:\n{self._format_sp_matrix(sp_matrix_adj)}\n\n"
                    f"逆矩阵:\n{self._format_sp_matrix(sp_matrix_inv) if sp_matrix_inv is not None else '  不可逆（行列式为0）'}\n\n"
                    f"特征值 / 特征向量:\n{eigen_text}\n"
                )
            else:
                result_text = (
                    f"矩阵{matrix} ({rows}×{cols})：\n"
                    f"{self._format_sp_matrix(sp_matrix)}\n\n"
                    f"是否为方阵: {is_square}\n"
                    f"秩: {rank}\n\n"
                    f"转置矩阵:\n{self._format_sp_matrix(transpose_matrix)}\n"
                )

            self.result_text.setText(result_text)
        except Exception as e:
            self.result_text.setText(f"错误: {str(e)}")

    def power_matrix(self, matrix):
        entries, rows, cols = self.get_entries_and_shape(matrix)
        if rows != cols:
            self.result_text.setText("输入必须是方阵")
            return
        try:
            sp_matrix, _, _ = self._parse_sp_matrix(matrix)
            power_spin = getattr(self, f"power_spin_{matrix}")
            n = power_spin.value()
            power_result = sp_matrix ** n
            self.result_text.setText(
                f"矩阵{matrix}^{n}：\n{self._format_sp_matrix(power_result)}"
            )
        except Exception as e:
            self.result_text.setText(f"错误: {str(e)}")

    def identity_matrix(self, matrix):
        entries, rows, cols = self.get_entries_and_shape(matrix)
        if rows != cols:
            self.result_text.setText("错误: 矩阵必须是方阵！")
            return
        for i in range(rows):
            for j in range(cols):
                entries[i][j].setText("1" if i == j else "0")

    def clear_matrix(self, matrix):
        self.result_text.setText("")
        entries, rows, cols = self.get_entries_and_shape(matrix)
        for i in range(rows):
            for j in range(cols):
                entries[i][j].setText("0")

    # 矩阵加法
    def add_matrix(self):
        try:
            if (
                self.rows_spin_A.value() != self.rows_spin_B.value()
                or self.cols_spin_A.value() != self.cols_spin_B.value()
            ):
                self.result_text.setText("错误: 矩阵的行数和列数必须相同！")
                return
            A, _, _ = self._parse_sp_matrix("A")
            B, _, _ = self._parse_sp_matrix("B")
            result = A + B
            self.result_text.setText(f"A + B =\n{self._format_sp_matrix(result)}")
        except Exception as e:
            self.result_text.setText(f"错误: {str(e)}")

    def sub_matrix(self):
        try:
            if (
                self.rows_spin_A.value() != self.rows_spin_B.value()
                or self.cols_spin_A.value() != self.cols_spin_B.value()
            ):
                self.result_text.setText("错误: 矩阵的行数和列数必须相同！")
                return
            A, _, _ = self._parse_sp_matrix("A")
            B, _, _ = self._parse_sp_matrix("B")
            result = A - B
            self.result_text.setText(f"A - B =\n{self._format_sp_matrix(result)}")
        except Exception as e:
            self.result_text.setText(f"错误: {str(e)}")

    def sub_BA_matrix(self):
        try:
            if (
                self.rows_spin_A.value() != self.rows_spin_B.value()
                or self.cols_spin_A.value() != self.cols_spin_B.value()
            ):
                self.result_text.setText("错误: 矩阵的行数和列数必须相同！")
                return
            A, _, _ = self._parse_sp_matrix("A")
            B, _, _ = self._parse_sp_matrix("B")
            result = B - A
            self.result_text.setText(f"B - A =\n{self._format_sp_matrix(result)}")
        except Exception as e:
            self.result_text.setText(f"错误: {str(e)}")

    def mul_matrix(self):
        try:
            if self.cols_spin_A.value() != self.rows_spin_B.value():
                self.result_text.setText("错误: 矩阵A的列数必须等于矩阵B的行数！")
                return
            A, _, _ = self._parse_sp_matrix("A")
            B, _, _ = self._parse_sp_matrix("B")
            result = A * B
            self.result_text.setText(f"AB =\n{self._format_sp_matrix(result)}")
        except Exception as e:
            self.result_text.setText(f"错误: {str(e)}")

    def mul_BA_matrix(self):
        try:
            if self.cols_spin_B.value() != self.rows_spin_A.value():
                self.result_text.setText("错误: 矩阵B的列数必须等于矩阵A的行数！")
                return
            A, _, _ = self._parse_sp_matrix("A")
            B, _, _ = self._parse_sp_matrix("B")
            result = B * A
            self.result_text.setText(f"BA =\n{self._format_sp_matrix(result)}")
        except Exception as e:
            self.result_text.setText(f"错误: {str(e)}")

    def solve_linear_system(self):
        """
        求解线性方程组 Ax = b
        矩阵A：系数矩阵（m × n）
        矩阵B的第一列：常数向量 b（m × 1）
        """
        try:
            rows_A = self.rows_spin_A.value()
            cols_A = self.cols_spin_A.value()
            rows_B = self.rows_spin_B.value()

            # 验证 b 的行数与 A 的行数一致
            if rows_A != rows_B:
                self.result_text.setText(
                    f"错误: 矩阵A的行数({rows_A})必须等于矩阵B的行数({rows_B})！\n"
                    "提示: 矩阵B的第一列用作常数向量 b"
                )
                return

            converter = SafeNumberConverter()

            # 读取系数矩阵 A（精确分数形式）
            A_data = []
            for i in range(rows_A):
                row = []
                for j in range(cols_A):
                    val = self.entries[i][j].text().strip()
                    if not SafeNumberConverter.is_number(val):
                        self.result_text.setText(f"错误: 矩阵A[{i+1}][{j+1}] 不是有效数字：{val}")
                        return
                    row.append(sp.Rational(val))
                A_data.append(row)

            # 读取常数向量 b（取矩阵B的第一列）
            b_data = []
            for i in range(rows_B):
                val = self.entriesB[i][0].text().strip()
                if not SafeNumberConverter.is_number(val):
                    self.result_text.setText(f"错误: 向量b[{i+1}] 不是有效数字：{val}")
                    return
                b_data.append(sp.Rational(val))

            A = sp.Matrix(A_data)
            b = sp.Matrix(b_data)

            m, n = A.shape  # m 个方程，n 个未知数

            # 构造增广矩阵 [A|b]
            Ab = A.row_join(b)

            rank_A  = A.rank()
            rank_Ab = Ab.rank()

            output = []
            output.append("=" * 50)
            output.append("  线性方程组  Ax = b  求解")
            output.append("=" * 50)
            output.append(f"\n系数矩阵 A ({m}×{n})：")
            output.append(self._format_sp_matrix(A))
            output.append(f"\n常数向量 b ({m}×1)：")
            output.append(self._format_sp_vector(b))
            output.append(f"\nrank(A) = {rank_A}，rank([A|b]) = {rank_Ab}，未知数个数 n = {n}")

            # ── 判断解的情况 ──────────────────────────────────
            if rank_Ab > rank_A:
                # 无解
                output.append("\n【结论】方程组 无解（不相容）")
                output.append("原因: rank(A) ≠ rank([A|b])，增广矩阵秩更大，矛盾方程存在。")

            elif rank_A == n:
                # 唯一解
                output.append("\n【结论】方程组有 唯一解")
                x = A.solve(b)
                output.append("\n解向量 x：")
                for idx, xi in enumerate(x):
                    output.append(f"  x{idx+1} = {xi}")

                # numpy 数值验证
                try:
                    A_np = np.array(A_data, dtype=float)
                    b_np = np.array(b_data, dtype=float)
                    x_np = np.linalg.solve(A_np, b_np)
                    output.append("\n数值验证（numpy）：")
                    for idx, xi in enumerate(x_np):
                        output.append(f"  x{idx+1} ≈ {xi:.6g}")
                    residual = np.linalg.norm(A_np @ x_np - b_np)
                    output.append(f"  残差 ‖Ax-b‖ = {residual:.2e}")
                except Exception:
                    pass

            else:
                # 无穷多解 —— 给出通解
                free_count = n - rank_A
                output.append(f"\n【结论】方程组有 无穷多解（{free_count} 个自由变量）")

                # 齐次方程组的基础解系
                null_space = A.nullspace()   # 每个向量是一个基础解

                # 特解（最小范数意义，用 sympy 的最小二乘 / 直接求一个特解）
                # 用增广矩阵行最简形提取一个特解
                particular = self._find_particular_solution(A, b, n)

                output.append("\n齐次方程组 Ax=0 的基础解系：")
                for k, vec in enumerate(null_space):
                    output.append(f"  ξ{k+1} = {[str(v) for v in vec]}")

                output.append("\n非齐次方程组的一个特解 η*：")
                output.append(f"  η* = {[str(v) for v in particular]}")

                output.append("\n通解（x = η* + c₁ξ₁ + c₂ξ₂ + ...，cᵢ为任意常数）：")
                terms = ["η*"]
                for k in range(len(null_space)):
                    terms.append(f"c{k+1}·ξ{k+1}")
                output.append("  x = " + " + ".join(terms))

                # 展开通解每个分量
                output.append("\n各分量通解：")
                for i in range(n):
                    expr = str(particular[i])
                    for k, vec in enumerate(null_space):
                        coeff = vec[i]
                        if coeff != 0:
                            expr += f" + c{k+1}·({coeff})"
                    output.append(f"  x{i+1} = {expr}")

            self.result_text.setText("\n".join(output))

        except Exception as e:
            import traceback
            self.result_text.setText(f"求解错误: {str(e)}\n{traceback.format_exc()}")

    def _find_particular_solution(self, A, b, n):
        """利用增广矩阵行最简形求一个特解（自由变量取0）"""
        Ab = A.row_join(b)
        rref, pivots = Ab.rref()
        # 自由变量取 0，主变量从方程中读出
        x = [sp.Integer(0)] * n
        for row_idx, col_idx in enumerate(pivots):
            if col_idx < n:   # 排除增广列
                x[col_idx] = rref[row_idx, n]
        return x

    def _format_sp_matrix(self, M):
        """格式化 sympy 矩阵为对齐字符串"""
        rows = M.tolist()
        lines = []
        for row in rows:
            lines.append("  [ " + "  ".join(str(v).rjust(6) for v in row) + " ]")
        return "\n".join(lines)

    def _format_sp_vector(self, v):
        """格式化 sympy 列向量为字符串"""
        items = [v[i] for i in range(v.rows)]
        return "  [ " + "  ".join(str(x).rjust(6) for x in items) + " ]"


class SafeNumberConverter:
    """
    安全数字转换工具类
    """

    @staticmethod
    def is_number(s: str) -> bool:
        """检查字符串是否为数字（整数、小数、分数）"""
        return (
            SafeNumberConverter.is_integer(s)
            or SafeNumberConverter.is_float(s)
            or SafeNumberConverter.is_fraction(s)
        )

    @staticmethod
    def is_integer(s: str) -> bool:
        """检查字符串是否为整数"""
        return bool(re.match(r"^-?\d+$", s.strip())) if s else False

    @staticmethod
    def is_float(s: str) -> bool:
        """检查字符串是否为小数"""
        s = s.strip() if s else ""
        pattern = r"^-?\d+\.\d*$|^-?\d*\.\d+$|^-?\d+(?:\.\d*)?[eE][+-]?\d+$"
        return bool(re.match(pattern, s))

    @staticmethod
    def is_fraction(s: str) -> bool:
        """检查字符串是否为分数"""
        s = s.strip() if s else ""
        return bool(re.match(r"^(-?\d+)\s*/\s*(-?\d+)$", s))

    @staticmethod
    def to_rational(s: str) -> "sp.Rational":
        """
        将字符串精确转换为 sympy.Rational：
          - 整数  → sp.Integer
          - 小数  → sp.Rational（精确，如 "0.1" → 1/10）
          - 分数  → sp.Rational（如 "1/3" → 1/3）
        无法识别时抛出 ValueError。
        """
        s = s.strip()
        if SafeNumberConverter.is_integer(s):
            return sp.Integer(int(s))
        if SafeNumberConverter.is_fraction(s):
            m = re.match(r"^(-?\d+)\s*/\s*(-?\d+)$", s)
            num, den = int(m.group(1)), int(m.group(2))
            if den == 0:
                raise ValueError(f"分母不能为零：{s}")
            return sp.Rational(num, den)
        if SafeNumberConverter.is_float(s):
            # 用 sympy 精确解析小数字符串，避免 float 精度丢失
            return sp.Rational(s)
        raise ValueError(f"无法识别的数字格式：{s}")

    @staticmethod
    def convert(s: str, default=None):
        """
        安全转换字符串为 Python 数字（向后兼容，返回 int/float/Fraction）。
        新代码请优先使用 to_rational。
        """
        if not s or not isinstance(s, str):
            return default

        s = s.strip()

        if SafeNumberConverter.is_integer(s):
            try:
                return int(s)
            except (ValueError, OverflowError):
                pass

        if SafeNumberConverter.is_float(s):
            try:
                return float(s)
            except (ValueError, OverflowError):
                pass

        if SafeNumberConverter.is_fraction(s):
            try:
                match = re.match(r"^(-?\d+)\s*/\s*(-?\d+)$", s)
                numerator = int(match.group(1))
                denominator = int(match.group(2))
                if denominator != 0:
                    from fractions import Fraction
                    return Fraction(numerator, denominator)
            except (ValueError, ZeroDivisionError, AttributeError):
                pass

        return default

    @staticmethod
    def convert_batch(strings: list, default=None):
        """批量转换字符串列表"""
        return [SafeNumberConverter.convert(s, default) for s in strings]


if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = MatrixInputWindow()
    window.show()
    sys.exit(app.exec_())
