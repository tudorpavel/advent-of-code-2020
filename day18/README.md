# Advent of Code 2020 - Day 18

https://adventofcode.com/2020/day/18

## Part 1

It seems there are no numbers larger than 9 in the given expressions, which means we can iterate it character by character.

We could try to use a stack as follows: iterate chars in the expression from right to left, push each character to a stack data structure and when we reach an open paren `(` it means we have to calculate values on the stack by gradually popping numbers and operators until reaching the first closing paren `)`. We push the result back onto the stack and move on iterating the expression.

For example:

```
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2

Stack i = 1
| 2 |
-----

Stack i = 2
| * |
| 2 |
-----

We keep pushing chars onto the stack...

Stack i = 17, right when we reach the first open paren '('
| 6 |
| + |
| 9 |
| * |
| 8 |
| + |
| 6 |
| ) |
| + |
| 6 |
| ) |
| + |
| 2 |
| + |
| 4 |
| * |
| 2 |
-----

At this point we have to calculate the expression:
6 + 9 * 8 + 6
15 * 8 + 6
120 + 6
126

And push the result back to the stack:

Stack i = 17
| 126 |
| +   |
| 6   |
| )   |
| +   |
| 2   |
| +   |
| 4   |
| *   |
| 2   |
-------

Stack i = 19
| )   |
| *   |
| 126 |
| +   |
| 6   |
| )   |
| +   |
| 2   |
| +   |
| 4   |
| *   |
| 2   |
-------

...

Stack i = 24, found '(', reduce until first ')'
| 2   |
| +   |
| 4   |
| *   |
| 9   |
| )   |
| *   |
| 126 |
| +   |
| 6   |
| )   |
| +   |
| 2   |
| +   |
| 4   |
| *   |
| 2   |
-------

Stack i = 24, reduced
| 54  |
| *   |
| 126 |
| +   |
| 6   |
| )   |
| +   |
| 2   |
| +   |
| 4   |
| *   |
| 2   |
-------

Stack i = 25, found '(' and reduced until first ')'
| 6810 |
| +    |
| 2    |
| +    |
| 4    |
| *    |
| 2    |
--------

At this point we are done iterating the expression chars,
we can reduce the entire stack and get the result:
| 13632 |
---------

```

## Part 2

Nice twist, but we should be able to adapt the solution from Part 1 by having 2 reduce procedures:
  - `reduce_addition` which begins when we encounter a `*` or `(` and adds everything until another `*` or `)`
  - `reduce_parens` which is the same one we have at Part 1, but it should only encounter `*` operators

So in our `evaluate` procedure we:
  - call `reduce_addition` when we encounter a `*` and then push back `*` and continue
  - call `reduce_addition` and then `reduce_parens` when we encounter a `(`
