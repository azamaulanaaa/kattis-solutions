fn main() {
    let base = vec!['P', 'E', 'R'].into_iter().cycle();
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let out = lines
        .by_ref()
        .next()
        .unwrap()
        .chars()
        .zip(base)
        .filter(|(a, b)| a != b)
        .count();
    println!("{}", out);
}
