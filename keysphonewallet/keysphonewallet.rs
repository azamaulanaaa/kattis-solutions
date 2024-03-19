fn main() {
    let mut set = std::collections::HashSet::from(["keys", "phone", "wallet"]);
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<usize>().unwrap();
    lines.take(n).for_each(|x| {
        set.remove(x.as_str());
    });
    match set.len() {
        0 => println!("ready"),
        _ => {
            let mut out = set.into_iter().collect::<Vec<_>>();
            out.sort();
            let out = out.join("\n");
            println!("{}", out);
        }
    }
}
