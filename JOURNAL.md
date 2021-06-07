# Learning Rust Journal

I'm learning by doing [Advent of Code 2020](https://adventofcode.com/2020). Since the exercises there are attached to "days" and I am also journaling over a period of "days" this will inevitably be confusing, sorry. We'll just use "day N" to refer to exercises except in my headers which will refer to my own personal timeline. Super clear, right?

## Day 1 of Learning

Completed [excercises for day 1-7](https://adventofcode.com/2020). At first I had to google absolutely everything. By day 5 I wrote the entire program including file-reading from scratch without any copy/paste.

For day 4 I tried to experiment with [monads in rust](https://hoverbear.org/blog/option-monads-in-rust/). When validating 7 pieces of data, instead of writing return statements on every other line, I combined optional states with `.and_then()` to perform the next step if each previous step had succeeded. The "success" result was done by Option's `.and()` operator to makes sure all the monads resulted in a value.

For day 7 was by far the most challenging. It required the equivalent of tree walking. I started by building a `struct` for the rules, but in the end it was simpler to just re-run regexes that I constructed dynamically.

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
  foo(c);
}
```

The next thing that caught me with the borrow checker was stashing references inside a for loop with the Regex library. Accessing a capture by [] like `captures[1]` [does not return a reference to the original string](https://docs.rs/regex/1.1.9/regex/struct.Captures.html#impl-Index%3Cusize%3E), which caught me by surprise. I was very confused by the error messages until I uncovered the linked snippet in the docs:

> The text can't outlive the Captures object if this method is used, because of how Index is defined (normally a[i] is part of a and can't outlive it); to do that, use get() instead.

So realizing that I wanted `captures.get(1).unwrap().as_str()` to get a `&str` with a lifetime of the original string made me a happy camper.

Lastly, I discovered that I have a hard time with not writing the trailing `;` for some reason when writing rust. This is weird since I've been writing C++ all day for the last few decades but 🤷.

## Day 2 of Learning

Completed the day 8 exercise, and learned a few interesting or
difficult things. The exercise itself was a bit tricky to solve
until I realized I just needed to brute force it and try flipping
each operation once. But I also ran into some rust pitfalls.

1) Adding signed and unsigned values.

I wanted to track my program counter, which is an index in an array,
so that makes sense to be a `usize`. But my program's instruction
arguments can jump forward or backward, so they need to be a signed
value, which I made `i32`. So then.. adding the argument to the
program counter should be easy, right?

In normal C++, at least with the warning set I am used to at work,
you would just write (with `pc` being `usize` program counter and
`jumparg` being the `i32` argument)

```cpp
int32_t jumparg = -2;
size_t pc = 5;

pc += jumparg;
```

This [just works](https://godbolt.org/z/77nYKbsKo), though I know this hides potential issues here and I would have to go read a standard to be sure `pc` won't get truncated and turned into an `int32_t` during the operation, losing state along the way.

In rust, since types do no just convert implicitly, I immediately had a bit of a headache. Doing `pc += jumparg` was an error because I cannot add different types. I thought ok sure I will just cast, as I have been until now

```rust
pc += jumparg as usize;
```

Except `jumparg` can be negative, so casting isn't enough here. Ok so I went to the internet to see what other people do, and it seems to be a bit of a sharp edge in the language. I couldn't find any good concensus. So I tried writing a generic function to add a signed value to an unsigned value, easy right? Not really, generics are not like C++ templates it turns out. You can't just put a "will be some type" and write code for any given type (or at least I didn't figure out how). Instead, you have to write to a trait, ok so I found traits for [signed](https://docs.rs/num/0.4.0/num/trait.Signed.html) and [unsigned](https://docs.rs/num/0.4.0/num/trait.Unsigned.html) numbers in [the num module](https://docs.rs/num/0.4.0/num/index.html).

```rust
fn add_signed<U, I>(u: U, i: I) -> U
  where
    U: num::Unsigned,
    I: num::Signed,
{
  if !i.is_negative() {
    u + i as ...
  } else {
    u - i.abs() as ...
  }
}
```

But now I still need to do something with the signed value to make it into the correct unsigned value. And it would be nice to promote it to a larger size if `u` is larger before doing `abs()`. But then, what if `u` is smaller? I didn't figure out what to do here, maybe because there is indeed no good answer, or no easy-and-correct answer.

So I fell back to something simpler, trying to do this with a few casts

```rust
pc = (pc as i32 + jumparg) as usize
```

which works, with well-defined behaviour, but it's truncating the `pc` from a potential 64 bytes down to 32. So what if we work in i64?

```rust
pc = (pc as i64 + jumparg as i64) as usize
```

This also works, but is still truncating our 64bit `pc` value to 63bits + sign bit. For this toy program it surely doesn't matter, but it made me unhappy. This didn't feel like the right way. How could I do this by only changing the smaller `jumparg` and applying the value to the `usize`?

The first helper I discovered was that `i32` has a method `unsigned_abs()` which will convert my `i32` to `u32`. Perfect so we can add a `usize` and a `u32` surely? Almost, we just need to cast the `u32` to `usize`. This shouldn't be lossy, so now we're happy. But we need to deal with the positive and negative values. At this point it was worth making a helper function, which let me add `pc` and `jumparg`.

```rust
// Add a signed i64 to an unsigned usize.
fn add_signed(u: usize, i: i32) -> usize {
  if i >= 0 {
    u + i.unsigned_abs() as usize
  } else {
    u - i.unsigned_abs() as usize
  }
}

// Then we can add pc and jumparg finally!
pc = add_signed(pc, jumparg);
```

Hooray! But not so fast. This problem had the argument be used in 2 different contexts. One, to move the `pc` as we discussed above. But it can also be used to modify an `accumulator`. Since a negative value in the accumulator makes sense (unlike for the program counter), I had made the accumulator be `i64`. See the problem? Yet more integer type conversions!

```rust
Operation::Acc => {
  accumulator += arg as i64;
  pc += 1;
}
Operation::Jmp => {
  pc = add_signed(pc, arg);
}
```

In the first line I'm adding an `i32` to an `i64` to I have to cast it again. I had over-optimized by using an `i32` at which point I felt like I had landed on what seemed like a good rule:

_Make all integer types 64bit unless you have a very good reason not to._

I changed the argument to `i64`, and I cast it to `usize` inside `add_signed()` after getting the absolute value. This gives a pretty decent defined behaviour for my program. If this was going to production I'd probably want to check that the conversion there didn't lose any state. Apparently [the TryFrom trait](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) ... implements methods for primitive types? I am not sure how to even word or explain that yet. `u64` does not have/implement the `TryFrom` trait, but it seems `TryFrom` provides an implementation itself as soon as you `use` it.

```rust
// Add a signed i64 to an unsigned usize.
fn add_signed(u: usize, i: i64) -> usize {
  use std::convert::TryFrom;
  if i >= 0 {
    u + usize::try_from(i.unsigned_abs()).unwrap()
  } else {
    u - usize::try_from(i.unsigned_abs()).unwrap()
  }
}
```

2) Borrow checker problems

I was iterating over a vector of tuples, so I was accessing my `program[pc].0` and `program[pc].1` etc. In C++ I would make a reference to these to give them a nicer name, so I tried to do similarly. But I also wanted to be able to write to `.0` so I needed a mutable reference.

```rust
loop {
  let visited = &mut program[pc].0;
  let op_flipped = &mut program[pc].1;
  let instruction = &program[pc].2;
}
```

Welp, we can't do that! The borrow checker immediately complains because I've to 2 mutable borrows of `program`. Since I did this up at the top of the loop, we use both `visited` and `op_flipped` below, the borrow checker sees that they both need to exist at the same time. My first unhappy moment with the borrow checker.

So I had to actually rethink how I wanted to structure my loop internals. If I could break it into parts that use `.0`, `.1` and `.2` separately, then I could borrow each of them in the section I wanted to use them only. Unfortunately, `op_flipped` and `instruction` were being used together within a single match statement.

My next thought was to scope the references more closely inside the match statement and other blocks. This ended up making me very unhappy. In the code above, it's very clear what each name is for the tuple values. It's easy to see the wrong name is not used as each tuple appears once and in order. With the references all over my loop, it would be too easy to write `.1` when I meant `.0` or something. This was equivalent to using magic numbers instead of constants.

My breakthrough realization here was that tuples were not serving me and that I should provide them names. So I wrote a struct and mapped the string into that. It's more verbose than constructing a tuple, since you have to name all the fields there as well, but now I had names instead of magic numbers.

```rust
  struct ProgramLine {
    // Was this line executed yet in the current execution.
    visited: bool,
    // Was this instruction's Operation flipped yet in the current
    // or any previous execution.
    op_flipped: bool,
    // Instruction to run.
    instruction: Instruction,
  }
  let mut program: Vec<ProgramLine> = input_all
    .split_terminator("\n")
    .map(|line| ProgramLine {
      visited: false,
      op_flipped: false,
      instruction: read_instruction(line),
    })
    .collect();
```

I thought about making `visited` and `op_flipped` default to false and not initializing them explicitly. It seems that isn't very easy however. You can't just assign a default value in the struct definition like you would in C++. You have to implement a Default trait, essentially writing a default constructor, but with more lines of code. I settled for explicit initialization.

Once these had names, instead of taking a reference to each field, I just took a reference to the ProgramLine struct as a whole. This meant I only needed one `&mut` reference, which made the borrow checker as happy as a plum.

```rust
let pline = &mut program[pc];
if pline.visited {
  break;
}
...
```

In conclusion for today...

No implicit narrowing, or even widening, of integers is a bit of a pain. However it did make me think about what types really needed and after a few iterations I think I am very happy with the result, and it has very well-defined behaviour or will crash in a defined way. I could give it any other defined behaviour instead of a crash very easily.

Use 64-bit integers unless you really need something else. Use `usize` for indexing though. `usize::try_from()` can reliably take you to `usize` from `u64`.

Eagerly promote tuples to structs. Don't use references to "give something a name". Just give it a name as a struct instead. Use references to express ownership. As as exception.. I did use `pline` to give `program[pc]` a name inside a loop iteration, but I have a feeling even that would be a bad idea and could bite me, since it forced `program[pc]` into a `&mut` reference, preventing multiple shared references if it were needed for part of the loop body.

## Day 3 of Learning

Maybe it's that the problems are getting harder, or that I am getting more familiar, but I spent very little time thinking about "how do I write this in Rust" today. I spent almost all my energy on how to solve the problems, and the writing of code was secondary. The way it should be. I solved day 9 and day 10 problems.

Noteworty moments were:
- I got to use slices, which I was just reading and learning about today. I understand they are like a base::Span in chromium, and super easy to use.
- I got to use dynamic programming. That was exciting since I don't think I've done that since 3rd year of university!
- Ranges with .rev() make it very easy to iterate a loop backwards with indices without going out of range or messing anything up. Yay!

## Day 4 of Learning

Today I used an `impl` for a `struct` when working on the day 11 problem. This gave me a chance to try out Object Oriented programing in rust. Since I only had one type, and I didn't need to build relationships between types, it went well! In my work time I've been trying to understand how to build an object relationship graph and getting the sense you Just Shouldn't, and you should separate "data this type holds" from "interfaces/structs this type uses" and pass the latter as a function argument instead of storing it as object state.

Anyhow, the difficulty of the problems are increasing, which is leading to me writing bugs more often and having to spend time in the edit/build/debug cycle more. It's still very very nice that my code is not doing something undefined (like reading off the end of an array) as those types of problems are much harder to debug.

One thing I miss is `#define` and `#if` to easily block off parts of the code to not run without changing scopes. That would let me more easily switch between different test cases. For the day 11 problem for example I was switching between reading the input file (large) and copy/pasting smaller examples out of the instructions, and I had to go switch comments at 4 or 5 different places each time I tried the other mode.

Since the difficulty of problems is increasing, and I am starting to as a result split my solution up into composable functions, I should also probably start writing unit tests. I think rust seems to make that easy to do but I haven't tried yet. Some simple tests of my edge cases would let me spend a lot less time debugging I suspect.

So overall today I think I learnt things I want/should do/improve rather than mistakes I was making in the language itself. The language mostly stayed out of my way again today, other than the way it forces you to think about your types a bit but in a way that I don't mind because it results in code that runs reasonably at each step of the way.

## Day 5 of Learning

I solved the problem for day 12. Along the way I used a number of rust concepts again.

Structs. Nothing new here.

Enums. I was going to write a struct for one piece of data with 2 fields, but I realized an enum with an argument for each variant would do the same job and seemed nicer to work with.

Impls. I implemented `std::fmt::Display` for my enums and I actually understood what I was writing this time!

Rebinding. This was new for me, being free to write `let` instead of making something mutable is nice, esp for things like fn arguments.

Unit tests!!! Ok singular. I wrote one enormous test for my most tricky logic function. It meant that when I finally compiled and ran my part 1 solution it worked the first try.

`anyhow::Result`, which is part of the `anyhow` crate, along with the `?` operator. I tried to work with `std::result::Result`, but mixing the Err types with `io::Result` got very confusing for my beginner self. But I learnt about `anyhow` at work today and thought I should try it, and it was great. I made any function that could have surprises return a `Result` instead of doing a panic!, which I am sure [Linus would be happy about too](https://lore.kernel.org/lkml/20210414184604.23473-1-ojeda@kernel.org/T/#ma8f901fffc0badc0f5a9a52046d984c4bb428dec). Those folks probably understand how to get `std::result::Result` to play with other error types though.

## Day 6+7 of Learning

I worked on the problem for day 13. Part 2 was tricky, and this took me a 2nd day of effort to figure out a solution for. Nothing really new or interesting happened rust-wise as this was really just how to solve a tricky problem. I enjoyed solving it in rust nonetheless, and had no moment where I wished for a different language, or felt like it was getting in my way. The loops and closure constructs, along with compiler warnings/errors all helped me solve it faster than I would have otherwise.

I also solved the puzzle for day 14. I started out by making abstractions to match the puzzle, including a "36 bit integer". I did this by wrapping a 64 bit integer in a struct, which worked out fine but in the end I didn't need anything special about being 36 bits, like wrapping or trucating or anything, so that made things harder than they needed to be.

One small lesson I learnt to prevent annoying sprawling refactoring: If your method takes something by value, and it makes sense, but you have a refernce.. don't go change the method signature to a reference. It will just annoy you later when you have values. Just do the deref in that place.

Tagged unions aka enums continue to be an awesome thing to build logic around, especially with matchers, and `if let Some(val)` syntax to branch when an Option is set is very wonderful.

## Day 8 of Learning

I solved the puzzle for day 15. I simply built out data structures to match the problem description, compiled and ran it, and got the right answer. Then for part 2, I needed to run the same problem on a much longer timeframe. I just increased the number of iterations, and it still completed in about 4 seconds. I blame Rust giving me fast codegen when I follow the defaults. That along with absolutely 0 seconds doing any runtime debugging, *Rust is amazing*.

On the puzzle for day 16, I learnt that containers that only expose an iterator (via `iter()`) and don't give other helpers that do runtime checks for bounds are a bit frustrating to use - see bit_set::BitSet. If you know you have a single bit set, and you want to get the value of it (which bit it is), as far as I know you have to write `assert_eq!(bitset.len(), 1); bitset.iter().next().unwrap()` which.. is all kinds of red flags. Using an iterator without a loop. Using `unwrap()`. But it doesn't even implement index (probably because iteration order seems to be undefined?) and it doesn't give like first() which would probably normally give an Option<usize> in case there's none set. That would still have an `unwrap()` but at least not a `.iter().next()` which is ugly. Mutating a hash map when you the value is there is a bit annoying too, it still gives an Option instead of panicing, which I guess is good but idk. I didn't like any of this:

```rust
  // `possible_fields` is HashMap<String, BitSet>. This gets the first bit from the bitset in the hash map value.
  let remove_bit = possible_fields[&fdefn.name].iter().next().unwrap();
  // This mutates a different hash map value to remove above bit from this different bitset.
  possible_fields.get_mut(&other_fdefn.name).unwrap().remove(remove_bit);
  ```

  Also `Vec<Vec<u64>>` was pretty confusing to work with when I ended up with a set of &Vec<u64> and I was trying to collect() them back into the final vector. I fiddled around with those compiler errors for a good 10 minutes. Now I know you can't collect references back into a set of value types, which makes sense. The problem was that I was doing `vector = vector.iter().filter(|x| ...).collect()` which looks fine, right?? No. `iter()` borrows the vector and thus iterators over references to its contents, which you can't collect back into values. Instead, you need to iterate over the values, consuming the first vector, with `into_iter()`: `vector = vector.into_iter().filter(|x| ...).collect()` will work correctly. This was really a lesson in thinking about references and reading error messages about them.

  A style lesson learnt is to use blocks to break up long iterator-based initializations. For example this
  ```rust
    let nearbys: Vec<Vec<u64>> = inputs
    .get_nearby_tickets()
    .into_iter()
    .filter(is_valid_ticket)
    .collect();
  ```
  takes a `Vec<Vec<u64>>`, consumes and iterates it, filters that and collects the result. But it wraps long since it's a long line, and it's less clear than it could be. Instead, using a block I can split up the first vector from the filtering for better clarity IMHO:
```rust
  let nearbys: Vec<Vec<u64>> = {
    let all_nearbys = inputs.get_nearby_tickets();
    all_nearbys.into_iter().filter(is_valid_ticket).collect()
  };
```
And because I didn't use any mutable variable, I can expect this to generate optimal code.

For the puzzle for day 17, I used an object oriented approach. Or is it called data-oriented? I built a model, with methods to query various things about it, and set states. Then I made a method that would generate the next version of the model from the current version, using those methods. This made for some extremely clean code IMO. And the proof of this was when the problem wanted me to add another dimension. I copy-pasted and added the 4th dimension everywhere. I had to hunt down one copy-paste mistake where I still had a `z` but wanted a `w` for the 4th dimension, and it Just Worked!

The puzzle for day 18 was super satisfying! Matchers were very helpful and made the branching very clear in the code. Part 2 was tricky to do without rewriting it to build an operation tree or something. But by accumulating multiplications in a 3rd stack, I was able to solve it without rewriting the world. I reused my strategy from the previous problem of writing the logic into a struct's methods. Then when part2 needed different logic, it was easy to copy/paste the whole object, rename it, and modify it to my needs. This is a very nice pattern for doing advent it seems.

I also found a better way to run test code vs production. Once the production code is all inside a class, I can write a `main()` that reads from a file, and a test function that has some fixed string input, and that's a lot nicer than editing the source code to inject test inputs.

## Day 9, 10, 11... of Learning

Ok it's been a month and 10 days since I completed a problem. Day 20's puzzle was very tricky, and I was too persistent in trying to solve it with permutations, which were just too expensive. Once I gave in an read what [someone else](https://sethgeoghegan.com/advent-of-code-2020) had done (in Ruby) in the solutions reddit, it took about 2 hours to write a solution that passed. Hooray!

I was trying to make use of the very nice `permutations()` method in Itertools that lets you generate an iterator of all permutations of items returned by another iterator. It's cool, but it was not what I wanted here. It worked for the 3x3 example, but it blew up for the 12x12 problem input.

The right way to solve this was to construct a mapping from tile edge patterns to tiles, then you can find, for a given tile slot, all tiles with an edge that matches the tile above, and an edge that matches the tile on the left. And you can do this in constant time. Then it's a matter of rotating each potential tile into place (if you can), and recurse. If the recursion bubbles up with a "Could not solve" you try the next tile in the candidate pool. If none of them match, you report "Could not solve".

This made for a pretty simple recusive function, returning an `Option<Vec<Tile>>`. On success, it returned the solved board once all tiles were placed. If any tile couldn't be placed at a step it would return `None` to try another path.

This was my first use of explicit lifetimes, as I built a context struct to pass along through the recursion, instead of passing through like 8 parameters. The struct is copied at each point of the recursion, in order to allow unwinding and continuing from an earlier state. So to avoid copying a bunch of structures inside, most of the fields are references, along with maps with reference values. They all share the lifetime of the struct itself, which is first created before entering the recursion. This Just Worked wonderfully.

I also had the borrow checker inform me when my refactoring to split 2 statements caused a reference to a temporary to outlive the temporary object. Thank you Rust!

Somewhere along the way I solved day 19's puzzle as well, but I don't remember at this point since I spent so long on day 20's. Most of that time was just spent avoiding working on it since I didn't want to rewrite everything again, but reading another solution gave me the motivation to go and solve it in Rust.

## Day N of Learning Continued

I solved the day 21 puzzle, and it was a fun one. I used a fair number of data structures to do so, a lot of HashMaps, and HashSets. Some new(ish?) things from Rust that I used:

 - Tuple structs. Instead of a `Vec<Vec<&str>>` which starts to get confusing, I put the inner vec into a tuple struct `struct ProductIngredients(Vec<&str>)`. These tuple structs help in terms of typing, but they're also a bit annoying/awkward since they require a `.0` to access the actual data. Being used to type aliases in C++ that's how I think of them more or less, at least for a single member, but that's not how they play out. They are useful for type-safety and code documentation regardless.

 The first time I found something that was like "this doesn't feel right", was when I tried to call HashSet<&str>::remove() with a String...

```
error[E0277]: the trait bound `&str: Borrow<String>` is not satisfied
   --> day21/day21.rs:148:27
    |
148 |           other_product.0.remove(&found_ingredient);
    |                           ^^^^^^ the trait `Borrow<String>` is not implemented for `&str`

error: aborting due to previous error; 1 warning emitted
```

The HashSet is holding `&str` so you have to pass it one. I had a `String`, which when you borrow, gives you a `&str`. So you'd think `hashset.remove(&string)` is going to work, but yet it doesn't.

I don't fully understand why exactly but my rough understanding was that the `remove()` function takes a `Borrow<T>` trait. And `Borrow<T>` expects that borrowing `T` returns a `&T`, however for whatever reason, borrowing a `String` returns a `&str`. I'm not sure why it tries to use `Borrow<String>` when it's being passed `&string` which I'd expect to be a `&str` type, but it seems to be a `&String`. The way to solve this was to give a `&str` more explicitly , by calling `hashset.remove(&*string)`. The `*` operator makes a `str`, which we then borrow. I found this weird.

Perhaps this is similar to how type conversions don't happen automatically in C++ when you call a templated thing, it just makes the templated type be what you gave it explicitly. In this case it's a trait bound, so maybe `&string` makes the bound be a `&String` type even though that'd normally convert to a `&str` outside of a trait bound?

I once again made use of explicit references, storing references to the input parsed strings instead of copying them all over. Of course, this made me run into the borrow checker a few times. If you pass A to B as a reference, and A had references in it, now they become tied together, and you can't mutable borrow B anymore. At least that's how it appeared to me. I solved it by having B consume A, then I could use it at will. I also did copy some strings in one place to avoid extending some `&str` references inside a structure, preventing a later mutable borrow. This seemed like an okay compromise. I'm not sure how I could have avoided it. The code was:

```
      let mut found = None;
      for (allergen, ingredients) in self.0.iter() {  // Shared borrow of self.0.
        if known.from_allergen.contains_key(allergen) { continue }
        if ingredients.0.len() == 1 {
          let ingredient = *ingredients.0.iter().next().unwrap();
          known.from_allergen.insert(allergen, ingredient);
          known.from_ingredient.insert(ingredient, allergen);
          found = Some((allergen.to_owned(), ingredient.to_owned()));  // to_owned() instead of refs.
          break;
        }
      }
  
      if let Some((found_allergen, found_ingredient)) = found {
        // Remove the found allergen as a candidate in all other 
        for (other_allergen, other_product) in self.0.iter_mut() {  // Mutable borrow of self.0.
```

The `found` variable was holding references to strings, which came from `self.0`. Even though the lifetimes would be fine since those strings lifetimes are longer, holding the references extended the borrow on `self.0` causing the later mutable borrow to fail. So I made `found` hold `String`s instead.