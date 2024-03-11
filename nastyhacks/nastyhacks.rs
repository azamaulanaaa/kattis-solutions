use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let n = &lines.next().unwrap().parse::<usize>().unwrap();
    let out = lines
        .take(*n)
        .map(|x| {
            let values = x
                .split(" ")
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();
            let no_ads = values[0];
            let ads = values[1] - values[2];
            if no_ads == ads {
                "does not matter"
            } else if no_ads > ads {
                "do not advertise"
            } else {
                "advertise"
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");
    println!("{}", out);
}
