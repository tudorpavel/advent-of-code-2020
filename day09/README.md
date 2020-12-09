# Advent of Code 2020 - Day 9

https://adventofcode.com/2020/day/9

## Part 1

This might be a good opportunity to use [std::slice::Windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows) to interate over a 26-length slice. Inside each slice we can brute force check all possible pairs from first 25 elements and compare their sum to the 26th element. I'm not sure if there's a more performant solution than brute-forcing it.

Also of note is that we need to use 64-bit unsigned integer values from what I can see in my input.
