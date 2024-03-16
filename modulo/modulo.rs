fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let out = lines
        .take(10)
        .map(|x| x.parse::<usize>().unwrap() % 42)
        .collect::<std::collections::HashSet<_>>()
        .len();
    println!("{}", out);
}
