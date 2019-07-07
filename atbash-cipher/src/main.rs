mod lib;
use lib::{encode, ALPHABET};
use std::str;

fn main() {
    let testing = "no";
    //let result = encode(testing);
    //println!("{}", result);
    let testing = "abdakdja"
        .as_bytes()
        .chunks(5)
        .map(|chunk| str::from_utf8(chunk))
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(" ");
    println!("{:?}", testing);
}
