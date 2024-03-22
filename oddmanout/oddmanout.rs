use std::collections::HashSet;
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

fn main() {
    solution(stdin().lock(), &mut stdout().lock());
}

pub fn solution(reader: impl Read, writer: &mut impl Write) {
    let mut lines = BufReader::new(reader).lines().map(|x| x.unwrap());

    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let out = lines
        .skip(1)
        .step_by(2)
        .take(n)
        .enumerate()
        .map(|(i, x)| {
            let mut set = HashSet::new();
            x.split(" ").for_each(|x| {
                if !set.remove(&x) {
                    set.insert(x);
                }
            });
            let x = set.into_iter().next().unwrap();
            format!("Case #{}: {}", i + 1, x)
        })
        .collect::<Vec<String>>()
        .join("\n");
    writeln!(writer, "{}", out);
}
