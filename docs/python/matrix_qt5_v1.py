import re
import sys

import numpy as np
import sympy as sp
from PyQt5.QtCore import QEasingCurve, QPropertyAnimation, Qt, pyqtProperty
from PyQt5.QtGui import QColor, QFont, QIcon, QPalette
from PyQt5.QtWidgets import (
    QApplication,
    QFrame,
    QGridLayout,
    QGroupBox,
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QMainWindow,
    QPushButton,
    QScrollArea,
    QSizePolicy,
    QSpinBox,
    QSplitter,
    QTextEdit,
    QVBoxLayout,
    QWidget,
)


class AnimatedButton(QPushButton):
    """带动画反馈的主色按钮（亮色主题）"""

    def __init__(self, text, parent=None):
        super().__init__(text, parent)
        self._anim = QPropertyAnimation(self, b"bg_color_value")
        self._anim.setDuration(180)
        self._anim.setEasingCurve(QEasingCurve.InOutCubic)
        self.setMinimumHeight(40)
        self.setCursor(Qt.PointingHandCursor)
        self.update_style()

    @pyqtProperty(float)
    def bg_color_value(self):
        return 0.0

    @bg_color_value.setter
    def bg_color_value(self, val):
        pass

    def enterEvent(self, event):
        self._anim.stop()
        self._anim.setStartValue(0.0)
        self._anim.setEndValue(1.0)
        self._anim.start()
        super().enterEvent(event)

    def leaveEvent(self, event):
        self._anim.stop()
        self._anim.setStartValue(1.0)
        self._anim.setEndValue(0.0)
        self._anim.start()
        super().leaveEvent(event)

    def update_style(self):
        self.setStyleSheet("""
            QPushButton {
                background: qlineargradient(x1:0, y1:0, x2:0, y2:1,
                    stop:0 #1976d2, stop:1 #1565c0);
                color: #ffffff;
                border: none;
                border-radius: 8px;
                padding: 8px 22px;
                font-weight: 700;
                font-size: 14px;
            }
            QPushButton:hover {
                background: qlineargradient(x1:0, y1:0, x2:0, y2:1,
                    stop:0 #2196f3, stop:1 #1976d2);
            }
            QPushButton:pressed {
                background: qlineargradient(x1:0, y1:0, x2:0, y2:1,
                    stop:0 #1565c0, stop:1 #0d47a1);
                padding-top: 9px;
                padding-bottom: 7px;
            }
            QPushButton:disabled {
                background: #bdbdbd;
                color: #757575;
            }
        """)


class SecondaryButton(QPushButton):
    """次级按钮：边框样式（亮色主题）"""

    def __init__(self, text, parent=None):
        super().__init__(text, parent)
        self.setMinimumHeight(40)
        self.setCursor(Qt.PointingHandCursor)
        self.setStyleSheet("""
            QPushButton {
                background: #ffffff;
                color: #1565c0;
                border: 2px solid #1976d2;
                border-radius: 8px;
                padding: 8px 22px;
                font-weight: 700;
                font-size: 14px;
            }
            QPushButton:hover {
                background: #e3f2fd;
                border-color: #1565c0;
                color: #0d47a1;
            }
            QPushButton:pressed {
                background: #bbdefb;
            }
        """)


class DangerButton(QPushButton):
    """危险操作按钮：红色系（亮色主题）"""

    def __init__(self, text, parent=None):
        super().__init__(text, parent)
        self.setMinimumHeight(40)
        self.setCursor(Qt.PointingHandCursor)
        self.setStyleSheet("""
            QPushButton {
                background: qlineargradient(x1:0, y1:0, x2:0, y2:1,
                    stop:0 #f44336, stop:1 #d32f2f);
                color: #ffffff;
                border: none;
                border-radius: 8px;
                padding: 8px 22px;
                font-weight: 700;
                font-size: 14px;
            }
            QPushButton:hover {
                background: qlineargradient(x1:0, y1:0, x2:0, y2:1,
                    stop:0 #ef5350, stop:1 #c62828);
            }
            QPushButton:pressed {
                background: qlineargradient(x1:0, y1:0, x2:0, y2:1,
                    stop:0 #d32f2f, stop:1 #b71c1c);
            }
        """)


class StyledSpinBox(QSpinBox):
    """美化后的数值输入框（亮色主题）"""

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setMinimumHeight(46)
        self.setStyleSheet("""
            QSpinBox {
                background: #ffffff;
                color: #1a202c;
                border: 2px solid #90caf9;
                border-radius: 7px;
                padding: 6px 12px;
                font-size: 15px;
                font-weight: 700;
                min-width: 70px;
            }
            QSpinBox:focus {
                border-color: #1976d2;
            }
            QSpinBox::up-button, QSpinBox::down-button {
                background: #e3f2fd;
                border: none;
                width: 30px;
            }
            QSpinBox::up-button:hover, QSpinBox::down-button:hover {
                background: #90caf9;
            }
            QSpinBox::up-arrow {
                image: none;
                border-left: 5px solid transparent;
                border-right: 5px solid transparent;
                border-bottom: 8px solid #1976d2;
                margin-top: 3px;
            }
            QSpinBox::down-arrow {
                image: none;
                border-left: 5px solid transparent;
                border-right: 5px solid transparent;
                border-top: 8px solid #1976d2;
                margin-bottom: 3px;
            }
        """)


class MatrixLineEdit(QLineEdit):
    """矩阵输入框：紧密"田"字格 + 等宽字体 + 聚焦高亮（亮色主题）"""

    CELL_W = 72  # 格子宽度（像素）
    CELL_H = 72  # 格子高度（像素），增大以获得更高的视觉效果

    def __init__(self, parent=None):
        super().__init__(parent)
        self.setFont(QFont("JetBrains Mono", 17, QFont.Normal))
        self.setAlignment(Qt.AlignCenter)
        # 固定宽高，形成田字格
        self.setFixedSize(self.CELL_W, self.CELL_H)
        self.setStyleSheet(f"""
            QLineEdit {{
                background: #ffffff;
                color: #1a202c;
                border: 1px solid #90caf9;
                border-radius: 0px;
                padding: 0px;
                font-size: 22px;
                font-weight: 600;
                selection-background-color: #1976d2;
                selection-color: #ffffff;
            }}
            QLineEdit:focus {{
                border-color: #1976d2;
                background: #e3f2fd;
                border-width: 2px;
            }}
            QLineEdit:hover {{
                border-color: #42a5f5;
            }}
        """)


class MatrixInputWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.initUI()

    def initUI(self):
        self.setWindowTitle("矩阵计算器")
        self.setGeometry(60, 60, 1380, 1200)
        self.setMinimumSize(1200, 660)

        central_widget = QWidget()
        self.setCentralWidget(central_widget)

        main_layout = QVBoxLayout(central_widget)
        main_layout.setSpacing(18)
        main_layout.setContentsMargins(24, 20, 24, 20)

        # 标题
        # title_label = QLabel("◆ 矩阵计算器")
        # title_label.setFont(QFont("Microsoft YaHei", 22, QFont.Bold))
        # title_label.setStyleSheet("color: #1565c0; margin-bottom: 2px;")
        # main_layout.addWidget(title_label)

        # subtitle = QLabel("支持矩阵运算、特征值分析、线性方程组求解")
        # subtitle.setFont(QFont("Microsoft YaHei", 12))
        # subtitle.setStyleSheet("color: #64b5f6; margin-bottom: 6px;")
        # main_layout.addWidget(subtitle)

        # 分隔线
        # sep = QFrame()
        # sep.setFrameShape(QFrame.HLine)
        # sep.setStyleSheet("color: #bbdefb; max-height: 2px; background: #bbdefb;")
        # main_layout.addWidget(sep)

        # 矩阵A、B 水平并排
        matrices_row = QHBoxLayout()
        matrices_row.setSpacing(18)

        card_A = self.create_matrix_card("A", "矩阵 A")
        card_B = self.create_matrix_card("B", "矩阵 B")
        card_A.setMinimumHeight(520)
        card_B.setMinimumHeight(520)
        matrices_row.addWidget(card_A, stretch=1)
        matrices_row.addWidget(card_B, stretch=1)
        main_layout.addLayout(matrices_row, stretch=4)

        # 运算按钮区域
        ops_card = QGroupBox("矩阵运算")
        ops_card.setStyleSheet(self.groupbox_style())
        ops_layout = QHBoxLayout(ops_card)
        ops_layout.setSpacing(12)

        self.calc_add_button = AnimatedButton("A + B")
        self.calc_add_button.clicked.connect(self.add_matrix)
        ops_layout.addWidget(self.calc_add_button)

        self.calc_sub_button = AnimatedButton("A − B")
        self.calc_sub_button.clicked.connect(self.sub_matrix)
        ops_layout.addWidget(self.calc_sub_button)

        self.calc_BsubA_button = AnimatedButton("B − A")
        self.calc_BsubA_button.clicked.connect(self.sub_BA_matrix)
        ops_layout.addWidget(self.calc_BsubA_button)

        self.calc_mul_button = AnimatedButton("A × B")
        self.calc_mul_button.clicked.connect(self.mul_matrix)
        ops_layout.addWidget(self.calc_mul_button)

        self.calc_mul_button2 = AnimatedButton("B × A")
        self.calc_mul_button2.clicked.connect(self.mul_BA_matrix)
        ops_layout.addWidget(self.calc_mul_button2)

        self.calc_solve_button = AnimatedButton("Ax = b")
        self.calc_solve_button.clicked.connect(self.solve_linear_system)
        ops_layout.addWidget(self.calc_solve_button)

        ops_layout.addStretch()
        main_layout.addWidget(ops_card, stretch=0)

        # 结果区域卡片
        result_card = QGroupBox("计算结果")
        result_card.setStyleSheet(self.groupbox_style())
        result_layout = QVBoxLayout(result_card)
        result_layout.setSpacing(8)

        self.result_text = QTextEdit()
        self.result_text.setReadOnly(True)
        self.result_text.setFont(QFont("JetBrains Mono", 13))
        self.result_text.setStyleSheet("""
            QTextEdit {
                background: #f8fbff;
                color: #1a202c;
                border: 2px solid #bbdefb;
                border-radius: 10px;
                padding: 14px;
                font-size: 14px;
                line-height: 1.6;
            }
            QTextEdit:focus {
                border-color: #1976d2;
            }
        """)
        self.result_text.setMinimumHeight(200)
        result_layout.addWidget(self.result_text)

        main_layout.addWidget(result_card, stretch=1)

        # 应用全局样式
        self.apply_global_styles()

    def groupbox_style(self):
        return """
            QGroupBox {
                background: #ffffff;
                border: 2px solid #bbdefb;
                border-radius: 12px;
                margin-top: 8px;
                padding-top: 10px;
                padding-bottom: 8px;
                padding-left: 12px;
                padding-right: 12px;
                font-weight: 700;
                font-size: 15px;
                color: #1565c0;
            }
            QGroupBox::title {
                subcontrol-origin: margin;
                left: 14px;
                padding: 0 10px;
                color: #1565c0;
                font-weight: 800;
                font-size: 16px;
            }
        """

    def apply_global_styles(self):
        self.setStyleSheet("""
            QMainWindow {
                background: #e8f4fd;
            }
            QWidget {
                background: #e8f4fd;
                color: #1a202c;
                font-family: "Microsoft YaHei", "PingFang SC", sans-serif;
            }
            QScrollBar:vertical {
                background: #e3f2fd;
                width: 10px;
                border-radius: 5px;
            }
            QScrollBar::handle:vertical {
                background: #90caf9;
                border-radius: 5px;
                min-height: 30px;
            }
            QScrollBar::handle:vertical:hover {
                background: #1976d2;
            }
            QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {
                height: 0px;
            }
            QScrollBar:horizontal {
                background: #e3f2fd;
                height: 10px;
                border-radius: 5px;
            }
            QScrollBar::handle:horizontal {
                background: #90caf9;
                border-radius: 5px;
                min-width: 30px;
            }
            QScrollBar::handle:horizontal:hover {
                background: #1976d2;
            }
            QScrollBar::add-line:horizontal, QScrollBar::sub-line:horizontal {
                width: 0px;
            }
        """)

    def create_matrix_card(self, matrix, title):
        card = QGroupBox(title)
        card.setStyleSheet(self.groupbox_style())
        layout = QVBoxLayout(card)
        layout.setSpacing(30)

        # 矩阵输入区域
        matrix_grid_container = QWidget()
        matrix_grid_container.setStyleSheet("background: transparent;")
        if matrix == "A":
            self.matrix_layout = QGridLayout(matrix_grid_container)
            self.matrix_layout.setSpacing(0)
            self.matrix_layout.setContentsMargins(0, 0, 0, 0)
            self.entries = []
            self.create_matrix_inputs("A", 3, 3)
            layout.addWidget(matrix_grid_container)
        else:
            self.matrixB_layout = QGridLayout(matrix_grid_container)
            self.matrixB_layout.setSpacing(0)
            self.matrixB_layout.setContentsMargins(0, 0, 0, 0)
            self.entriesB = []
            self.create_matrix_inputs("B", 3, 3)
            layout.addWidget(matrix_grid_container)

        # 控制按钮行
        ctrl_layout = QHBoxLayout()
        ctrl_layout.setSpacing(8)

        get_btn = AnimatedButton("矩阵信息")
        get_btn.clicked.connect(lambda: self.get_matrix(matrix))
        ctrl_layout.addWidget(get_btn)

        identity_btn = SecondaryButton("单位矩阵")
        identity_btn.clicked.connect(lambda: self.identity_matrix(matrix))
        ctrl_layout.addWidget(identity_btn)

        power_btn = AnimatedButton("幂运算")
        power_btn.clicked.connect(lambda: self.power_matrix(matrix))
        ctrl_layout.addWidget(power_btn)

        power_spin = StyledSpinBox()
        power_spin.setRange(2, 10)
        power_spin.setValue(2)
        setattr(self, f"power_spin_{matrix}", power_spin)
        power_spin.valueChanged.connect(lambda: self.power_matrix(matrix))
        ctrl_layout.addWidget(power_spin)

        ctrl_layout.addStretch()

        # 尺寸控制行
        size_layout = QHBoxLayout()
        size_layout.setSpacing(8)

        rows_label = QLabel("行数:")
        rows_label.setStyleSheet("color: #1565c0; font-size: 16px; font-weight: 600;")
        size_layout.addWidget(rows_label)

        rows_spin = StyledSpinBox()
        rows_spin.setRange(1, 6)
        rows_spin.setValue(3)
        rows_spin.valueChanged.connect(lambda: self.update_matrix_size(matrix))
        setattr(self, f"rows_spin_{matrix}", rows_spin)
        size_layout.addWidget(rows_spin)

        cols_label = QLabel("列数:")
        cols_label.setStyleSheet("color: #1565c0; font-size: 16px; font-weight: 600;")
        size_layout.addWidget(cols_label)

        cols_spin = StyledSpinBox()
        cols_spin.setRange(1, 6)
        cols_spin.setValue(3)
        cols_spin.valueChanged.connect(lambda: self.update_matrix_size(matrix))
        setattr(self, f"cols_spin_{matrix}", cols_spin)
        size_layout.addWidget(cols_spin)

        size_layout.addStretch()

        clear_btn = DangerButton("清空")
        clear_btn.clicked.connect(lambda: self.clear_matrix(matrix))
        size_layout.addWidget(clear_btn)

        layout.addLayout(ctrl_layout)
        layout.addLayout(size_layout)
        return card

    def create_matrix_inputs(self, matrix, rows, cols):
        if matrix == "A":
            layout = self.matrix_layout
            entries = self.entries
        else:
            layout = self.matrixB_layout
            entries = self.entriesB

        while layout.count():
            child = layout.takeAt(0)
            if child.widget():
                child.widget().deleteLater()

        entries.clear()
        for i in range(rows):
            row_entries = []
            for j in range(cols):
                entry = MatrixLineEdit()
                entry.setText("0")
                layout.addWidget(entry, i, j)
                row_entries.append(entry)
            entries.append(row_entries)

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

    def _parse_sp_matrix(self, matrix_name):
        entries, rows, cols = self.get_entries_and_shape(matrix_name)
        data = []
        for i in range(rows):
            row = []
            for j in range(cols):
                val = entries[i][j].text().strip()
                if not SafeNumberConverter.is_number(val):
                    raise ValueError(
                        f"矩阵{matrix_name}[{i + 1}][{j + 1}] 不是有效数字：{val}\n支持格式：整数、小数、分数（如 1/3）"
                    )
                row.append(SafeNumberConverter.to_rational(val))
            data.append(row)
        return sp.Matrix(data), rows, cols

    def get_matrix(self, matrix):
        try:
            sp_matrix, rows, cols = self._parse_sp_matrix(matrix)

            is_square = rows == cols
            rank = sp_matrix.rank()
            transpose_matrix = sp_matrix.T
            is_symmetric = is_square and sp_matrix == transpose_matrix

            det = sp_matrix.det() if is_square else None
            sp_matrix_adj = None
            sp_matrix_inv = None
            eigen_text = ""

            if is_square:
                sp_matrix_adj = sp_matrix.adjugate()
                if det != 0:
                    sp_matrix_inv = sp_matrix.inv()

                eigenvals = sp_matrix.eigenvals()
                eigenvects = sp_matrix.eigenvects()
                trace = sp_matrix.trace()

                lines = []
                idx = 1
                for eigenval, multiplicity, eigenvectors in eigenvects:
                    lines.append(
                        f"  λ{idx} = {eigenval}  (代数重数={multiplicity}，几何重数={len(eigenvectors)})"
                    )
                    for k, vec in enumerate(eigenvectors):
                        lines.append(f"    特征向量{k + 1}: {list(vec)}")
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
            power_result = sp_matrix**n
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
        try:
            rows_A = self.rows_spin_A.value()
            cols_A = self.cols_spin_A.value()
            rows_B = self.rows_spin_B.value()

            if rows_A != rows_B:
                self.result_text.setText(
                    f"错误: 矩阵A的行数({rows_A})必须等于矩阵B的行数({rows_B})！\n"
                    "提示: 矩阵B的第一列用作常数向量 b"
                )
                return

            A_data = []
            for i in range(rows_A):
                row = []
                for j in range(cols_A):
                    val = self.entries[i][j].text().strip()
                    if not SafeNumberConverter.is_number(val):
                        self.result_text.setText(
                            f"错误: 矩阵A[{i + 1}][{j + 1}] 不是有效数字：{val}"
                        )
                        return
                    row.append(sp.Rational(val))
                A_data.append(row)

            b_data = []
            for i in range(rows_B):
                val = self.entriesB[i][0].text().strip()
                if not SafeNumberConverter.is_number(val):
                    self.result_text.setText(
                        f"错误: 向量b[{i + 1}] 不是有效数字：{val}"
                    )
                    return
                b_data.append(sp.Rational(val))

            A = sp.Matrix(A_data)
            b = sp.Matrix(b_data)

            m, n = A.shape

            Ab = A.row_join(b)

            rank_A = A.rank()
            rank_Ab = Ab.rank()

            output = []
            output.append("=" * 50)
            output.append("  线性方程组  Ax = b  求解")
            output.append("=" * 50)
            output.append(f"\n系数矩阵 A ({m}×{n})：")
            output.append(self._format_sp_matrix(A))
            output.append(f"\n常数向量 b ({m}×1)：")
            output.append(self._format_sp_vector(b))
            output.append(
                f"\nrank(A) = {rank_A}，rank([A|b]) = {rank_Ab}，未知数个数 n = {n}"
            )

            if rank_Ab > rank_A:
                output.append("\n【结论】方程组 无解（不相容）")
                output.append(
                    "原因: rank(A) ≠ rank([A|b])，增广矩阵秩更大，矛盾方程存在。"
                )

            elif rank_A == n:
                output.append("\n【结论】方程组有 唯一解")
                x = A.solve(b)
                output.append("\n解向量 x：")
                for idx, xi in enumerate(x):
                    output.append(f"  x{idx + 1} = {xi}")

                try:
                    A_np = np.array(A_data, dtype=float)
                    b_np = np.array(b_data, dtype=float)
                    x_np = np.linalg.solve(A_np, b_np)
                    output.append("\n数值验证（numpy）：")
                    for idx, xi in enumerate(x_np):
                        output.append(f"  x{idx + 1} ≈ {xi:.6g}")
                    residual = np.linalg.norm(A_np @ x_np - b_np)
                    output.append(f"  残差 ‖Ax-b‖ = {residual:.2e}")
                except Exception:
                    pass

            else:
                free_count = n - rank_A
                output.append(f"\n【结论】方程组有 无穷多解（{free_count} 个自由变量）")

                null_space = A.nullspace()
                particular = self._find_particular_solution(A, b, n)

                output.append("\n齐次方程组 Ax=0 的基础解系：")
                for k, vec in enumerate(null_space):
                    output.append(f"  ξ{k + 1} = {[str(v) for v in vec]}")

                output.append("\n非齐次方程组的一个特解 η*：")
                output.append(f"  η* = {[str(v) for v in particular]}")

                output.append("\n通解（x = η* + c₁ξ₁ + c₂ξ₂ + ...，cᵢ为任意常数）：")
                terms = ["η*"]
                for k in range(len(null_space)):
                    terms.append(f"c{k + 1}·ξ{k + 1}")
                output.append("  x = " + " + ".join(terms))

                output.append("\n各分量通解：")
                for i in range(n):
                    expr = str(particular[i])
                    for k, vec in enumerate(null_space):
                        coeff = vec[i]
                        if coeff != 0:
                            expr += f" + c{k + 1}·({coeff})"
                    output.append(f"  x{i + 1} = {expr}")

            self.result_text.setText("\n".join(output))

        except Exception as e:
            import traceback

            self.result_text.setText(f"求解错误: {str(e)}\n{traceback.format_exc()}")

    def _find_particular_solution(self, A, b, n):
        Ab = A.row_join(b)
        rref, pivots = Ab.rref()
        x = [sp.Integer(0)] * n
        for row_idx, col_idx in enumerate(pivots):
            if col_idx < n:
                x[col_idx] = rref[row_idx, n]
        return x

    def _format_sp_matrix(self, M):
        rows = M.tolist()
        lines = []
        for row in rows:
            lines.append("  [ " + "  ".join(str(v).rjust(6) for v in row) + " ]")
        return "\n".join(lines)

    def _format_sp_vector(self, v):
        items = [v[i] for i in range(v.rows)]
        return "  [ " + "  ".join(str(x).rjust(6) for x in items) + " ]"


class SafeNumberConverter:
    @staticmethod
    def is_number(s: str) -> bool:
        return (
            SafeNumberConverter.is_integer(s)
            or SafeNumberConverter.is_float(s)
            or SafeNumberConverter.is_fraction(s)
        )

    @staticmethod
    def is_integer(s: str) -> bool:
        return bool(re.match(r"^-?\d+$", s.strip())) if s else False

    @staticmethod
    def is_float(s: str) -> bool:
        s = s.strip() if s else ""
        pattern = r"^-?\d+\.\d*$|^-?\d*\.\d+$|^-?\d+(?:\.\d*)?[eE][+-]?\d+$"
        return bool(re.match(pattern, s))

    @staticmethod
    def is_fraction(s: str) -> bool:
        s = s.strip() if s else ""
        return bool(re.match(r"^(-?\d+)\s*/\s*(-?\d+)$", s))

    @staticmethod
    def to_rational(s: str) -> "sp.Rational":
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
            return sp.Rational(s)
        raise ValueError(f"无法识别的数字格式：{s}")

    @staticmethod
    def convert(s: str, default=None):
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
        return [SafeNumberConverter.convert(s, default) for s in strings]


if __name__ == "__main__":
    app = QApplication(sys.argv)
    app.setStyle("Fusion")

    # 全局亮色调色板
    palette = QPalette()
    palette.setColor(QPalette.Window, QColor(232, 244, 253))  # 浅蓝灰背景
    palette.setColor(QPalette.WindowText, QColor(26, 32, 44))  # 深色文字
    palette.setColor(QPalette.Base, QColor(255, 255, 255))  # 输入框白色背景
    palette.setColor(QPalette.AlternateBase, QColor(227, 242, 253))  # 交替行浅蓝
    palette.setColor(QPalette.ToolTipBase, QColor(25, 118, 210))  # 提示框蓝色背景
    palette.setColor(QPalette.ToolTipText, QColor(255, 255, 255))  # 提示框白色文字
    palette.setColor(QPalette.Text, QColor(26, 32, 44))  # 文本深色
    palette.setColor(QPalette.Button, QColor(227, 242, 253))  # 按钮浅蓝
    palette.setColor(QPalette.ButtonText, QColor(26, 32, 44))  # 按钮文字深色
    palette.setColor(QPalette.BrightText, QColor(21, 101, 192))  # 亮文字蓝色
    palette.setColor(QPalette.Highlight, QColor(25, 118, 210))  # 选中高亮蓝色
    palette.setColor(QPalette.HighlightedText, QColor(255, 255, 255))  # 选中文字白色
    app.setPalette(palette)

    window = MatrixInputWindow()
    window.show()
    sys.exit(app.exec_())
