fn main() {
    let table = std::collections::HashMap::from([
        ('.', 20),
        ('O', 10),
        ('\\', 25),
        ('/', 25),
        ('A', 35),
        ('^', 5),
        ('v', 22),
    ]);
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let params = lines
        .by_ref()
        .next()
        .unwrap()
        .split(" ")
        .take(2)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let out: usize = lines
        .take(params[0])
        .map(|x| x.chars().filter_map(|x| table.get(&x)).sum::<usize>())
        .sum();
    println!("{}", out);
}
