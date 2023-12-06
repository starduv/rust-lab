use std::vec;

pub fn run() -> u64 {
    let mut races = vec![Race::new(); 1];
    for (line_index, line) in include_str!("day_six.txt").lines().enumerate() {
        for (race_index, race) in line.split_whitespace().enumerate() {
            if line_index == 0 {
                races[race_index].time = race.parse::<u64>().unwrap()
            }

            if line_index == 1 {
                races[race_index].distance = race.parse::<u64>().unwrap()
            }
        }
    }

    let mut tallies = vec![];
    for race in races {
        let mut tally = 0;
        for time in (0..race.time).rev(){
            let speed = time;
            let remaining = race.time - time;
            let distance = speed * remaining;
            if distance > race.distance {
                tally += 1;
            }
        }
        tallies.push(tally);
    }

    tallies.iter().product()
}

#[derive(Debug, Clone)]
struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    pub fn new() -> Self {
        Race {
            time: 0,
            distance: 0,
        }
    }
}
