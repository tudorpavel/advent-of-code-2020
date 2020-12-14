# Advent of Code 2020 - Day 14

https://adventofcode.com/2020/day/14

## Part 1

I think we should be able to use bitwise operators AND and OR. In order to get the input mask to override 1s and 0s for each value, we need to generate 2 separate masks:
  - AND mask which is the input mask with Xs replaced with 1s
  - OR mask which is the input mask with Xs replaced with 0s
  
By apply these masks one by one with their respective bitwise operator to a value we get the desired result to be saved in our virtual memory.

```
# example input:
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11

#          1XXXX0X
and_mask = 1111101
or_mask  = 1000000

# first we bitwise AND the value with the and_mask
0001011 &
1111101 ==
0001001

# then we bitwise OR the partial result from above with the or_mask
0001001 |
1000000 ==
1001001

# and finally 1001001 is the value we need to save in memory
```

## Part 2

So the bitmask can be considered an `or_mask` except for the floating bits. We can try to brute-force it and actually compute all 2^N floating addresses and update our HashMap with the values. It might run in reasonable time and with reasonable memory usage, but if not we'll find a better solution.

Turns out my brute-force solution was fast enough, however the code is quite ugly because of all the duplication, bitwise operations and other binary numbers magic. Oh well...
