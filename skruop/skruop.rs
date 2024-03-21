fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let out = lines
        .take(n)
        .fold(7_usize, |current, x| match (current, x.as_str()) {
            (..=9, "Skru op!") => current + 1,
            (1.., "Skru ned!") => current - 1,
            _ => current,
        });
    println!("{}", out);
}
