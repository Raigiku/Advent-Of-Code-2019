use crate::day_3::part_1::{
    manhattan_distance, minimum_distance_intersection, next_point, wire_intersections,
    wire_positions,
};
use std::collections::HashSet;

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
        "U2".to_string(),
        "D3".to_string(),
    ];
    let mut expected_output = HashSet::new();
    expected_output.insert((1, 0));
    expected_output.insert((0, 0));
    expected_output.insert((0, 1));
    expected_output.insert((0, 2));
    expected_output.insert((0, -1));
    assert_eq!(wire_positions(&input), expected_output);
}

#[test]
fn test_wire_intersections() {
    let mut wire_1 = HashSet::new();
    wire_1.insert((1, 0));
    wire_1.insert((2, 0));
    wire_1.insert((2, -1));
    let mut wire_2 = HashSet::new();
    wire_2.insert((0, -1));
    wire_2.insert((1, -1));
    wire_2.insert((2, -1));
    let input = vec![wire_1, wire_2];
    assert_eq!(wire_intersections(&input), vec![(2, -1)]);
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(manhattan_distance((3, 3)), 6);
    assert_eq!(manhattan_distance((3, -2)), 5);
    assert_eq!(manhattan_distance((-2, 3)), 5);
    assert_eq!(manhattan_distance((-1, -1)), 2);
}

#[test]
fn test_minimum_distance_intersection() {
    assert_eq!(
        minimum_distance_intersection(&vec![(3, 3), (3, -2), (-2, 3), (-1, -1)]),
        2
    );
}
