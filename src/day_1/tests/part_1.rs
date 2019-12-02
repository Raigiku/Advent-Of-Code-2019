use crate::day_1::part_1::{fuel_required, module_fuel};

#[test]
fn test_module_fuel() {
    assert_eq!(module_fuel(12.), 2.);
    assert_eq!(module_fuel(14.), 2.);
    assert_eq!(module_fuel(1969.), 654.);
    assert_eq!(module_fuel(100756.), 33583.);
}

#[test]
fn test_fuel_required() {
    let input = vec![12., 14., 1969., 100756.];
    assert_eq!(fuel_required(input), 34241.);
}
