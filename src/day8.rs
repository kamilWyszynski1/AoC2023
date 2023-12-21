use anyhow::Context;
use std::hash::Hash;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("invalid direction character"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Crossroad {
    id: String,
    left: String,
    right: String,
}

impl Hash for Crossroad {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl From<String> for Crossroad {
    fn from(value: String) -> Self {
        let (id, rest) = value.split_once(" = ").unwrap();

        let binding = rest.replace("(", "").replace(")", "").replace(" ", "");
        let (left, right) = binding.split_once(",").unwrap();

        Self {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut res = 0;

    let mut lines = reader.lines();
    let dirs: Vec<Direction> = lines
        .next()
        .context("no first line")??
        .chars()
        .map(Direction::from)
        .collect();
    lines.next();

    let crossroads: Vec<Crossroad> = lines
        .into_iter()
        .map(Result::unwrap)
        .map(Crossroad::from)
        .collect();

    let mut starting_points = vec![];
    for c in &crossroads {
        if c.id.ends_with("A") {
            starting_points.push(c.id.clone());
        }
    }

    let crossroads: HashMap<String, (String, String)> = crossroads
        .into_iter()
        .map(|c| (c.id, (c.left, c.right)))
        .collect();

    let mut each_steps: Vec<usize> = vec![];
    'point: for (_i, point) in starting_points.iter_mut().enumerate() {
        let mut steps = 0;
        dbg!(&point);
        loop {
            for d in &dirs {
                match d {
                    Direction::Left => *point = crossroads.get(point).unwrap().0.clone(),
                    Direction::Right => *point = crossroads.get(point).unwrap().1.clone(),
                }
                steps += 1;
                if point.ends_with("Z") {
                    each_steps.push(steps);
                    continue 'point;
                }
            }
        }
    }

    let mut curr = 1;
    for step in each_steps {
        curr = lcm(curr, step)
    }

    println!("result day8 b: {}", curr);
    Ok(())
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let res = 0;

    let mut lines = reader.lines();
    let dirs: Vec<Direction> = lines
        .next()
        .context("no first line")??
        .chars()
        .map(Direction::from)
        .collect();
    lines.next();

    let crossroads: Vec<Crossroad> = lines
        .into_iter()
        .map(Result::unwrap)
        .map(Crossroad::from)
        .collect();

    let mut curr = "AAA";

    let crossroads: HashMap<String, (String, String)> = crossroads
        .into_iter()
        .map(|c| (c.id, (c.left, c.right)))
        .collect();

    let mut steps = 0;
    while curr != "ZZZ" {
        for d in &dirs {
            match d {
                Direction::Left => curr = &crossroads.get(curr).unwrap().0,
                Direction::Right => curr = &crossroads.get(curr).unwrap().1,
            }
            steps += 1;
        }
    }

    println!("result day8 a: {}", steps);
    Ok(())
}
