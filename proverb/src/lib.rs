//For want of a nail the shoe was lost.
//For want of a shoe the horse was lost.
//For want of a horse the rider was lost.
//For want of a rider the message was lost.
//For want of a message the battle was lost.
//For want of a battle the kingdom was lost.
//And all for the want of a nail.

pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() == 0 {
        return String::from("");
    }
    let pairs = list.iter().zip(list[1..].iter());
    let sentences =
        pairs.map(|(first, second)| format!("For want of a {} the {} was lost.\n", first, second));
    let mut result: String = "".to_string();
    for sentence in sentences {
        result = result + &sentence;
    }
    result + &format!("And all for the want of a {}.", list[0])
}
