use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs;

fn parse(path: &str) -> Vec<Hand> {
    let hands = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .lines()
        .map(|l| Hand::new(l))
        .collect::<Vec<Hand>>();
    hands
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: usize,
    tier: u8,
    sortable_cards: String,
}

fn count_cards(cards: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for item in cards.chars() {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

fn get_hand_tier(cards: &str) -> u8 {
    let count_map = count_cards(cards);
    let mut counts = count_map.values().cloned().collect::<Vec<usize>>();
    counts.sort();

    if counts.len() == 1 {
        6
    } else if counts.contains(&4) {
        5
    } else if counts.contains(&3) & counts.contains(&2) {
        4
    } else if (counts.len() == 3) & counts.contains(&3) {
        3
    } else if (counts.len() == 3) & counts.contains(&2) {
        2
    } else if (counts.len() == 4) & counts.contains(&2) {
        1
    } else {
        0
    }
}

// pub const CARD_MAP: HashMap<char, char> =
//     HashMap::from([('A', 'E'), ('K', 'D'), ('Q', 'C'), ('J', 'B'), ('T', 'A')]);

fn get_sortable_cards(cards: &str) -> String {
    let card_map: HashMap<char, char> =
        HashMap::from([('A', 'E'), ('K', 'D'), ('Q', 'C'), ('J', 'B'), ('T', 'A')]);
    let mut sortable_cards = String::new();
    for item in cards.chars() {
        let is_not_numeric = !item.is_numeric();
        if is_not_numeric {
            sortable_cards.push(*card_map.get(&item).unwrap());
        } else {
            sortable_cards.push(item);
        }
    }
    sortable_cards
}

impl Hand {
    fn new(l: &str) -> Self {
        let (cards, bid) = l.trim().split_once(" ").unwrap();

        Self {
            cards: cards.to_string(),
            bid: bid.parse::<usize>().unwrap(),
            tier: get_hand_tier(cards),
            sortable_cards: get_sortable_cards(cards),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.tier > other.tier {
            return Ordering::Greater;
        }
        if self.tier < other.tier {
            return Ordering::Less;
        }
        if self.tier == other.tier {
            return self.sortable_cards.cmp(&other.sortable_cards);
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn pt1() {
    let mut hands = parse("src/input.txt");
    hands.sort();
    // println!("{:?}", hands);
    let mut total = 0;
    for (idx, hand) in hands.iter().enumerate() {
        total += (idx + 1) * hand.bid;
        println!("Hand: {:?}", hand);
    }
    println!("\nTotal: {:?}", total);
}

fn pt2() {}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
