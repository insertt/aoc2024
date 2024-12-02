use std::{collections::HashMap, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/1.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let (mut f, mut s): (Vec<_>, Vec<_>) = input
        .lines()
        .flat_map(|line| {
            line.split_once("   ")
                .map(|(f, s)| (f.parse::<i64>().unwrap(), s.parse::<i64>().unwrap()))
        })
        .unzip();

    f.sort_unstable();
    s.sort_unstable();

    let sum: i64 = f.iter().zip(s.iter()).map(|(f, s)| (f - s).abs()).sum();

    println!("sum is: {sum}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (mut f, mut s): (Vec<_>, Vec<_>) = input
        .lines()
        .flat_map(|line| {
            line.split_once("   ")
                .map(|(f, s)| (f.parse::<i64>().unwrap(), s.parse::<i64>().unwrap()))
        })
        .unzip();

    f.sort_unstable();
    s.sort_unstable();

    let mut sim_score = HashMap::new();

    for n in s.iter() {
        *sim_score.entry(n).or_insert(0) += 1;
    }

    let sum = f.iter().map(|f| f * sim_score.get(f).copied().unwrap_or_default()).sum::<i64>();
    println!("sum is: {sum}");

    Ok(())
}
