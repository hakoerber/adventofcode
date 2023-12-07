use std::fmt::Debug;
use std::hash::Hash;
use std::{cmp::Ordering, collections::HashMap};

use nom::{
    character::complete::{anychar, char, digit1, multispace0, multispace1},
    combinator::map,
    multi::{many1, many_m_n, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

use strum_macros::EnumIter;

fn number(i: &str) -> IResult<&str, usize> {
    map(digit1, |f: &str| f.parse::<usize>().unwrap())(i)
}

fn spaces(i: &str) -> IResult<&str, Vec<char>> {
    many1(char(' '))(i)
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

trait Parse {
    type Out;
    fn parse(s: &str) -> IResult<&str, Self::Out>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CardWithoutJoker {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
enum CardWithJoker {
    Joker,
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

impl Parse for CardWithoutJoker {
    type Out = Self;
    fn parse(s: &str) -> IResult<&str, Self> {
        let (rest, c) = anychar(s)?;
        Ok((
            rest,
            match c {
                '2' => Self::Two,
                '3' => Self::Three,
                '4' => Self::Four,
                '5' => Self::Five,
                '6' => Self::Six,
                '7' => Self::Seven,
                '8' => Self::Eight,
                '9' => Self::Nine,
                'T' => Self::Ten,
                'J' => Self::Jack,
                'Q' => Self::Queen,
                'K' => Self::King,
                'A' => Self::Ace,
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        "unknown card",
                        nom::error::ErrorKind::Char,
                    )))
                }
            },
        ))
    }
}

impl Parse for CardWithJoker {
    type Out = Self;
    fn parse(s: &str) -> IResult<&str, Self> {
        let (rest, c) = anychar(s)?;
        Ok((
            rest,
            match c {
                '2' => Self::Two,
                '3' => Self::Three,
                '4' => Self::Four,
                '5' => Self::Five,
                '6' => Self::Six,
                '7' => Self::Seven,
                '8' => Self::Eight,
                '9' => Self::Nine,
                'T' => Self::Ten,
                'J' => Self::Joker,
                'Q' => Self::Queen,
                'K' => Self::King,
                'A' => Self::Ace,
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        "unknown card",
                        nom::error::ErrorKind::Char,
                    )))
                }
            },
        ))
    }
}

trait Hand: PartialEq + Eq + PartialOrd + Ord + ParseHand<Out = Self> + Debug {
    type CardType: PartialOrd + PartialEq + Eq + Ord + Parse<Out = Self::CardType>;

    fn hand_type(&self) -> HandType;

    fn cards(&self) -> &Vec<Self::CardType>;

    fn from_vec(v: Vec<Self::CardType>) -> Self;

    fn eq(&self, other: &Self) -> bool {
        for (i, card) in self.cards().iter().enumerate() {
            if *card != other.cards()[i] {
                return false;
            }
        }
        true
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (our_hand_type, their_hand_type) = (self.hand_type(), other.hand_type());

        if our_hand_type < their_hand_type {
            return Some(Ordering::Less);
        }

        if our_hand_type > their_hand_type {
            return Some(Ordering::Greater);
        }

        for (i, card) in self.cards().iter().enumerate() {
            if *card < other.cards()[i] {
                return Some(Ordering::Less);
            }
            if *card > other.cards()[i] {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }

    fn cmp(&self, other: &Self) -> Ordering {
        <Self as Hand>::partial_cmp(self, other).unwrap()
    }
}

#[derive(Debug)]
struct HandWithoutJoker(Vec<CardWithoutJoker>);

trait ParseHand {
    type Out: Hand;
    fn parse(s: &str) -> IResult<&str, Box<Self::Out>> {
        let (rest, cards) = many_m_n(5, 5, <<Self as ParseHand>::Out as Hand>::CardType::parse)(s)?;

        Ok((
            rest,
            Box::new(Self::Out::from_vec(
                cards
                    .into_iter()
                    .collect::<Vec<<Self::Out as Hand>::CardType>>(),
            )),
        ))
    }
}

impl ParseHand for HandWithoutJoker {
    type Out = Self;
}

#[derive(Debug)]
struct HandWithJoker(Vec<CardWithJoker>);

impl ParseHand for HandWithJoker {
    type Out = Self;
}

impl Hand for HandWithoutJoker {
    type CardType = CardWithoutJoker;

    fn cards(&self) -> &Vec<Self::CardType> {
        &self.0
    }

    fn from_vec(v: Vec<Self::CardType>) -> Self {
        Self(v)
    }

    fn hand_type(&self) -> HandType {
        let mut types: HashMap<Self::CardType, usize> = HashMap::new();
        self.0.iter().for_each(|card| {
            types
                .entry(*card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
        match types.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                let max_count = types.values().max().unwrap();
                if *max_count == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                let max_count = types.values().max().unwrap();
                if *max_count == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            1 => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for HandWithoutJoker {
    fn eq(&self, other: &Self) -> bool {
        Hand::eq(self, other)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for HandWithoutJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Hand::partial_cmp(self, other)
    }
}

impl Eq for HandWithoutJoker {}

impl Ord for HandWithoutJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        Hand::partial_cmp(self, other).unwrap()
    }
}

impl Hand for HandWithJoker {
    type CardType = CardWithJoker;

    fn cards(&self) -> &Vec<Self::CardType> {
        &self.0
    }

    fn from_vec(v: Vec<Self::CardType>) -> Self {
        Self(v)
    }

    fn hand_type(&self) -> HandType {
        let mut types: HashMap<Self::CardType, usize> = {
            let mut types = HashMap::new();

            self.cards().iter().for_each(|card| {
                types
                    .entry(*card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });

            types
        };

        // The joker logic is simple: The best way to improve our hand it to use the jokers as the
        // same card that we have the *most* of. As jokers substitutions are only used for type
        // calcuation and not for the value, it does not matter *which* card we pick if there are
        // multiple with the same number
        if let Some(joker_count) = types.get(&CardWithJoker::Joker).copied() {
            if let Some(highest_non_joker_card) = types
                .iter()
                .filter(|(card, _count)| **card != CardWithJoker::Joker)
                .max_by_key(|(_card, count)| *count)
                .map(|(card, _number)| *card)
            {
                *types.get_mut(&highest_non_joker_card).unwrap() += joker_count;
                types.remove(&CardWithJoker::Joker);
            } else {
                // all cards are jokers
                return HandType::FiveOfAKind;
            }
        }

        match types.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                let max_count = types.values().max().unwrap();
                if *max_count == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            2 => {
                let max_count = types.values().max().unwrap();
                if *max_count == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            1 => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for HandWithJoker {
    fn eq(&self, other: &Self) -> bool {
        Hand::eq(self, other)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Hand::partial_cmp(self, other)
    }
}

impl Eq for HandWithJoker {}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        Hand::partial_cmp(self, other).unwrap()
    }
}

#[derive(Debug)]
struct HandWithBid<T: Hand<Out = T>> {
    hand: Box<T>,
    bid: usize,
}

impl<T: Hand<Out = T>> HandWithBid<T> {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (rest, (hand, bid)) = separated_pair(T::parse, spaces, number)(s)?;
        Ok((
            rest,
            Self {
                hand: Box::new(*hand),
                bid,
            },
        ))
    }
}

impl<T: Hand<Out = T>> PartialEq for HandWithBid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl<T: Hand<Out = T>> Eq for HandWithBid<T> {}

impl<T: Hand<Out = T>> PartialOrd for HandWithBid<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}

impl<T: Hand<Out = T>> Ord for HandWithBid<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Hands<T: Hand<Out = T>>(Vec<HandWithBid<T>>);

impl<T: Hand<Out = T>> Hands<T> {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (rest, hands) = delimited(
            multispace0,
            separated_list1(multispace1, HandWithBid::parse),
            multispace0,
        )(s)?;

        Ok((rest, Self(hands)))
    }

    fn sort_by_cards(&mut self) {
        self.0.sort();
    }
}

fn part1(input: &str) -> Result<usize, String> {
    let (rest, mut hands) = Hands::<HandWithoutJoker>::parse(input).map_err(|e| e.to_string())?;
    assert!(rest.is_empty());

    hands.sort_by_cards();

    let out = hands
        .0
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();

    Ok(out)
}

fn part2(input: &str) -> Result<usize, String> {
    let (rest, mut hands) = Hands::<HandWithJoker>::parse(input).map_err(|e| e.to_string())?;
    assert!(rest.is_empty());

    hands.sort_by_cards();

    let out = hands
        .0
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();

    Ok(out)
}

fn main() -> Result<(), String> {
    let input = include_str!("../input");

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let part = args[0].parse::<usize>().unwrap();

    if part == 1 {
        println!("Part 1 : {}", part1(input)?);
    } else if part == 2 {
        println!("Part 2 : {}", part2(input)?);
    } else {
        return Err("unknown part".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_01() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        assert_eq!(part1(input).unwrap(), 6440);
    }

    #[test]
    fn example_02() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};
        assert_eq!(part2(input).unwrap(), 5905);
    }
}
