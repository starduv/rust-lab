use std::collections::VecDeque;

pub fn run() -> i32 {
    let mut readings = include_str!("day_nine.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<VecDeque<i32>>()
        })
        .collect::<Vec<VecDeque<i32>>>();

    readings
        .iter_mut()
        .map(|log| {
            predict(log);
            log[0]
        })
        .sum()
}

fn predict(readings: &mut VecDeque<i32>) -> i32 {
    let mut diffs = VecDeque::new();
    let mut is_zero = true;
    for i in 1..readings.len() {
        let diff = readings[i] - readings[i - 1];
        is_zero = is_zero && diff == 0;
        diffs.push_back(diff);
    }

    let predicted_change = match is_zero {
        true => 0,
        false => predict(&mut diffs),
    };

    let prediction = readings[0] - predicted_change;

    readings.push_front(prediction);

    prediction
}
