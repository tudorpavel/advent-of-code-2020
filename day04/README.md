# Advent of Code 2020 - Day 4

https://adventofcode.com/2020/day/4

## Part 1

The tricky part is to sanitize the input to join multi-line passports so we get a collection of strings which represent the entire key-value passport. After that it's just a matter of making sure all required key substrings are contained in a passport to validate it.

It was pretty fun to implement using a functional style.

## Part 2

Okay, so Part 2 required a bunch of regex gymnastics. I created a Passport type with methods that check validity of each field. I'm not quite happy with how the code turned out, but it gets the job done.

What I could probably have done better is use regexes only to initialize a more detailed/concrete Passport struct, instead of checking the entire passport string in each method.
