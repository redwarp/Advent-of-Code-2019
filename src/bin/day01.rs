use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn fuel_for_mass(mass: i32) -> i32 {
    (mass - (mass % 3)) / 3 - 2
}

fn fuel_for_mass_with_extra(mass: i32) -> i32 {
    let mut fuel = fuel_for_mass(mass);
    let mut reminder = fuel;

    while reminder > 0 {
        reminder = fuel_for_mass(reminder);
        if reminder > 0 {
            fuel = fuel + reminder
        }
    }

    fuel
}

fn main() {
    println!("Mass: {}", fuel_for_mass(100756));
    let lines = lines_from_file("inputs/day01.txt");
    let fuel: i32 = lines
        .iter()
        .map(|x| fuel_for_mass(x.parse().unwrap()))
        .sum();

    println!("Total fuel: {}", fuel);

    let fuel_with_extra: i32 = lines
        .iter()
        .map(|x| fuel_for_mass_with_extra(x.parse().unwrap()))
        .sum();

    println!("Total fuel with extra fuel: {}", fuel_with_extra);
}
