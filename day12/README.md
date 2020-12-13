# Advent of Code 2020 - Day 12

https://adventofcode.com/2020/day/12

## Part 1

For this puzzle we could make use of a Ship object which holds the current position coordinates and the current facing direction of the ship and a method which can receive and interpret instructions to mutate the Ship's state.

## Part 2

We will extend the Ship struct to keep track of waypoint position and implement a separate method to compute next step with these new rules. The trickiest part would be to rotate the waypoint relative to the ship, but it shouldn't be too hard.

I was a bit lazy when implementing waypoint rotations and only did the 90 degree left or right implementations and delegated to them for the 180 and 270 degree rotations.
