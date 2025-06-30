def euler(f, Y0, a, b, n):
  h = (b - a) / n
  y = [Y0]
  t = a
  for _ in range(n):
    y_new = y[-1] + h * f(t, y[-1])
    y.append(y_new)
    t += h
  return y
  
