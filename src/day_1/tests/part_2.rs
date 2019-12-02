use crate::day_1::part_2::{fuel_required, module_fuel};

#[test]
fn test_module_fuel() {
    assert_eq!(module_fuel(12.), 2.);
    assert_eq!(module_fuel(1969.), 966.);
    assert_eq!(module_fuel(100_756.), 50346.);
}

#[test]
fn test_fuel_required() {
    let input = vec![12., 1969., 100_756.];
    assert_eq!(fuel_required(input), 51314.);
}
