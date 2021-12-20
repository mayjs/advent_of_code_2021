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

## Day 9

This day was not too difficult.
Part 1 only required some simple parsing and iterating over neighbors of points.

Part 2 could be solved using a breadth first search.
My solution is based on the assumption that every basin is always constrained by points with height 9,
if two points could also compete in one enclosed area, it will not work properly and just combine those two basins into one, resulting in a much higher output value.

The concept of a two-dimensional field like my `Heightmap` was required before to solve the Bingo problem.
It may be a nice idea to provide the core functionality of this pattern in my aoc library.

## Day 10

Finally in the double digits! 😃

This day was quite fun, a simple stack-based parser without any backtracking was easy to set up and the results were quickly calculated.
The only part that I don't like is that `tokenize` returns a `Vec` instead of an `Iterator`.
Ownership issues made this the simplest solution I could figure out, but it should be possible to somehow "bind" the line context to the resulting iterator.
Do you really have to create a separate `struct` for this to work?

## Day 11

Since this day once again required the creation of a 2D field and iterating over neighbors, I decided to finally implement the 2D field.
It has some nice parsing routines that should make future assignments like this easier for me.
I may include the logic of `OctopusEnergies::parse` into `field2d` as well.

Other than that, the assignment was quickly solved.
I originally planned to keep track of octopuses that needed to flash using a stack or queue, but ended up just iterating over whole field instead.
There may be some room for improvements here.

## Day 12

This was the first day involving a graph data structure.

The graph implementation for this day is not really clean, I'm not really happy with the way that the internal indices are exposed by the API.
Unfortunately, we can't easily map back from indices to node values using this approach, so other solutions would have been more difficult
and I didn't want to spend too much time on the implementation of this data structure.

The actual algorithm for finding the paths was not too hard - a simple recursive DFS which also keeps track about seen small caves.
This solution relies on the fact that there will be no cycles between big caves, which is implied by the assignment since that would yield an infinite amount of paths.
If such cycles exist, the solution will just fail due to a stack overflow.

## Day 13

This day was interesting.
No complicated algorithms this time, just simple mirroring on vertical and horizontal axes.
The second part was weird because you needed to actually create a visual output and manually decipher the resulting passphrase.

I think my implementation could be a little more efficient if `drain_filter` was available in non-nightly Rust, but as it is, I couldn't think of a cleaner way to solve this problem.

## Day 14

I was in a bit of a hurry today, but I think the solution worked out nicely in the end.
Since the assignment for part 1 already mentioned how quickly the input would grow, I immediately suspected that part 2 would require handling many more iterations,
so I decided to implement an approach that does keep track of element positions.

Instead, my solutions only keeps track of the amounts of distinct element pairs.
The example would give the following map: `{"NN":1, "NC": 1, "NB": 1}`.
Applying the rule `NN -> C` would then create two different pairs: `NC` and `CN`, while all `NN` pairs are consumed.
The resulting map is `{"CN":1, "NC": 2, "NB": 1}`.

## Day 15

This day just required some basic knowledge of pathfinding algorithms to find a solution.
I opted for the A* algorithm and used the euclidean distance as an heuristic to get a quicker result.
Path reconstruction was not required, so we can even skip storing the predecessors for every node.

I noticed two issues that I'd like to improve for this solution:

1. Mixing of `usize` and `u32`.
    A lot of casting was needed because the risk levels are stored as `u32` but the node coordinates are `usize`. 
    This should be a matter of just casting the converted `char` values once in the parsing function.
2. Inefficient cloning of the risk field for part 2.
    It would have been possible to just create a lazy implementation for this.
    This implementation could have just calculated the new values as needed and we could have avoided a lot of copying work.

## Day 16

This day took me way longer than any of the previous days (about 1.5h).
I made some dumb mistakes in my recursive parser function, resulting in invalid amounts of bits getting reported in some cases.
This could have been mitigated by actually counting the number of bits that were received from the iterator through some wrapper that would keep track of the amount of times that next is called.
Instead, I took the simple option of manually adding bytes up and that caused some troubles.

Apart from that issue, this was a fun day though and the result felt very rewarding.

## Day 17

I didn't like this day.
Part 1 turned out to have a trivial solution and I just ended up solving part 2 using brute force, which never feels right but worked just fine for this assignment.

## Day 18

Today was an exciting day, I never had to build a traversable tree-like structure in Rust before.
I decided to just use reference counting pointers and runtime borrow checks, though I think this would be a nice application for the GhostCell concept.

The assignment was a little ambiguous, so I was not sure at which points a reduction was needed and it's also not really clear if explosions only happen exactly at nesting depth 4 or also at larger depths.

## Day 19

The transformations gave me a lot of trouble today.
My first solution considered way to many transforms, but I finally gave in and refreshed my memory about 3D transformation matrices.
Using those it was easy to get the proper 24 possible transforms.

As always, part 2 was done quickly once part 1 worked properly.

All in all, this days solution is the slowest I had so far.
It takes about .25 seconds to calculate the full beacon map and since we built it twice for part 1 and part 2, the total runtime increases to half a second.
There definitely is some potential for optimizations here!

## Day 20

Today was an easier day for a change.
Of course there still was one trap in the assignment: You can't assume that index 0 in the enhancement algorithm is always `.`,
so you have to make sure to simulate an infinite field of pixels toggling on every iteration.

My solution initially relied on just growing the image by two unlit pixels in all directions in every step.
This was based on the assumption that index 0 would never be lit, but that was exactly the case in the actual input.
The solution was easily added: On every even step, if index 0 of the replacement table is `#`, replace the unlit ring around the image by lit pixels.

Of course, my solution is not very efficient with regards to memory consumption.
The rings should only be added if the image is actually growing towards the border, but I didn't want to create
a new Field2D that could be dynamically grown, so this was the easiest solution and the performance seems to be fine.
