mod euler; // using Euler.rs as a module in this to use the function euler().

// Define function f(t,y) such that ODE: dy/dt=f(t,y) 
fn f(t: f64, y: f64) -> f64{
    t.cos() - y // return function value ( dont't use semicolon)
}


fn main(){
    let step_sizes = [20, 100, 1000];

    for &n in &step_sizes{
        let sol = euler::euler(f, 1.0, 0.0, 5.0, n); // find solution using Euler method for different values of n 
        println!("{}", n);
        println!("{:?}", sol);
    }

}
