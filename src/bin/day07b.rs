use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, Hash, Ord)]
enum Label {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Label {
    fn value(&self) -> u8 {
        match self {
            Label::Ace => 14,
            Label::King => 13,
            Label::Queen => 12,
            Label::Ten => 10,
            Label::Nine => 9,
            Label::Eight => 8,
            Label::Seven => 7,
            Label::Six => 6,
            Label::Five => 5,
            Label::Four => 4,
            Label::Three => 3,
            Label::Two => 2,
            Label::Joker => 1,
        }
    }

    fn from_char(c: char) -> Option<Label> {
        match c {
            'A' => Some(Label::Ace),
            'K' => Some(Label::King),
            'Q' => Some(Label::Queen),
            'T' => Some(Label::Ten),
            '9' => Some(Label::Nine),
            '8' => Some(Label::Eight),
            '7' => Some(Label::Seven),
            '6' => Some(Label::Six),
            '5' => Some(Label::Five),
            '4' => Some(Label::Four),
            '3' => Some(Label::Three),
            '2' => Some(Label::Two),
            'J' => Some(Label::Joker),
            _ => None,
        }
    }
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
enum Strength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Strength {
    fn value(&self) -> u8 {
        match self {
            Strength::FiveOfAKind => 8,
            Strength::FourOfAKind => 7,
            Strength::FullHouse => 6,
            Strength::ThreeOfAKind => 5,
            Strength::TwoPair => 4,
            Strength::OnePair => 3,
            Strength::HighCard => 2,
        }
    }
}

impl PartialEq for Strength {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Strength {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, Ord)]
struct Hand {
    cards: [Label; 5],
}

impl Hand {
    fn get_strength(&self) -> Strength {
        let mut counts = HashMap::new();
        for card in self.cards.iter() {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        if counts.len() == 1 {
            return Strength::FiveOfAKind;
        }
        let mut count_values = counts.values().copied().collect::<Vec<u32>>();
        count_values.sort();
        if let Some(joker_count) = counts.remove(&Label::Joker) {
            // remove joker_count from count_values
            let mut joker_count_index = None;
            for (i, count_value) in count_values.iter().enumerate() {
                if *count_value == joker_count {
                    joker_count_index = Some(i);
                    break;
                }
            }
            let joker_count_index = joker_count_index.unwrap();
            count_values.remove(joker_count_index);
            // add joker_count to largest count
            let largest_count = count_values.last_mut().unwrap();
            *largest_count += joker_count;
        }
        assert!(count_values.iter().sum::<u32>() == 5);
        match counts.len() {
            1 => Strength::FiveOfAKind,
            2 => match count_values.last() {
                Some(&3) => Strength::FullHouse,
                Some(&4) => Strength::FourOfAKind,
                _ => panic!("Unexpected counts: {:?}", counts),
            },
            3 => match count_values.last() {
                Some(&2) => Strength::TwoPair,
                Some(&3) => Strength::ThreeOfAKind,
                _ => panic!("Unexpected counts: {:?}", counts),
            },
            4 => Strength::OnePair,
            5 => Strength::HighCard,
            _ => panic!("Unexpected counts: {:?}", counts),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_strength() == other.get_strength()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // First compare on strength, then if they have equal strength, compare on card value from first to last card
        match self.get_strength().partial_cmp(&other.get_strength()) {
            Some(std::cmp::Ordering::Equal) => {
                for i in 0..5 {
                    match self.cards[i].partial_cmp(&other.cards[i]) {
                        Some(std::cmp::Ordering::Equal) => continue,
                        Some(std::cmp::Ordering::Less) => return Some(std::cmp::Ordering::Less),
                        Some(std::cmp::Ordering::Greater) => {
                            return Some(std::cmp::Ordering::Greater)
                        }
                        None => panic!("Unexpected ordering"),
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            Some(std::cmp::Ordering::Less) => Some(std::cmp::Ordering::Less),
            Some(std::cmp::Ordering::Greater) => Some(std::cmp::Ordering::Greater),
            None => panic!("Unexpected ordering"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Play {
    hand: Hand,
    bid: u32,
}

fn main() {
    let plays: Vec<Play> = include_str!("../../inputs/day07.txt")
        .trim()
        .split("\n")
        .map(|line| {
            let line = line.trim().split_whitespace().collect::<Vec<&str>>();
            let hand = line[0];
            let bid = line[1].parse::<u32>().unwrap();
            let cards = hand
                .chars()
                .map(|c| Label::from_char(c).unwrap())
                .collect::<Vec<Label>>();
            Play {
                hand: Hand {
                    cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
                },
                bid,
            }
        })
        .collect();
    let mut plays = plays.iter().collect::<Vec<&Play>>();
    plays.sort();
    // print out plays in order with their strength
    for play in plays.iter() {
        println!("{:?} {:?}", play, play.hand.get_strength());
    }
    let ans = plays
        .iter()
        .enumerate()
        .map(|(i, play)| (i as u32 + 1) * play.bid)
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_strength() {
        // Test case 1: Five of a kind
        let hand = Hand {
            cards: [Label::Ace, Label::Ace, Label::Ace, Label::Ace, Label::Ace],
        };
        assert_eq!(hand.get_strength(), Strength::FiveOfAKind);

        // Test case 2: Four of a kind
        let hand = Hand {
            cards: [Label::Ace, Label::Ace, Label::Ace, Label::Ace, Label::King],
        };
        assert_eq!(hand.get_strength(), Strength::FourOfAKind);

        // Test case 3: Full house
        let hand = Hand {
            cards: [Label::Ace, Label::Ace, Label::Ace, Label::King, Label::King],
        };
        assert_eq!(hand.get_strength(), Strength::FullHouse);

        // Test case 4: Three of a kind
        let hand = Hand {
            cards: [
                Label::Ace,
                Label::Ace,
                Label::Ace,
                Label::King,
                Label::Queen,
            ],
        };
        assert_eq!(hand.get_strength(), Strength::ThreeOfAKind);

        // Test case 5: Two pair
        let hand = Hand {
            cards: [
                Label::Ace,
                Label::Ace,
                Label::King,
                Label::King,
                Label::Queen,
            ],
        };
        assert_eq!(hand.get_strength(), Strength::TwoPair);

        // Test case 6: One pair
        let hand = Hand {
            cards: [
                Label::Ace,
                Label::Ace,
                Label::King,
                Label::Queen,
                Label::Ten,
            ],
        };
        assert_eq!(hand.get_strength(), Strength::OnePair);

        // Test case 7: High card
        let hand = Hand {
            cards: [
                Label::Ace,
                Label::King,
                Label::Queen,
                Label::Ten,
                Label::Nine,
            ],
        };
        assert_eq!(hand.get_strength(), Strength::HighCard);
    }
}
