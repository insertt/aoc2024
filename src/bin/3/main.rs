use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/3.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut sum = 0;
    for (begin, _) in input.match_indices("mul(") {
        let Some(end) = (&input[begin..]).find(")") else {
            continue;
        };

        let inner = &input[begin + 4..begin + end];

        let Some((f, s)) = inner.split_once(",") else {
            continue;
        };

        if f.chars().any(|c| !c.is_ascii_alphanumeric())
            || s.chars().any(|c| !c.is_ascii_alphanumeric())
        {
            continue;
        }

        let Ok(f) = f.parse::<i64>() else { continue };

        let Ok(s) = s.parse::<i64>() else { continue };

        sum += f * s;
    }

    println!("part1: sum is {sum}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut sum = 0;
    let mut disabled_ranges = vec![];

    let mut disabled = false;
    let mut current_start = 0;
    for (pos, _) in input.match_indices("do") {
        let slice = &input[pos..];

        let enable = slice.starts_with("do()");
        let disable = slice.starts_with("don't()");

        if enable && !disabled {
            disabled = false;
        }

        if disable && !disabled {
            disabled = true;
            current_start = pos;
        }

        if disabled && enable {
            disabled_ranges.push(current_start..pos);
            disabled = false;
        }
    }

    for (begin, _) in input.match_indices("mul(") {
        let Some(end) = (&input[begin..]).find(")") else {
            continue;
        };

        let inner = &input[begin + 4..begin + end];

        let Some((f, s)) = inner.split_once(",") else {
            continue;
        };

        if f.chars().any(|c| !c.is_ascii_alphanumeric())
            || s.chars().any(|c| !c.is_ascii_alphanumeric())
        {
            continue;
        }

        let Ok(f) = f.parse::<i64>() else { continue };

        let Ok(s) = s.parse::<i64>() else { continue };

        if disabled_ranges.iter().any(|range| range.contains(&begin)) {
            continue;
        }

        sum += f * s;
    }

    println!("part2: sum is {sum}");

    Ok(())
}
