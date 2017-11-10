Rust's concurrency model doesn't require the language to know much about things
like mutexes or message passing. This allows one to build their own concurrency
primitives.


The language itself cares about only 2 traits when thinking about concurrency.
Both are in std::marker.
1. The *Send* trait
- indicates that ownership of that type may be transferred between threads.
- most types implement Send by default. Rc does not.
- Any type that is composed entirely of Send types is automatically marked as
  Send as well.

2. *Sync* trait
- indicates that access from multiple threads is safe.
