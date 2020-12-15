# Advent of Code 2020 - Day 15

https://adventofcode.com/2020/day/15

## Part 1

In order to be memory efficient, we should use a HashMap of which number was last spoken at which turn. We load it up with the input numbers and begin looping turns starting from input length.

## Part 2

Let's see if the brute-force solution from Part 1 works in reasonable time for this much larger N... Well, if you consider 2 seconds when compiled with `--release` flag reasonable, then I guess it worked. :)

Still, could we find a pattern that helps us figure out a much more efficient algorithm?

For example, the input `3,2,1` is interesting:
  - the 2020th number spoken is 438
  - the 30000000th number spoken is 18
