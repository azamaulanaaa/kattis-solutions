use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

fn main() {
    solution(stdin().lock(), &mut stdout().lock())
}

pub fn solution(reader: impl Read, writer: &mut impl Write) {
    let mut lines = BufReader::new(reader).lines().map(|x| x.unwrap());
    let params = lines
        .by_ref()
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let h = params[0];
    let v = params[1];
    let out = (h / (v * std::f32::consts::PI / 180.0).sin()).ceil();
    writeln!(writer, "{:}", out);
}
