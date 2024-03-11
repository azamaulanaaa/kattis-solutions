fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let values = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let out = (vec![values[1], values[2]])
        .into_iter()
        .map(|x| {
            let delta = values[0] - x;
            if delta > x {
                delta
            } else {
                x
            }
        })
        .product::<usize>()
        * 4;
    println!("{}", out);
}
