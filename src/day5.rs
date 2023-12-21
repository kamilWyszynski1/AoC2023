use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Context;

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    /*
    -----------
        -----------

        -----------
    --------

     */
    fn contains(&self, another: &Self) -> bool {
        self.from <= another.from && another.from <= self.to
            || another.from <= self.from && self.from <= another.to
    }

    fn map_intersection(&mut self, source: &Self, dest: &Self) -> Option<Vec<Self>> {
        let start = self.from.max(source.from);
        let end = self.to.min(source.to);

        if start > end {
            return None;
        }

        let diff = dest.from as isize - source.from as isize;

        println!(
            "   seed: {:?}, source: {:?}, start: {}, end: {} diff: {}",
            self, source, start, end, diff
        );

        let mut ranges = vec![];
        if self.from < start {
            ranges.push(Range {
                from: self.from,
                to: start - 1,
            })
        }
        if self.to > end {
            ranges.push(Range {
                from: end + 1,
                to: self.to,
            });
        }

        self.from = (start as isize + diff) as usize;
        self.to = (end as isize + diff) as usize;

        Some(ranges)
    }
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let mut lines = vec![];

    for line in reader.lines() {
        lines.push(line?)
    }

    let mut lines_iter = lines.into_iter();
    let seeds_pairs: Vec<usize> = lines_iter
        .next()
        .context("no next")?
        .split_once(": ")
        .context("split onect")?
        .1
        .trim()
        .split(" ")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();

    let mut seeds = vec![];
    for chunk in seeds_pairs.chunks(2) {
        assert!(chunk.len() == 2);
        let first = chunk[0];
        let range = chunk[1];

        seeds.push(Range {
            from: first,
            to: first + range - 1,
        })
    }
    dbg!(seeds.len());

    lines_iter.next();

    let mut mappings: Vec<Vec<String>> = vec![];
    let mut curr = vec![];
    for line in lines_iter {
        if line.is_empty() {
            mappings.push(curr.clone());
            curr.clear();
            continue;
        }
        curr.push(line);
    }
    mappings.push(curr);
    dbg!(&seeds);

    for mapping in mappings {
        println!("======= {} =========", mapping[0]);
        let mut new_ranges = vec![];
        'seed: for seed in seeds.iter_mut() {
            for line in &mapping {
                if line.contains("to") {
                    continue;
                }

                let ranges: [usize; 3] = line
                    .split(" ")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap();
                assert!(ranges.len() == 3);

                let dest = Range {
                    from: ranges[0],
                    to: ranges[0] + ranges[2] - 1,
                };
                let source = Range {
                    from: ranges[1],
                    to: ranges[1] + ranges[2] - 1,
                };

                match seed.map_intersection(&source, &dest) {
                    Some(mut nr) => {
                        println!("after seed: {:?}, new: {:?}", seed, &nr);
                        new_ranges.append(&mut nr);
                        continue 'seed;
                    }
                    None => {}
                }
            }
        }
        seeds.append(&mut new_ranges);
    }

    dbg!(&seeds);
    println!(
        "result day5 a: {:?}",
        seeds.into_iter().map(|r| r.from).min()
    );
    Ok(())
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut lines = vec![];

    for line in reader.lines() {
        lines.push(line?)
    }

    let mut lines_iter = lines.into_iter();
    let mut seeds: Vec<usize> = lines_iter
        .next()
        .context("no next")?
        .split_once(": ")
        .context("split onect")?
        .1
        .trim()
        .split(" ")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    lines_iter.next();

    let mut mappings: Vec<Vec<String>> = vec![];
    let mut curr = vec![];
    for line in lines_iter {
        if line.is_empty() {
            mappings.push(curr.clone());
            curr.clear();
            continue;
        }
        curr.push(line);
    }
    mappings.push(curr);

    for mapping in mappings {
        println!("mapping: {}", mapping[0]);
        'seed: for seed in seeds.iter_mut() {
            for line in &mapping {
                if line.contains("to") {
                    continue;
                }

                let ranges: [usize; 3] = line
                    .split(" ")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap();
                assert!(ranges.len() == 3);

                let dest = ranges[0];
                let source = ranges[1];
                let range = ranges[2];

                if (source..source + range).contains(seed) {
                    println!("{} {} {} {}", *seed, dest, source, range);
                    *seed = dest + (*seed - source);
                    println!("  {}", *seed);
                    continue 'seed;
                }
            }
        }
    }

    println!("result day5 a: {:?}", seeds.into_iter().min().unwrap());
    Ok(())
}
