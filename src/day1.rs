use anyhow::{bail, Context};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;

        let mut first: Option<char> = None;
        let mut next: Option<char> = None;

        for char in line.chars() {
            if char.is_numeric() {
                if first.is_none() {
                    first = Some(char)
                } else {
                    next = Some(char)
                }
            }
        }

        if next.is_none() {
            sum += format!("{}{}", first.unwrap(), first.unwrap()).parse::<i32>()?
        } else {
            sum += format!("{}{}", first.unwrap(), next.unwrap()).parse::<i32>()?
        }
    }
    println!("output a: {sum}");
    Ok(())
}

fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;

        let first = take_first(line.chars().next().unwrap().to_string(), line.clone())?;
        let last = take_last(line.chars().last().unwrap().to_string(), line.clone())?;

        println!("{first}{last}");
        sum += format!("{first}{last}").parse::<i32>()?;
    }
    println!("output b: {sum}");
    Ok(())
}

fn take_first(curr: String, line: String) -> anyhow::Result<i32> {
    let ch = curr.chars().last().context("no first char")?;

    if ch.is_numeric() {
        return Ok(ch.to_string().parse::<i32>()?);
    } else {
        if curr.ends_with("one") {
            return Ok(1);
        }
        if curr.ends_with("two") {
            return Ok(2);
        }
        if curr.ends_with("three") {
            return Ok(3);
        }
        if curr.ends_with("four") {
            return Ok(4);
        }
        if curr.ends_with("five") {
            return Ok(5);
        }
        if curr.ends_with("six") {
            return Ok(6);
        }
        if curr.ends_with("seven") {
            return Ok(7);
        }
        if curr.ends_with("eight") {
            return Ok(8);
        }
        if curr.ends_with("nine") {
            return Ok(9);
        }
    }
    take_first(line[0..curr.len() + 1].to_string(), line)
}

fn take_last(curr: String, line: String) -> anyhow::Result<i32> {
    let ch = curr.chars().next().context("no first char")?;

    if ch.is_numeric() {
        return Ok(ch.to_string().parse::<i32>()?);
    } else {
        if curr.starts_with("one") {
            return Ok(1);
        }
        if curr.starts_with("two") {
            return Ok(2);
        }
        if curr.starts_with("three") {
            return Ok(3);
        }
        if curr.starts_with("four") {
            return Ok(4);
        }
        if curr.starts_with("five") {
            return Ok(5);
        }
        if curr.starts_with("six") {
            return Ok(6);
        }
        if curr.starts_with("seven") {
            return Ok(7);
        }
        if curr.starts_with("eight") {
            return Ok(8);
        }
        if curr.starts_with("nine") {
            return Ok(9);
        }
    }
    if curr.len() == line.len() {
        bail!("empty");
    }
    take_last(line[line.len() - curr.len() - 1..].to_string(), line)
}
