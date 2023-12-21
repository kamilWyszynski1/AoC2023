use anyhow::{bail, Context, Error, Ok};
use core::num;
use itertools::{Either, Itertools};
use pathfinding::matrix::directions::{E, N};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::vec;
use std::{
    any,
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Index, Sub},
    path::Path,
};

#[derive(Debug)]
struct Container {
    values: HashMap<char, usize>,
}

enum Cond {
    Gt,
    Lt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rating {
    Accepted,
    Rejected,
    Unknown, // special case for Condition when we need to skip condition and check another one
}

struct Eval {
    id: String,
    conds: Vec<Condition>,
}

trait Evaluable {
    fn eval(&self, container: &Container, m: &HashMap<String, Box<dyn Evaluable>>) -> Rating;
}

struct Passer(String);

impl Evaluable for Passer {
    fn eval(&self, container: &Container, m: &HashMap<String, Box<dyn Evaluable>>) -> Rating {
        let Passer(pass_id) = self;
        // println!("passer with {pass_id}");

        match pass_id.as_str() {
            "R" => Rating::Rejected,
            "A" => Rating::Accepted,
            _ => m
                .get(pass_id)
                .context("no registered evaluable")
                .unwrap()
                .eval(container, m),
        }
    }
}

struct Conditions((String, Vec<Box<dyn Evaluable>>));

impl Evaluable for Conditions {
    fn eval(&self, container: &Container, m: &HashMap<String, Box<dyn Evaluable>>) -> Rating {
        let Conditions((id, conditions)) = self;

        for c in conditions {
            let rating = c.eval(container, m);
            // println!("{id} conditions evaluated {:?}", rating);
            match rating {
                Rating::Accepted | Rating::Rejected => {
                    return rating;
                }
                Rating::Unknown => continue,
            }
        }
        panic!("should not happend")
    }
}

struct Condition {
    field: char,
    cond: Cond,
    value: usize,

    if_true: Either<String, Rating>, // condition id or rating
}

impl Evaluable for Condition {
    fn eval(&self, container: &Container, m: &HashMap<String, Box<dyn Evaluable>>) -> Rating {
        let container_value = container
            .values
            .get(&self.field)
            .context("container has not value")
            .unwrap();

        let evaled = match &self.cond {
            Cond::Gt => self.value < *container_value,
            Cond::Lt => self.value > *container_value,
        };

        // println!("evaled: {evaled}");

        if evaled {
            match &self.if_true {
                Either::Left(pass_id) => Passer(pass_id.to_string()).eval(container, m),
                Either::Right(rating) => *rating,
            }
        } else {
            Rating::Unknown // pass to next condition
        }
    }
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let mut m: HashMap<String, Box<dyn Evaluable>> = HashMap::new();
    let mut after_empty_line = false;

    let mut data = vec![];

    for l in BufReader::new(file).lines().map(Result::unwrap) {
        if after_empty_line {
            let mut l: HashMap<char, usize> = l[1..l.len() - 1]
                .to_string()
                .split(",")
                .map(|v| v.split_once("=").unwrap())
                .map(|(field, value)| {
                    (
                        field.chars().next().unwrap(),
                        value.parse::<usize>().unwrap(),
                    )
                })
                .collect();
            data.push(Container { values: l })
        } else {
            if l.is_empty() {
                after_empty_line = true;
                continue;
            }
            let (id, rest) = l.split_once("{").unwrap();
            let rest = rest.replace("}", "");

            let mut evals: Vec<Box<dyn Evaluable>> = vec![];

            for sub in rest.split(",") {
                if !sub.contains(":") {
                    // rating
                    evals.push(Box::new(Passer(sub.to_string())));
                    continue;
                }

                let mut chars = sub.chars();
                let field = chars.next().unwrap();
                let cond = match chars.next().unwrap() {
                    '>' => Cond::Gt,
                    '<' => Cond::Lt,
                    _ => panic!("invalid cond"),
                };

                let (value, what_to_do) = chars.as_str().split_once(":").unwrap();

                let if_true = match what_to_do {
                    "A" => Either::Right(Rating::Accepted),
                    "R" => Either::Right(Rating::Rejected),
                    _ => Either::Left(what_to_do.to_string()),
                };

                evals.push(Box::new(Condition {
                    field,
                    cond,
                    value: value.parse::<usize>().unwrap(),
                    if_true,
                }))
            }
            m.insert(
                id.to_string(),
                Box::new(Conditions((id.to_string(), evals))),
            );
        }
    }

    let start = m.get("in").unwrap();
    let mut res = 0;
    for (i, workload) in data.into_iter().enumerate() {
        // println!("{i} EVAL");
        let rating = start.eval(&workload, &m);

        if Rating::Accepted == rating {
            res += workload.values.values().sum::<usize>();
        }

        // println!("data: {:?} with {:?} rating", workload, rating);
        // println!()
    }

    println!("day19 a: {res}");

    Ok(())
}

#[derive(Debug)]
struct Node {
    id: String,
    cond: String,

    result: Either<Vec<Box<Node>>, Rating>,
}

impl Node {
    fn print(&self, mut ident: String) {
        println!("{} | {}  ", self.id, self.cond);

        ident.push_str(" ");
        match &self.result {
            Either::Left(children) => {
                for ch in children {
                    ch.print(ident.clone())
                }
            }
            Either::Right(rating) => println!("rating: {:?}", rating),
        }
    }
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let mut lines = HashMap::new();

    for l in BufReader::new(file).lines().map(Result::unwrap) {
        if l.is_empty() {
            break;
        }
        let (id, rest) = l.split_once("{").unwrap();
        let rest = rest.replace("}", "");

        lines.insert(id.to_string(), rest);
    }

    let start_line = lines.get("in").unwrap().to_string();

    let root = parse_line("in".to_string(), &start_line, &lines);

    // let mut start = Node{}

    dbg!(&root);
    root.print("".to_string());
    walk("".to_string(), &root);
    println!("day19 b");

    Ok(())
}

fn parse_line(id: String, rest: &String, lines: &HashMap<String, String>) -> Node {
    let parts = rest.split(",");

    let mut children = vec![];

    let mut current_cond = String::new();
    for part in parts {
        match part.split_once(":") {
            Some((left, right)) => {
                let res = match right {
                    "R" => Either::Right(Rating::Rejected),
                    "A" => Either::Right(Rating::Accepted),
                    _ => {
                        let nrest = lines.get(right).unwrap();
                        Either::Left(vec![Box::new(parse_line(right.to_string(), nrest, lines))])
                    }
                };

                if !current_cond.is_empty() {
                    current_cond.push_str(" && ");
                }
                let mut current_local = current_cond.clone();
                current_local.push_str(left);
                current_cond.push_str(&negate(left.to_string()));

                children.push(Box::new(Node {
                    id: "".to_string(),
                    cond: current_local.clone(),
                    result: res,
                }))
            }
            None => {
                match match part {
                    "R" => Some(Rating::Rejected),
                    "A" => Some(Rating::Accepted),
                    _ => None,
                } {
                    Some(rating) => children.push(Box::new(Node {
                        id: "".to_string(),
                        cond: current_cond.clone(),
                        result: Either::Right(rating),
                    })),
                    None => children.push(Box::new(Node {
                        id: "".to_string(),
                        cond: current_cond.clone(),
                        result: Either::Left(vec![Box::new(parse_line(
                            part.to_string(),
                            lines.get(part).unwrap(),
                            lines,
                        ))]),
                    })),
                }
            }
        }
    }
    Node {
        id,
        cond: "".to_string(),
        result: Either::Left(children),
    }
}

fn negate(cond: String) -> String {
    if cond.contains(">") {
        cond.replace(">", "<=")
    } else {
        cond.replace("<", ">=")
    }
}

fn walk(cond: String, root: &Node) {
    match &root.result {
        Either::Left(children) => {
            for ch in children {
                let mut ncond = cond.clone();
                if !ch.cond.is_empty() {
                    if !ncond.is_empty() {
                        ncond.push_str(" && ")
                    }
                    ncond.push_str(&ch.cond)
                }
                walk(ncond, &ch)
            }
        }
        Either::Right(rating) => {
            if *rating == Rating::Accepted {
                println!("cond: {} for rating: {:?}", cond, rating)
            }
        }
    }
}

fn parse_condition(s: &String) {
    let mut res = 1;
    for c in s.split(" && ") {
        let mut chrs = c.chars();
        let value = chrs.next().unwrap();

        chrs.next();
        let sign = if c.contains("=") {
            chrs.next();
            c[1..3].to_string()
        } else {
            c[1..2].to_string()
        };

        let number: usize = chrs.collect::<String>().parse::<usize>().unwrap();

        let rest = match sign.as_str() {
            ">" => 4000 - number,
            ">=" => 4000 - number + 1,
            "<" => number - 1,
            "<=" => number,
            _ => panic!("invalid sign"),
        };

        res *= rest;
    }

    println!("{res}");
}
