pub fn is_equal_length(password: &Vec<char>, length: usize) -> bool {
    password.len() == length
}

pub fn has_pair_same(password: &Vec<char>) -> bool {
    (0..password.len() - 1).any(|i| password[i] == password[i + 1])
}

pub fn has_decrease(password: &Vec<char>) -> bool {
    (0..password.len() - 1).any(|i| password[i + 1] < password[i])
}

pub fn good_password(password: &Vec<char>) -> bool {
    is_equal_length(&password, 6) && has_pair_same(&password) && !has_decrease(&password)
}

pub fn total_passwords(minimum: i32, maximum: i32) -> u32 {
    (minimum..=maximum).fold(0, |total, password| {
        let password = password.to_string().chars().collect();
        if good_password(&password) {
            total + 1
        } else {
            total
        }
    })
}

pub fn solution() {
    let input = std::fs::read_to_string("./src/day_4/input.txt").unwrap();
    let input: Vec<i32> = input
        .trim()
        .split('-')
        .map(|number| number.parse().unwrap())
        .collect();

    let (minimum, maximum) = (input[0], input[1]);
    println!("{}", total_passwords(minimum, maximum));
}
