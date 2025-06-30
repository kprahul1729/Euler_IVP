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
  plt.plot(t_values, y_values, label=f'n={n}')
  
t_values = np.linspace(0, 5, 1001)
y_analytical = 1/2*(np.sin(t_values) + np.cos(t_values)+np.exp(-1*t_values))
plt.plot(t_values, y_analytical, label='Analytical Solution')

plt.xlabel('t')
plt.ylabel('y')
plt.title('Euler Method for different n vs Analytical Solution')
plt.legend()
plt.grid(True)
plt.show()

