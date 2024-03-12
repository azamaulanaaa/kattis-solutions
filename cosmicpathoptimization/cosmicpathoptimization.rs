fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = &lines.next().unwrap().parse::<usize>().unwrap();
    let result = &lines
        .next()
        .unwrap()
        .split(" ")
        .take(*n)
        .map(|x| x.parse::<usize>().unwrap())
        .sum::<usize>()
        / *n;
    println!("{}", result);
}
