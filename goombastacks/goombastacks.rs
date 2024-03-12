fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let mut total = 0;
    let res = lines
        .take(n)
        .filter_map(|x| {
            let mut x = x.split(" ").map(|x| x.parse::<usize>().unwrap());
            total += x.by_ref().next().unwrap();
            if total >= x.by_ref().next().unwrap() {
                Some(())
            } else {
                None
            }
        })
        .count();
    if res == n {
        println!("possible");
    } else {
        println!("impossible");
    }
}
