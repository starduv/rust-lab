use std::collections::BTreeMap;

pub fn run() -> usize {
    let moves = "LRRLRLRRLRRRLRLRLRRLRRRLRRRLRRLRRRLRLRLRLRLRLRLRRRLRRLRRRLLLLRRRLRLLLRRRLLRLLRRRLRRRLRLRRLRRRLRRRLLRRRLRLRRRLLRRRLRLLRRRLRRLLRLRLRLRRRLRLLRLRLRRRLRLLRLRLRRRLLRRRLRRLRRRLRLRRLRLRRLRLRRLRRRLLRRRLLLRRRLLRRLRRLRRLRLLRRLRRRLRRLRLRLRRLRRLLLRRLRLRRRLRRRLRRRLLLRLRRRLLRRRLRLLRRRR".chars().map(|c| {
        match c {
            'L' => 0,
            _ => 1
        }
    }).collect::<Vec<usize>>();

    let mut locations = vec![];
    let mut map = BTreeMap::new();
    for line in include_str!("day_eight.txt").lines() {
        let ids = line.split(',').collect::<Vec<&str>>();
        map.insert(ids[0], [ids[1], ids[2]]);

        if ids[0].ends_with('A') {
            locations.push(ids[0])
        }
    }

    let modulo = moves.len();
    let mut steps = vec![];
    for i in 0..locations.len() {
        let mut step = 0;
        while !locations[i].ends_with('Z') {
            let current_move = moves[step % modulo];
            locations[i] = map.get(locations[i]).unwrap()[current_move];
            step += 1;
        }
        steps.push(step);
    }

    let mut gcf = None;
    let mut factor = *steps.iter().min().unwrap();
    while gcf.is_none() {
        if steps.iter().all(|step| step % factor == 0) {
            gcf = Some(factor);
        }
        factor -= 1;
    }

    match gcf {
        // leat common multiple!
        Some(gcf) => steps[1..].iter().map(|step| step / gcf).product::<usize>() * steps[0],
        None => 0,
    }
}
