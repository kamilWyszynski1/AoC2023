use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Sub,
    path::Path,
};

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    let mut numbers = Vec::new();

    for (_, line) in lines.clone().into_iter().enumerate() {
        let mut curr = String::new();
        let mut first_inx = 0;
        let mut last_inx = 0;
        let mut subnums: Vec<_> = vec![];
        for (i, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if curr.is_empty() {
                    first_inx = i;
                    last_inx = i;
                } else {
                    last_inx = i
                }
                curr.push(ch);
                if i == line.len() - 1 {
                    subnums.push((curr.parse::<usize>().unwrap(), first_inx, last_inx));
                    curr.clear();
                    first_inx = 0;
                    last_inx = 0;
                }
            } else {
                if !curr.is_empty() {
                    subnums.push((curr.parse::<usize>().unwrap(), first_inx, last_inx));
                    curr.clear();
                    first_inx = 0;
                    last_inx = 0;
                }
            }
        }
        numbers.push(subnums);
    }

    let mut sum = 0;

    for (line_inx, line) in lines.clone().into_iter().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            if ch == '*' {
                let mut adjs = vec![];

                if let Some(inx) = line_inx.checked_sub(1) {
                    let ns = numbers.get(inx).unwrap();
                    dbg!(i, ns);
                    for (number, first, last) in ns {
                        if check_b(i, *first, *last) {
                            adjs.push(*number);
                        }
                    }
                }

                let local_numbers = numbers.get(line_inx).unwrap();
                for (number, first, last) in local_numbers {
                    if check_b(i, *first, *last) {
                        adjs.push(*number);
                    }
                }

                if let Some(next_nums) = numbers.get(line_inx + 1) {
                    for (number, first, last) in next_nums {
                        if check_b(i, *first, *last) {
                            adjs.push(*number);
                        }
                    }
                }

                dbg!(&adjs);
                if adjs.len() == 2 {
                    sum += adjs[0] * adjs[1];
                }
            }
        }
    }

    println!("result day3 b: {}", sum);
    Ok(())
}

fn check_b(i: usize, first: usize, last: usize) -> bool {
    i.max(first).sub(i.min(first)) < 2 || last.max(i).sub(last.min(i)) < 2
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    let mut sum = 0;

    for (line_inx, line) in lines.clone().into_iter().enumerate() {
        let mut curr = String::new();
        let mut first_inx = 0;
        let mut last_inx = 0;
        for (i, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if curr.is_empty() {
                    first_inx = i;
                    last_inx = i;
                } else {
                    last_inx = i
                }
                curr.push(ch);

                if i == line.len() - 1 {
                    if check(&lines, line_inx, first_inx, last_inx) {
                        match line.get(first_inx..=last_inx) {
                            Some(number_str) => {
                                // let mut l: String = String::new();
                                // let b1 = std::io::stdin().read_line(&mut l).unwrap();

                                let n = number_str.parse::<usize>()?;
                                dbg!(number_str, n, line_inx, first_inx, last_inx + 1);
                                sum += n;
                            }
                            None => {}
                        }
                    }
                    curr.clear();
                    first_inx = 0;
                    last_inx = 0;
                }

                continue;
            } else {
                if !curr.is_empty() && check(&lines, line_inx, first_inx, last_inx + 1) {
                    match line.get(first_inx..=last_inx) {
                        Some(number_str) => {
                            let n = number_str.parse::<usize>()?;
                            dbg!(number_str, n, line_inx, first_inx, last_inx + 1);
                            sum += n;
                        }
                        None => {}
                    }
                }
                curr.clear();
                first_inx = 0;
                last_inx = 0;
            }
        }
    }

    println!("result day3 a: {}", sum);
    Ok(())
}

fn check(lines: &Vec<String>, line_inx: usize, first: usize, last: usize) -> bool {
    for i in line_inx.checked_sub(1).unwrap_or_default()..=line_inx + 1 {
        for j in first.checked_sub(1).unwrap_or_default()..=last {
            match lines.get(i).and_then(|line| {
                line.chars()
                    .nth(j)
                    .and_then(|ch| Some(!ch.is_numeric() && ch != '.'))
            }) {
                Some(v) => {
                    if v {
                        return true;
                    }
                }
                None => {}
            }
        }
    }
    return false;
}
