use std::{cmp::Ordering, collections::HashMap};

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
    pub fn get_type(mut char_map: HashMap<char, u8>) -> HandType {
        // Convert HashMap to a vector of key-value pairs
        let mut char_counter_vec: Vec<(&char, &u8)> = char_map.iter().collect();

        // Sort the vector based on values (ascending order)
        char_counter_vec.sort_by(|a, b| b.1.cmp(a.1));

        let max_card = *char_counter_vec.iter().next().unwrap().0;
        if char_map.contains_key(&'J') && char_map.len() > 1 {
            let j_count = *char_map.get(&'J').unwrap();

            if !max_card.eq(&'J') {
                char_map
                    .entry(max_card)
                    .and_modify(|count| (*count += j_count));
            } else {
                char_map
                    .entry(*char_counter_vec.iter().next().unwrap().0)
                    .and_modify(|count| (*count += j_count));
            }
            char_map.remove(&'J');
        }

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
        'J' => 1, // Changed this for Joker
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
    let mut all_hand_map = HashMap::<u8, Vec<(&str, u64)>>::new();
    for i in 1..(HandType::Count as u8) {
        all_hand_map.insert(i, Vec::new());
    }

    //Iterate over lines
    input
        .lines()
        .map(|line| {
            //PARSE INPUT
            let line_split = line.split_once(" ").unwrap();
            let (cards, bid) = (line_split.0, line_split.1.parse::<u64>().unwrap());

            //MAP CARDS
            let mut character_count_tracker = HashMap::<char, u8>::new();

            cards
                .chars()
                .map(|c| {
                    character_count_tracker
                        .entry(c)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                })
                .for_each(drop);

            let hand_type = HandType::get_type(character_count_tracker);

            let datas = all_hand_map.get_mut(&(hand_type as u8)).unwrap();
            datas.push((cards, bid));
            datas.sort_by(|a, b| compare_cards(&a.0, &b.0))
        })
        .for_each(drop);

    dbg!(&all_hand_map);

    let mut rank: u64 = 0;
    let mut answer: u64 = 0;
    for i in 1..(HandType::Count as u8) {
        let hand_datas = all_hand_map.get(&i).unwrap();
        if hand_datas.len() > 0 {
            for hand_data in hand_datas {
                rank += 1;
                answer += hand_data.1 * rank;

                println!("RANK:{}, CARD:{}", &rank, &hand_data.0);
            }
        }
    }

    answer
}
