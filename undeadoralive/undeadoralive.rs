fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let value = &lines.next().unwrap();
    let smile = value.contains(":)");
    let frowny = value.contains(":(");
    if smile && frowny {
        println!("double agent");
    } else if smile {
        println!("alive")
    } else if frowny {
        println!("undead")
    } else {
        println!("machine")
    }
}
