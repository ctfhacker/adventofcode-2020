```
$ cargo run --release
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/day01`
Here are the answers
Part 1: 1018944
Part 2: 8446464
Part 2: 8446464
Here are the benchmarks
naive_part1_hashset           : 9865.2 cycles/iter (9.865200000000002 kCycles/iter)
naive_part1_vec               : 50665.2 cycles/iter (50.6652 kCycles/iter)
part2_using_part1_hashset     : 2340737.6 cycles/iter (2340.7376 kCycles/iter)
part2_using_part1_vec         : 8469.6 cycles/iter (8.4696 kCycles/iter)
part2_take2                   : 9139395.2 cycles/iter (9139.395199999999 kCycles/iter)
part2_quad                    : 1500122582.8 cycles/iter (1500122.5828 kCycles/iter)
```
