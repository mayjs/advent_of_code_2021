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