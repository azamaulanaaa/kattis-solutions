fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let params = &lines
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let avg = lines
        .take(params[0])
        .map(|x| x.parse::<f32>().unwrap())
        .sum::<f32>();
    let delta = (3.0 * (params[0] - params[1]) as f32);
    let min = (avg - delta) / (params[0] as f32);
    let max = (avg + delta) / (params[0] as f32);
    println!("{} {}", min, max);
}
