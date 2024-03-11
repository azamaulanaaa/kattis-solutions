fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let chars = lines.next().unwrap().chars().collect::<Vec<char>>();
    let mut set = std::collections::HashSet::new();
    for c in chars.iter() {
        set.insert(c);
    }
    if chars.len() == set.len() {
        println!("1");
    } else {
        println!("0");
    }
}
