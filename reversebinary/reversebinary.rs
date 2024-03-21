fn main() {
    println!("{}", solution(std::io::stdin().lock()));
}

fn solution(input: impl std::io::BufRead) -> String {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let n_rev = format!("{:b}", n)
        .chars()
        .enumerate()
        .fold(0, |res, x| match x {
            (i, '1') => res + 2_usize.pow(i as u32),
            _ => res,
        });
    return format!("{}", n_rev);
}
