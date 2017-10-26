pub fn run() {
    iterate();
}

// Iterators in rust are lazy, i.e. they no effect until we call methods on them.
fn iterate() {
    let v1:Vec<i32> = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got {}", val);
    }
}

// Iterators have to implement the Iterator trait defined in the standard library.
/*
trait Iterator {
    type Item;
    // self is borrowed mutably becausing calling 'next' on an iterator changes the iterator's
    // state.
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementation...
}
*/

#[test]
fn iterator_demo() {
    let v1:Vec<i32> = vec![1,2,3];
    // This was not required in the for loop because for takes ownership of the iterator and
    // mutates it behind the scenes.
    let mut v1_iter = v1.iter();
    // iter() returns iterator over immutable references. into_iter() creates an iterator that
    // takes ownership of the v1 and returns owned values. iter_mut() iterates over mutable
    // references.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
// Consuming adaptors.
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // The sum method on an iterator is a "consuming adaptor", because it takes ownership of the
    // adaptor itself.
    let total: i32 = v1_iter.sum();
    // We are not allowed to use v1_iter after this.
    assert_eq!(total, 6);
}

// Iterator adaptors. allow us to change iterators into different kind of iterators. We can chain
// multiple calls to iterator adaptors. Because all iterators are lazy, however, we have to call one
// of the consuming adaptor methods in order to get results from calls to iterator adaptors.
#[test]
fn map_sum() {
    let v1 = vec![1,2,3];
    // doesn't really do anything and produces a compiler warning.
    v1.iter().map(|x| x + 1);
    // consuming values: Collect consumes an iterator into a collection specified by the type of
    // LHS.
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);
}

// Custom iterator which counts from 1 to 5.
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count +=1 ;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}


// Now that we have an iterator for Counter, we can use other iterator methods, such as skip() or
// zip on it.
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
