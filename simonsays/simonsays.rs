fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines
        .by_ref()
        .next()
        .unwrap_or(String::from("0"))
        .parse::<usize>()
        .unwrap_or(0);
    let out = lines
        .take(n)
        .filter_map(|x| {
            if x.starts_with("Simon says ") {
                Some(x[10..].to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", out);
}
