mod euler; // using Euler.rs as a module in this to use the function euler().
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
// Define function f(t,y) such that ODE: dy/dt=f(t,y) 
fn f(t: f64, y: f64) -> f64{
    t.cos() - y // return function value ( dont't use semicolon)
}


fn main()-> Result<(), Box<dyn std::error::Error>>{
    let step_sizes = [20, 100, 1000];
    // Create and write a csv file named euler_solutions.csv for storing the values.
    let file = File::create("euler_solutions.csv")?;   
    let mut writer = BufWriter::new(file);
    // Writing header for csv file
    writeln!(writer, "n,t,y")?;
    
    for &n in &step_sizes{
        let sol = euler::euler(f, 1.0, 0.0, 5.0, n); // find solution using Euler method for different values of n 
        for (t,y) in sol{
            writeln!(writer, "{},{:.10},{:.10}", n, t, y)?;
        }
    }
Ok(())
}
