fn main() {
    println!(
        "Number increases for <{}>?: {}",
        111111,
        meets_criteria(111111)
    );
    println!(
        "Number increases for <{}>?: {}",
        223450,
        meets_criteria(223450)
    );
    println!(
        "Number increases for <{}>?: {}",
        123789,
        meets_criteria(123789)
    );

    let result = (158126..624574)
        .map(|x| meets_criteria(x))
        .fold(0, |sum, matches| if matches { sum + 1 } else { sum });

    println!(
        "Numbers matching criteria in range 158126..624574: {}",
        result
    );

    let result = (158126..624574)
        .map(|x| meets_criteria_two(x))
        .fold(0, |sum, matches| if matches { sum + 1 } else { sum });

    println!(
        "Numbers matching criteria for part two in range 158126..624574: {}",
        result
    );
}

fn meets_criteria(number: u32) -> bool {
    number_increases(number) && number_has_double(number)
}
fn meets_criteria_two(number: u32) -> bool {
    number_increases(number) && number_has_double_and_no_more(number)
}

fn number_increases(number: u32) -> bool {
    let number_as_chars: Vec<char> = format!("{:0>5}", number).chars().collect();

    for i in 0..5 {
        if number_as_chars[i].to_digit(10) > number_as_chars[i + 1].to_digit(10) {
            return false;
        }
    }

    true
}

fn number_has_double(number: u32) -> bool {
    let number_as_chars: Vec<char> = format!("{:0>6}", number).chars().collect();

    for i in 0..5 {
        if number_as_chars[i].to_digit(10) == number_as_chars[i + 1].to_digit(10) {
            return true;
        }
    }

    false
}

fn number_has_double_and_no_more(number: u32) -> bool {
    let number_as_chars: Vec<char> = format!("{:0>6}", number).chars().collect();

    let mut offset = 0;
    while offset < 5 {
        let digit = number_as_chars[offset].to_digit(10);
        let mut identical_count = 1;
        let mut next_offset = offset + 1;
        while next_offset < 6 {
            if number_as_chars[next_offset].to_digit(10) == digit {
                identical_count += 1;
                next_offset += 1;
            } else {
                next_offset = std::usize::MAX;
            }
        }
        offset += identical_count;

        if identical_count == 2 {
            return true;
        }
    }

    false
}
