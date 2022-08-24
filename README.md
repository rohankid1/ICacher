# ICacher
This crate provides 1 new type for caching functions and 1 trait for
implementing your own caching struct. Although there are 2 more traits,
those are deprecated and should not be used.

Running the same function (that return the same value) over and over again can be inefficient. This lightweight, dependency-free crate attempts to 
solve this problem by caching each return value. It will only, unless explicitly called to run multiple times or if the value isn't cached, be called once.

This crate will probably receive regular updates from time to time.
Updates *may* or *may not* have new features in order to keep this
lightweight and simple.

[GitHub](https://github.com/rohankid1/ICacher) |
[Crates.io](https://crates.io/crates/icacher)