use crate::day_4::part_1::{
    good_password, has_decrease, has_pair_same, is_equal_length, total_passwords,
};

#[test]
fn test_is_equal_length() {
    let input = "154897".chars().collect();
    assert_eq!(is_equal_length(&input, 6), true);
    assert_eq!(is_equal_length(&input, 3), false);
}

#[test]
fn test_has_pair_same() {
    let input = "26".chars().collect();
    assert_eq!(has_pair_same(&input), false);
    let input = "66".chars().collect();
    assert_eq!(has_pair_same(&input), true);
}

#[test]
fn test_has_decrease() {
    let input = "22".chars().collect();
    assert_eq!(has_decrease(&input), false);
    let input = "12".chars().collect();
    assert_eq!(has_decrease(&input), false);
    let input = "21".chars().collect();
    assert_eq!(has_decrease(&input), true);
}

#[test]
fn test_password_good() {
    let input = vec![
        "111211".chars().collect(),
        "11111".chars().collect(),
        "111110".chars().collect(),
        "112221".chars().collect(),
        "123456".chars().collect(),
    ];
    for password in &input {
        assert_eq!(good_password(password), false);
    }
}

#[test]
fn test_total_passwords() {
    assert_eq!(total_passwords(168_630, 718_098), 1686);
}
