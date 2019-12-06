use std::collections::{HashMap, HashSet};

pub fn next_point(unit: i32, direction: char, point: (i32, i32)) -> (i32, i32) {
    let mut directions = HashMap::new();
    directions.insert('R', (unit, 0));
    directions.insert('L', (-unit, 0));
    directions.insert('U', (0, unit));
    directions.insert('D', (0, -unit));
    let (x, y) = point;
    let (direction_x, direction_y) = *directions.get(&direction).unwrap();
    (x + direction_x, y + direction_y)
}

pub fn wire_positions(line: &Vec<String>) -> HashSet<(i32, i32)> {
    let mut start_point = (0, 0);
    let mut last_point = start_point;
    let mut wire = HashSet::new();
    for command in line {
        if let Some(direction) = command.chars().nth(0) {
            let units: i32 = (&command[1..]).parse().unwrap();
            for unit in 1..=units {
                last_point = next_point(unit, direction, start_point);
                wire.insert(last_point);
            }
            start_point = last_point;
        }
    }
    wire
}

pub fn wire_intersections(wires: &Vec<HashSet<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for i in 0..wires.len() {
        for j in i + 1..wires.len() {
            let intersections = wires[i].intersection(&wires[j]);
            for intersection in intersections {
                result.push(*intersection);
            }
        }
    }
    result
}

pub fn manhattan_distance(distance: (i32, i32)) -> i32 {
    let (x, y) = distance;
    x.abs() + y.abs()
}

pub fn minimum_distance_intersection(intersections: &Vec<(i32, i32)>) -> i32 {
    intersections
        .iter()
        .min_by_key(|intersection| manhattan_distance(**intersection))
        .map(|intersection| manhattan_distance(*intersection))
        .unwrap()
}

pub fn solution() {
    let input = std::fs::read_to_string("./src/day_3/input.txt").unwrap();
    let input: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|command| command.parse().unwrap())
                .collect()
        })
        .collect();

    let mut wires = Vec::new();
    for line in input.iter() {
        wires.push(wire_positions(line));
    }
    let intersections = wire_intersections(&wires);
    let minimum_intersection = minimum_distance_intersection(&intersections);
    println!("{:?}", minimum_intersection);
}
