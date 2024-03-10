use std::io::stdin;

fn main() {
    let out = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    println!("{}", out[0]);
}
