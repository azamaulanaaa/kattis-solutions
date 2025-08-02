fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());

    let s = lines.take(2).last().unwrap();

    let step1: i32 = (s.contains("l") || s.contains("v")).into();
    let step2: i32 = s.contains("lv").into();
    let step = 2 - step1 - step2;

    println!("{}", step)
}
