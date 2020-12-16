# Advent of Code 2020 - Day 16

https://adventofcode.com/2020/day/16

## Part 1

I'm sure the field names will be needed for Part 2, but I will create the minimum necessary data structures just for Part 1. We need to have a collection of all the Ranges from all the rules and a collection of all numbers from all the nearby tickets and then filter the list of numbers based on wether they are part of any of the ranges or not.

It should be possible to express all of this using functional pipelines, let's see how it goes.

## Part 2

Okay so now that we know the entire problem space we can create some data structures to keep things nice and tidy. We need a `Ticket` which can be a type alias for a `Vec<usize>` and we need a `Field` which is a struct something like so:

```rust
struct Field {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}
```

As for the algorithm of finding which Ticket position corresponds to which field, we have to be careful for overlapping ranges. I assume it's possible some Fields are valid for multiple Ticket positions and then we would need some backtracking to match Fields to Ticket positions, but we can check this assumption. Checked and it's true, multiple fields can match for each Ticket position.

In fact, sorting by the number of matched fields we get this nice sequence:

```rust
[
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
]
```

Which means we can assign each Ticket position a Field in this order, always choosing out of the valid Fields the only Field that wasn't previously picked. No need for backtracking this time. :)
