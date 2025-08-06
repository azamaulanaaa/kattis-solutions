use std::array;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::mem;
use std::ops::{Add, Deref};

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

struct SquareMatrix<T, const N: usize>([[T; N]; N]);

impl<T, const N: usize> SquareMatrix<T, N> {
    pub fn size(&self) -> usize {
        N
    }

    pub fn transpose(self) -> Self {
        let mut matrix = self.0;

        for i in 0..N {
            for j in (i + 1)..N {
                let (first, second) = matrix.split_at_mut(i + 1);
                mem::swap(&mut first[i][j], &mut second[j - i - 1][i]);
            }
        }

        SquareMatrix(matrix)
    }

    pub fn flip_y_axis(self) -> Self {
        let matrix = self.0.map(|e| {
            let mut e = e;
            e.reverse();
            e
        });

        SquareMatrix(matrix)
    }

    pub fn flip_x_axis(self) -> Self {
        let mut matrix = self.0;
        matrix.reverse();

        SquareMatrix(matrix)
    }

    pub fn rotate_left(self) -> Self {
        self.flip_y_axis().transpose()
    }

    pub fn rotate_right(self) -> Self {
        self.transpose().flip_y_axis()
    }
}

impl<T, const N: usize> Deref for SquareMatrix<T, N> {
    type Target = [[T; N]; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> TryFrom<Vec<Vec<T>>> for SquareMatrix<T, N> {
    type Error = String;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        if value.iter().any(|e| e.len() != N) {
            return Err(String::from("matrix size is not NxN").into());
        }
        let mut value = value;

        Ok(SquareMatrix(array::from_fn(|i| {
            array::from_fn(|_| value[i].remove(0))
        })))
    }
}

impl<T: Display, const N: usize> Display for SquareMatrix<T, N> {
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

impl<T: Add<Output = T> + Default + Eq, const N: usize> SquareMatrix<T, N> {
    fn slide_to_left(self) -> Self {
        let matrix = self.0.map(|row| {
            let mut row = row;

            for i in 0..N - 1 {
                for j in i..i + 2 {
                    if let Some(k) = row[j..].iter().position(|e| *e != T::default()) {
                        row.swap(j, j + k);
                    } else {
                        break;
                    }
                }

                if row[i] == row[i + 1] {
                    row[i] = mem::take(&mut row[i]) + mem::take(&mut row[i + 1]);
                }
            }

            row
        });

        SquareMatrix(matrix)
    }

    pub fn slide(self, direction: Direction) -> Self {
        let matrix = match direction {
            Direction::Left => self.slide_to_left(),
            Direction::Up => self.rotate_left().slide_to_left().rotate_right(),
            Direction::Right => self.flip_y_axis().slide_to_left().flip_y_axis(),
            Direction::Down => self.rotate_right().slide_to_left().rotate_left(),
        };

        matrix
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = std::io::stdin().lines().into_iter();
    let lines = lines.take(5).collect::<Result<Vec<String>, _>>()?;

    let (direction, matrix) = lines.split_last().unwrap();

    let direction = Direction::try_from(direction.as_str())?;
    let matrix = {
        let matrix = matrix
            .into_iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|v| v.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let matrix = SquareMatrix::<_, 4>::try_from(matrix)?;
        matrix
    };

    let matrix = matrix.slide(direction);

    println!("{}", matrix);

    Ok(())
}
