use std::io::stdin;

fn main() {
    let lines = stdin()
        .lines()
        .map(|x| x.unwrap())
        .take(2)
        .collect::<Vec<String>>();

    let gtn = lines[0]
        .split(" ")
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    let items = lines[1]
        .split(" ")
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let total_items_weight = items.into_iter().take(gtn[2] as usize).sum::<f64>();
    let max_truct = (0.9 * (gtn[0] - gtn[1])) - total_items_weight;
    println!("{:0}", max_truct);
}
