use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let cost = &lines.next().unwrap().parse::<f64>().unwrap();
    let n = &lines.next().unwrap().parse::<usize>().unwrap();
    let area: f64 = lines
        .take(*n)
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<f64>().unwrap())
                .product::<f64>()
        })
        .sum();
    println!("{:.7}", area * cost);
}
