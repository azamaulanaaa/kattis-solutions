use std::convert::TryFrom;
use std::error::Error;

#[repr(u8)]
enum Direction {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "0" => Direction::Left,
            "1" => Direction::Up,
            "2" => Direction::Right,
            "3" => Direction::Down,
            _ => return Err(String::from("invalid moves")),
        })
    }
}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    let size = matrix.len();
    if matrix.iter().any(|e| e.len() != size) {
        return Err(String::from("matrix size is not NxN").into());
    }

    let mut matrix = matrix
        .into_iter()
        .map(|row| row.into_iter())
        .collect::<Vec<_>>();
    let matrix = (0..size)
        .map(|_| {
            matrix
                .iter_mut()
                .map(|row| row.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(matrix)
}

fn reverse<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    matrix
        .into_iter()
        .map(|row| row.into_iter().rev().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn rotate_90<T>(matrix: Vec<Vec<T>>) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    let matrix = transpose(matrix)?;
    let matrix = reverse(matrix);

    Ok(matrix)
}

fn rotate_270<T>(matrix: Vec<Vec<T>>) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    let matrix = reverse(matrix);
    let matrix = transpose(matrix)?;

    Ok(matrix)
}

fn slide(matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let len = matrix.len();
    matrix
        .into_iter()
        .map(|row| {
            let mut row = row.into_iter().filter(|e| *e != 0).peekable();

            (0..len)
                .map(|_| {
                    let mut current = match row.next() {
                        Some(v) => v,
                        None => return 0,
                    };
                    if let Some(next) = row.next_if_eq(&current) {
                        current = current + next
                    }

                    current
                })
                .collect()
        })
        .collect()
}

fn main() {
    let lines = std::io::stdin().lines().into_iter().map(|e| e.unwrap());
    let lines = lines.take(5).collect::<Vec<String>>();

    let matrix = lines[0..4]
        .iter()
        .map(|line| {
            line.split(" ")
                .take(4)
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let direction = Direction::try_from(lines[4].as_str()).unwrap();

    let repeat = direction as u8;

    let matrix = (0..repeat).fold(matrix, |matrix, _| rotate_270(matrix).unwrap());
    let matrix = slide(matrix);
    let matrix = (0..repeat).fold(matrix, |matrix, _| rotate_90(matrix).unwrap());

    println!(
        "{}",
        matrix
            .into_iter()
            .map(|row| row
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
