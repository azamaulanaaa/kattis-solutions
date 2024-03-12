fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut value = lines
        .take(2)
        .map(|x| x.chars().collect::<Vec<char>>())
        .flatten()
        .into_iter()
        .collect::<Vec<char>>();
    value.sort();
    let value = value.into_iter().collect::<String>();
    println!("{}", value);
}
