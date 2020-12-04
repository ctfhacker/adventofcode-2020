fn part1_split(input: &str) -> usize {
    let mut result = 0;
    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }

        // Each password has the following form:
        // min-max letter: password

        // Start the split based on spaces
        let mut split = line.split(' ');

        // The range is the first element delimited by a `-`
        let mut range = split.next().unwrap().split('-');

        // Minimum is first
        let min = range.next().unwrap().parse::<u32>().unwrap();

        // Max is second, but it is an inclusive range, so we need to add 1 to ensure the 
        // check is properly handled
        let max = range.next().unwrap()
                       .parse::<u32>().unwrap()
                       .checked_add(1).unwrap();

        // Next is the letter to check
        let checked_letter = split.next().unwrap().chars().nth(0).unwrap();

        // Finally the password
        let password = split.next().unwrap();

        // Next count the number of requested letters in the password
        let mut counter = 0;
        for curr_letter in password.chars() {
            if curr_letter == checked_letter { 
                counter += 1;
            } 
        }

        if (min..max).contains(&counter) {
            result += 1;
        } 
    }

    result
}

fn part2_split(input: &str) -> usize {
    let mut result = 0;

    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }

        // Each password has the following form:
        // first_index-second_index letter: password

        // Start the split based on spaces
        let mut split = line.split(' ');

        // The range is the first element delimited by a `-`
        let mut range = split.next().unwrap().split('-');

        // The elements for this test are 1-indexed and not 0-indexed. Sub one to satisfy
        let first = range.next().unwrap()
                         .parse::<usize>().unwrap()
                         .checked_sub(1).unwrap();

        // The elements for this test are 1-indexed and not 0-indexed. Sub one to satisfy
        let second = range.next().unwrap()
                          .parse::<usize>().unwrap()
                          .checked_sub(1).unwrap();

        // Next is the letter to check
        let checked_letter = split.next().unwrap().chars().nth(0).unwrap();

        // Finally the password
        let password = split.next().unwrap();

        let found_first  = password.chars().nth(first) == Some(checked_letter);
        let found_second = password.chars().nth(second) == Some(checked_letter);

        if found_first ^ found_second {
            result += 1;
        }
    }

    result
}

fn main() {
    let input_str = include_str!("../input");

    let part1 = part1_split(&input_str);
    print!("Part1: {}\n", part1);

    let part2 = part2_split(&input_str);
    print!("Part2: {}\n", part2);
}
