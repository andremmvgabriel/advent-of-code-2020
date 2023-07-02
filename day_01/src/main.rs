use std::time::Instant;

fn time_block(func: &dyn Fn() -> () ) -> f64 {
    let start = Instant::now();
    func();
    return start.elapsed().as_nanos() as f64 / 10e9;
}

fn sometest() -> () {
    println!("Goodbye!");
}

fn main() {
    println!("Hello, world!");
    let elapsed = time_block(&sometest);
    println!("Awesome time! {elapsed} seconds");
}
