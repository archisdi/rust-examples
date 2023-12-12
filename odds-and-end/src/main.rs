extern crate rand;
extern crate phrases;

use rand::Rng;
use phrases::greetings::french;

fn main() {
    let mut rnf = rand::thread_rng();
    if rnf.gen() {
        println!("i32: {}", rnf.gen::<i32>());
    }

    println!("English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye());

    println!("French: {}, {}",
        french::hello(),
        french::goodbye()); 
}
