fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let top_bottom = (0..(n + 3))
        .map(|x| {
            if (x == 0) || (x == n + 1) {
                '+'
            } else if (x == n + 2) {
                '\n'
            } else {
                '-'
            }
        })
        .collect::<String>();
    let mid = top_bottom.replace("+", "|").replace("-", " ").repeat(n);
    print!("{}", top_bottom);
    print!("{}", mid);
    print!("{}", top_bottom);
}
