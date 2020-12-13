# Advent of Code 2020 - Day 13

https://adventofcode.com/2020/day/13

## Part 1

Seems like we have to get the remainder of dividing the given earliest departure timestamp with each of the bus IDs and use it to get the minimum difference from our departure timestamp to the next bus departure.

## Part 2

The following brute-force solution ran for longer than 1 minute on my input, so I guess we should figure out some math solution.

```rust
let mut current_timestamp: u64 = 0;

loop {
    let is_solution = primes_with_offset
        .iter()
        .all(|(prime, offset)| (current_timestamp + *offset as u64) % prime == 0);

    if is_solution {
        return current_timestamp;
    }

    current_timestamp += primes_with_offset[0].0;
}
```

If we needed to find a timestamp when all buses leave at the same time, we could use [Least common multiple](https://en.wikipedia.org/wiki/Least_common_multiple) which in our case is the product of all bus IDs because they are all prime numbers. But figuring in the offsets doesn't seem trivial...

We might be able to adapt this [LCM simple algorithm](https://en.wikipedia.org/wiki/Least_common_multiple#Using_a_simple_algorithm) to our needs and find the solution more quickly than a brute force search. Update: Still way too slow.

## Part 2 - one day later

Okay, so I woke up the second day with the solution stuck in my head already. We can still use Least common multiple (LCM) to figure out how often a pattern repeats itself. For example with `3,5` first solution is 9 and then it repeats itself every 15 steps because LCM of 3 and 5 is 15:

```
T  | 3 | 5
0  | D | D
...
8  | - | -
9  | D | -
10 | - | D
11 | - | -
...
24 | D | -
25 | - | D
...
39 | D | -
40 | - | D
```

We can apply this for each prime in our input and start looking starting from previous solution with increment of LCM of previous primes. The solution at each step is based on the previous solution, so we can narrow down the search space for each prime by a lot.
