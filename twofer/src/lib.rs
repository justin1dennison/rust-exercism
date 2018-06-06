pub fn twofer(name: &str)-> String {
    if name == "" {
       String::from("One for you, one for me.")
    } else {
        format!("One for {}, one for me.", name) 
    }
}
