use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (left, right) = l.split_once(" ").unwrap();
            (left.to_string(), right.to_string())
        })
        .map(|(left, right)| {
            (
                left,
                right
                    .split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let mut res = 0;
    for (pattern, code) in lines {
        let mut generated = vec![];
        gen(&pattern, &mut generated);

        for g in generated {
            if to_code(&g) == code {
                res += 1
            }
        }
        // println!("generated values for {pattern}:\n {:?}", generated)
    }

    println!("result day12: {res}",);

    Ok(())
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (left, right) = l.split_once(" ").unwrap();

            let cp = left.clone();
            let cpr = right.clone();
            let mut left = left.to_string();
            let mut right = right.to_string();

            for _ in 0..5 {
                left.push('?');
                left.push_str(cp);

                right.push(',');
                right.push_str(cpr);
            }

            (left.to_string(), right.to_string())
        })
        .map(|(left, right)| {
            (
                left,
                right
                    .split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    dbg!(&lines);

    let res = 0;
    for (pattern, _code) in lines {
        let mut generated = vec![];
        gen(&pattern, &mut generated);

        println!("generated values for {pattern}:\n {:?}", generated)
    }

    println!("result day12: {res}",);

    Ok(())
}

fn to_code(line: &str) -> Vec<usize> {
    line.split(".")
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|l| l.len())
        .collect_vec()
}

fn gen(pattern: &str, generated: &mut Vec<String>) {
    if !pattern.contains("?") {
        generated.push(pattern.to_string());
        return;
    }
    gen(&pattern.clone().replacen("?", ".", 1), generated);
    gen(&pattern.clone().replacen("?", "#", 1), generated);
}
