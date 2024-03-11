fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = &mut lines.next().unwrap().parse::<usize>().unwrap();
    let all = &lines
        .next()
        .unwrap()
        .split(" ")
        .take(*n)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let known = &lines
        .next()
        .unwrap()
        .split(" ")
        .take(*n - 1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for x in all {
        if known.contains(x) == false {
            println!("{}", x);
            return;
        }
    }
}
