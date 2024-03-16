fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut res = vec![0, 1, 2];
    let _ = lines
        .by_ref()
        .next()
        .unwrap()
        .chars()
        .for_each(|c| match c {
            'A' => res.swap(0, 1),
            'B' => res.swap(1, 2),
            'C' => res.swap(0, 2),
            _ => (),
        });
    let x = res
        .into_iter()
        .enumerate()
        .filter(|x| x.1 == 0)
        .map(|x| x.0)
        .next()
        .unwrap();
    println!("{}", x + 1);
}
