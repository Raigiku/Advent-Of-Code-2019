use std::collections::HashMap;

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

pub fn wire_positions(
    line: &Vec<String>,
    circuit_board: &mut HashMap<(i32, i32), (u32, usize)>,
    intersections: &mut Vec<(i32, i32)>,
    i: usize,
) {
    let mut steps = 1;
    let mut start_point = (0, 0);
    let mut last_point = start_point;
    for command in line {
        if let Some(direction) = command.chars().nth(0) {
            let units: i32 = (&command[1..]).parse().unwrap();
            for unit in 1..=units {
                last_point = next_point(unit, direction, start_point);
                if let Some((last_steps, j)) = circuit_board.get_mut(&last_point) {
                    if i != *j {
                        *last_steps += steps;
                        intersections.push(last_point);
                    }
                } else {
                    circuit_board.insert(last_point, (steps, i));
                }
                steps += 1;
            }
            start_point = last_point;
        }
    }
}

pub fn fewest_combined_steps(
    intersections: &Vec<(i32, i32)>,
    circuit_board: &HashMap<(i32, i32), (u32, usize)>,
) -> u32 {
    intersections
        .iter()
        .map(|intersection| circuit_board.get(&intersection).unwrap().0)
        .min()
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

    let mut circuit_board = HashMap::new();
    let mut intersections = Vec::new();
    for (i, line) in input.iter().enumerate() {
        wire_positions(line, &mut circuit_board, &mut intersections, i);
    }
    println!(
        "{:?}",
        fewest_combined_steps(&intersections, &circuit_board)
    );
}
