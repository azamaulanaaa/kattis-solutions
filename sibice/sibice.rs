use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());

    let params = &lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let w = params[1] as f64;
    let h = params[2] as f64;

    let max = f64::sqrt((w * w) + (h * h));

    let out = lines
        .map(|x| {
            let x = x.parse::<f64>().unwrap();
            if x <= max {
                return "DA";
            } else {
                return "NE";
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");
    println!("{}", out);
}
