# Advent of Code 2020 - Day 19

https://adventofcode.com/2020/day/19

## Part 1

There is a way to construct a binary tree such that it codifies all possibilities by marking a left branch as `a` and a right branch as `b` and this should make verifying a given message easy. However, it might not be so easy to construct the tree from the given rules, especially since I'm a Rust newbie that needs to implement a data structure with pointers. Let's see how it goes...

## Part 2

So... I went down a huge rabbit hole on this one. I thought I could accomodate the recursive rules in my binary tree solution using some reference cycles but I had to give up. The way I implemented building the trees is cloning subproblem trees and I couldn't manage to reuse subproblems by just having pointers to them.

After many hours in many different days I looked closer at the input and realized rule 0 is composed of just rules 8 and 11 which are the ones that change for Part 2 and become recursive:

```
0: 8 11
8: 42 | 42 8
11: 42 31 | 42 11 31
```

What this means is that we can split each message in chunks and try different sequences of rules 42 and 31 to validate a message. Quite conveniently the subproblem size is 8 for both rule 42 and 31 so we can split the message in chunks of 8 and we have a finite set of combinations we can try depending on the number of chunks.

For example, if the message is split in 9 chunks we have to try the following sequences:

```
42 42 42 42 42 42 42 42 31
42 42 42 42 42 42 42 31 31
42 42 42 42 42 42 31 31 31
42 42 42 42 42 31 31 31 31
```

The message is valid if at least one sequence of rules matches the message chunks one by one.
