use std::cmp::Ordering;

#[cfg(not(target_os = "windows"))]
const EMPTY_LINE_PATTERN: &str = "\n\n";
#[cfg(target_os = "windows")]
const EMPTY_LINE_PATTERN: &str = "\r\n\r\n";

fn main() {
    let packet_pairs = include_str!("../input.txt")
        .split(EMPTY_LINE_PATTERN)
        .map(PacketPair::parse)
        .collect::<Vec<_>>();

    println!("First part: {}", first_solution(&packet_pairs));
    println!("Second part: {}", second_solution(packet_pairs));
}

fn second_solution(packet_pairs: Vec<PacketPair>) -> usize {
    let divider2 = Packet::List(vec![Packet::Val(2)]);
    let divider6 = Packet::List(vec![Packet::Val(6)]);

    let mut packets = packet_pairs
        .into_iter()
        .flat_map(|pp| [pp.left, pp.right])
        .collect::<Vec<_>>();
    packets.push(divider2.clone());
    packets.push(divider6.clone());

    packets.sort();

    packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| *p == divider2 || *p == divider6)
        .map(|(index, _)| index + 1)
        .reduce(|acc, val| acc * val)
        .unwrap()
}

fn first_solution(packet_pairs: &[PacketPair]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, pp)| pp.left < pp.right)
        .map(|(index, _)| index + 1)
        .sum::<usize>()
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, Clone, Eq, Ord)]
enum Packet {
    Val(u32),
    List(Vec<Packet>),
}

impl PacketPair {
    fn parse(str: &str) -> Self {
        let mut lines = str.lines();
        Self {
            left: Packet::parse(lines.next().unwrap()),
            right: Packet::parse(lines.next().unwrap()),
        }
    }
}

impl Packet {
    fn parse(line: &str) -> Self {
        match line {
            "[]" => Packet::List(Vec::new()),
            l if l.starts_with('[') && l.ends_with(']') => {
                let mut counter = 0;
                Packet::List(
                    l[1..l.len() - 1]
                        .split(|c| {
                            if c == '[' {
                                counter += 1;
                            } else if c == ']' {
                                counter -= 1;
                            }
                            c == ',' && counter == 0
                        })
                        .map(|s| Packet::parse(s))
                        .collect::<Vec<_>>(),
                )
            }
            l => l
                .parse::<u32>()
                .map(Packet::Val)
                .unwrap_or_else(|_| panic!("Cannot parse: {}", l)),
        }
    }
}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Val(x), Self::Val(y)) => *x == *y,
            (Self::Val(_), Self::List(l)) => l.len() == 1 && l[0] == *self,
            (Self::List(l), Self::Val(_)) => l.len() == 1 && l[0] == *other,
            (Self::List(l1), Self::List(l2)) => {
                l1.len() == l2.len() && {
                    for i in 0..l1.len() {
                        if l1[i] != l2[i] {
                            return false;
                        }
                    }
                    true
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Val(x), Self::Val(y)) => x.partial_cmp(y),
            (Self::Val(_), Self::List(l)) => {
                if l.is_empty() {
                    return Some(Ordering::Greater);
                }
                let ord = self.partial_cmp(&l[0]);
                if l.len() == 1 {
                    ord
                } else if ord.is_some() && ord.unwrap() == Ordering::Equal {
                    Some(Ordering::Less)
                } else {
                    ord
                }
            }
            (Self::List(l), Self::Val(_)) => {
                if l.is_empty() {
                    return Some(Ordering::Less);
                }
                let ord = l[0].partial_cmp(other);
                if l.len() == 1 {
                    ord
                } else if ord.is_some() && ord.unwrap() == Ordering::Equal {
                    Some(Ordering::Greater)
                } else {
                    ord
                }
            }
            (Self::List(l1), Self::List(l2)) => {
                for i in 0..l1.len().min(l2.len()) {
                    let ord = l1[i].partial_cmp(&l2[i]);
                    if ord != Some(Ordering::Equal) {
                        return ord;
                    }
                }
                if l1.len() > l2.len() {
                    Some(Ordering::Greater)
                } else if l1.len() < l2.len() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Packet;

    #[test]
    fn packet2_equal_test() {
        let packet2 = Packet::List(vec![Packet::Val(2)]);

        let packet_list_list_0 = Packet::List(vec![Packet::List(vec![Packet::Val(0)])]);
        let packet_list_list_8 = Packet::List(vec![Packet::List(vec![Packet::Val(8)])]);

        assert_ne!(packet2, packet_list_list_0);
        assert_ne!(packet2, packet_list_list_8);
    }

    #[test]
    fn packet6_equal_test() {
        let packet6 = Packet::List(vec![Packet::Val(6)]);

        let packet_list_list_0 = Packet::List(vec![Packet::List(vec![Packet::Val(0)])]);
        let packet_list_list_8 = Packet::List(vec![Packet::List(vec![Packet::Val(8)])]);

        assert_ne!(packet6, packet_list_list_0);
        assert_ne!(packet6, packet_list_list_8);
    }
}
