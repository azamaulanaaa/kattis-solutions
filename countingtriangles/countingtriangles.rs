use std::collections::HashMap;

const EPSILON: f64 = 1e-6;

fn is_between(x: f64, a: f64, b: f64) -> bool {
    (x - a.min(b)) > -EPSILON && (a.max(b) - x) > -EPSILON
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn is_in(&self, segment: &Segment) -> bool {
        (segment.coef_x() * self.x + segment.coef_y() * self.y + segment.constant()).abs() < EPSILON
            && is_between(self.x, segment.0.x, segment.1.x)
            && is_between(self.y, segment.0.y, segment.1.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

#[derive(Copy, Clone)]
pub struct Segment(pub Point, pub Point);

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Segment {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self(Point::new(x0, y0), Point::new(x1, y1))
    }

    pub fn coef_x(&self) -> f64 {
        self.1.y - self.0.y
    }

    pub fn coef_y(&self) -> f64 {
        self.0.x - self.1.x
    }

    pub fn constant(&self) -> f64 {
        -self.coef_x() * self.0.x - self.coef_y() * self.0.y
    }

    pub fn uniquely_intersect_at(&self, segment: &Segment) -> Option<Point> {
        let d = self.coef_x() * segment.coef_y() - segment.coef_x() * self.coef_y();
        let dx = self.coef_y() * segment.constant() - segment.coef_y() * self.constant();
        let dy = segment.coef_x() * self.constant() - self.coef_x() * segment.constant();

        if d.abs() < EPSILON {
            return None;
        }
        let point = Point::new(dx / d, dy / d);
        if point.is_in(self) && point.is_in(segment) {
            return Some(point);
        } else {
            return None;
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|e| e.unwrap());

    while let Some(n) = lines.next().map(|e| e.parse::<usize>().unwrap()) {
        if n == 0 {
            break;
        }

        let segments = (0..n)
            .map(|_| {
                let values = lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .take(4)
                    .map(|e| e.parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
                Segment::new(values[0], values[1], values[2], values[3])
            })
            .collect::<Vec<_>>();

        let mut cache = HashMap::<(usize, usize), Option<Point>>::new();
        let mut get_intersection = |idx1: usize, idx2: usize| -> Option<Point> {
            let key = (idx1.min(idx2), idx1.max(idx2));
            *cache
                .entry(key)
                .or_insert_with(|| segments[key.0].uniquely_intersect_at(&segments[key.1]))
        };

        let mut count = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let a = get_intersection(i, j);
                if let Some(a) = a {
                    for k in (j + 1)..n {
                        let b = get_intersection(i, k);
                        let c = get_intersection(j, k);
                        match (b, c) {
                            (Some(b), Some(c)) => {
                                if a != b && a != c && b != c {
                                    count = count + 1;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        println!("{}", count);
    }
}
