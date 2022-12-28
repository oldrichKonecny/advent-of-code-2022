use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z,\s]+)$")
        .unwrap()
});

fn main() {
    let input = include_str!("../test_input.txt");
    let map = compute_map(input);

    println!("First part: {:?}", first_solution(&map));
}

fn first_solution(valves: &HashMap<String, ValveInfo>) -> Option<usize> {
    let final_results = Rc::new(RefCell::new(Vec::new()));
    let mut opened_valves = HashSet::new();
    recurse_the_thing_out_of_it(
        valves,
        &mut opened_valves,
        30,
        "AA".to_string(),
        0,
        final_results.clone(),
    );

    let result = final_results.as_ref().borrow().iter().map(|val| *val).max();
    result
}

fn recurse_the_thing_out_of_it(
    valves: &HashMap<String, ValveInfo>,
    opened_valves: &mut HashSet<String>,
    timer: u32,
    current_valve: String,
    result: usize,
    final_results: Rc<RefCell<Vec<usize>>>,
) {
    if timer == 0 || opened_valves.len() == 7 {
        if result > 0 {
            final_results.borrow_mut().push(result);
        }
        return;
    }
    let valve = &valves[&current_valve];

    // if !opened_valves.contains(&current_valve) && valve.flow > 0 && timer > 1 {
    //     let new_timer = timer - 1;
    //     let new_res = result + new_timer as usize * valve.flow;
    //     opened_valves.insert(current_valve.clone());
    //     for tunnel in &valve.tunnels {
    //         recurse_the_thing_out_of_it(
    //             valves.clone(),
    //             opened_valves,
    //             new_timer - 1,
    //             tunnel.clone(),
    //             new_res,
    //             final_results.clone(),
    //         );
    //     }
    // }
    //
    // for tunnel in &valve.tunnels {
    //     recurse_the_thing_out_of_it(
    //         valves.clone(),
    //         opened_valves,
    //         timer - 1,
    //         tunnel.clone(),
    //         result,
    //         final_results.clone(),
    //     );
    // }
}

#[derive(Debug)]
struct ValveInfo {
    flow: usize,
    tunnels: HashMap<String, usize>,
}

fn compute_map(input: &str) -> HashMap<String, ValveInfo> {
    let parsed_input = input
        .lines()
        .map(parse_input)
        .collect::<HashMap<String, (usize, HashSet<String>)>>();

    let mut map = HashMap::new();
    for (name, (flow, _)) in parsed_input.iter().filter(|x| x.1 .0 != 0) {
        let name = name.clone();
        let flow = *flow;
        let tunnels = compute_tunnels(&parsed_input, &name);
        map.insert(name, ValveInfo { flow, tunnels });
    }
    map
}

fn compute_tunnels(
    pre_map: &HashMap<String, (usize, HashSet<String>)>,
    start: &str,
) -> HashMap<String, usize> {
    let mut res = HashMap::new();
    let (_, tunnels) = &pre_map[start];
    for t in tunnels.iter() {
        let (f, tt) = &pre_map[t];
        if *f > 0 {
            if res.contains_key(t) {
            } else {
            }
        }
    }

    res
}

fn parse_input(str: &str) -> (String, (usize, HashSet<String>)) {
    let captured = REGEX
        .captures(str)
        .expect(&format!("Cannot parse: {}", str));
    let name = captured[1].to_string();
    let flow = captured[2].parse().unwrap();
    let tunnels = captured[3].split(", ").map(|s| s.to_string()).collect();
    (name, (flow, tunnels))
}
