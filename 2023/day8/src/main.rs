use std::collections::HashMap;

use nom::{
    character::complete::{anychar, char, multispace0, newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn lcm(numbers: impl Iterator<Item = usize>) -> usize {
    fn lcm(a: usize, b: usize) -> usize {
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else if a == 0 {
                b
            } else if a > b {
                gcd(a - b, b)
            } else {
                gcd(a, b - a)
            }
        }
        a / gcd(a, b) * b
    }

    numbers.reduce(lcm).unwrap()
}

trait Parse {
    type Out;
    fn parse(s: &str) -> IResult<&str, Self::Out>;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node(char, char, char);

fn spaces(i: &str) -> IResult<&str, Vec<char>> {
    many1(char(' '))(i)
}

impl Parse for Node {
    type Out = Self;

    fn parse(s: &str) -> IResult<&str, Self::Out> {
        let (rest, element) = tuple((anychar, anychar, anychar))(s)?;
        Ok((rest, element.into()))
    }
}

impl TryFrom<&str> for Node {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars = value.chars().take(3).collect::<Vec<char>>();
        Ok(Self(chars[0], chars[1], chars[2]))
    }
}

impl From<(char, char, char)> for Node {
    fn from((a, b, c): (char, char, char)) -> Self {
        Self(a, b, c)
    }
}

impl Node {
    fn ends_with(&self, c: char) -> bool {
        self.2 == c
    }
}

#[derive(Debug)]
struct NodeEdges(HashMap<Node, (Node, Node)>);

impl Parse for NodeEdges {
    type Out = Self;

    fn parse(s: &str) -> IResult<&str, Self::Out> {
        let (rest, edges) = separated_list1(
            newline,
            separated_pair(
                Node::parse,
                tuple((spaces, char('='), spaces)),
                delimited(
                    char('('),
                    separated_pair(Node::parse, tuple((char(','), spaces)), Node::parse),
                    char(')'),
                ),
            ),
        )(s)?;
        let edges: HashMap<Node, (Node, Node)> = HashMap::from_iter(edges);
        Ok((rest, Self(edges)))
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => Err(format!("unknow direction {value}"))?,
        })
    }
}

impl Parse for Direction {
    type Out = Self;

    fn parse(s: &str) -> IResult<&str, Self::Out> {
        let (rest, c) = map(one_of("RL"), |f: char| f.try_into().unwrap())(s)?;
        Ok((rest, c))
    }
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    edges: NodeEdges,
}

impl Parse for Map {
    type Out = Self;

    fn parse(s: &str) -> IResult<&str, Self::Out> {
        let (rest, (directions, edges)) = delimited(
            multispace0,
            separated_pair(many1(Direction::parse), many1(newline), NodeEdges::parse),
            multispace0,
        )(s)?;
        Ok((rest, Self { directions, edges }))
    }
}

fn part1(input: &str) -> Result<usize, String> {
    let (rest, map) = Map::parse(input).map_err(|e| e.to_string())?;
    assert!(rest.is_empty());

    let start: Node = "AAA".try_into().unwrap();
    let goal: Node = "ZZZ".try_into().unwrap();

    let mut current_node = start;

    let mut steps = 0;
    for direction in map.directions.into_iter().cycle() {
        if current_node == goal {
            break;
        }
        let new_node = match direction {
            Direction::Left => map.edges.0[&current_node].0,
            Direction::Right => map.edges.0[&current_node].1,
        };
        current_node = new_node;
        steps += 1;
    }
    Ok(steps)
}

fn part2(input: &str) -> Result<usize, String> {
    let (rest, map) = Map::parse(input).map_err(|e| e.to_string())?;
    assert!(rest.is_empty());

    let node_chain_lengths = map
        .edges
        .0
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| {
            let mut depth = 1;
            let mut node = *node;
            for direction in map.directions[..].iter().cycle() {
                let new_node = match direction {
                    Direction::Left => map.edges.0[&node].0,
                    Direction::Right => map.edges.0[&node].1,
                };
                if new_node.ends_with('Z') {
                    break;
                }
                node = new_node;
                depth += 1;
            }

            depth
        });

    Ok(lcm(node_chain_lengths))
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
    fn part_1_example_1() {
        let input = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};
        assert_eq!(part1(input).unwrap(), 2);
    }

    #[test]
    fn part_1_example_2() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};
        assert_eq!(part1(input).unwrap(), 6);
    }

    #[test]
    fn part_2_example_1() {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};
        assert_eq!(part2(input).unwrap(), 6);
    }
}
