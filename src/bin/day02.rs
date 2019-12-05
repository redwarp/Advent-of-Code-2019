use adventofcode::files;

fn main() {
    let data: Vec<u32> = files::read_file_as_csv("inputs/day02.txt")
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    let result = intcode(&data);

    println!("Value at zero is {}", result[0]);
}

fn intcode(source: &Vec<u32>) -> Vec<u32> {
    let mut data = source.to_vec();
    data[1] = 12;
    data[2] = 2;
    
    let mut cursor: usize = 0;
    while cursor < source.len() {
        match data[cursor] {
            99 => {
                cursor = data.len();
            }
            1 => {
                let write_data_cursor = data[cursor + 3] as usize;
                data[write_data_cursor] =
                    data[data[cursor + 1] as usize] + data[data[cursor + 2] as usize];
                cursor += 4;
            }
            2 => {
                let write_data_cursor = data[cursor + 3] as usize;
                data[write_data_cursor] =
                    data[data[cursor + 1] as usize] * data[data[cursor + 2] as usize];
                cursor += 4;
            }
            _ => {
                cursor += 1;
            }
        }
    }

    data
}
