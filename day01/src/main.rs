use anyhow::Result;
use std::collections::HashSet;

const ITER: usize = 5;

/// Naive solution to part 1 using a HashSet as an input
fn naive_part1_hashset(input: &HashSet<u32>) -> u32 {
    let (a, b) = generic_part1_hashset(input, 2020).expect("Failed to find solution");

    // Found two numbers adding to 2020, return the product
    a * b
}

/// Naive solution to part 1 usina a Vec as an input
fn naive_part1_vec(input: &Vec<u32>) -> u32 {
    let (a, b) = generic_part1_vec(input, 2020).expect("Failed to find solution");

    // Found two numbers adding to 2020, return the product
    a * b
}

/// Naive solution to part 1
fn generic_part1_hashset(input: &HashSet<u32>, value_needed: u32) -> Option<(u32, u32)> {
    let mut res = None;
    for value in input {
        let needed_val = value_needed - value;
        if input.contains(&needed_val) {
            res = Some((*value, needed_val));
        }
    }

    res
}

/// Naive solution to part 1
fn generic_part1_vec(input: &Vec<u32>, value_needed: u32) -> Option<(u32, u32)> {
    let mut res = None;
    for value in input {
        let needed_val = value_needed - value;
        if input.contains(&needed_val) {
            res = Some((*value, needed_val));
        }
    }

    res
}

/// Naive solution to part 2 using a quadratic solution
fn part2_quad(input: &Vec<u32>) -> u32 {
    let mut seen = HashSet::new();

    let mut res = 0;

    for a in 0..input.len() {
        for b in 1..input.len() {
            for c in 2..input.len() {
                let mut sorted = [a, b, c];
                sorted.sort();
                let check = sorted[0] << 16 | sorted[1] << 8 | sorted[2];

                if seen.contains(&check) {
                    continue;
                }

                let first = input[a];
                let second = input[b];
                let third = input[c];

                seen.insert(a);
                seen.insert(b);
                seen.insert(c);

                if first + second + third == 2020 {
                    res = first * second * third;
                }
            }
        }
    }

    res
}

/// Solution to part 2 using a similar lookup method as part 1
fn part2_take2(input: &Vec<u32>) -> u32 {
    // Keep track of offsets that we've seen before as the associative property of add/mul
    // means offset 0, 1 have the same result as offsets 1, 0
    let mut seen = HashSet::new();

    // Return the solution
    let mut res = 0;

    // Iterate through two sets of offsets in the input. We only use two offsets since we can 
    // calculate what the third value will be by `2020 - input[a] - input[b]`
    for a in 0..input.len() {
        for b in 1..input.len() {
            // Calculate a naive "hash" of the two offsets to quickly check if we've seen these
            // two offsets before
            let mut sorted = [a, b];
            sorted.sort();
            let check = sorted[0] << 16 | sorted[1];

            // If we've seen these offsets before, no need to check again
            if seen.contains(&check) {
                continue;
            }

            // New offsets, add the check to the list
            seen.insert(check);

            // Grab the values at the offsets
            let first = input[a];
            let second = input[b];

            // Calculate the needed third value
            let third = 2020 - first - second;

            // If this third value is in the input, we know we have our answer!
            if input.contains(&third) {
                res = first * second * third;
            }
        }
    }

    res
}

/// Solution to part 2 using a similar lookup method as part 1
fn part2_using_part1_hashset(input: &HashSet<u32>) -> u32 {
    // Return the solution
    let mut res = 0;

    for first in input {
        if let Some((second, third)) = generic_part1_hashset(&input, 2020 - first) {
            res = first * second * third;
        }
    }

    res
}

/// Solution to part 2 using a similar lookup method as part 1
fn part2_using_part1_vec(input: &Vec<u32>) -> u32 {
    // Return the solution
    let mut res = 0;

    for first in input {
        if let Some((second, third)) = generic_part1_vec(&input, 2020 - first) {
            res = first * second * third;
        }
    }

    res
}

/// Dump the answers to the question itself
pub fn answer() -> Result<()> {
    // Compile the input string
    let input_str = include_str!("../input");

    // Part 2 requires a indexable Vec rather than just a HashSet
    let input_vec: Vec<u32> = input_str.split('\n').filter_map(|x| x.parse::<u32>().ok()).collect();

    // Solve part 1
    let part1 = naive_part1_vec(&input_vec);
    print!("Part 1: {}\n", part1);

    // Solve part 2
    let part2 = part2_take2(&input_vec);
    print!("Part 2: {}\n", part2);

    // Solve part 2
    let part2 = part2_using_part1_vec(&input_vec);
    print!("Part 2: {}\n", part2);

    Ok(())
}


macro_rules! benchmark {
    ($func:ident, $input:ident) => {
        let start = unsafe { std::arch::x86_64::_rdtsc() };

        for _ in 0..ITER {
            let _res = $func(&$input);
        }

        let end = unsafe { std::arch::x86_64::_rdtsc() };

        let cycles_per_iter = (end - start) as f64 / ITER as f64;
        print!("{:30}: {} cycles/iter ({} kCycles/iter)\n", stringify!($func), cycles_per_iter,
            cycles_per_iter / 1000.0);
    }
}
/// Benchmark various methods using a simple `rdtsc` check
pub fn bench() -> Result<()> {
    // Compile the input string
    let input_str = include_str!("../input");

    // Convert the input string into a Vec of u16
    let input_hashset: HashSet<u32> = input_str.split('\n').filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let input_vec: Vec<u32> = input_str.split('\n').filter_map(|x| x.parse::<u32>().ok()).collect();

    benchmark!(naive_part1_hashset, input_hashset);
    benchmark!(naive_part1_vec, input_vec);
    benchmark!(part2_using_part1_hashset, input_hashset);
    benchmark!(part2_using_part1_vec, input_vec);
    benchmark!(part2_take2, input_vec);
    benchmark!(part2_quad, input_vec);
    
    Ok(())
}

fn main() -> Result<()> {
    print!("Here are the answers\n");
    answer()?;

    print!("Here are the benchmarks\n");
    bench()?;

    Ok(())
}
