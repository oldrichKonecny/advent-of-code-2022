use once_cell::sync::Lazy;
use regex::Regex;
use std::time::Instant;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
        .unwrap()
});

fn main() {
    let start = Instant::now();
    let sensors = include_str!("../input.txt")
        .lines()
        .map(Sensor::parse)
        .collect::<Vec<_>>();

    println!("First res: {}", first_solution(&sensors));
    let sec_sol = second_solution(&sensors)
        .map(|(x, y)| x as i64 * 4_000_000 + y as i64)
        .unwrap();
    println!("Second part: {}", sec_sol);

    let end = start.elapsed();
    println!("Duration:");
    println!("\t{} ms", end.as_millis());
    println!("\t{} us", end.as_micros());
    println!("\t{} ns", end.as_nanos());
}

fn second_solution(sensors: &[Sensor]) -> Option<(i32, i32)> {
    for sensor in sensors {
        let borders = sensor.get_outer_border();
        for border in borders {
            if !sensors.iter().any(|sen| sen.contains(border)) {
                return Some(border);
            }
        }
    }
    None
}

fn first_solution(sensors: &[Sensor]) -> i32 {
    let mut intervals = sensors
        .iter()
        .flat_map(|sensor| sensor.interval_on_y(2_000_000)) //10
        .collect::<Vec<_>>();

    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut unique_intervals = Vec::new();
    let mut possible_merge = 0;
    unique_intervals.push(intervals.remove(0));
    for i in intervals {
        let int = unique_intervals.get_mut(possible_merge).unwrap();
        if i.0 <= int.1 && i.1 > int.1 {
            int.1 = i.1;
        } else if i.0 > int.1 {
            unique_intervals.push(i);
            possible_merge += 1;
        }
    }

    unique_intervals
        .iter()
        .fold(0, |acc, val| acc + ((val.1 - val.0) + 1))
}

#[derive(Debug)]
struct Sensor {
    coordinates: (i32, i32),
    beacon_coordinates: (i32, i32),
    radius: i32,
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
            radius: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs(),
        }
    }

    fn interval_on_y(&self, y: i32) -> Option<(i32, i32)> {
        let r = (self.coordinates.1 - y).abs();
        if r > self.radius {
            None
        } else {
            let pl = self.radius - r;
            let mut left = self.coordinates.0 - pl;
            let mut right = self.coordinates.0 + pl;
            if self.beacon_coordinates == (left, y) {
                left += 1;
            } else if self.beacon_coordinates == (right, y) {
                right -= 1;
            }
            Some((left, right))
        }
    }

    fn get_outer_border(&self) -> Vec<(i32, i32)> {
        fn check_boundaries(point: (i32, i32)) -> bool {
            let (a, b) = point;
            (a >= 0 && a <= 4_000_000) && (b >= 0 && b <= 4_000_000)
        }
        let (s_x, s_y) = self.coordinates;
        let mut res = Vec::new();

        let j = self.radius + 1;
        for i in 0..=j {
            let p1 = (s_x + i, s_y + (j - i));
            let p2 = (s_x + i, s_y - (j - i));
            let p3 = (s_x - i, s_y + (j - i));
            let p4 = (s_x - i, s_y - (j - i));
            if check_boundaries(p1) {
                res.push(p1);
            }
            if check_boundaries(p2) {
                res.push(p2);
            }
            if check_boundaries(p3) {
                res.push(p3);
            }
            if check_boundaries(p4) {
                res.push(p4);
            }
        }
        res
    }

    fn contains(&self, point: (i32, i32)) -> bool {
        let (x, y) = self.coordinates;
        self.radius >= (x - point.0).abs() + (y - point.1).abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::Sensor;

    #[test]
    fn sensor_interval_on_y_0_0_test() {
        let sensor = Sensor::parse("Sensor at x=0, y=0: closest beacon is at x=0, y=10");
        assert_eq!(sensor.radius, 10);

        assert_eq!(sensor.interval_on_y(0), Some((-10, 10)));
        assert_eq!(sensor.interval_on_y(10), Some((0, 0)));
        assert_eq!(sensor.interval_on_y(-10), Some((0, 0)));
        assert_eq!(sensor.interval_on_y(1), Some((-9, 9)));
        assert_eq!(sensor.interval_on_y(-1), Some((-9, 9)));
        assert_eq!(sensor.interval_on_y(5), Some((-5, 5)));
        assert_eq!(sensor.interval_on_y(5), Some((-5, 5)));
        assert_eq!(sensor.interval_on_y(11), None);
        assert_eq!(sensor.interval_on_y(-11), None);
        assert_eq!(sensor.interval_on_y(100), None);
        assert_eq!(sensor.interval_on_y(-500), None);
    }

    #[test]
    fn sensor_interval_on_y_8_7_test() {
        let sensor = Sensor::parse("Sensor at x=8, y=7: closest beacon is at x=2, y=10");
        assert_eq!(sensor.radius, 9);

        assert_eq!(sensor.interval_on_y(7), Some((-1, 17)));
        assert_eq!(sensor.interval_on_y(-2), Some((8, 8)));
        assert_eq!(sensor.interval_on_y(16), Some((8, 8)));
        assert_eq!(sensor.interval_on_y(-1), Some((7, 9)));
        assert_eq!(sensor.interval_on_y(15), Some((7, 9)));
        assert_eq!(sensor.interval_on_y(6), Some((0, 16)));
        assert_eq!(sensor.interval_on_y(10), Some((2, 14)));
        assert_eq!(sensor.interval_on_y(-3), None);
        assert_eq!(sensor.interval_on_y(17), None);
        assert_eq!(sensor.interval_on_y(100), None);
        assert_eq!(sensor.interval_on_y(-500), None);
    }

    #[test]
    fn sensor_get_borders_test() {
        let sensor = Sensor::parse("Sensor at x=10, y=10: closest beacon is at x=5, y=5");

        let borders = sensor.get_outer_border();
        println!("{:?}", borders);
    }

    #[test]
    fn sensor_contains_test() {
        let sensor = Sensor::parse("Sensor at x=10, y=10: closest beacon is at x=5, y=5");

        let borders = sensor.get_outer_border();
        for border in borders {
            assert_eq!(sensor.contains(border), false);
        }
    }
}
