fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());

    let params = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let lvl = lines.take(params[0]).fold(params[1] as u64, |lvl, x| {
        let mut x = x
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        if (x[0] <= lvl) && (x[1] >= lvl) {
            lvl + 1
        } else {
            lvl
        }
    });
    println!("{}", lvl);
}
