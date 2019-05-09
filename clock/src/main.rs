use clock::Clock;

fn main() {
    let c = Clock::new(-25, 0);
    println!("{:#?}", c.to_string());
}
