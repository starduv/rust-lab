use std::{io::Error, vec};

use crate::read::read_to_format;

pub fn run() -> Result<u32, Error> {
    let mut sum = 0u32;
    let (gears, numbers) = read_to_format("src/_2023_/day_three/day_three.txt", |input| {
        let mut gears: Vec<Gear> = vec![];
        let mut numbers: Vec<Vec<Number>> = vec![vec![]; 140];
        let mut temp = vec![];
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    temp.push(col);
                } else {
                    if c.eq(&'*') {
                        gears.push(Gear { row, col });
                    }

                    if temp.len() > 0 {
                        let col_start = temp[0];
                        let col_end = temp[temp.len() - 1];
                        numbers[row].push(Number {
                            row,
                            col_start,
                            col_end,
                            value: line
                                .get(col_start..col_end + 1)
                                .unwrap()
                                .parse::<u32>()
                                .unwrap(),
                        });
                        temp.clear();
                    }
                }
            }

            if temp.len() > 0 {
                let col_start = temp[0];
                let col_end = temp[temp.len() - 1];
                numbers[row].push(Number {
                    row,
                    col_start,
                    col_end,
                    value: line
                        .get(col_start..col_end + 1)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap(),
                });
                temp.clear();
            }
        }

        (gears, numbers)
    })?;

    let mut parts = vec![];
    for gear in &gears {
        for row in (gear.row - 1)..(gear.row + 2) {
            for number in &numbers[row] {
                if number.is_in_range(gear.col) {
                    parts.push(number);
                }
            }
        }

        if parts.len() == 2 {
            sum += parts[0].value * parts[1].value;
        }

        parts.clear();
    }

    Ok(sum)
}

#[derive(Debug, Clone)]
struct Gear {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Number {
    col_end: usize,
    col_start: usize,
    row: usize,
    value: u32,
}

impl Number {
    pub fn is_in_range(&self, index: usize) -> bool {
        self.col_start.abs_diff(index) <= 1 || self.col_end.abs_diff(index) <= 1
    }
}
