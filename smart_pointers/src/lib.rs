// This Rust code demonstrates the use of the `RefCell` type, which is a part of the Rust standard library's `std::cell` module.
// `RefCell` provides interior mutability, allowing for mutable borrows of data even within a structure that is considered immutable.
// In this code, we use `RefCell` within the `MockMessenger` struct to enable the recording of sent messages, which can be mutated
// within a method that is otherwise limited by Rust's borrowing rules.
//
// The code also defines a trait `Messenger`, a struct `LimitTracker` that uses a reference to an object implementing the trait,
// and a set of tests for the functionality.
//
// `RefCell` ensures runtime borrow checking, which means that if the borrowing rules are violated (e.g., attempting a mutable
// borrow while another borrow is in scope), it will panic at runtime. This allows for more flexible borrowing patterns when
// necessary but requires careful usage to prevent runtime panics.
//
// For example, `RefCell` is particularly useful in cases where you need to mutate data within a struct that is otherwise
// considered immutable by Rust's ownership system, as seen in the `MockMessenger` implementation.

// Define a trait named "Messenger" that represents an object capable of sending messages.
pub trait Messenger {
    // Define a method to send a message, taking a reference to a string.
    // send() takes an Immutabe Reference of self
    fn send(&self, msg: &str);
}

// Define a struct named "LimitTracker" that holds information about tracking a limit.
pub struct LimitTracker<'a, T: Messenger> {
    // A reference to an object implementing the "Messenger" trait.
    messenger: &'a T,
    // The current value being tracked.
    value: usize,
    // The maximum allowed value.
    max: usize,
}

// Implement methods for the "LimitTracker" struct.
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    // Constructor for creating a new "LimitTracker" instance.
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // Method to set the current value and send messages based on the percentage of the limit.
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        // Calculate the percentage of the current value relative to the maximum value.
        let percentage_of_max = self.value as f64 / self.max as f64;

        // Determine which message to send based on the percentage.

        // If the percentage exceeds or equals 100%, send an error message.
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
        // If the percentage exceeds or equals 90%, send an urgent warning message.
        else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        }
        // If the percentage exceeds or equals 75%, send a warning message.
        else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// Define tests for the code.
#[cfg(test)]
mod tests {
    // Import the symbols from the parent module.
    use super::*;
    // Import the "RefCell" type from the standard library.
    use std::cell::RefCell; // RefCell is used for interior mutability.

    // Define a struct named "MockMessenger" for testing purposes.
    struct MockMessenger {
        // A container to hold sent messages (using "RefCell" for interior mutability).
        sent_messages: RefCell<Vec<String>>, // RefCell allows mutable borrowing at runtime.
    }

    // Implement methods for the "MockMessenger" struct.
    impl MockMessenger {
        // Constructor for creating a new "MockMessenger" instance.
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]), // Initialize RefCell with an empty vector.
            }
        }
    }

    // Implement the "Messenger" trait for the "MockMessenger" struct.
    impl Messenger for MockMessenger {
        // Implementation of the "send" method to record sent messages.
        fn send(&self, message: &str) {
            // Borrow the sent_messages vector mutably, and push the message into it.
            self.sent_messages.borrow_mut().push(String::from(message));
            // Even though send() takes an immutable reference, we are able to modify it
            // as it is placed inside RefCell<T>, hence interior mutlability
        }
    }

    // Define a test that checks if the warning message is sent correctly.
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // Create a new instance of "MockMessenger" for testing.
        let mock_messenger = MockMessenger::new();
        // Create a new instance of "LimitTracker" with a maximum limit of 100.
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // Set the current value to 80, triggering a warning message.
        limit_tracker.set_value(80);

        // Assert that one message has been recorded in the "sent_messages" vector.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/* 

With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, 
these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error. 
With RefCell<T>, if you break these rules, your program will panic and exit.

-> borrow_mut() -> gives mutable borrow
-> borrow() -> gives immutable borrow

*/ 