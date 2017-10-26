// Interior Mutability Pattern:
// ----------------------------
// Interior mutability is a design pattern in Rust for allowing you to mutate data even when there
// are immutable references to that data, normally disallowed by the borrowing rules. To do so, the
// pattern uses unsafe code inside a data structure to bend Rust’s usual rules around mutation and
// borrowing.
// E.g. Rc<T> used in the rc module are not declared as mutable. However, we observe that their
// count goes up or down as new references are created or destroyed respectively. Same for mutexes.
// Or for vectors when we push more and more elements into the vector and it needs to mutate
// internal state.
// Refer:
// https://ricardomartins.cc/2016/06/08/interior-mutability,
// https://ricardomartins.cc/2016/06/25/interior-mutability-thread-safety, and
// https://ricardomartins.cc/2016/07/11/interior-mutability-behind-the-curtain.
//
// Cells and Refcells:
// -------------------
// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With
// RefCell<T>, these invariants are enforced at runtime. This is useful when the compiler is unable
// to guarantee the safety of a program which might look safe to us. Having these checks at runtime
// has a cost though.
//
// Box vs Rc vs RefCell:
// ---------------------
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> only allows immutable
// borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at
// runtime.
//
#![allow(unused_variables)]


mod mutref;

pub fn run() {
    use self::mutref;
    mutref::run();
}


// Listing 15-15.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send(
                "Warning: You've used up over 75% of your quota!",
            );
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send(
                "Urgent warning: You've used up over 90% of your quota!",
            );
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        // Immuable borrow.
        fn send(&self, message: &str) {
            // mutation.!!!
            let mut one_borrow = self.sent_messages.borrow_mut();
            one_borrow.push(String::from(message));
            // Doing this again while the previous borrow is in scope, will cause a panic!
            // let mut two_borrow = self.sent_messages.borrow_mut();
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
