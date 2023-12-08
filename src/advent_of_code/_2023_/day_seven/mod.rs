use std::{cmp::Ordering, collections::BTreeMap, vec};

pub fn run() -> usize {
    let symbol_map: BTreeMap<char, u32> = BTreeMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('J', 1),
    ]);

    let mut hands: Vec<Hand> = vec![];
    let mut card_count = BTreeMap::new();
    for line in include_str!("day_seven.txt").lines() {
        let mut j_count = 0;
        let cards_wager = line.split_whitespace().collect::<Vec<&str>>();
        let mut hand = Hand::new(cards_wager[1].parse::<usize>().unwrap());
        for card in cards_wager[0].chars() {
            let value = match symbol_map.get(&card) {
                Some(value) => *value,
                None => card.to_digit(10).unwrap(),
            };

            if value.ne(&1) {
                card_count.entry(value).and_modify(|n| *n += 1).or_insert(1);
            } else {
                j_count += 1;
            }

            hand.cards.push(value);
        }

        let max = card_count
            .iter()
            .max_by(
                |(key, value), (other_key, other_value)| match value.cmp(&other_value) {
                    Ordering::Equal => key.cmp(&other_key),
                    ordering => ordering,
                },
            );

        if let Some((max_key, _)) = max {
            card_count.entry(*max_key).and_modify(|n| *n += j_count);
        } else {
            card_count.insert(1, 5);
        }

        for (_, count) in &card_count {
            hand.set_state(count);
        }

        card_count.clear();

        hands.push(hand);
    }

    hands.sort_by(|a, b| a.cmp(&b));

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.wager)
        .sum::<usize>()
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    state: HandState,
    wager: usize,
}

impl Hand {
    fn new(wager: usize) -> Self {
        Hand {
            cards: Vec::new(),
            state: HandState::HighCard,
            wager,
        }
    }

    fn set_state(&mut self, count: &u32) -> () {
        match self.state {
            HandState::HighCard if count.eq(&2) => self.state = HandState::OnePair,
            HandState::HighCard if count.eq(&3) => self.state = HandState::ThreeOfKind,
            HandState::HighCard if count.eq(&4) => self.state = HandState::FourOfKind,
            HandState::HighCard if count.eq(&5) => self.state = HandState::FiveOfKind,
            HandState::OnePair if count.eq(&2) => self.state = HandState::TwoPair,
            HandState::OnePair if count.eq(&3) => self.state = HandState::FullHouse,
            HandState::ThreeOfKind if count.eq(&2) => self.state = HandState::FullHouse,
            _ => {}
        }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        let this_number = self.state.get_number();
        let other_number = other.state.get_number();
        match this_number.cmp(&other_number) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

#[derive(Debug)]
enum HandState {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandState {
    fn get_number(&self) -> u32 {
        match self {
            HandState::FiveOfKind => 21,
            HandState::FourOfKind => 20,
            HandState::FullHouse => 19,
            HandState::ThreeOfKind => 18,
            HandState::TwoPair => 17,
            HandState::OnePair => 16,
            HandState::HighCard => 15,
        }
    }
}
