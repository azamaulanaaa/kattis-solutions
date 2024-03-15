fn main() {
    let table = [
        ("Farvidri", 32.7),
        ("Ofsavedur", 28.5),
        ("Rok", 24.5),
        ("Stormur", 20.8),
        ("Hvassvidri", 17.2),
        ("Allhvass vindur", 13.9),
        ("Stinningskaldi", 10.8),
        ("Kaldi", 8.0),
        ("Stinningsgola", 5.5),
        ("Gola", 3.4),
        ("Kul", 1.6),
        ("Andvari", 0.3),
        ("Logn", 0.0),
    ];
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let n = lines.by_ref().next().unwrap().parse::<f32>().unwrap();
    for (name, min) in table.iter() {
        if n >= *min {
            println!("{}", name);
            return;
        }
    }
}
