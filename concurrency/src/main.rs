extern crate concurrency;

fn main() {
    use concurrency::*;
    // Threads.
    threads::run();
    // Message passing.
    messages::run();
    // Shared (mutable) state.
    state::run();
}
