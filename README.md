# Solutions to [Advent of Code 2019](https://adventofcode.com/2019)

All in rust, pretty efficient out of the box! Went back to optimize day 18 which was the only one running over 500ms 
```bash
$ hyperfine --warmup 5 target/release/day{01..25}

Benchmark #1: target/release/day01
  Time (mean ± σ):       1.9 ms ±   0.3 ms    [User: 1.2 ms, System: 0.5 ms]
  Range (min … max):     1.6 ms …   4.5 ms    435 runs
 
Benchmark #2: target/release/day02
  Time (mean ± σ):       2.5 ms ±   0.4 ms    [User: 1.9 ms, System: 0.5 ms]
  Range (min … max):     2.2 ms …   6.2 ms    496 runs
 
Benchmark #3: target/release/day03
  Time (mean ± σ):      66.3 ms ±   1.2 ms    [User: 61.8 ms, System: 3.7 ms]
  Range (min … max):    64.5 ms …  68.9 ms    42 runs
 
Benchmark #4: target/release/day04
  Time (mean ± σ):      51.8 ms ±   0.7 ms    [User: 50.6 ms, System: 0.8 ms]
  Range (min … max):    51.1 ms …  55.5 ms    54 runs
 
Benchmark #5: target/release/day05
  Time (mean ± σ):       1.9 ms ±   0.3 ms    [User: 1.3 ms, System: 0.5 ms]
  Range (min … max):     1.6 ms …   4.1 ms    511 runs

Benchmark #6: target/release/day06
  Time (mean ± σ):       3.0 ms ±   0.4 ms    [User: 2.3 ms, System: 0.5 ms]
  Range (min … max):     2.7 ms …   5.3 ms    502 runs

Benchmark #7: target/release/day07
  Time (mean ± σ):      35.6 ms ±   0.4 ms    [User: 34.2 ms, System: 0.9 ms]
  Range (min … max):    35.0 ms …  36.8 ms    76 runs
 
Benchmark #8: target/release/day08
  Time (mean ± σ):       1.9 ms ±   0.3 ms    [User: 1.2 ms, System: 0.5 ms]
  Range (min … max):     1.5 ms …   4.2 ms    627 runs

Benchmark #9: target/release/day09
  Time (mean ± σ):      59.4 ms ±   0.7 ms    [User: 58.1 ms, System: 0.9 ms]
  Range (min … max):    58.4 ms …  61.5 ms    46 runs
 
Benchmark #10: target/release/day10
  Time (mean ± σ):      23.3 ms ±   0.7 ms    [User: 22.1 ms, System: 0.8 ms]
  Range (min … max):    22.5 ms …  27.3 ms    113 runs
 
Benchmark #11: target/release/day11
  Time (mean ± σ):      23.3 ms ±   1.1 ms    [User: 21.5 ms, System: 1.0 ms]
  Range (min … max):    21.8 ms …  30.7 ms    112 runs
 
Benchmark #12: target/release/day12
  Time (mean ± σ):      53.5 ms ±   1.2 ms    [User: 51.8 ms, System: 1.2 ms]
  Range (min … max):    51.5 ms …  56.0 ms    55 runs
 
Benchmark #13: target/release/day13
  Time (mean ± σ):     141.7 ms ±   1.1 ms    [User: 139.9 ms, System: 1.1 ms]
  Range (min … max):   139.3 ms … 143.7 ms    20 runs
 
Benchmark #14: target/release/day14
  Time (mean ± σ):      12.5 ms ±   0.5 ms    [User: 11.5 ms, System: 0.7 ms]
  Range (min … max):    11.8 ms …  15.1 ms    200 runs
 
Benchmark #15: target/release/day15
  Time (mean ± σ):      81.8 ms ±   3.3 ms    [User: 64.0 ms, System: 16.1 ms]
  Range (min … max):    78.4 ms …  95.3 ms    30 runs
 
Benchmark #16: target/release/day16
  Time (mean ± σ):     272.9 ms ±   2.9 ms    [User: 267.4 ms, System: 3.3 ms]
  Range (min … max):   268.9 ms … 276.3 ms    10 runs
 
Benchmark #17: target/release/day17
  Time (mean ± σ):      35.2 ms ±   1.1 ms    [User: 33.5 ms, System: 1.0 ms]
  Range (min … max):    33.9 ms …  39.0 ms    77 runs
 
Benchmark #18: target/release/day18
  Time (mean ± σ):      32.9 ms ±   0.8 ms    [User: 31.2 ms, System: 1.3 ms]
  Range (min … max):    31.7 ms …  36.1 ms    81 runs
 
Benchmark #19: target/release/day19
  Time (mean ± σ):     274.3 ms ±   1.3 ms    [User: 271.1 ms, System: 2.2 ms]
  Range (min … max):   272.3 ms … 276.1 ms    10 runs
 
Benchmark #20: target/release/day20
  Time (mean ± σ):     163.5 ms ±  17.0 ms    [User: 154.8 ms, System: 7.3 ms]
  Range (min … max):   138.0 ms … 199.9 ms    17 runs
 
Benchmark #21: target/release/day21
  Time (mean ± σ):     118.4 ms ±   1.8 ms    [User: 117.3 ms, System: 0.8 ms]
  Range (min … max):   115.0 ms … 120.6 ms    24 runs
 
Benchmark #22: target/release/day22
  Time (mean ± σ):       1.7 ms ±   0.2 ms    [User: 1.2 ms, System: 0.4 ms]
  Range (min … max):     1.5 ms …   4.1 ms    458 runs

Benchmark #23: target/release/day23
  Time (mean ± σ):     127.1 ms ±   0.4 ms    [User: 125.4 ms, System: 1.3 ms]
  Range (min … max):   126.4 ms … 128.1 ms    23 runs
 
Benchmark #24: target/release/day24
  Time (mean ± σ):     153.7 ms ±   1.4 ms    [User: 152.1 ms, System: 1.1 ms]
  Range (min … max):   151.4 ms … 156.7 ms    19 runs
 
Benchmark #25: target/release/day25
  Time (mean ± σ):      15.4 ms ±   0.6 ms    [User: 14.4 ms, System: 0.6 ms]
  Range (min … max):    14.8 ms …  20.0 ms    163 runs
```
