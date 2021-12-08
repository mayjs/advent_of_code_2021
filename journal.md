# Journal of my Advent Of Code 2021 solutions

## Day 1

Day 1 was nothing difficult and a nice opportunity to setup some basic framework code in `lib.rs`.
I think the implementation turned out nice, though the functions in `day01.rs` are probably overly generic.

## Day 2

Day 2 required some more parsing logic compared to part 1, but nothing crazy and it was easy to do by implementing `FromStr` for custom structure.
I feel like the IntVec should be generalized into a generic vector supporting more operations and types, I may come back to this if something like this is required again.

## Day 3

Day 3 seemed easy at first glance but some details of it proved to be a hassle.
Specifically, the `gamma` and `epsilon` calculation would have benefitted from some datatype offering variable bit-width integers so `epsilon` could have been the bitwise negation of `gamma`.
I'm not aware of a crate offering such types and I'm also not sure about the most efficient way to implement it - One way would be to support 64 bit at most and combine one `u64` value with a second `u64` bit mask.

Part 2 got a little messy due to limitations in my previous API design.
`count_digits` should have taken in an iterator of string references as that would have allowed us to get rid of cloning on every call.

Altogether, I think this day could be refactored to clean up some parts of it.

## Day 4

This day took way more time to solve than the previous days.
This seems to have been the case for a lot of players, as is evident by the solve times in [this nice scatterplot](http://www.maurits.vdschee.nl/scatterplot/).

The main reason for this was the parsing and evaluation of Bingo sheets. Once that was working, the actual solving logic was nothing complex. I'm happy with the Bingo parsing and the new `stream_file_blocks` utility function.

I also hugely benefitted from my sorting approach for finding the best sheet in part 1 as it allowed me to just search for the minimal value in part 2.
The obvious approach of collecting all sheets into a vector and then applying the numbers stepwise to all sheets at the same time would have probably made that part more difficult.

## Day 5

Day 5 seemed pretty simple at first: Some minimal parsing to get the lines, creating a map of crossed points and evaluating this map should not be too difficult, right?
As it turns out, Rust ranges can not iterate from high values to low values (i.e. they are empty if `start > end`), which really through me off for this day.
I created the `BidiRange` to solve this issue, which is essentially an iterator that can iterate both up and down, similar to the `range_step` function in the `num` crate.

Also, I finally gave in and added a generalized 2D vector implementation, which I'll hopefully expand in the future.

Part 2 turned out to be easier than I expected, the 45° constraint allowed me to just zip two separate coordinate iterators to generate points without any further issues.

## Day 6

Day 6 immediately made me think about the bad space efficiency that would become an issue if you were to go with the naive approach that's strongly suggested in the description.
To mitigate this, I decided to choose another strategy: Since the lanternfish had no other attributes than their age, why don't we just store the number of lanternfish with each possible age in an array?
This proved to be the right approach and part 2 just worked by changing the number of days accordingly.

## Day 7

This was an easier day once again.
Once you realize that the optimal position has to be somewhere between the minimal and maximal input values, it becomes a matter of simply calculating the total fuel consumptions for all viable positions.

The `parse_lines` was used twice now, maybe I should add this to my library of helper functions.
Unfortunately, it's difficult to make a non-collection variant of it, since something like `lines.map(|line| line.split(',')).flatten().map(...)` will fail since line is only borrowed to the split function.

## Day 8

Day 8 was intimidating due to the large amount of text.
In the end, part 1 turned out to be easy and part 2 was quickly implemented once I figured out some deduction rules using the known for digits from part 1.

I created the deduction rules for this day by hand, some sort of solver would probably have helped here.
I think this day would have been nice to implement using Prolog.
