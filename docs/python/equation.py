from sympy import symbols, solve
import sympy as sp
x1,x2,x3,x4 = symbols('x1 x2 x3 x4')
eq1 = sp.Eq(2*x1 + x2 -5*x3 + x4,8)
eq2 = sp.Eq(x1 - 3*x2 - 6*x4,9)
eq3 = sp.Eq(2*x2 - x3 + 2*x4,-5)
eq4 = sp.Eq(x1 + 4*x2 - 7*x3 + 6*x4,0)
print(solve([eq1,eq2,eq3,eq4],(x1,x2,x3,x4)))

