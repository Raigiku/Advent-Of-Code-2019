pub fn solution() {
    let file_input = std::fs::read_to_string("./src/day_2/input.txt").unwrap();
    let input: Vec<usize> = file_input
        .trim()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect();
    println!("{:?}", noun_verb_result(input, 19690720));
}

pub fn opcode_output(instruction_pointer: usize, mut input: Vec<usize>) -> Vec<usize> {
    let opcode = input[instruction_pointer];
    if opcode == 1 || opcode == 2 {
        let position_input_1 = input[instruction_pointer + 1];
        let position_input_2 = input[instruction_pointer + 2];
        let position_output = input[instruction_pointer + 3];
        let output = if opcode == 1 {
            input[position_input_1] + input[position_input_2]
        } else {
            input[position_input_1] * input[position_input_2]
        };
        input[position_output] = output;
        opcode_output(instruction_pointer + 4, input)
    } else {
        input
    }
}

pub fn noun_verb_result(input: Vec<usize>, expected_result: usize) -> Option<usize> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_copy = input.clone();
            input_copy[1] = noun;
            input_copy[2] = verb;
            let input_copy = opcode_output(0, input_copy);
            if input_copy[0] == expected_result {
                return Some(100 * input_copy[1] + input_copy[2]);
            }
        }
    }
    None
}
