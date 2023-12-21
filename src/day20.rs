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

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),                      // %
    Conjunction(HashMap<String, Pulse>), // &
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    name: String,

    module_type: ModuleType,
    destinations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

type FromTo = (String, String);

impl Module {
    fn pulse(
        &mut self,
        from: String,
        pulse: Pulse,
        queue: &mut VecDeque<(FromTo, Pulse)>,
        counter: &mut Counter,
    ) {
        match &mut self.module_type {
            ModuleType::Broadcast => self.send(queue, pulse, counter),
            ModuleType::FlipFlop(on) => {
                match pulse {
                    // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                    Pulse::High => (),
                    Pulse::Low => {
                        if *on {
                            // If it was on, it turns off and sends a low pulse.
                            *on = false;
                            self.send(queue, Pulse::Low, counter)
                        } else {
                            // If it was off, it turns on and sends a high pulse.
                            *on = true;

                            self.send(queue, Pulse::High, counter)
                        }
                    }
                }
            }
            ModuleType::Conjunction(mem) => {
                // Conjunction modules (prefix &) remember the type of the most recent pulse received from each of
                // their connected input modules; they initially default to remembering a low pulse for each input.
                // When a pulse is received, the conjunction module first updates its memory for that input.
                // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.

                *mem.get_mut(&from).unwrap() = pulse; // must exist thus unwrap
                let remembered = mem.values().all(|p| *p == Pulse::High);

                self.send(
                    queue,
                    if remembered { Pulse::Low } else { Pulse::High },
                    counter,
                );
            }
        }
    }

    fn send(&self, queue: &mut VecDeque<(FromTo, Pulse)>, pulse: Pulse, counter: &mut Counter) {
        for dest in &self.destinations {
            counter.inc(&pulse);
            if dest == "ns" && pulse == Pulse::High {
                println!("{} -{:?}-> {}", self.name, pulse, dest);
            }
            queue.push_front(((self.name.clone(), dest.clone()), pulse))
        }
    }
}

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let mut modules = parse(file);
    let broadcaster = modules.get("broadcaster").unwrap();

    iterate(modules);

    println!("day20 a");

    Ok(())
}

#[derive(Debug)]
struct Counter {
    highs: usize,
    lows: usize,
}

impl Counter {
    fn inc(&mut self, p: &Pulse) {
        match p {
            Pulse::High => self.highs += 1,
            Pulse::Low => self.lows += 1,
        }
    }
}

fn prepare_conjuctions(modules: &mut HashMap<String, Module>) {
    let con_ids = modules
        .iter()
        .filter_map(|(id, module)| match &module.module_type {
            ModuleType::Broadcast => None,
            ModuleType::FlipFlop(_) => None,
            ModuleType::Conjunction(_) => Some(id.clone()),
        })
        .collect_vec();

    let mut init: HashMap<String, Vec<String>> = HashMap::new();
    for (id, module) in &*modules {
        for con_id in &con_ids {
            if module.destinations.contains(con_id) {
                init.entry(con_id.to_string())
                    .and_modify(|v| v.push(id.to_string()))
                    .or_insert(vec![id.to_string()]);
            }
        }
    }

    for (con_id, tos) in init {
        let con = modules.get_mut(&con_id).unwrap();

        for to in tos {
            match &mut con.module_type {
                ModuleType::Broadcast | ModuleType::FlipFlop(_) => (),
                ModuleType::Conjunction(mem) => {
                    mem.insert(to, Pulse::Low);
                }
            }
        }
    }
}

fn iterate(mut modules: HashMap<String, Module>) {
    prepare_conjuctions(&mut modules);
    let starting_point = modules.clone();
    dbg!(&starting_point);

    let mut i = 0;
    let mut counter = Counter { highs: 0, lows: 0 };

    let mut part_b_iters = HashMap::new();

    loop {
        let mut queue: VecDeque<(FromTo, Pulse)> = VecDeque::new();
        let broadcaster = modules.get_mut("broadcaster").unwrap();
        counter.inc(&Pulse::Low);
        broadcaster.pulse("button".to_string(), Pulse::Low, &mut queue, &mut counter);

        while let Some(((from, to), pulse)) = queue.pop_back() {
            for check in vec!["rv", "vp", "cq", "dc"] {
                if from == check && to == "ns" && pulse == Pulse::High {
                    if !part_b_iters.contains_key(&from) {
                        part_b_iters.insert(from.clone(), i + 1);
                    }
                }
            }
            match modules.get_mut(&to) {
                Some(module) => module.pulse(from, pulse, &mut queue, &mut counter),
                None => (),
            }
        }

        if part_b_iters.len() == 4 {
            dbg!(part_b_iters); // result calculated manually using LCM
            return;
        }

        // // dbg!(&modules);
        // if starting_point == modules {
        //     println!("found loop! {}, counter: {:?}", i, counter);
        //     println!(
        //         "res: {}",
        //         (1000 / (i + 1) * counter.highs) * (1000 / (i + 1) * counter.lows)
        //     );
        //     return;
        // }

        // if i == 999 {
        //     break;
        // }

        i += 1;
        println!("==== {i} ====")
    }

    println!("loop not found, res: {}", counter.highs * counter.lows)
}

fn parse(f: File) -> HashMap<String, Module> {
    let mut modules = HashMap::new();

    BufReader::new(f)
        .lines()
        .map(Result::unwrap)
        .for_each(|line| {
            let (module, dests) = line.split_once(" -> ").unwrap();
            let destinations = dests
                .split(",")
                .map(|s| s.trim())
                .map(|s| s.to_string())
                .collect_vec();

            if module == "broadcaster" {
                modules.insert(
                    "broadcaster".to_string(),
                    Module {
                        name: "broadcaster".to_string(),
                        module_type: ModuleType::Broadcast,
                        destinations,
                    },
                );
            } else if module.contains("%") {
                let name = module[1..].to_string();
                modules.insert(
                    name.clone(),
                    Module {
                        name,
                        module_type: ModuleType::FlipFlop(false),
                        destinations,
                    },
                );
            } else if module.contains("&") {
                let name = module[1..].to_string();
                modules.insert(
                    name.clone(),
                    Module {
                        name,
                        module_type: ModuleType::Conjunction(HashMap::new()),
                        destinations,
                    },
                );
            }
        });
    modules
}
