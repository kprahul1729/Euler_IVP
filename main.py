from Euler import euler
import math
import numpy as np
import matplotlib.pyplot as plt

def f(t, y):
  return math.cos(t) - y

step_sizes = {20, 100, 1000}
for n in step_sizes:
  y_values = euler(f, 1, 0, 5, n)
  t_values = np.linspace(0, 5, n + 1)
  plt.plot(t_values, y_values, label='n='n)
  
plt.xlabel('t')
plt.ylabel('y')
plt.title('Euler Method for different n')
plt.legend()
plt.grid(True)
plt.show()

