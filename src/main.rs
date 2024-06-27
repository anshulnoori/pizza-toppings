use std::io;
use rand::Rng;

// outrageous food combinations
const TOPPINGS1: [&str; 6] = ["pineapple", "cactus", "anchovies", "peanut butter", "bark", "toothpaste"];
const TOPPINGS2: [&str; 5] = ["gummy bears", "apple", "garlic", "rose petals", "caviar"];
const TOPPINGS1_LEN: usize = TOPPINGS1.len();
const TOPPINGS2_LEN: usize = TOPPINGS2.len();

fn main() {
    // setup random number generatord
    let mut rng = rand::thread_rng();
    let mut topping1: &str;
    let mut topping2: &str;

    // get number of combinations from user
    println!("How many outrageous food combinations would you like me to generate?");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let combinations: u32 = input.trim().parse().expect("Please type a number!");

    // generate random combinations
    for i in 0..combinations {
        topping1 = TOPPINGS1[rng.gen_range(0..TOPPINGS1_LEN)];
        topping2 = TOPPINGS2[rng.gen_range(0..TOPPINGS2_LEN)];
        println!("Combination #{}: {} and {}!", i+1, topping1, topping2);
    }
}
