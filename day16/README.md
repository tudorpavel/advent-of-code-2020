# Advent of Code 2020 - Day 16

https://adventofcode.com/2020/day/16

## Part 1

I'm sure the field names will be needed for Part 2, but I will create the minimum necessary data structures just for Part 1. We need to have a collection of all the Ranges from all the rules and a collection of all numbers from all the nearby tickets and then filter the list of numbers based on wether they are part of any of the ranges or not.

It should be possible to express all of this using functional pipelines, let's see how it goes.
