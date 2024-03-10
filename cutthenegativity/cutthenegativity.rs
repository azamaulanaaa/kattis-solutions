use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    let values = lines
        .enumerate()
        .map(|(i, x)| {
            x.split(" ")
                .enumerate()
                .map(|(j, x)| (i + 1, j + 1, x.parse::<isize>().unwrap()))
                .filter(|x| x.2 != -1)
                .take(n)
                .collect::<Vec<(usize, usize, isize)>>()
        })
        .take(n)
        .flatten()
        .collect::<Vec<(usize, usize, isize)>>();

    let n = values.len();
    let out = values
        .into_iter()
        .map(|x| format!("{} {} {}", x.0, x.1, x.2))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}\n{}", n, out);
}
