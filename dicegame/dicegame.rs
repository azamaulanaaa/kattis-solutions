use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};

fn main() {
    solution(stdin().lock(), &mut stdout().lock());
}

pub fn solution(reader: impl Read, writer: &mut impl Write) {
    let mut lines = BufReader::new(reader).lines().map(|x| x.unwrap());
    let dices = lines
        .take(2)
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let gunnar = (dices[0][0] + dices[0][2], dices[0][1] + dices[0][3]);
    let emma = (dices[1][0] + dices[1][2], dices[1][1] + dices[1][3]);

    let out = gunnar.0 - emma.0 + (gunnar.1 - emma.1);

    let out = match out {
        1.. => "Gunnar",
        ..=-1 => "Emma",
        _ => "Tie",
    };
    writeln!(writer, "{}", out);
}
