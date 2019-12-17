use adventofcode::files;

fn intcode(source: &Vec<i32>, input: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    let mut data = source.to_vec();

    let mut cursor: usize = 0;
    while cursor < source.len() {
        let instruction = data[cursor];
        let opcode = instruction % 100;
        let first_param_mode = (instruction / 100) % 10;
        let second_param_mode = (instruction / 1000) % 10;
        let third_param_mode = (instruction / 10000) % 10;
        match opcode {
            99 => {
                cursor = data.len();
            }
            1 => {
                // Addition
                let write_data_cursor = data[cursor + 3] as usize;
                let first_data = if first_param_mode == 0 {
                    data[data[cursor + 1] as usize]
                } else {
                    data[cursor + 1]
                };
                let second_data = if second_param_mode == 0 {
                    data[data[cursor + 2] as usize]
                } else {
                    data[cursor + 2]
                };
                data[write_data_cursor] = first_data + second_data;
                cursor += 4;
            }
            2 => {
                // Multiplication
                let write_data_cursor = data[cursor + 3] as usize;
                let first_data = if first_param_mode == 0 {
                    data[data[cursor + 1] as usize]
                } else {
                    data[cursor + 1]
                };
                let second_data = if second_param_mode == 0 {
                    data[data[cursor + 2] as usize]
                } else {
                    data[cursor + 2]
                };
                data[write_data_cursor] = first_data * second_data;
                cursor += 4;
            }
            3 => {
                // Input
                let write_data_cursor = data[cursor + 1] as usize;
                data[write_data_cursor] = input;
                cursor += 2;
            }
            4 => {
                // Output
                let value = if first_param_mode == 0 {
                    data[data[cursor + 1] as usize]
                } else {
                    data[cursor + 1]
                };
                output.push(value);
                cursor += 2;
            }
            5 => {
                // Jump-if-true
                let value = if first_param_mode == 0 {
                    data[data[cursor + 1] as usize]
                } else {
                    data[cursor + 1]
                };
                let jump_position = if second_param_mode == 0 {
                    data[data[cursor + 2] as usize]
                } else {
                    data[cursor + 2]
                } as usize;

                cursor = if value != 0 {
                    jump_position
                } else {
                    cursor + 3
                }
            }
            6 => {
                // Jump-if-false
                let value = if first_param_mode == 0 {
                    data[data[cursor + 1] as usize]
                } else {
                    data[cursor + 1]
                };
                let jump_position = if second_param_mode == 0 {
                    data[data[cursor + 2] as usize]
                } else {
                    data[cursor + 2]
                } as usize;

                cursor = if value == 0 {
                    jump_position
                } else {
                    cursor + 3
                }
            }
            7 => {
                // Less than
                let write_data_cursor = data[cursor + 3] as usize;
                let first_data = if first_param_mode == 0 {
                    data[data[cursor + 1] as usize]
                } else {
                    data[cursor + 1]
                };
                let second_data = if second_param_mode == 0 {
                    data[data[cursor + 2] as usize]
                } else {
                    data[cursor + 2]
                };
                data[write_data_cursor] = if first_data < second_data { 1 } else { 0 };
                cursor += 4;
            }
            8 => {
                // Equals
                let write_data_cursor = data[cursor + 3] as usize;
                let first_data = if first_param_mode == 0 {
                    data[data[cursor + 1] as usize]
                } else {
                    data[cursor + 1]
                };
                let second_data = if second_param_mode == 0 {
                    data[data[cursor + 2] as usize]
                } else {
                    data[cursor + 2]
                };
                data[write_data_cursor] = if first_data == second_data { 1 } else { 0 };
                cursor += 4;
            }
            _ => {
                cursor += 1;
            }
        }
    }

    output
}

fn main() {
    let data: Vec<i32> = files::read_file_as_csv("inputs/day05.txt")
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
        
    let result_1 = intcode(&data, 1);
    println!("Result 1: {:?}", result_1);

    let result_2 = intcode(&data, 5);
    println!("Result 2: {:?}", result_2);
}
