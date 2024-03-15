fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let params = lines
        .by_ref()
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let result = lines
        .take(params[0])
        .filter(|x| {
            let x = x.parse::<usize>().unwrap();
            (x + 14 <= params[1] + params[2])
        })
        .count();
    println!("{}", result);
}
