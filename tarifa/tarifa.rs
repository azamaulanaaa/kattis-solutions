use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let x = &lines.next().unwrap().parse::<usize>().unwrap();
    let n = &lines.next().unwrap().parse::<usize>().unwrap();
    let spent: usize = lines.take(*n).map(|x| x.parse::<usize>().unwrap()).sum();
    println!("{}", *x * (*n + 1) - spent);
}
