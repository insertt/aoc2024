#![feature(array_windows)]

use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/2.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut uns = 0;
    'nums: for nums in &lines {
        let mut first = true;
        let mut increasing = false;
        for [f, s] in nums.array_windows::<2>() {
            let diff = (f - s).abs();
            let curr_incr = (f - s) < 0;

            if !first && curr_incr != increasing {
                uns += 1;
                continue 'nums;
            }

            if diff == 0 || diff > 3 {
                uns += 1;
                continue 'nums;
            }

            increasing = curr_incr;
            first = false;
        }
    }

    let safe = lines.len() - uns;
    println!("part1: number of reports that are safe: {safe}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut safe = 0;
    'nums: for nums in &lines {
        let Some((first_inc, first_dec)) =
            find_first_unsafe(nums, 1).zip(find_first_unsafe(nums, -1))
        else {
            safe += 1;
            continue;
        };

        for first_unsafe in [first_inc, first_dec] {
            for i in [first_unsafe, first_unsafe + 1] {
                let mut dampened = nums.clone();
                dampened.remove(i);

                if is_ok(&dampened) {
                    safe += 1;
                    continue 'nums;
                }
            }
        }

        fn is_ok(nums: &Vec<i64>) -> bool {
            let signum = (nums[0] - nums[1]).signum();
            find_first_unsafe(nums, signum).is_none()
        }

        fn find_first_unsafe(nums: &Vec<i64>, signum: i64) -> Option<usize> {
            for (i, [f, s]) in nums.array_windows::<2>().copied().enumerate() {
                let diff = (f - s).abs();

                fn is_ok(a: i64, b: i64, signum: i64) -> bool {
                    (a - b).signum() == signum && (1..=3).contains(&(a - b).abs())
                }

                is_ok(f, s, signum);

                if (f - s).signum() != signum {
                    return Some(i);
                }

                if diff == 0 || diff > 3 {
                    return Some(i);
                }
            }

            return None;
        }
    }

    println!("part2: number of reports that are safe: {safe}");

    Ok(())
}
