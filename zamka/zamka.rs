use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

fn main() {
    solution(stdin().lock(), &mut stdout().lock());
}

pub fn solution(reader: impl Read, writer: &mut impl Write) {
    let lines = BufReader::new(reader).lines().map(|x| x.unwrap());
    let (l, d, x) = {
        let params = lines
            .take(3)
            .map(|x| x.parse::<usize>().unwrap_or(0))
            .collect::<Vec<_>>();
        (params[0], params[1], params[2])
    };
    let out = (l..=d).fold((0, 0), |res, a| {
        let mut res = res;
        let b = sum_digit(a);

        if b == x {
            if res.0 == 0 {
                res.0 = a;
            }
            res.1 = a;
        }

        res
    });
    writeln!(writer, "{}\n{}", out.0, out.1).unwrap();
}

pub fn sum_digit(x: usize) -> usize {
    let x = x
        .to_string()
        .chars()
        .map(|x| String::from(x).parse::<usize>().unwrap())
        .sum();
    return x;
}
