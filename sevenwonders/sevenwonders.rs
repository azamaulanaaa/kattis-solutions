fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let value = lines.by_ref().next().unwrap();
    let count = value
        .chars()
        .fold((0usize, 0usize, 0usize), |res, x| match x {
            'T' => (res.0 + 1, res.1, res.2),
            'C' => (res.0, res.1 + 1, res.2),
            'G' => (res.0, res.1, res.2 + 1),
            _ => res,
        });
    let out =
        count.0.pow(2) + count.1.pow(2) + count.2.pow(2) + count.0.min(count.1).min(count.2) * 7;
    println!("{}", out);
}
