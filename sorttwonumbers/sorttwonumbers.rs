fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut values = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    values.sort();
    println!("{} {}", values[0], values[1]);
}
