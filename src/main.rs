use rand::Rng;
use std::{self,io};
fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..10);
    println!("guess a number between 1 and 10");
    let mut b = a.to_string();
    io::stdin()
    .read_line( &mut b)
    .expect("Failed to read line");
let x: i32 = b.trim().parse().expect("Input not an integer");
    println!("the number was {}", a);
    if x == a {
        println!("you guessed right")
    } else {
        println!("wrong")
    }
}