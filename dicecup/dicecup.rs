use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let params = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let low = params.iter().min().unwrap() + 1;
    let high = params.iter().max().unwrap() + 1;
    for i in low..=high {
        println!("{}", i);
    }
}
