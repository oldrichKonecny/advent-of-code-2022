fn main() {
    let input = include_str!("../test_input.txt");
}

struct PacketPair {
    left: Packet,
    right: Packet,
}

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
    fn parse(line: &str) -> Self {}
}
