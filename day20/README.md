# Advent of Code 2020 - Day 20

https://adventofcode.com/2020/day/20

## Part 1

I was thinking of shortcutting Part 1 and finding just the corner tiles as the only ones which have 2 borders that don't match any other borders, but I saw some spoilers on Twitter that this year there were 3 days with Conway's Game of Life as a tribute to John Conway, it seems like this might be the second one. So I think Part 2 will require the full image to be assembled correctly.

Each image tile can be rotated or flipped so we have 8 possible states:

```
a b c   g h a   e f g   c d e
h   d   f   b   d   h   b   f
g f e   e d c   c b a   a h g

c b a   e d c   g f e   a h g
d   h   f   b   h   d   b   f
e f g   g h a   a b c   c d e
```

Basically each border can also be matched in reverse order.

Looking at the input there are 144 tiles so the final image is 12x12 tiles.
