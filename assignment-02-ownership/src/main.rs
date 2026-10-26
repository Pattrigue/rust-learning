fn main() {
    println!("{}", last_word("I am learning Rust!"));

    let mut cake = String::from("Cake");
    add_exclamation(&mut cake);
    println!("{cake}");

    let mut pie = String::from("I love chocolate cake");
    println!("{}", last_word(&pie));
    add_exclamation(&mut pie);
    println!("{pie}");
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    s
}

fn add_exclamation(s: &mut String) {
    s.push('!');
}
