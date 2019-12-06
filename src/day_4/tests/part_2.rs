use crate::day_4::part_2::{
    good_password, has_decrease, has_one_pair_same, is_equal_length, total_passwords,
};

#[test]
fn test_is_equal_length() {
    let input = "154897".to_string().chars().collect();
    assert_eq!(is_equal_length(&input, 6), true);
    assert_eq!(is_equal_length(&input, 3), false);
}

#[test]
fn test_has_one_pair_same() {
    let input = "000".chars().collect();
    assert_eq!(has_one_pair_same(&input), false);
    let input = "001".chars().collect();
    assert_eq!(has_one_pair_same(&input), true);
    let input = "010".chars().collect();
    assert_eq!(has_one_pair_same(&input), false);
    let input = "011".chars().collect();
    assert_eq!(has_one_pair_same(&input), true);
    let input = "011110".chars().collect();
    assert_eq!(has_one_pair_same(&input), false);
    let input = "001110".chars().collect();
    assert_eq!(has_one_pair_same(&input), true);
}

#[test]
fn test_has_decrease() {
    let input = "11".chars().collect();
    assert_eq!(has_decrease(&input), false);
    let input = "12".chars().collect();
    assert_eq!(has_decrease(&input), false);
    let input = "21".chars().collect();
    assert_eq!(has_decrease(&input), true);
}

#[test]
fn test_good_password() {
    let input = vec![
        "111211".to_string().chars().collect(),
        "11111".to_string().chars().collect(),
        "111110".to_string().chars().collect(),
        "112221".to_string().chars().collect(),
        "123456".to_string().chars().collect(),
        "122234".to_string().chars().collect(),
    ];
    for password in &input {
        assert_eq!(good_password(password), false);
    }
}

#[test]
fn test_total_passwords() {
    assert_eq!(total_passwords(168_630, 718_098), 1145);
}
