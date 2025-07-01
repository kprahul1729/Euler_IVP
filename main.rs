mod Euler; // using Euler.rs as a module in this to use the function euler().

// Define function f(t,y) such that ODE: dy/dt=f(t,y) 
fn f(t: f64, y: f64) -> f64{
    t.cos() - y;
}


fn main(){
    let step_sizes = [20, 100, 1000];

    for &n in &step_sizes{
        let sol = Euler::euler(f, 1, 0, 5, n);
        println!(":,?", sol);
    }

}