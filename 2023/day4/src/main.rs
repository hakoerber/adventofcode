use std::cmp::min;

use nom::{
    bytes::complete::tag,
    character::complete::char,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
};

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    have_numbers: Vec<usize>,
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::take_while1,
        character::complete::{char, satisfy},
        combinator::map,
        multi::count,
        sequence::preceded,
        IResult,
    };

    pub fn double_digit_number(i: &str) -> IResult<&str, usize> {
        let digit = satisfy(|c| c.is_ascii_digit());

        let single_digit = preceded(char(' '), count(&digit, 1));
        let double_digit = count(&digit, 2);

        let digit = alt((single_digit, double_digit));

        let mut digit = map(digit, |chars| {
            chars
                .into_iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        });

        digit(i)
    }

    pub fn number(i: &str) -> IResult<&str, usize> {
        map(
            take_while1::<_, &str, _>(|c| c.is_ascii_digit()),
            |s: &str| s.parse::<usize>().unwrap(),
        )(i)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_double_digit() {
            assert_eq!(double_digit_number("23").unwrap(), ("", 23));
            assert_eq!(double_digit_number("234").unwrap(), ("4", 23));
            assert_eq!(double_digit_number(" 234").unwrap(), ("34", 2));
            assert_eq!(double_digit_number(" 3").unwrap(), ("", 3));
            assert_eq!(double_digit_number(" 3 ").unwrap(), (" ", 3));
            assert!(double_digit_number("  3").is_err());
        }

        #[test]
        fn parse_number() {
            assert_eq!(number("23").unwrap(), ("", 23));
            assert_eq!(number("2").unwrap(), ("", 2));
            assert_eq!(number("2x").unwrap(), ("x", 2));
            assert!(number(" 3").is_err());
        }
    }
}

impl Card {
    fn parse(input: &str) -> Self {
        let card_prefix = tuple((tag("Card"), many1(char(' '))));

        let winning_numbers = separated_list1(char(' '), parser::double_digit_number);
        let have_numbers = separated_list1(char(' '), parser::double_digit_number);

        let numbers = separated_pair(winning_numbers, tag(" | "), have_numbers);

        let card_info = terminated(preceded(card_prefix, parser::number), char(':'));

        let mut card = separated_pair(card_info, char(' '), numbers);

        let (rest, (card_id, (winning_numbers, have_numbers))) = card(input).unwrap();

        assert!(rest.is_empty());

        Self {
            id: card_id,
            winning_numbers,
            have_numbers,
        }
    }

    fn matching_numbers(&self) -> Vec<usize> {
        self.have_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .copied()
            .collect()
    }

    fn value(&self) -> usize {
        let matches = self.matching_numbers().len();
        if matches == 0 {
            0
        } else {
            2_usize.pow(matches as u32 - 1)
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Card::parse)
        .map(|card| card.value())
        .sum()
}

fn part2(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().map(Card::parse).collect();
    let mut card_count: Vec<usize> = std::iter::repeat(1).take(cards.len()).collect();

    for i in 0..cards.len() {
        let matches = cards[i].matching_numbers().len();
        for j in (i + 1)..min(i + matches + 1, cards.len()) {
            card_count[j] += card_count[i];
        }
    }
    card_count.into_iter().sum()
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_01() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(part1(input), 13);
    }

    #[test]
    fn parse_card() {
        let input = "Card   1: 41 48 83  6 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            Card::parse(input),
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 6, 17],
                have_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );
    }

    #[test]
    fn example_02() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};

        assert_eq!(part2(input), 30);
    }
}
