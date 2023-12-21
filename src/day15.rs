use anyhow::{bail, Context, Error};
use core::num;
use itertools::Itertools;
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

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let s = std::fs::read_to_string(path)?;

    let res: usize = s.split(",").map(hash).sum();

    println!("result day15a: {res}",);
    Ok(())
}

enum Op {
    Equal,
    Minus,
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let s = std::fs::read_to_string(path)?;

    let res: Vec<(&str, Op, Option<usize>)> = s
        .split(",")
        .map(|s| {
            if let Some((label, lens)) = s.split_once("=") {
                (label, Op::Equal, Some(lens.parse::<usize>().unwrap()))
            } else if let Some((label, _)) = s.split_once("-") {
                (label, Op::Minus, None)
            } else {
                unreachable!()
            }
        })
        .collect_vec();

    let mut state = HashMap::new();

    for (label, op, lens) in res {
        let box_id = hash(label);
        match op {
            Op::Equal => {
                let lens = lens.unwrap();

                state
                    .entry(box_id)
                    .and_modify(|e: &mut Vec<(&str, usize)>| {
                        for (la, le) in e.iter_mut() {
                            if la.to_string() == label.to_string() {
                                *le = lens;
                                return;
                            }
                        }
                        e.push((label, lens))
                    })
                    .or_insert(vec![(label, lens)]);
            }
            Op::Minus => {
                state
                    .entry(box_id)
                    .and_modify(|e: &mut Vec<(&str, usize)>| {
                        if let Some((inx, _)) = e
                            .iter()
                            .find_position(|(la, _)| la.to_string() == label.to_string())
                        {
                            e.remove(inx);
                        }
                    });
            }
        }
    }

    let mut res = 0;

    for (box_id, leneses) in state {
        for (inx, (_, lens)) in leneses.iter().enumerate() {
            res += (box_id + 1) * (inx + 1) * lens;
        }
    }

    println!("result day15b: {res}",);
    Ok(())
}

fn hash<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().chars().into_iter().fold(0, |acc, ch| {
        let mut acc = acc + ch as usize;
        acc *= 17;
        acc %= 256;

        acc
    })
}

mod tests {
    use crate::day15::hash;

    #[test]
    fn test_hash() {
        println!("{}", hash("cm"));
        assert_eq!(0, hash("rn"));
        assert_eq!(30, hash("rn=1"));
        assert_eq!(253, hash("cm-"));
        assert_eq!(97, hash("qp=3"));
    }
}
