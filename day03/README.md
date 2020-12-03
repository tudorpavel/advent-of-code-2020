# Advent of Code 2020 - Day 3

https://adventofcode.com/2020/day/3

## Part 1 - Initial thoughts

Initial thoughts are we could have a struct to store size of the map and positions of trees in a hashmap for O(1) lookup by coordinates (x, y) where `x` references columns and `y` references rows.

```
{
  width: 11,
  height: 11,
  tree_positions: {
    (2, 0): true,
    (3, 0): true
  }
}
```

When looking up by coordinates, the `x` coordinate should be the remainder of dividing the given `x` by the `width` to handle infinite repeating to the right.

That's the idea anyway, we'll see how it goes translating it into Rust.

I assume Part 2 will ask us to find a slope which minimizes the number of trees encountered, but I will refrain from adapting my Part 2 solution to this assumption. Other than maybe having a procedure which counts the number of trees encountered given a slope.

## Part 1 - After implementing

The implementation was mostly straightforward, for the HashMap of tree positions I found [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html) in Rust's standard library which is perfect since I actually wanted a Set to easily check presence of a given Point.

I thought about using a more functional style in `GeoMap#tree_count` but decided to go with this imperative approach. I was worried I couldn't easily generate a path Vector due to the wrapping of `x` by `self.width`.

## Part 2

The solution is simple enough, we just have to map `GeoMap#tree_count` over the given collection of Points and compute the product.
