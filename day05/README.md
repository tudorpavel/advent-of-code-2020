# Advent of Code 2020 - Day 5

https://adventofcode.com/2020/day/5

## Part 1

I'm really tempted to build two binary trees to traverse the given paths, but I think there should be an easier solution since the entire range (0..128) is covered.

```
Trace RLR

0 1 2 3 4 5 6 7
4 5 6 7
4 5
5
```

After some quick research, I think we could use [split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at) to iteratively split the ranges (0..128) and (0..8) in half and choose left/right at each step.
