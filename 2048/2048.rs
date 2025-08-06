use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;

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

struct SquareMatrix<T>(Vec<Vec<T>>);

impl<T> TryFrom<Vec<Vec<T>>> for SquareMatrix<T> {
    type Error = String;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let size = value.len();
        if value.iter().any(|e| e.len() != size) {
            return Err(String::from("matrix size is not NxN").into());
        }

        Ok(SquareMatrix(value))
    }
}

impl<T: Display> Display for SquareMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.0
                .iter()
                .map(|row| row
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl<T> SquareMatrix<T> {
    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn transpose(self) -> Result<Self, Box<dyn Error>> {
        let size = self.size();

        let mut matrix = self
            .0
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

        Ok(SquareMatrix(matrix))
    }

    pub fn reverse(self) -> Self {
        let matrix = self
            .0
            .into_iter()
            .map(|row| row.into_iter().rev().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        SquareMatrix(matrix)
    }

    pub fn rotate_90(self) -> Result<Self, Box<dyn Error>> {
        let matrix = self.transpose()?.reverse();

        Ok(matrix)
    }

    pub fn rotate_270(self) -> Result<Self, Box<dyn Error>> {
        let matrix = self.reverse().transpose()?;

        Ok(matrix)
    }
}

impl SquareMatrix<usize> {
    pub fn slide(self) -> Self {
        let len = self.size();
        let matrix = self
            .0
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
            .collect();

        SquareMatrix(matrix)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = std::io::stdin().lines().into_iter();
    let lines = lines.take(5).collect::<Result<Vec<String>, _>>()?;

    let matrix = lines[0..4]
        .iter()
        .map(|line| {
            line.split(" ")
                .take(4)
                .map(|v| v.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let matrix = SquareMatrix::try_from(matrix)?;

    let direction = Direction::try_from(lines[4].as_str())?;

    let matrix = match direction {
        Direction::Left => matrix.slide(),
        Direction::Up => matrix.rotate_270()?.slide().rotate_90()?,
        Direction::Right => matrix.reverse().slide().reverse(),
        Direction::Down => matrix.rotate_90()?.slide().rotate_270()?,
    };

    println!("{}", matrix);

    Ok(())
}
