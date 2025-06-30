fn euler( f: impl Fn(f64, f64) -> f64,
         y0: f64,
         t_start: f64,
         t_end: f64,
         n: usize
         ) -> Vec<(f64, f64)> { 
  let h = (t_end - t_start) / n as f64;
  let mut result = Vec::with_capacity(n+1);
  result.push((t_start, y0));
  let mut y = y0;
  let mut t = t_start;
  for _ in 0..n{
    y += h * f(t,y);
    result.push((t,y));
  }
  result
}
    
  
