fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let values = lines
        .next()
        .unwrap()
        .split("()")
        .map(|x| x.len())
        .collect::<Vec<usize>>();
    if values[0] == values[1] {
        println!("correct");
    } else {
        println!("fix");
    }
}
