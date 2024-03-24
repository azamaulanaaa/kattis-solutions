use std::collections::HashMap;

fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());

    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();

    let mut count = HashMap::new();
    lines.take(n).for_each(|x| {
        let mut x = x.split(" ").take(5).collect::<Vec<_>>();
        x.sort();
        let x = x.join(" ");
        if let Some(nn) = count.get_mut(&x) {
            *nn += 1;
        } else {
            let _ = count.insert(x, 1);
        }
    });
    let max = count.clone().into_values().max().unwrap();
    let n_max = count.into_values().filter(|x| *x == max).sum::<usize>();
    println!("{}", n_max);
}
