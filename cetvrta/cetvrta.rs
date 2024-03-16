fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let vertices = lines
        .take(3)
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let u = &vertices[0];
    let v = &vertices[1];
    let w = &vertices[2];

    let a = vec![norm_sqr(u) - norm_sqr(v), norm_sqr(v) - norm_sqr(w)];

    let b = vec![(u, v), (v, w)]
        .into_iter()
        .map(|(a, b)| {
            a.into_iter()
                .zip(b.into_iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let adj_b = vec![vec![b[1][1], 0 - b[0][1]], vec![0 - b[1][0], b[0][0]]];
    let det_b = (b[0][0] * b[1][1]) - (b[0][1] * b[1][0]);

    let x = 2 * ((a[0] * adj_b[0][0]) + (a[1] * adj_b[0][1])) / det_b - u[0] - v[0] - w[0];
    let y = 2 * ((a[0] * adj_b[1][0]) + (a[1] * adj_b[1][1])) / det_b - u[1] - v[1] - w[1];

    println!("{} {}", x, y)
}

fn norm_sqr(x: &Vec<isize>) -> isize {
    x.iter().map(|x| x * x).sum()
}
