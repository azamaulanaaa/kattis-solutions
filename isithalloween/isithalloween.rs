use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|x| x.unwrap());
    let value = lines.next().unwrap();
    if (value == "OCT 31") || (value == "DEC 25") {
        println!("yup");
    } else {
        println!("nope");
    }
}
