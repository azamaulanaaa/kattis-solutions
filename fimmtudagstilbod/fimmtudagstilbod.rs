fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let out = 1000 + (n.checked_sub(2020).unwrap_or(0) * 100);
    println!("{}", out);
}
