use itertools::Itertools;
use std::fmt::Debug;
use std::vec;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let data = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let mut s = l.split(" ");
            (
                s.next().unwrap().to_string(),
                s.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect_vec();

    let mut cords = vec![];

    let mut curr = Coordinate { x: 0, y: 0 };

    for (dir, distance) in data {
        match dir.as_str() {
            "R" => curr.y += distance as i32,
            "L" => curr.y -= distance as i32,
            "U" => curr.x -= distance as i32,
            "D" => curr.x += distance as i32,
            _ => panic!("invalid dir"),
        }
        cords.push(curr.clone());
    }

    dbg!(&cords);

    println!("day17 part a: {}", polygon_area(&cords));

    Ok(())
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let data = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let s = l.split(" ");
            let last = s.last().unwrap().replace("(", "").replace(")", "");
            let hex: String = Into::<String>::into(&last[..last.len() - 1]).replace("#", "");
            let dir_char: char = last.chars().last().unwrap();

            let dir_string = match dir_char {
                '0' => "R",
                '1' => "D",
                '2' => "L",
                '3' => "U",
                _ => panic!("invalid dir char"),
            }
            .to_string();

            dbg!(&hex);

            (dir_string, i32::from_str_radix(&hex, 16).unwrap())
        })
        .collect_vec();

    let mut cords = vec![];

    let mut curr = Coordinate { x: 0, y: 0 };

    for (dir, distance) in data {
        match dir.as_str() {
            "R" => curr.y += distance as i32,
            "L" => curr.y -= distance as i32,
            "U" => curr.x -= distance as i32,
            "D" => curr.x += distance as i32,
            _ => panic!("invalid dir"),
        }
        cords.push(curr.clone());
    }

    dbg!(&cords);

    println!("day17 part b: {}", polygon_area(&cords));

    Ok(())
}

#[derive(Debug, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(&self, another: &Self) -> i32 {
        (self.x - another.x).abs() + (self.y - another.y).abs()
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn polygon_area(cords: &Vec<Coordinate>) -> f64 {
    let mut new_cords = cords.clone();
    new_cords.push(new_cords.first().unwrap().clone());

    let mut sum: f64 = 0.;
    let mut border: f64 = 0.;
    for pair in new_cords.windows(2) {
        let (first, second) = (&pair[0], &pair[1]);
        border += first.distance(second) as f64;

        let x1 = first.x as f64;
        let y1 = first.y as f64;
        let x2 = second.x as f64;
        let y2 = second.y as f64;
        sum += (y1 + y2) * (x1 - x2);

        dbg!(border);
    }

    //  sum.abs() / 2 - boundary_count / 2 + 1 + boundary_count
    sum.abs() / 2. - border / 2. + 1. + border
    // (sum.abs()) + (border) / 2. + 1.
}

fn shoelace_formula(points: &Vec<Coordinate>) -> i32 {
    let mut s1 = 0;
    let mut s2 = 0;
    for positions in points.windows(2) {
        s1 += positions[0].x * positions[1].y;
        s2 += positions[1].x * positions[0].y;
    }
    let area = (s1 - s2).abs() / 2;
    let perimeter = (points.len() - 1) as i32;
    area - perimeter / 2 + 1
}

mod tests {
    use super::Coordinate;
    use crate::day18::{polygon_area, shoelace_formula};

    #[test]
    fn test_polygon_area() {
        let cords: Vec<Coordinate> = vec![(0, 0), (0, 6), (2, 6), (2, 0)]
            .into_iter()
            .map(|v| v.into())
            .collect();

        assert_eq!(21., polygon_area(&cords));

        let cords: Vec<Coordinate> = vec![
            (0, 0),
            (0, 6),
            (5, 6),
            (5, 4),
            (7, 4),
            (7, 6),
            (9, 6),
            (9, 1),
            (7, 1),
            (7, 0),
            (5, 0),
            (5, 2),
            (2, 2),
            (2, 0),
        ]
        .into_iter()
        .map(|v| v.into())
        .collect();

        assert_eq!(62., polygon_area(&cords))
    }
}
