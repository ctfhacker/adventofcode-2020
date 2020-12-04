```
(cmd)$ cargo run --release
   Compiling day01 v0.1.0 (/mnt/c/Users/rando/workspace/adventofcode-2020/day01)
    Finished release [optimized] target(s) in 0.99s
     Running `target/release/day01`
Here are the answers
Part 1: 1018944
Part 2: 8446464
Part 2: 8446464
Here are the benchmarks
naive_part1_fnvhashset        :       4809 cycles/iter (    4.81   kCycles/iter)
naive_part1_vec               :      66084 cycles/iter (   66.08   kCycles/iter)
naive_part1_hashset           :      10265 cycles/iter (   10.27   kCycles/iter)
part2_using_part1_fnvhashset  :    1476465 cycles/iter ( 1476.47   kCycles/iter)
part2_using_part1_hashset     :    2120158 cycles/iter ( 2120.16   kCycles/iter)
part2_using_part1_vec         :   11370083 cycles/iter (11370.08   kCycles/iter)
part2_take2                   :   11576060 cycles/iter (11576.06   kCycles/iter)
part2_quad                    : 1799902006 cycles/iter (1799902.01 kCycles/iter)
```
