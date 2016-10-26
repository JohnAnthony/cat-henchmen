extern crate rand;

mod dice;
mod henchman;

use henchman::Henchman;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let times;
    if args.len() == 1 {
        times = 1;
    } else if args.len() == 2 {
        times = args[1].parse().unwrap();
    } else {
        panic!("Too many arguments!");
    }

    println!("----------------------------------------------------------");
    for _ in 0..times {
        let h = Henchman::new();
        println!("{}", h);
        println!("----------------------------------------------------------");
    }
}
