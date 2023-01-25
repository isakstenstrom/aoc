use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(elem_l), Packet::Integer(elem_r)) => elem_l.cmp(elem_r),
            (Packet::List(_), Packet::Integer(elem_r)) => {
                self.cmp(&Packet::List(vec![Packet::Integer(*elem_r)]))
            }
            (Packet::Integer(elem_l), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*elem_l)]).cmp(other)
            }
            (Packet::List(vec_l), Packet::List(vec_r)) => {
                for (elem_l, elem_r) in vec_l.iter().zip(vec_r.iter()) {
                    let res = elem_l.cmp(elem_r);
                    if res != Ordering::Equal {
                        return res;
                    }
                }

                vec_l.len().cmp(&vec_r.len())
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePacketError;

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or(ParsePacketError)?;

        // Solution for splitting string including getting the separators as
        // separate elements from Stack Overflow user robinst:
        // https://stackoverflow.com/questions/32257273/split-a-string-keeping-the-separators
        let mut split_string = Vec::new();
        let mut prev_index = 0;
        for (index, separator) in input.match_indices(&[',', '[', ']'][..]) {
            if prev_index != index {
                split_string.push(&input[prev_index..index]);
            }
            if separator != "," {
                split_string.push(separator);
            }
            prev_index = index + separator.len();
        }
        if prev_index < input.len() {
            split_string.push(&input[prev_index..]);
        }

        Packet::parse_packet_from_iterator(&mut split_string.into_iter())
    }
}

impl Packet {
    fn parse_packet_from_iterator<'a, I>(input_iter: &mut I) -> Result<Self, ParsePacketError>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut packet_contents: Vec<Packet> = Vec::new();
        while let Some(elem) = input_iter.next() {
            if elem == "]" {
                break;
            } else if elem == "[" {
                packet_contents.push(Packet::parse_packet_from_iterator(input_iter)?);
            } else {
                packet_contents.push(Packet::Integer(
                    elem.parse::<i32>().map_err(|_| ParsePacketError)?,
                ));
            }
        }
        Ok(Packet::List(packet_contents))
    }
}

pub fn part1(input: &[String]) -> usize {
    let packets: Vec<Packet> = input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    packets
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, packets)| packets[0] < packets[1])
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &[String]) -> usize {
    // I assume that the divider packets are unique and the input will not
    // contain any packets identical to those. If there were, the problem would
    // not have a definite solution since the order of those packets isn't
    // specified.

    let packets: Vec<Packet> = input
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    let p1 = "[[2]]".parse::<Packet>().unwrap();
    let p2 = "[[6]]".parse::<Packet>().unwrap();

    (packets.iter().filter(|packet| *packet < &p1).count() + 1)
        * (packets.iter().filter(|packet| *packet < &p2).count() + 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_input_from_file;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&read_input_from_file("sample/day13.txt")), 13);
        assert_eq!(part1(&read_input_from_file("input/day13.txt")), 5808);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&read_input_from_file("sample/day13.txt")), 140);
        assert_eq!(part2(&read_input_from_file("input/day13.txt")), 22713);
    }
}
