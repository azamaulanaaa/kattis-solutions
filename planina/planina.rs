fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<u32>().unwrap();
    let out = (2usize.pow(n) + 1).pow(2);
    println!("{}", out);
}
