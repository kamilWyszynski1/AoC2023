use anyhow::{bail, Context};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn solvea<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    // only 12 red cubes, 13 green cubes, and 14 blue cubes

    let mut res = 0;

    'lines: for line in reader.lines() {
        let line = line?;

        let (game, rest) = line.split_once(": ").context("could not split")?;

        let game_number = {
            let (_, number) = game
                .split_once("Game ")
                .context("could not split at game")?;
            number.parse::<i32>()
        }?;

        let games = rest.split("; ");

        let mut possible = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        for gamel in games {
            let cubes = gamel.split(", ");
            for cube in cubes {
                let (cube_number, cube_value) =
                    cube.split_once(" ").context("could not split at cube")?;
                let cube_number = cube_number.parse::<i32>()?;

                match possible.get_mut(cube_value) {
                    Some(v) => {
                        if *v < cube_number {
                            continue 'lines;
                        }
                        *v -= cube_number
                    }
                    None => continue 'lines,
                }
            }
        }
        res += game_number;
    }

    println!("result day2 a: {res}");
    Ok(())
}

pub fn solveb<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    // only 12 red cubes, 13 green cubes, and 14 blue cubes

    let mut res = 0;

    'lines: for line in reader.lines() {
        let line = line?;

        let (game, rest) = line.split_once(": ").context("could not split")?;

        let games = rest.split("; ");

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for gamel in games {
            let cubes = gamel.split(", ");
            for cube in cubes {
                let (cube_number, cube_value) =
                    cube.split_once(" ").context("could not split at cube")?;
                let cube_number = cube_number.parse::<i32>()?;

                match cube_value {
                    "red" => red = cube_number.max(red),
                    "blue" => blue = cube_number.max(blue),
                    "green" => green = cube_number.max(green),
                    _ => bail!("invalid cube value"),
                }
            }
        }
        res += red * blue * green;
    }

    println!("result day2 a: {res}");
    Ok(())
}
