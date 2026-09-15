"""几何变换与图形动画示例

演示：图形渐变(Transform)、旋转、圆周率展开、极坐标玫瑰线

渲染:
  manim -ql examples/manim/geometry.py ShapeMorph
  manim -ql examples/manim/geometry.py UnrollCircle
  manim -ql examples/manim/geometry.py PolarRose
"""

from manim import *


class ShapeMorph(Scene):
    """正方形 -> 圆 -> 三角形的连续渐变，以及旋转/缩放"""

    def construct(self):
        square = Square(side_length=2, color=BLUE, fill_opacity=0.5)
        circle = Circle(radius=1.2, color=GREEN, fill_opacity=0.5)
        triangle = Triangle(color=RED, fill_opacity=0.5).scale(1.5)

        self.play(Create(square))
        self.wait(0.3)
        self.play(Transform(square, circle))
        self.wait(0.3)
        self.play(Transform(square, triangle))
        self.wait(0.3)

        # 旋转 + 缩放 + 移动组合动画
        self.play(square.animate.rotate(PI / 2).scale(0.6).shift(LEFT * 3))
        # 复制多个并环形排列
        copies = VGroup(*[
            square.copy().rotate(i * PI / 3).set_color(
                interpolate_color(RED, YELLOW, i / 5)
            )
            for i in range(6)
        ])
        copies.arrange_in_grid(2, 3, buff=0.8).move_to(RIGHT * 2)
        self.play(TransformFromCopy(VGroup(square), copies))
        self.wait(1)


class UnrollCircle(Scene):
    """把圆周展开成直线，直观展示周长 = 2πr"""

    def construct(self):
        radius = 1.5
        circle = Circle(radius=radius, color=BLUE).shift(UP * 1.5)
        r_line = Line(circle.get_center(), circle.get_center() + RIGHT * radius,
                      color=YELLOW)
        r_label = MathTex("r", color=YELLOW).next_to(r_line, DOWN, buff=0.1)
        self.play(Create(circle), Create(r_line), Write(r_label))

        # 展开后的线段：长度 2πr
        unrolled = Line(
            LEFT * PI * radius, RIGHT * PI * radius, color=BLUE
        ).shift(DOWN * 1.5)
        length_label = MathTex(r"2\pi r", color=BLUE).next_to(unrolled, DOWN)

        self.play(TransformFromCopy(circle, unrolled), run_time=2)
        self.play(Write(length_label))

        formula = MathTex(r"C = 2\pi r", font_size=60).to_edge(UP)
        self.play(Write(formula))
        self.play(Create(SurroundingRectangle(formula, color=GOLD)))
        self.wait(1)


class PolarRose(Scene):
    """极坐标玫瑰线 r = cos(kθ)，k 从 2 变到 5"""

    def construct(self):
        plane = PolarPlane(radius_max=2.2, size=5.5).add_coordinates()
        self.play(Create(plane), run_time=1.5)

        k_label = MathTex(r"r = \cos(2\theta)").to_corner(UR)
        rose = plane.plot_polar_graph(
            lambda theta: 2 * np.cos(2 * theta), [0, 2 * PI], color=PINK
        )
        self.play(Create(rose), Write(k_label), run_time=2)
        self.wait(0.5)

        # k 逐步变化，花瓣数随之改变
        for k in [3, 4, 5]:
            new_rose = plane.plot_polar_graph(
                lambda theta: 2 * np.cos(k * theta), [0, 2 * PI],
                color=interpolate_color(PINK, PURPLE, (k - 2) / 3),
            )
            new_label = MathTex(rf"r = \cos({k}\theta)").to_corner(UR)
            self.play(Transform(rose, new_rose), Transform(k_label, new_label),
                      run_time=1.5)
            self.wait(0.3)

        self.wait(1)
