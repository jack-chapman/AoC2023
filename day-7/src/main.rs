use std::{collections::HashMap, fmt::Debug, hash::Hash, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let part_1 = process_part_1(input);
    let part_2 = process_part_2(input);

    println!("{part_1}");
    println!("{part_2}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Jack),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            _ => Err("Invalid input".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card2 {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromStr for Card2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Jack),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            _ => Err("Invalid input".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for HandType {
    fn from(cards: &[Card; 5]) -> Self {
        let mut map: HashMap<&Card, u32> = HashMap::new();
        for card in cards {
            if let Some(count) = map.get(&card) {
                map.insert(card, count + 1);
            } else {
                map.insert(card, 1);
            }
        }
        let counts: Vec<u32> = map.into_values().sorted().collect();
        match counts[0..] {
            [5] => Self::FiveOfAKind,
            [1, 4] => Self::FourOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        HandType::from(&self.cards)
    }

    fn jokerify(&mut self) -> Self {
        if !self.cards.contains(&Card::Jack) {
            return self.clone();
        }
        let mut frequency_map: HashMap<Card, usize> = HashMap::new();

        for card in &self.cards {
            *frequency_map.entry(card.clone()).or_insert(0) += 1;
        }

        let most_numerous_best_card = frequency_map
            .iter()
            .max_by(|(card_a, freq_a), (card_b, freq_b)| {
                if freq_a == freq_b {
                    return card_a.cmp(card_b);
                }
                freq_a.cmp(freq_b)
            })
            .map(|(card, _)| card)
            .unwrap();

        dbg!(most_numerous_best_card);

        let new_cards: Vec<Card> = self
            .cards
            .iter()
            .map(|card| {
                if *card == Card::Jack {
                    most_numerous_best_card
                } else {
                    card
                }
            })
            .cloned()
            .collect();

        Self {
            cards: new_cards.try_into().unwrap(),
            bid: self.bid,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let same_hand_type = self.hand_type() == other.hand_type();
        if same_hand_type {
            return self.cards == other.cards;
        }
        same_hand_type
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let equal = self == other;
        if equal {
            return std::cmp::Ordering::Equal;
        }
        let (our_hand, other_hand) = (self.hand_type(), other.hand_type());
        if our_hand != other_hand {
            match our_hand > other_hand {
                true => std::cmp::Ordering::Greater,
                false => std::cmp::Ordering::Less,
            }
        } else {
            match self.cards > other.cards {
                true => std::cmp::Ordering::Greater,
                false => std::cmp::Ordering::Less,
            }
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = line.split_once(' ').unwrap();
        let bid = bid.trim().parse().unwrap();
        let cards: Vec<_> = cards
            .split("")
            .filter(|s| !s.is_empty())
            .map(|card| card.parse().unwrap())
            .collect();
        let cards = cards.try_into().unwrap();

        Ok(Self { bid, cards })
    }
}

fn process_part_1(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input.lines().map(|line| line.parse().unwrap()).collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum()
}

fn process_part_2(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| line.parse::<Hand>().unwrap().jokerify())
        .collect();

    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_order() {
        let tests = [
            (Card::Ace, Card::Ace, Card::Ace),
            (Card::Ace, Card::Two, Card::Ace),
            (Card::Two, Card::Ace, Card::Ace),
            (Card::Queen, Card::Ten, Card::Queen),
        ];

        for test in tests {
            let result = test.0.max(test.1);

            assert_eq!(result, test.2);
        }
    }

    #[test]
    fn cmp_array_of_cards() {
        let a = [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace];
        let b = [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five];

        let result = a > b;

        assert!(result);
    }

    #[test]
    fn hand_from_cards() {
        let tests = [
            (
                [
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Three,
                ],
                HandType::FiveOfAKind,
            ),
            (
                [
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::King,
                ],
                HandType::FourOfAKind,
            ),
            (
                [
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::King,
                    Card::King,
                ],
                HandType::FullHouse,
            ),
            (
                [
                    Card::Three,
                    Card::Three,
                    Card::Three,
                    Card::King,
                    Card::Queen,
                ],
                HandType::ThreeOfAKind,
            ),
            (
                [
                    Card::Three,
                    Card::Three,
                    Card::King,
                    Card::King,
                    Card::Queen,
                ],
                HandType::TwoPair,
            ),
            (
                [Card::Four, Card::Ace, Card::King, Card::King, Card::Queen],
                HandType::OnePair,
            ),
            (
                [Card::Four, Card::Ace, Card::Jack, Card::King, Card::Queen],
                HandType::HighCard,
            ),
            (
                [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                HandType::ThreeOfAKind,
            ),
            (
                [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                HandType::ThreeOfAKind,
            ),
        ];

        for test in tests {
            let result = HandType::from(&test.0);

            assert_eq!(result, test.1);
        }
    }

    #[test]
    fn cmp_hands() {
        let a = Hand {
            cards: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
            bid: 0,
        };
        let b = Hand {
            cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
            bid: 0,
        };

        assert!(a > b);
    }

    #[test]
    fn jokerify() {
        let a = Hand {
            cards: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
            bid: 0,
        }
        .jokerify();

        let b = Hand {
            cards: [
                Card::Queen,
                Card::Queen,
                Card::Queen,
                Card::Queen,
                Card::Ace,
            ],
            bid: 0,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn jokerify_two_pair() {
        let a = Hand {
            cards: [Card::Queen, Card::Queen, Card::Ace, Card::Jack, Card::Ace],
            bid: 0,
        }
        .jokerify();

        let b = Hand {
            cards: [Card::Queen, Card::Queen, Card::Ace, Card::Ace, Card::Ace],
            bid: 0,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn part_1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = process_part_1(input);

        assert_eq!(result, 6440);
    }

    #[test]
    fn part_2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = process_part_2(input);

        assert_eq!(result, 5905);
    }
}
