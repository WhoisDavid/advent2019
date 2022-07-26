# Solutions to [Advent of Code 2019](https://adventofcode.com/2019)

All in rust, pretty efficient out of the box! Went back to optimize day 18 which was the only one running over 500ms 
```bash
$ hyperfine --warmup 5 target/release/day{01..25}

Benchmark #1: target/release/day01
  Time (mean ± σ):       1.8 ms ±   0.2 ms    [User: 1.2 ms, System: 0.5 ms]
  Range (min … max):     1.5 ms …   3.5 ms    690 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark #2: target/release/day02
  Time (mean ± σ):       2.5 ms ±   0.3 ms    [User: 1.9 ms, System: 0.5 ms]
  Range (min … max):     2.2 ms …   5.8 ms    598 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark #3: target/release/day03
  Time (mean ± σ):      65.6 ms ±   1.4 ms    [User: 61.8 ms, System: 3.2 ms]
  Range (min … max):    63.1 ms …  68.7 ms    43 runs

Benchmark #4: target/release/day04
  Time (mean ± σ):      54.3 ms ±   0.8 ms    [User: 53.3 ms, System: 0.7 ms]
  Range (min … max):    53.2 ms …  56.2 ms    51 runs

Benchmark #5: target/release/day05
  Time (mean ± σ):       1.8 ms ±   0.1 ms    [User: 1.3 ms, System: 0.5 ms]
  Range (min … max):     1.6 ms …   2.6 ms    677 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark #6: target/release/day06
  Time (mean ± σ):       2.6 ms ±   0.2 ms    [User: 1.9 ms, System: 0.6 ms]
  Range (min … max):     2.4 ms …   4.8 ms    580 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark #7: target/release/day07
  Time (mean ± σ):      33.9 ms ±   0.2 ms    [User: 33.0 ms, System: 0.7 ms]
  Range (min … max):    33.7 ms …  34.3 ms    82 runs

Benchmark #8: target/release/day08
  Time (mean ± σ):       1.8 ms ±   0.1 ms    [User: 1.2 ms, System: 0.5 ms]
  Range (min … max):     1.5 ms …   2.8 ms    688 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark #9: target/release/day09
  Time (mean ± σ):      49.2 ms ±   0.5 ms    [User: 48.3 ms, System: 0.7 ms]
  Range (min … max):    47.8 ms …  50.9 ms    58 runs

Benchmark #10: target/release/day10
  Time (mean ± σ):      22.6 ms ±   0.2 ms    [User: 21.8 ms, System: 0.6 ms]
  Range (min … max):    22.2 ms …  23.5 ms    120 runs

Benchmark #11: target/release/day11
  Time (mean ± σ):      18.9 ms ±   0.2 ms    [User: 18.1 ms, System: 0.6 ms]
  Range (min … max):    18.5 ms …  19.7 ms    136 runs

Benchmark #12: target/release/day12
  Time (mean ± σ):      36.3 ms ±   0.2 ms    [User: 35.4 ms, System: 0.7 ms]
  Range (min … max):    35.5 ms …  36.7 ms    77 runs

Benchmark #13: target/release/day13
  Time (mean ± σ):     125.2 ms ±   1.2 ms    [User: 123.8 ms, System: 1.0 ms]
  Range (min … max):   122.2 ms … 128.0 ms    23 runs

Benchmark #14: target/release/day14
  Time (mean ± σ):      10.5 ms ±   0.2 ms    [User: 9.8 ms, System: 0.6 ms]
  Range (min … max):    10.2 ms …  11.3 ms    229 runs

Benchmark #15: target/release/day15
  Time (mean ± σ):      71.8 ms ±   1.2 ms    [User: 57.0 ms, System: 14.0 ms]
  Range (min … max):    70.4 ms …  78.2 ms    41 runs


Benchmark #16: target/release/day16
  Time (mean ± σ):     267.4 ms ±   0.4 ms    [User: 264.8 ms, System: 1.8 ms]
  Range (min … max):   266.9 ms … 267.9 ms    11 runs

Benchmark #17: target/release/day17
  Time (mean ± σ):      30.6 ms ±   0.2 ms    [User: 29.7 ms, System: 0.7 ms]
  Range (min … max):    29.8 ms …  31.1 ms    91 runs

Benchmark #18: target/release/day18
  Time (mean ± σ):      29.4 ms ±   0.2 ms    [User: 28.0 ms, System: 1.1 ms]
  Range (min … max):    29.1 ms …  30.0 ms    93 runs

Benchmark #19: target/release/day19
  Time (mean ± σ):     244.9 ms ±   0.5 ms    [User: 242.9 ms, System: 1.4 ms]
  Range (min … max):   244.0 ms … 245.7 ms    12 runs

Benchmark #20: target/release/day20
  Time (mean ± σ):     152.7 ms ±  12.7 ms    [User: 146.2 ms, System: 5.7 ms]
  Range (min … max):   134.5 ms … 174.6 ms    16 runs

Benchmark #21: target/release/day21
  Time (mean ± σ):      99.1 ms ±   1.1 ms    [User: 97.6 ms, System: 1.0 ms]
  Range (min … max):    97.0 ms … 100.7 ms    29 runs

Benchmark #22: target/release/day22
  Time (mean ± σ):       1.8 ms ±   0.2 ms    [User: 1.2 ms, System: 0.5 ms]
  Range (min … max):     1.5 ms …   3.8 ms    661 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark #23: target/release/day23
  Time (mean ± σ):     114.4 ms ±   0.9 ms    [User: 112.4 ms, System: 1.5 ms]
  Range (min … max):   112.6 ms … 115.5 ms    25 runs

Benchmark #24: target/release/day24
  Time (mean ± σ):     153.8 ms ±   0.9 ms    [User: 152.1 ms, System: 1.2 ms]
  Range (min … max):   150.7 ms … 155.4 ms    19 runs

Benchmark #25: target/release/day25
  Time (mean ± σ):      13.1 ms ±   0.5 ms    [User: 12.3 ms, System: 0.6 ms]
  Range (min … max):    12.7 ms …  16.7 ms    192 runs
```
