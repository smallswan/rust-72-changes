"""勾股定理动画示例

演示：基本图形、文字、数学公式、图形变换

渲染: manim -ql examples/manim/pythagorean.py PythagoreanTheorem
"""

from manim import *


class PythagoreanTheorem(Scene):
    """用面积可视化勾股定理 a² + b² = c²"""

    def construct(self):
        # ===== 标题 =====
        title = Text("勾股定理", font="Microsoft YaHei", font_size=48)
        self.play(Write(title))
        self.play(title.animate.to_edge(UP).scale(0.6))

        # ===== 直角三角形 (3-4-5) =====
        scale = 0.5
        a, b = 3 * scale, 4 * scale
        triangle = Polygon(
            ORIGIN, RIGHT * b, RIGHT * b + UP * a,
            color=WHITE, fill_opacity=0.3, fill_color=BLUE,
        ).shift(LEFT * 2 + DOWN * 1.5)
        self.play(Create(triangle))

        # 三边标注
        verts = triangle.get_vertices()
        label_a = MathTex("a", color=YELLOW).next_to(
            Line(verts[1], verts[2]).get_center(), RIGHT, buff=0.2)
        label_b = MathTex("b", color=GREEN).next_to(
            Line(verts[0], verts[1]).get_center(), DOWN, buff=0.2)
        label_c = MathTex("c", color=RED).next_to(
            Line(verts[0], verts[2]).get_center(), UP + LEFT, buff=0.1)
        self.play(Write(label_a), Write(label_b), Write(label_c))
        self.wait(0.5)

        # ===== 公式推导 =====
        formula = MathTex("a^2", "+", "b^2", "=", "c^2", font_size=60)
        formula.set_color_by_tex("a^2", YELLOW)
        formula.set_color_by_tex("b^2", GREEN)
        formula.set_color_by_tex("c^2", RED)
        formula.shift(RIGHT * 3 + UP * 1)
        self.play(Write(formula))
        self.wait(0.5)

        # 代入 3-4-5
        numbers = MathTex("3^2", "+", "4^2", "=", "5^2", font_size=48)
        numbers.next_to(formula, DOWN, buff=0.6)
        result = MathTex("9 + 16 = 25", font_size=48).next_to(numbers, DOWN, buff=0.6)
        self.play(TransformFromCopy(formula, numbers))
        self.play(Write(result))

        # 结果高亮
        box = SurroundingRectangle(result, color=GOLD, buff=0.2)
        self.play(Create(box))
        self.wait(1)

        # ===== 收尾 =====
        self.play(*[FadeOut(m) for m in self.mobjects])
