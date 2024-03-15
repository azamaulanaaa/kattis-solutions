fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let out = lines
        .by_ref()
        .next()
        .unwrap()
        .matches(|x| char::is_ascii_alphanumeric(&x))
        .count();
    println!("{}", out);
}
