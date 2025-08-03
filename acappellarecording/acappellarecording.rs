fn main() {
    let mut lines = std::io::stdin().lines().map(|e| e.unwrap());

    let (n, d) = {
        let params = lines
            .next()
            .unwrap()
            .split(" ")
            .take(2)
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (params[0], params[1])
    };

    let mut notes = lines
        .take(n)
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    notes.sort();

    let step = notes
        .iter()
        .fold((1, notes[0]), |acc, &pitch| {
            if acc.1.abs_diff(pitch) > d {
                (acc.0 + 1, pitch)
            } else {
                acc
            }
        })
        .0;

    println!("{}", step);
}
