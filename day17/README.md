# Advent of Code 2020 - Day 17

https://adventofcode.com/2020/day/17

## Part 1

We can use a 50x50x50 cube defined as 3 nested vectors of booleans and initialize a slice somewhere in the middle. If at any point we go out of bounds, we need a bigger cube. Once in place, we iterate with 3 nested loops all cells inside the cube and determine if each cell lives, dies or is born again.

## Part 2

Okay then, we can duplicate most of the logic and use 4 dimensions. Introducing separate modules for part1 and part2 can help separate functions better. Also, after some trial and error, it seems we can tighten the cube size to 27 to keep things fast.

Runtime with a release build is about 480ms for both parts.
