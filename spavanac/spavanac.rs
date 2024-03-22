use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

fn main() {
    solution(stdin().lock(), &mut stdout().lock());
}

pub fn solution(reader: impl Read, writer: &mut impl Write) {
    let mut lines = BufReader::new(reader).lines().map(|x| x.unwrap());
    let params = lines
        .by_ref()
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<isize>().unwrap())
        .take(2)
        .collect::<Vec<_>>();
    let m = params[0] * 60 + params[1] - 45;
    let h = (m as f32 / 60.0).floor().rem_euclid(24.0);
    let m = m.rem_euclid(60);
    writeln!(writer, "{} {}", h, m);
}
