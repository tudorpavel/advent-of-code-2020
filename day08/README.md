# Advent of Code 2020 - Day 8

https://adventofcode.com/2020/day/8

## Part 1

Should be straightforward to execute the instructions in a while loop and break when visiting the same instruction twice. Seems like a good opportunity to use [Pattern Syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html) for switching based on operation name.

## Part 2

Okay so we need to extract the existing while loop in a procedure which can report back success/failure and the last value of the `accumulator`. Other than that we need to generate all possible combinations of instructions obtained by switching exactly one `nop` to a `jmp` or vice-versa.
