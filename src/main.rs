mod utils;
mod fib;

use std::env;

fn main() {
    let benchmark = utils::cli::Benchmark::new();

    // arg[0] is options, arg[1] is the first CLI arg
    let arg: Vec<String> = env::args().collect();
    let length: usize = utils::cli::parse_arg(arg[1].clone());

    let mut fibonacci = fib::Fibonacci::new(length);
    fibonacci.update();
    println!("{}", fibonacci.last_value.unwrap());
    
    /*
     * To update again with new values:
     * fibonacci.update_with_length(length + 10); -- length + 10 is just an example of a new length to test with
     * println!("{}", fibonacci.last_value.unwrap());
     */

    println!("Took {:?} to calculate", benchmark.get_elapsed());
}
