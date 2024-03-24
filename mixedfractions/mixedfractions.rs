fn main() {
    let mut lines = std::io::stdin().lines().filter_map(|x| {
        let x = x.unwrap();
        if x.as_str() == "0 0" {
            None
        } else {
            Some(x)
        }
    });
    for line in lines {
        let (n, d) = {
            let mut values = line.split(" ").take(2).map(|x| x.parse::<usize>().unwrap());
            (
                values.by_ref().next().unwrap(),
                values.by_ref().next().unwrap(),
            )
        };
        let a = n / d;
        let b = n % d;
        println!("{} {} / {}", a, b, d);
    }
}
