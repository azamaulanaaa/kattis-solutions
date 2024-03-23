use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

fn main() {
    solution(stdin().lock(), &mut stdout().lock());
}

pub fn solution(reader: impl Read, writer: &mut impl Write) {
    let mut lines = BufReader::new(reader).lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let out = lines
        .take(n)
        .map(|x| {
            x[..x.len() - 2]
                .split(" ")
                .fold((1, 0), |res, y| {
                    let y = y.parse::<usize>().unwrap();
                    let a = (res.0 * 2 + res.1).min(y - res.1);
                    let b = y - a;
                    (a, b)
                })
                .1
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    writeln!(writer, "{}", out).unwrap();
}
