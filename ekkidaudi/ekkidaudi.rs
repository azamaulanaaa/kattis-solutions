fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let values = lines.take(2).collect::<Vec<String>>();
    for (x, y) in values[0].split("|").zip(values[1].split("|")) {
        print!("{}{} ", x, y);
    }
}
