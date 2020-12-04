fn part1(input: &str) -> u32 {
    let width = input.split('\n').next().unwrap().len();

    let mut result = 0;

    // The puzzle was RIGHT 3 DOWN 1, so we can hard code the directions here
    for (i, line) in input.split('\n').enumerate() {
        // Skip empty lines
        if line.len() == 0 {
            continue;
        }

        // Skip the first line since there won't ever be a selection in the first line
        if i == 0 {
            continue;
        }

        let index = (i * 3) % width;

        if line.chars().nth(index).unwrap() == '#' {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> u32 {
    let width = input.split('\n').next().unwrap().len();

    let mut result = 1;

    for (right_offset, down_num) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut curr_result = 0;

        // The puzzle was RIGHT 3 DOWN 1, so we can hard code the directions here
        for (i, line) in input.split('\n').enumerate() {
            // Skip empty lines
            if line.len() == 0 {
                continue;
            }

            // Skip the first line since there won't ever be a selection in the first line
            if i == 0 {
                continue;
            }

            // For the down 2 offset, we skip all odd rows
            if i % down_num != 0 {
                continue;
            }

            // Shortened formula for determining the index here.
            // For DOWN 1 rows, we always take into account every line, 
            //   so down_num has no effect
            // For DOWN 2 rows, we need to divide by 2 in order to get the correct index
            //   since it skips a row 
            let index = (i / down_num * right_offset) % width;

            if line.chars().nth(index).unwrap() == '#' {
                curr_result += 1;
            }
        }

        result *= curr_result;
    }

    result
}

fn main() {
    let input_test = include_str!("../input.test");
    assert!(part1(input_test) == 7);
    assert!(part2(input_test) == 336);

    let input_str = include_str!("../input");

    let part1 = part1(input_str);
    print!("Part1: {}\n", part1);
    let part2 = part2(input_str);
    print!("Part2: {}\n", part2);
}
