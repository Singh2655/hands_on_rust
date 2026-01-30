use std::io::stdin;

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read your name");
    name.trim().to_lowercase()
}

fn main() {
    println!("Hey, what's your name?");

    let your_name = what_is_your_name();

    println!("hey, {your_name}");
}
