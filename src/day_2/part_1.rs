pub fn solution() {
    let input = std::fs::read_to_string("./src/day_2/input.txt").unwrap();
    let input: Vec<usize> = input
        .trim()
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect();
    println!("{}", program_result(input));
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

pub fn program_result(mut input: Vec<usize>) -> usize {
    input[1] = 12;
    input[2] = 2;
    opcode_output(0, input)[0]
}
