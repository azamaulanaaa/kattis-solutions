fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let out = lines
        .by_ref()
        .next()
        .unwrap()
        .split(" ")
        .take(n)
        .map(|x| x.chars().next().unwrap())
        .filter(|x| char::is_uppercase(*x))
        .collect::<String>();
    println!("{}", out);
}
