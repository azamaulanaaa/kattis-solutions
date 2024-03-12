fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let dict = &lines.next().unwrap().chars().collect::<Vec<char>>();
    let encoded = &lines.next().unwrap();
    let mut result = Vec::new();
    for i in (0..encoded.len()).step_by(3) {
        let index = (encoded[i..i + 3]).parse::<usize>().unwrap();
        result.push(dict[index - 1]);
    }
    println!("{}", result.iter().collect::<String>());
}
