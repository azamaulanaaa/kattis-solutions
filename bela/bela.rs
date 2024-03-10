use std::{collections::HashMap, io::stdin};

fn main() {
    let table = HashMap::from([
        ('A', (11, 11)),
        ('K', (4, 4)),
        ('Q', (3, 3)),
        ('J', (20, 2)),
        ('T', (10, 10)),
        ('9', (14, 0)),
        ('8', (0, 0)),
        ('7', (0, 0)),
    ]);

    let mut lines = stdin().lines().map(|x| x.unwrap());

    let firstline = &lines.next().unwrap();
    let params = firstline.split_once(" ").unwrap();
    let n = params.0.parse::<usize>().unwrap();
    let dominant = params.1.chars().next().unwrap();

    let out: usize = lines
        .take(n * 4)
        .map(|x| {
            let mut x = x.chars();
            let values = table.get(&x.next().unwrap()).unwrap();
            if x.next().unwrap() == dominant {
                values.0
            } else {
                values.1
            }
        })
        .sum();
    println!("{}", out);
}
