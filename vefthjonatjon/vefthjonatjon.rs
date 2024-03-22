use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

fn main() {
    solution(stdin().lock(), stdout().lock());
}

fn solution(reader: impl Read, mut writer: impl Write) {
    let mut lines = BufReader::new(reader).lines().map(|x| x.unwrap());

    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let out = lines
        .take(n)
        .fold(vec![0, 0, 0].into_iter(), |current, x| {
            let x = x.chars().step_by(2);
            current
                .zip(x)
                .map(|(a, b)| match b {
                    'J' => a + 1,
                    _ => a,
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .min()
        .unwrap();

    let _ = writer.write(format!("{}", out).as_bytes());
}
