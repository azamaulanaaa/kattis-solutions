fn main() {
    let mut lines = std::io::stdin().lines().map(|e| e.unwrap());

    while let Some(line) = lines.next() {
        let (a, b) = {
            let values = line
                .split(" ")
                .take(2)
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (values[0], values[1])
        };
        let diff = a.abs_diff(b);

        println!("{}", diff);
    }
}
