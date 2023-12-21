use itertools::Itertools;
use std::collections::VecDeque;
use std::vec;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let mut map = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    take_steps(&mut map, 64);
    Ok(())
}

fn take_steps(map: &mut Vec<Vec<char>>, steps: usize) {
    let mut queue = VecDeque::new();

    // find starting point
    for (x, row) in map.iter().enumerate() {
        for (y, v) in row.iter().enumerate() {
            if *v == 'S' {
                queue.push_back((x, y));
            }
        }
    }

    let mut steps_count = 0;
    loop {
        let mut local_queue = HashSet::new();
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if is_valid_point((x as isize + dx, y as isize + dy), map) {
                    local_queue.insert(((x as isize + dx) as usize, (y as isize + dy) as usize));
                }
            }
        }
        steps_count += 1;

        if steps_count == steps {
            println!("{}", local_queue.len());
            return;
        }
        for p in local_queue {
            queue.push_back(p);
        }
    }
}

fn is_valid_point(p: (isize, isize), map: &Vec<Vec<char>>) -> bool {
    let (x, y) = p;
    if let Ok(x) = TryInto::<usize>::try_into(x) {
        if let Ok(y) = TryInto::<usize>::try_into(y) {
            if let Some(row) = map.get(x) {
                if let Some(v) = row.get(y) {
                    return *v != '#';
                }
            }
        }
    }
    false
}
