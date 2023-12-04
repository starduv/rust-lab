use std::{collections::HashMap, io::Error};

use crate::read::read_to_format;

pub fn run<'c>() -> Result<u32, Error> {
    let number_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]);

    let patterns = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "zero",
    ];

    let mut sum: u32 = 0;
    let values = read_to_format("src/_2023_/day_one/day_one.txt", |input| {
        input
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    })?;

    for ref input in values {
        let mut first = input.len() as i32;
        let mut first_value: &str = "";

        let mut last: i32 = -1;
        let mut last_value: &str = "";

        for pattern in &patterns {
            let left_most = input.find(pattern).map_or(first, |index| index as i32);
            if left_most < first {
                first = left_most;
                first_value = pattern;
            }

            let right_most = input.rfind(pattern).map_or(last, |index| index as i32);
            if right_most > last {
                last = right_most;
                last_value = pattern;
            }
        }

        let mut value = match number_map.get(first_value) {
            Some(value) => value.to_string(),
            None => first_value.to_string(),
        };

        value.push_str(match number_map.get(last_value) {
            Some(value) => value,
            None => last_value,
        });

        sum += value.parse::<u32>().unwrap();
    }

    Ok(sum)
}
