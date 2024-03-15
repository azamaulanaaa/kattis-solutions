fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    let set = lines
        .take(n)
        .filter(|x| (x == "kjuklingur") || (x == "nautakjot"))
        .collect::<std::collections::HashSet<String>>();
    if set.len() == 1 {
        println!("{}", set.iter().next().unwrap())
    } else {
        println!("blandad best");
    }
}
