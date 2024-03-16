fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let params = lines
        .by_ref()
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut value = lines
        .take(params[0])
        .map(|x| {
            let mut x = x.chars().enumerate().collect::<Vec<_>>().repeat(params[3]);
            x.sort_by(|a, b| a.0.cmp(&b.0));
            let mut x = x.into_iter().map(|x| x.1).collect::<String>();
            x.push('\n');
            x.repeat(params[2])
        })
        .collect::<Vec<_>>()
        .join("");
    println!("{}", value);
}
