use std::cmp::Ordering;

#[cfg(not(target_os = "windows"))]
const EMPTY_LINE_PATTERN: &str = "\n\n";
#[cfg(target_os = "windows")]
const EMPTY_LINE_PATTERN: &str = "\r\n\r\n";

fn main() {
    let packet_pairs = include_str!("../test_input.txt")
        .split(EMPTY_LINE_PATTERN)
        .map(PacketPair::parse)
        // .inspect(|pp| println!("{:?}\n{:?}", pp.left, pp.right))
        .collect::<Vec<_>>();
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

#[derive(Debug)]
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
            (Self::Val(x), Self::List(l)) => l.len() == 1 && l[0] == *self,
            (Self::List(l), Self::Val(x)) => l.len() == 1 && l[0] == *self,
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
            _ => panic!("Dude, this partial_eq should be unreachable.."),
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
                let ord = self.partial_cmp(*l[0]);
                if l.len() == 1 {
                    ord
                } else if ord.is_some() && ord.unwrap() == Ordering::Equal {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Less)
                }
            }
            (Self::List(l), Self::Val(_)) => l[0].partial_cmp(other),
            (Self::List(l1), Self::List(l2)) => {
                l1.len() == l2.len() && {
                    for i in 0..l1.len() {
                        if *l1[i] != *l2[i] {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => panic!("Dude, this partial_ord should be unreachable.."),
        }
    }
}
