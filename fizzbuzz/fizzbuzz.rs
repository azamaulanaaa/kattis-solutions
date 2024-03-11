use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let params = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let out = (1..=params[2])
        .map(|x| {
            if (x % params[0] == 0) && (x % params[1] == 0) {
                String::from("FizzBuzz")
            } else if x % params[0] == 0 {
                String::from("Fizz")
            } else if x % params[1] == 0 {
                String::from("Buzz")
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", out);
}
