extern crate prime_factors;

use prime_factors::factors;

fn main(){
    let result = factors(8);
    println!("{:?}", result);
}
