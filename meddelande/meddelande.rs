use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());

    let params = &lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let value = lines
        .take(params[0])
        .collect::<Vec<String>>()
        .join("")
        .replace(".", "");
    println!("{}", value);
}
