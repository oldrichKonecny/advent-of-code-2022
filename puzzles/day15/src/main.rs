use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Sensor at x=(\d+), y=(\d+): closest beacon is at x=(\d+), y=(\d+)$").unwrap()
});

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(Sensor::parse)
        .collect::<Vec<_>>();
}

#[derive(Debug)]
struct Sensor {
    coordinates: (i32, i32),
    beacon_coordinates: (i32, i32),
    radius: u32,
}

impl Sensor {
    fn parse(str: &str) -> Self {
        let mut captured = REGEX.captures(str).unwrap();
        let sensor_x = captured[1].parse().unwrap();
        let sensor_y = captured[2].parse().unwrap();
        let beacon_x = captured[3].parse().unwrap();
        let beacon_y = captured[4].parse().unwrap();
        Self {
            coordinates: (sensor_x, sensor_y),
            beacon_coordinates: (beacon_x, beacon_y),
            radius: (sensor_x - beacon_x).abs() as u32 + (sensor_y - beacon_y).abs() as u32,
        }
    }

    fn interval_on_y(&self, y: i32) -> Option<(i32, i32)> {
        // let r = self.coordinates.1 -
        todo!()
    }
}
