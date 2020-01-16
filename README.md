# Solutions to [Advent of Code 2019](https://adventofcode.com/2019)

All in rust, pretty efficient out of the box! Went back to optimize day 18 which was the only one running over 500ms 
```bash
$ hyperfine --warmup 5 target/release/day{01..25}

Benchmark #1: target/release/day01
  Time (mean ± σ):       6.0 ms ±   1.0 ms    [User: 2.7 ms, System: 1.8 ms]
  Range (min … max):     4.7 ms …  10.8 ms    260 runs
 
  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
 
Benchmark #2: target/release/day02
  Time (mean ± σ):       6.5 ms ±   0.8 ms    [User: 3.4 ms, System: 1.7 ms]
  Range (min … max):     5.5 ms …   9.6 ms    237 runs
 
Benchmark #3: target/release/day03
  Time (mean ± σ):      74.8 ms ±   1.5 ms    [User: 66.2 ms, System: 5.8 ms]
  Range (min … max):    72.2 ms …  79.2 ms    39 runs
 
Benchmark #4: target/release/day04
  Time (mean ± σ):     100.7 ms ±   1.3 ms    [User: 96.8 ms, System: 2.0 ms]
  Range (min … max):    99.1 ms … 103.5 ms    28 runs
 
Benchmark #5: target/release/day05
  Time (mean ± σ):       6.0 ms ±   1.3 ms    [User: 2.8 ms, System: 1.7 ms]
  Range (min … max):     4.7 ms …  19.2 ms    328 runs
 
  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
 
Benchmark #6: target/release/day06
  Time (mean ± σ):       8.6 ms ±   0.9 ms    [User: 5.1 ms, System: 1.9 ms]
  Range (min … max):     7.3 ms …  13.5 ms    229 runs
 
Benchmark #7: target/release/day07
  Time (mean ± σ):      59.4 ms ±   0.9 ms    [User: 55.3 ms, System: 2.2 ms]
  Range (min … max):    57.3 ms …  61.2 ms    46 runs
 
Benchmark #8: target/release/day08
  Time (mean ± σ):       5.8 ms ±   0.7 ms    [User: 2.7 ms, System: 1.7 ms]
  Range (min … max):     4.9 ms …   8.9 ms    289 runs
 
  Warning: Command took less than 5 ms to complete. Results might be inaccurate.
 
Benchmark #9: target/release/day09
  Time (mean ± σ):     128.4 ms ±   6.2 ms    [User: 124.2 ms, System: 2.2 ms]
  Range (min … max):   124.2 ms … 154.8 ms    23 runs

Benchmark #10: target/release/day10
  Time (mean ± σ):      37.0 ms ±   1.5 ms    [User: 33.1 ms, System: 2.1 ms]
  Range (min … max):    34.7 ms …  39.7 ms    69 runs
 
Benchmark #11: target/release/day11
  Time (mean ± σ):      46.4 ms ±   1.5 ms    [User: 42.0 ms, System: 2.4 ms]
  Range (min … max):    43.8 ms …  52.1 ms    58 runs
 
Benchmark #12: target/release/day12
  Time (mean ± σ):     108.6 ms ±   2.5 ms    [User: 104.6 ms, System: 2.0 ms]
  Range (min … max):   104.1 ms … 113.9 ms    25 runs
 
Benchmark #13: target/release/day13
  Time (mean ± σ):     285.5 ms ±   3.1 ms    [User: 280.8 ms, System: 2.4 ms]
  Range (min … max):   280.6 ms … 290.6 ms    10 runs
 
Benchmark #14: target/release/day14
  Time (mean ± σ):      24.6 ms ±   1.3 ms    [User: 21.0 ms, System: 2.0 ms]
  Range (min … max):    22.8 ms …  27.7 ms    101 runs
 
Benchmark #15: target/release/day15
  Time (mean ± σ):     178.4 ms ±   4.6 ms    [User: 134.9 ms, System: 40.6 ms]
  Range (min … max):   174.4 ms … 194.7 ms    16 runs

Benchmark #16: target/release/day16
  Time (mean ± σ):     238.1 ms ±   2.3 ms    [User: 231.9 ms, System: 3.7 ms]
  Range (min … max):   236.1 ms … 242.5 ms    12 runs
 
Benchmark #17: target/release/day17
  Time (mean ± σ):      73.6 ms ±   1.4 ms    [User: 68.9 ms, System: 2.6 ms]
  Range (min … max):    71.8 ms …  78.8 ms    39 runs
 
Benchmark #18: target/release/day18
  Time (mean ± σ):      66.3 ms ±   2.1 ms    [User: 59.7 ms, System: 4.2 ms]
  Range (min … max):    63.5 ms …  71.5 ms    44 runs
 
Benchmark #19: target/release/day19
  Time (mean ± σ):     448.5 ms ±   4.8 ms    [User: 443.2 ms, System: 2.9 ms]
  Range (min … max):   442.6 ms … 454.6 ms    10 runs
 
Benchmark #20: target/release/day20
  Time (mean ± σ):     417.4 ms ±   7.2 ms    [User: 394.6 ms, System: 18.9 ms]
  Range (min … max):   407.0 ms … 431.7 ms    10 runs
 
Benchmark #21: target/release/day21
  Time (mean ± σ):     223.0 ms ±   4.1 ms    [User: 218.1 ms, System: 2.4 ms]
  Range (min … max):   216.6 ms … 230.4 ms    13 runs
 
Benchmark #22: target/release/day22
  Time (mean ± σ):       5.8 ms ±   0.9 ms    [User: 2.7 ms, System: 1.7 ms]
  Range (min … max):     4.7 ms …   9.1 ms    289 runs
 
Benchmark #23: target/release/day23
  Time (mean ± σ):     263.5 ms ±   9.6 ms    [User: 255.7 ms, System: 5.3 ms]
  Range (min … max):   256.3 ms … 290.9 ms    11 runs
 
Benchmark #24: target/release/day24
  Time (mean ± σ):     231.2 ms ±   2.6 ms    [User: 226.5 ms, System: 2.6 ms]
  Range (min … max):   227.6 ms … 236.5 ms    12 runs
 
Benchmark #25: target/release/day25
  Time (mean ± σ):      33.6 ms ±   1.7 ms    [User: 29.7 ms, System: 2.3 ms]
  Range (min … max):    31.6 ms …  39.6 ms    77 runs
  ```