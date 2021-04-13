# Learning Rust Journal

## Day 1

Completed excercises for day 1-7. At first I had to google absolutely everything. By day 5 I wrote the entire program including file-reading from scratch without any copy/paste.

Day 4 I tried to experiment with [monads in rust](https://hoverbear.org/blog/option-monads-in-rust/). When validating 7 pieces of data, instead of writing return statements on every other line, I combined optional states with `.and_then()` to perform the next step if each previous step had succeeded. The "success" result was done by Option's `.and()` operator to makes sure all the monads resulted in a value.

Day 7 was by far the most challenging. It required the equivalent of tree walking. I started by building a `struct` for the rules, but in the end it was simpler to just re-run regexes that I constructed dynamically.

I hit a few things that became obvious once I understood what was going on. First, I ran into some confusion with the borrow checker and for loops.

```rust
let con = ...
for c in con {
  foo(c);
}
```

Here I was suprised to learn through some trial and error that `con` is being passed to the for loop by value. Other ways to say that are it is moved into the for loop, or the for loop acquires ownership. What that means then is that each `c` is also an owner, and in this case ownership is given to `foo()`.

This led to some real confusion when what I wanted was a reference. I realized that I should "borrow" the container into the for loop instead, in which case each value `c` is also a borrowed reference, like so:

```rust
let con = ...
for c in &con {
  for(c);
}
```

The next thing that caught me with the borrow checker was stashing references inside a for loop with the Regex library. Accessing a capture by [] like `captures[1]` [does not return a reference to the original string](https://docs.rs/regex/1.1.9/regex/struct.Captures.html#impl-Index%3Cusize%3E), which caught me by surprise. I was very confused by the error messages until I uncovered the linked snippet in the docs:

> The text can't outlive the Captures object if this method is used, because of how Index is defined (normally a[i] is part of a and can't outlive it); to do that, use get() instead.

So realizing that I wanted `captures.get(1).unwrap().as_str()` to get a `&str` with a lifetime of the original string made me a happy camper.

Lastly, I discovered that I have a hard time with not writing the trailing `;` for some reason when writing rust. This is weird since I've been writing C++ all day for the last few decades but 🤷.
