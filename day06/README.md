# Advent of Code 2020 - Day 6

https://adventofcode.com/2020/day/6

## Part 1

I'm going to attempt to create Sets of chars to count unique answers in each group, all in a single functional pipeline.

## Part 2

Ah, I should've seen this coming. Basically I think we need to construct bitstrings for each person's answers and use bitwise logical operators OR for Part 1, AND for Part 2.

```
abc will be:

abcdefghijklmnopqrstuvwxyz
11100000000000000000000000
```

And for counting 1s we can also use bitwise shift like [explained here](https://www.techiedelight.com/brian-kernighans-algorithm-count-set-bits-integer/).

I think it will feel a bit uncomfortable, but this might be fun to implement. Let's see how it goes.
