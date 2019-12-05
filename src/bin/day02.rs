use adventofcode::files;

fn main() {
    let data: Vec<u32> = files::read_file_as_csv("inputs/day02.txt")
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    let result = intcode(&data, 12, 2);

    println!("Value at zero is {}", result[0]);

    let target = 19690720;
    let mut noun: u32 = 0;
    let mut verb: u32 = 0;
    for x in 0..10000 {
        noun = x / 100;
        verb = x % 100;
        let result = intcode(&data, noun, verb);

        if result[0] == target {
            break;
        }
    }

    println!(
        "Target {} found for noon {} and verb {}",
        target, noun, verb
    );
}

fn intcode(source: &Vec<u32>, noun: u32, verb: u32) -> Vec<u32> {
    let mut data = source.to_vec();
    data[1] = noun;
    data[2] = verb;

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
