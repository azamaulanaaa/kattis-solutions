fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();

    let mut out = 0;
    let mut res = n;
    for i in 1..res {
        match res.checked_sub(((2 * i) - 1).pow(2)) {
            Some(x) => {
                res = x;
                out += 1
            }
            None => break,
        }
    }
    println!("{}", out);
}
