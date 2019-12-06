pub fn module_fuel(mass: f32) -> f32 {
    f32::floor(mass / 3.) - 2.
}

pub fn fuel_required(input: Vec<f32>) -> f32 {
    input
        .iter()
        .fold(0., |fuel, mass| fuel + module_fuel(*mass))
}

pub fn solution() {
    let input = std::fs::read_to_string("./src/day_1/input.txt").unwrap();
    let input: Vec<f32> = input.lines().map(|line| line.parse().unwrap()).collect();
    println!("{}", fuel_required(input));
}
