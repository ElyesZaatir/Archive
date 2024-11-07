use rand::prelude::*;
use colored::Colorize;
fn main() {
    let mut perfect:i32 = 0;
    use std::io::stdin;
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");
    let mut tries: i32 = input.trim().parse().expect("Input not an integer");
    println!("{}",tries);
    while tries != 0{
        let succ = calc(tries);
        if succ == 10{
            perfect += 1;
        }
        tries -= 1;
    }
    let msg = "perfect attempts".red();
    println!("{} {}",perfect,msg);
}
fn calc(tries:i32)->i32{
    let mut chance: f64 = 0.7;
    let mut succ: i32 = 0;
    let mut attempts:i32 = 10;
    while attempts!=0 {
        if is_proc(chance) {
            chance -= 0.1;
            succ += 1;
        }else{
            chance += 0.1;
        }
        attempts -= 1;
        chance = chance.clamp(0.3,0.7);
    }
    println!("Try number {} : {} successful attempts",tries,succ);
    succ
}

fn is_proc(chance: f64) -> bool {
    let mut rng = rand::thread_rng();
    let gen: f64 = rng.gen();
    gen <= chance
}
