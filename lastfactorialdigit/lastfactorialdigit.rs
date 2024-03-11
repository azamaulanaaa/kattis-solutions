fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = &lines.next().unwrap().parse::<usize>().unwrap();
    let out = lines
        .take(*n)
        .map(|x| {
            let x = x.parse::<usize>().unwrap();

            (1..=x).fold(1, |res, x| (res * x) % 10).to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", out);
}
