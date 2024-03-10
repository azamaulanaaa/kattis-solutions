use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());

    let param = &lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let required = lines
        .map(|x| x.parse::<usize>().unwrap())
        .take(param[0])
        .sum();

    if param[1] >= required {
        println!("Jebb");
    } else {
        println!("Neibb");
    }
}
