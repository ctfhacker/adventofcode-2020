fn part1(input: &str) -> u32 {
    let mut result = 0;

    for passport in input.split("\n\n") {
        let mut valid = true;
        for check in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
            if !passport.contains(check) {
                valid = false;
                break;
            }
        }

        if valid {
            result += 1;
        }
    }

    result
}

/*
struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<Height>,
    hair: String,
    eye: String,
    passport_id: String,
}
*/

fn part2(input: &str) -> u32 {
    let mut result = 0;

    for passport in input.split("\n\n") {
        let mut valid = true;
        let passport = passport.replace("\n", " ");

        // print!("Passport {}\n", passport);
        for check in &["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"] {
            if !passport.contains(check) {
                valid = false;
                break;
            }
        }

        if !valid {
            // print!("INVALID check 1\n");
            continue;
        }

        for field in passport.split(' ') {
            // If this is already not valid, no need to continue parsing
            if !valid {
                break;
            }

            let mut field = field.split(":");
            let category = field.next().unwrap();
            let data     = field.next().unwrap();
            match category {
                "byr" => {
                    if data.len() != 4 {
                        valid = false;
                    }

                    if !(1920..=2002).contains(&data.parse::<u32>().unwrap()) {
                        valid = false;
                    }
                }
                "iyr" => {
                    if data.len() != 4 {
                        valid = false;
                    }

                    if !(2010..=2020).contains(&data.parse::<u32>().unwrap()) {
                        valid = false;
                    }
                }
                "eyr" => {
                    if data.len() != 4 {
                        valid = false;
                    }

                    if !(2020..=2030).contains(&data.parse::<u32>().unwrap()) {
                        valid = false;
                    }
                }
                "hgt" => {
                    if !data.contains(&"in") && !data.contains(&"cm") {
                        valid = false;
                    }

                    for (measurement, range) in &[("in", (59..=76)), 
                                                  ("cm", (150..=193))] {
                        if data.contains(measurement) {
                            let num = data.split(measurement).next().unwrap()
                                          .parse::<u32>().unwrap();

                            if !range.contains(&num) {
                                valid = false;
                            }
                        } 
                    }
                }
                "hcl" => {
                    // Check for the correct number of characters
                    if data.len() != 7 {
                        valid = false;
                    }

                    // Ensure the next 6 characters are hex digits
                    let hexdigits = [
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
                        'a', 'b', 'c', 'd', 'e', 'f', 
                        'A', 'B', 'C', 'D', 'E', 'F',
                    ];

                    for (i, curr_char) in data.char_indices() {
                        if i == 0 && curr_char != '#' {
                            valid = false;
                        }

                        if i > 0 && !hexdigits.contains(&curr_char) {
                            valid = false;
                        }
                    }
                }
                "ecl" => {
                    let colors = [
                        "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
                    ];

                    if !colors.contains(&data) {
                        valid = false;
                    }
                }
                "pid" => {
                    if data.len() != 9 {
                        valid = false;
                    }

                    let digits = [
                        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 
                    ];

                    for curr_num in data.chars() {
                        if !digits.contains(&curr_num) {
                            valid = false;
                            break;
                        }
                    }
                }
                "cid" => {
                    /* Optional, no need to handle it */
                }
                _ => panic!("unknown: {}\n", category)
            }

            if !valid {
                // print!("INVALID: {} {}\n", category, data);
                break;
            }
        }

        if valid {
            result += 1;
        }
    }

    result
}

fn main() {
    let input_test = include_str!("../input.test");
    assert!(part1(input_test) == 10);
    assert!(part2(input_test) == 6);

    let input_str = include_str!("../input");
    let part1 = part1(input_str);
    print!("Part1: {}\n", part1);
    let part2 = part2(input_str);
    print!("Part2: {}\n", part2);
}
