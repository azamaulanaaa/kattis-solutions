use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines().map(|e| e.unwrap());

    let (n, m) = {
        let params = lines
            .next()
            .unwrap()
            .split(" ")
            .take(2)
            .map(|e| e.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        (params[0], params[1])
    };

    let mut rules = HashMap::<String, usize>::new();
    for _ in 0..n {
        let (keyword, score) = {
            let rule = lines
                .next()
                .unwrap()
                .split(" ")
                .take(2)
                .map(|e| String::from(e))
                .collect::<Vec<_>>();
            (rule[0].clone(), rule[1].parse::<usize>().unwrap())
        };

        let _ = rules.insert(keyword, score);
    }

    for _ in 0..m {
        let mut score = 0;

        while let Some(desc) = lines.next() {
            if desc == "." {
                break;
            }

            score = score
                + desc
                    .split(" ")
                    .filter_map(|word| rules.get(word))
                    .sum::<usize>();
        }

        println!("{}", score);
    }
}
