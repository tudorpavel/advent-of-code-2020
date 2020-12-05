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

## Part 2

I sorted the IDs in my input to understand what "some of the seats at the very front and back of the plane don't exist on this aircraft" means exactly:
  - the minimum ID is `85` so it must mean that the first seat is row `10` column `5`
  - the maximum ID is `883` so it must mean that the last seat is row `110` column `3`

I'm assuming these are chosen randomly for each user's input, while making sure the search space for IDs is a contiuous range with just one element missing. In order to solve for the general case, I guess we can find the min and max ID which should be the limits for the range of all possible IDs.

I'm thinking that to find my seat I could simply do a Set diff between all the possible seat IDs and my input seat IDs.
