# Advent of Code 2020 - Day 1

https://adventofcode.com/2020/day/1

I figured for Part 1 I could try something fancy and sort the list of numbers and then gradually move 2 pointers until converging on the solution.

Of course, for Part 2 I couldn't really adapt that solution to using 3 pointers. So I gave up and imported a Rust crate to generate all combinations of `k`-length.

I had to really struggle to understand what the type system needed from me in terms of type annotations until I realized all that was missing is `sum::<i32>()` instead of `sum()`. I need to read more about the Rust type system.
