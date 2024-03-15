fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let name = lines.by_ref().next().unwrap();
    let values = lines
        .take(3)
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let delta = values[0] - values[1];
    if delta >= 0 {
        println!("VEIT EKKI");
    } else if delta == values[2] {
        println!("JEDI");
    } else {
        println!("SITH");
    }
}
