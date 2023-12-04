use std::cmp::min;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    have_numbers: Vec<usize>,
}

impl Card {
    fn parse(input: &str) -> Result<Self, String> {
        fn number(i: &str) -> IResult<&str, usize> {
            map_res(digit1, str::parse)(i)
        }

        let card_prefix = tuple((
            tag("Card"),
            many1(char::<&str, nom::error::Error<&str>>(' ')),
        ));

        let winning_numbers = separated_list1(many1(char(' ')), number);
        let have_numbers = separated_list1(many1(char(' ')), number);

        let numbers = separated_pair(
            winning_numbers,
            tuple((many1(char(' ')), char('|'), many1(char(' ')))),
            have_numbers,
        );

        let card_info = terminated(preceded(card_prefix, number), char(':'));

        let mut card = separated_pair(card_info, many1(char(' ')), numbers);

        let (rest, (card_id, (winning_numbers, have_numbers))) =
            card(input).finish().map_err(|e| e.to_string())?;

        assert!(rest.is_empty());

        Ok(Self {
            id: card_id,
            winning_numbers,
            have_numbers,
        })
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

fn part1(input: &str) -> Result<usize, String> {
    input
        .lines()
        .map(Card::parse)
        .try_fold(0, |accum, card| Ok(accum + card?.value()))
}

fn part2(input: &str) -> Result<usize, String> {
    let cards: Vec<Card> = input.lines().map(Card::parse).collect::<Result<_, _>>()?;
    let mut card_count = vec![1; cards.len()];

    for i in 0..cards.len() {
        let matches = cards[i].matching_numbers().len();
        for j in (i + 1)..min(i + matches + 1, cards.len()) {
            card_count[j] += card_count[i];
        }
    }
    Ok(card_count.into_iter().sum())
}

fn main() -> Result<(), String> {
    let input = include_str!("../input");

    println!("Part 1 : {}", part1(input)?);
    println!("Part 2 : {}", part2(input)?);

    Ok(())
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

        assert_eq!(part1(input).unwrap(), 13);
    }

    #[test]
    fn parse_card() {
        let input = "Card   1: 41 48 83  6 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            Card::parse(input).unwrap(),
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

        assert_eq!(part2(input).unwrap(), 30);
    }
}
