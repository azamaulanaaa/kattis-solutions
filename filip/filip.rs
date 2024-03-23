fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let out = lines
        .by_ref()
        .next()
        .unwrap()
        .split(" ")
        .take(2)
        .map(|x| {
            x.chars()
                .rev()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .max()
        .unwrap()
        .to_string();
    println!("{}", out);
}
