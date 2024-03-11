use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let values = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let out = values[0] * (values[1] - 1) + 1;
    println!("{}", out);
}
