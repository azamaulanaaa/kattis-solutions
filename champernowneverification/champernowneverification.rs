fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut value = lines.next().unwrap();

    let mut i = 0;
    loop {
        i += 1;
        let i_string = &i.to_string();
        if !value.starts_with(i_string) {
            if value.len() == 0 {
                println!("{}", i - 1);
            } else {
                println!("{}", -1);
            }
            return;
        }
        value = value[i_string.len()..].to_string();
    }
}
