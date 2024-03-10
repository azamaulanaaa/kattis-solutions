use std::io::stdin;

fn main() {
    let values = stdin()
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .take(2)
        .collect::<Vec<usize>>();
    println!("{}", values[0] - values[1]);
}
