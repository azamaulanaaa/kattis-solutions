fn main() {
    let mut lines = std::io::stdin().lines().into_iter().map(|e| e.unwrap());

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let day = (0..n).find(|day| usize::pow(2, *day as u32) >= n).unwrap() + 1;

    println!("{}", day);
}
