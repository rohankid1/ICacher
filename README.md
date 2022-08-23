# ICacher
This crate provides one new type (and 1 public (2 have been deprecated) trait for which
you can implement your cacher struct(s) with) which is useful for
optimisations (it is called [memoization](https://en.wikipedia.org/wiki/Memoization)). Running 
same function (that
return the same value) over and over again can be inefficient.
This lightweight, dependency-free crate attempts solve this problem by caching
each return value. Only once, unless explicitly called to run
multiple times, it will be called.
This crate will probably receive regular updates from time to time.
Updates *may* or *may not* have new features in order to keep this
lightweight and simple.