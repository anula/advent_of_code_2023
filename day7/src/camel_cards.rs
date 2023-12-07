//use std::cmp::{max, min};
use std::io::{BufRead, BufReader, Write};
use std::collections::{HashMap};
use std::cmp::Ordering;

macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Num(i32),
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Card {
    fn from_char(s: char) -> Card {
        match s.to_string().parse::<i32>() {
            Ok(int) => {
                if int > 9 || int < 2 {
                    panic!("Got incorrect card: {}", int)
                }
                Card::Num(int)
            },
            Err(_) => {
                match s {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::T,
                    _ => panic!("Got incorrect card: {}", s)
                }
            }
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i64,
}

impl Hand {
    fn from_string(line: &str) -> Hand {
        let els: Vec<&str> = line.split_whitespace().collect();
        let mut cards = vec![];
        for c in els[0].chars() {
            cards.push(Card::from_char(c));
        }

        Hand {
            cards: cards,
            bid: els[1].parse().unwrap(),
        }
    }

    fn counted_cards(&self) -> HashMap<Card, usize> {
        let mut counts = HashMap::<Card, usize>::new();
        for card in &self.cards {
            *counts.entry(*card).or_insert(0) += 1;
        }
        counts
    }

    fn get_type(&self) -> HandType {
        let counts = self.counted_cards();
        let mut nums = counts.values().copied().collect::<Vec<usize>>();
        nums.sort();
        if counts.len() == 1 {
            return HandType::FiveOfKind;
        }

        if counts.len() == 2 {
            if nums[0] == 1 {
                return HandType::FourOfKind;
            } else {
                return HandType::FullHouse;
            }
        }

        if counts.len() == 3 {
            if nums[2] == 3 {
                return HandType::ThreeOfKind;
            }
            return HandType::TwoPairs;
        }

        if counts.len() == 4 {
            return HandType::OnePair;
        }

        return HandType::HighCard;
    }
}

#[derive(Debug)]
struct HandComparator {
    card_order: HashMap<Card, usize>,
    type_order: HashMap<HandType, usize>,
}

impl HandComparator {
    fn build() -> HandComparator {
        let mut card_order = HashMap::<Card, usize>::from([
            (Card::T, 10),
            (Card::J, 11),
            (Card::Q, 12),
            (Card::K, 13),
            (Card::A, 14),
        ]);
        for i in 2..=9 {
            card_order.insert(Card::Num(i), i as usize);
        }

        let type_order = HashMap::from([
            (HandType::HighCard, 0),
            (HandType::OnePair, 1),
            (HandType::TwoPairs, 2),
            (HandType::ThreeOfKind, 3),
            (HandType::FullHouse, 4),
            (HandType::FourOfKind, 5),
            (HandType::FiveOfKind, 6),
        ]);

        HandComparator {
            card_order: card_order,
            type_order: type_order,
        }
    }

    fn cmp(&self, a: &Hand, b: &Hand) -> Ordering {
        let type_a = a.get_type();
        let type_b = b.get_type();

        if type_a != type_b {
            return self.type_order[&type_a].cmp(&self.type_order[&type_b]);
        }

        for i in 0..5 {
            let card_a = a.cards[i];
            let card_b = b.cards[i];
            if card_a == card_b {
                continue
            }
            return self.card_order[&card_a].cmp(&self.card_order[&card_b]);
        }

        return Ordering::Equal;
    }
}

fn solve<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut solution: i64 = 0;

    let comp = HandComparator::build();
    let mut hands = vec![];

    for line_res in BufReader::new(input).lines() {
        let line = line_res.unwrap();
        hands.push(Hand::from_string(&line));
    }
    hands.sort_by(|a, b| comp.cmp(a, b));

    dprintln!("hands: {:?}", hands);

    for (i, hand) in hands.iter().enumerate() {
        solution += (i+1) as i64 * hand.bid;
    }

    writeln!(output, "{}", solution).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    solve(stdin.lock(), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_exact(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        assert_eq!(String::from_utf8(actual_out).unwrap(), output);
    }

    fn test_ignore_whitespaces(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve(input.as_bytes(), &mut actual_out);
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[test]
    fn sample() {
        test_ignore_whitespaces(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
            "6440",
        );
    }

    #[test]
    fn test_type_order() {
        test_ignore_whitespaces(
            "AAAAA 100000
            AA8AA 10000
            23332 1000
            TTT98 100
            23432 10
            A23A4 1
            23456 0",
            "765432",
        );
    }
}
