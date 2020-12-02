# Advent of Code 2020 - Day 2

https://adventofcode.com/2020/day/2

## Part 1

The idea would be to count occurences of the given char in the given password and compare with the given range. For parsing each line of the input we could use a regex.

## Part 2

Should be simple enough, we have to only check 2 chars at the given positions in the password. We can also refactor the code a bit to extract input parsing into a function.
