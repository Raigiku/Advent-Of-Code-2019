use crate::day_3::part_2::{fewest_combined_steps, next_point, wire_positions};
use std::collections::HashMap;

#[test]
fn test_next_point() {
    assert_eq!(next_point(1, 'R', (0, 0)), (1, 0));
    assert_eq!(next_point(2, 'L', (1, 2)), (-1, 2));
    assert_eq!(next_point(3, 'D', (-2, -2)), (-2, -5));
    assert_eq!(next_point(4, 'U', (3, 2)), (3, 6));
}

#[test]
fn test_wire_positions() {
    let input = vec![
        "R1".to_string(),
        "L1".to_string(),
        "U1".to_string(),
        "D1".to_string(),
    ];
    let mut circuit_board = HashMap::new();
    let mut intersections = Vec::new();
    wire_positions(&input, &mut circuit_board, &mut intersections, 0);
    let mut expected_output = HashMap::new();
    expected_output.insert((1, 0), (1, 0));
    expected_output.insert((0, 0), (2, 0));
    expected_output.insert((0, 1), (3, 0));
    assert_eq!(circuit_board, expected_output);
}

#[test]
fn test_wire_intersections() {
    let input = vec![
        vec!["R1".to_string(), "U1".to_string()],
        vec!["U1".to_string(), "R1".to_string()],
    ];
    let mut circuit_board = HashMap::new();
    let mut intersections = Vec::new();
    for (i, line) in input.iter().enumerate() {
        wire_positions(line, &mut circuit_board, &mut intersections, i);
    }
    let expected_output = vec![(1, 1)];
    assert_eq!(intersections, expected_output);
}

#[test]
fn test_fewest_combined_steps() {
    let intersections = vec![(1, 1), (3, 3)];
    let mut circuit_board = HashMap::new();
    circuit_board.insert((1, 0), (1, 0));
    circuit_board.insert((1, 1), (2 + 2, 0));
    circuit_board.insert((2, 1), (3, 0));
    circuit_board.insert((2, 2), (4, 0));
    circuit_board.insert((3, 2), (5, 0));
    circuit_board.insert((3, 3), (6 + 6, 0));

    circuit_board.insert((0, 1), (1, 1));
    //    circuit_board.insert((1,1),(2,1));
    circuit_board.insert((1, 2), (3, 1));
    circuit_board.insert((1, 3), (4, 1));
    circuit_board.insert((2, 3), (5, 1));
    //    circuit_board.insert((3,3),(6,1));
    assert_eq!(fewest_combined_steps(&intersections, &circuit_board), 4);
}
