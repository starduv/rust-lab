use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

pub fn run() -> u32 {
    let mut sum = 0u32;
    let mut cache: HashMap<usize, u32> = HashMap::new();
    let lines: Vec<&str> = include_str!("day_four.txt").lines().collect();
    for index in 0..lines.len() {
        sum += find_winners(index, &lines, &mut cache);
    }
    sum
}

fn find_winners(index: usize, lines: &Vec<&str>, cache: &mut HashMap<usize, u32>) -> u32 {
    let mut sum = 1;
    let line = lines[index];
    let header = line.find(':').unwrap();
    let delimiter = line.find('|').unwrap();
    let mut winners: HashSet<&str> = HashSet::new();

    for winner in line[header + 1..delimiter].split_whitespace() {
        winners.insert(winner);
    }

    let mut offset = 1;
    for entry in line[delimiter + 1..].split_whitespace() {
        if winners.contains(entry) {
            sum += match cache.get(&(index + offset)) {
                Some(total) => *total,
                None => find_winners(index + offset, lines, cache),
            };
            offset += 1;
        }
    }

    cache.insert(index, sum);

    sum
}
