use std::io::Error;

use crate::read::read_to_format;

pub fn run() -> Result<u32, Error> {
    let mut sum = 0;

    let games = read_to_format("src/_2023_/day_two/day_two.txt", |input| {
        let num_pattern = regex::Regex::new(r"\d+").unwrap();
        let color_pattern =
            regex::Regex::new(r"(?P<blue>\d+ blue)|(?P<red>\d+ red)|(?P<green>\d+ green)").unwrap();

        input
            .lines()
            .map(|line| {
                let details = line.split(':').collect::<Vec<&str>>();
                let game_id = num_pattern
                    .find(details[0])
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                let mut game = Game {
                    id: game_id,
                    blue: 0,
                    red: 0,
                    green: 0,
                };

                for rounds in details[1].split(';') {
                    for color in color_pattern.captures_iter(rounds) {
                        if let Some(blue) = color.name("blue") {
                            let value = num_pattern
                                .find(blue.as_str())
                                .unwrap()
                                .as_str()
                                .parse::<u32>()
                                .unwrap();
                            game.blue = std::cmp::max(game.blue, value);
                        }
                        if let Some(red) = color.name("red") {
                            let value = num_pattern
                                .find(red.as_str())
                                .unwrap()
                                .as_str()
                                .parse::<u32>()
                                .unwrap();
                            game.red = std::cmp::max(game.red, value);
                        }
                        if let Some(green) = color.name("green") {
                            let value = num_pattern
                                .find(green.as_str())
                                .unwrap()
                                .as_str()
                                .parse::<u32>()
                                .unwrap();
                            game.green = std::cmp::max(game.green, value);
                        }
                    }
                }
                game
            })
            .collect::<Vec<Game>>()
    })?;

    for game in games {
        let power = game.blue * game.red * game.green;
        sum += power;
    }

    Ok(sum)
}

struct Game {
    pub id: u32,
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}
