use adventofcode::files;

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
    let lines: Vec<i32> = files::lines_from_file("inputs/day01.txt")
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    let fuel: i32 = lines.iter().fold(0, |sum, &x| sum + fuel_for_mass(x));

    println!("Total fuel: {}", fuel);

    let fuel_with_extra: i32 = lines.iter().fold(0, |sum, &x| sum + fuel_for_mass_with_extra(x));

    println!("Total fuel with extra fuel: {}", fuel_with_extra);
}
