use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct HandData {
    pub cards: String,
    pub bid: u64,
}

impl HandData {
    pub fn new(input: &str) -> HandData {
        let (cards_str, bid_str) = input.split_once(" ").unwrap();
        HandData {
            cards: String::from(cards_str),
            bid: bid_str.parse::<u64>().unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
    Count = 8,
}

impl HandType {
    pub fn get_type(char_map: &HashMap<char, u8>) -> HandType {
        match char_map.len() {
            4 => HandType::OnePair,
            3 => {
                let product = char_map.values().fold(1, |acc, value| (acc * value));
                if product == 4 {
                    HandType::TwoPair
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                let product = char_map.values().fold(1, |acc, value| (acc * value));
                if product == 6 {
                    HandType::FullHouse
                } else {
                    HandType::FourOfAKind
                }
            }
            1 => HandType::FiveOfAKind,
            _ => HandType::HighCard,
        }
    }
}

fn value_of_card(card: char) -> u8 {
    match card {
        '2'..='9' => card.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card rank"),
    }
}

pub fn compare_cards(card_a: &str, card_b: &str) -> Ordering {
    let mut iter_a = card_a.chars();
    let mut iter_b = card_b.chars();

    while let (Some(a), Some(b)) = (iter_a.next(), iter_b.next()) {
        let rank_a = value_of_card(a);
        let rank_b = value_of_card(b);

        match rank_a.cmp(&rank_b) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }

    // If we reach here, it means both strings are equal
    Ordering::Equal
}

pub fn solve(input: String) -> u64 {
    //Initialize Hashmap
    let mut all_hand_map = HashMap::<u8, Vec<HandData>>::new();
    for i in 1..(HandType::Count as u8) {
        all_hand_map.insert(i, Vec::new());
    }

    //Iterate over lines
    input
        .lines()
        .map(|line| {
            let hand_data = HandData::new(line);

            //Map character counts
            let mut character_map = HashMap::<char, u8>::new();
            hand_data
                .cards
                .chars()
                .map(|c| {
                    character_map
                        .entry(c)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                })
                .for_each(drop);

            let hand_type = HandType::get_type(&character_map);

            let datas = all_hand_map.get_mut(&(hand_type as u8)).unwrap();
            datas.push(hand_data);
            datas.sort_by(|data_a, data_b| compare_cards(&data_a.cards, &data_b.cards))
        })
        .for_each(drop);

    let mut rank: u64 = 0;
    let mut answer: u64 = 0;
    for i in 1..(HandType::Count as u8) {
        let hand_datas = all_hand_map.get(&i).unwrap();
        if hand_datas.len() > 0 {
            for hand_data in hand_datas {
                rank += 1;
                answer += hand_data.bid * rank;

                println!("RANK:{}, CARD:{}", &rank, &hand_data.cards);
            }
        }
    }

    answer
}
