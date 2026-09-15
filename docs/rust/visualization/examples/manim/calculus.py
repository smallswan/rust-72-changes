"""函数图像与微积分动画示例

演示：坐标系、函数绘制、切线动画、黎曼和（定积分近似）

渲染:
  manim -ql examples/manim/calculus.py SinePlot
  manim -ql examples/manim/calculus.py RiemannSum
"""

from manim import *


class SinePlot(Scene):
    """绘制 sin(x) 曲线并演示切线滑动"""

    def construct(self):
        # ===== 坐标系 =====
        axes = Axes(
            x_range=[-PI, 2 * PI, PI / 2],
            y_range=[-1.5, 1.5, 0.5],
            axis_config={"include_tip": True},
        ).add_coordinates()
        labels = axes.get_axis_labels(x_label="x", y_label="y")
        self.play(Create(axes), Write(labels))

        # ===== 绘制 sin(x) =====
        sin_graph = axes.plot(lambda x: np.sin(x), color=BLUE)
        sin_label = MathTex(r"y = \sin x", color=BLUE).to_corner(UR).shift(DOWN * 0.5)
        self.play(Create(sin_graph), Write(sin_label))
        self.wait(0.5)

        # ===== 切线沿曲线滑动 =====
        t = ValueTracker(-PI / 2)
        dot = always_redraw(
            lambda: Dot(axes.c2p(t.get_value(), np.sin(t.get_value())), color=YELLOW)
        )
        tangent = always_redraw(
            lambda: axes.get_secant_slope_group(
                x=t.get_value(),
                graph=sin_graph,
                dx=0.01,
                secant_line_color=RED,
                secant_line_length=3,
            )
        )
        self.play(FadeIn(dot), FadeIn(tangent))
        self.play(t.animate.set_value(3 * PI / 2), run_time=5, rate_func=linear)
        self.wait(1)


class RiemannSum(Scene):
    """黎曼和逼近定积分：矩形越窄近似越好"""

    def construct(self):
        axes = Axes(
            x_range=[0, 4, 1],
            y_range=[0, 9, 2],
            x_length=8,
            y_length=5,
        ).add_coordinates()
        graph = axes.plot(lambda x: 0.5 * x**2, x_range=[0, 4], color=GREEN)
        graph_label = MathTex(r"f(x) = \tfrac{1}{2}x^2", color=GREEN).to_corner(UL)
        self.play(Create(axes), Create(graph), Write(graph_label))

        # 积分表达式
        integral = MathTex(r"\int_0^4 \tfrac{1}{2}x^2 \, dx = \tfrac{32}{3}").to_corner(UR)
        self.play(Write(integral))

        # ===== 矩形逐步加密 =====
        rects = axes.get_riemann_rectangles(
            graph, x_range=[0, 4], dx=1.0, color=[BLUE, PURPLE], fill_opacity=0.6
        )
        self.play(Create(rects))
        self.wait(0.5)

        for dx in [0.5, 0.25, 0.1]:
            new_rects = axes.get_riemann_rectangles(
                graph, x_range=[0, 4], dx=dx, color=[BLUE, PURPLE], fill_opacity=0.6
            )
            self.play(Transform(rects, new_rects), run_time=1.5)
            self.wait(0.3)

        self.wait(1)
