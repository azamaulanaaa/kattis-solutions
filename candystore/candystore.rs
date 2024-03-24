fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());

    let (n, q) = {
        let values = lines
            .by_ref()
            .next()
            .unwrap()
            .split(" ")
            .take(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (values[0], values[1])
    };

    let mut map = std::collections::HashMap::<String, String>::new();
    lines.by_ref().take(n).for_each(|x| {
        let initial = x.clone().replace(|x: char| !x.is_uppercase(), "");
        if map.contains_key(&initial) {
            map.insert(initial, String::from("ambiguous"));
        } else {
            map.insert(initial, x);
        }
    });
    let out = lines
        .by_ref()
        .take(q)
        .map(|x| map.get(&x).unwrap_or(&String::from("nobody")).clone())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", out);
}
