# Advent of Code 2020 - Day 10

https://adventofcode.com/2020/day/10

## Part 1

This seems to require sorting the numbers and then reducing over them with a sliding window of 2 and constructing a tuple of `(diff1_count, diff3_count)`. Apart from the beginning and end rules, I hope there are no gotchas to this.

## Part 2

Okay, this should be a bit trickier to find a rule of calculating the number of arrangements based on the differences between consecutive adapters.

First insight would be that differences of 3 are immutable, those 2 adapters are always required. So we can split the list at each of these 3-diff points, calculate arrangements in each slice and multiply the results.

Second insight is that any arrangement is strictly increasing because of the rule that adapters can't downgrade their input joltage.

Third insight is that the complete arrangement (using all power adapters) doesn't have any 2-diff pairs, so this might simplify the logic a bit. This rule isn't explicit in the requirements, but it at least holds true on the example inputs and my input.

Given insights 1 and 3 from above, any slice has only elements 1-diff apart so arrangements are directly proportional to the number of elements in the slice. We just need to figure out the formula...

## Part 2 - 2 days later

Okay so I went down a 2 day rabbit hole trying to find a generic formula to count all power adapter arangements for a slice of N elements which are 1-diff apart.

After reading the [Wikipedia - Combination](https://en.wikipedia.org/wiki/Combination) page, I realized we are looking for all combinations of N taken K for all K from 0 to N, which ends up being just 2^n as explained [here](https://en.wikipedia.org/wiki/Combination#Number_of_k-combinations_for_all_k).

The problem now was out of all 2^n combinations how can we ignore the ones which have 3 consecutive zeros. For example:

```
N=8

000xxxxx
x000xxxx
xx000xxx
xxx000xx
xxxx000x
xxxxx000
```

For each row we have to compute combinations on the right and multiply with combinations on the left, but also be careful about not repeating previously counted combinations from above rows.

Needless to say, I didn't manage to define a generic formula for counting all combinations which have 3 consecutive zeros. After giving up, I thought I would check the input and see how large our N can get... One ultra mega facepalm later I can say that the max N in my input is 3!!!

After slicing the input by 3-diffs, the largest slice is 5 elements, for example 17, 18, 19, 20, 21. Of these 5 elements, the first and last are immutable because of the first insight above so we get N=3 for the middle elements. The only combination _not_ allowed out of the 8 possible combinations is skipping all three.

So counting combinations inside a slice can be just a switch statement... :facepalm:

```rust
match slice.len() {
  1 => 1,
  2 => 1,
  3 => 2,
  4 => 4,
  5 => 7,
  _ => panic!('panik'),
}
```
