fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    for _ in 0..10 {
        let n = lines.by_ref().next().unwrap().parse::<isize>().unwrap();
        if n < 0 {
            break;
        }
        let out = solution(n as usize, lines.by_ref());
        println!("{} miles", out);
    }
}

fn solution(n: usize, lines: &mut impl Iterator<Item = String>) -> usize {
    let out = lines.take(n).fold((0, 0), |res, x| {
        let x = x
            .split(" ")
            .take(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (res.0 + x[0] * (x[1] - res.1), x[1])
    });
    return out.0;
}
