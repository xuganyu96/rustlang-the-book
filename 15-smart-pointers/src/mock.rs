//! # Mock Objects
//! Implement a mock struct that works around the constraint that the trait
//! imposes.

pub trait Messenger {
    /// All messengers need to implement this method for sending messages.
    /// This trait requires that messengers are stateless, hence the immutable
    /// reference to self.
    fn emit(&self, msg: &str);
}

/// Each quota tracker needs to carry a reference to a messenger, hence the
/// lifetime notation: the tracker requires reference to the messenger to be
/// valid for as long as the tracker is valid
pub struct QuotaTracker<'a, T: Messenger> {
    pub limit: usize,
    pub level: usize,
    pub messenger: &'a T,
}

impl<'a, T: Messenger> QuotaTracker<'a, T> {
    /// Instantiated a new instance with level set to 0
    pub fn new(limit: usize, m: &'a T) -> Self {
        return QuotaTracker{
            limit,
            level: 0,
            messenger: m,
        };
    }

    /// Emit message as new level is set.
    pub fn set_level(&mut self, level: usize) {
        self.level = level;

        if level <= self.limit {
            self.messenger.emit("OK");
        } else {
            self.messenger.emit("ERROR");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;

    /// # Mock Messenger
    /// We want a mock messenger that doesn't send messages, but stores them
    /// in a vector, but the mock messenger cannot own the vector or a mutable
    /// reference to such vector because "emit" only takes immutable reference
    /// to the messenger.
    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        /// instantiate a new mock messenger with empty message
        fn new() -> Self {
            return MockMessenger {
                messages: RefCell::new(vec![]),
            };
        }
    }

    impl Messenger for MockMessenger {
        fn emit(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn test_mock_messenger() {
        let messenger = MockMessenger::new();
        let mut tracker = QuotaTracker::new(10, &messenger);
        tracker.set_level(1);
        tracker.set_level(10);
        tracker.set_level(100);

        assert_eq!(messenger.messages.borrow().get(0).unwrap(), "OK");
        assert_eq!(messenger.messages.borrow().get(1).unwrap(), "OK");
        assert_eq!(messenger.messages.borrow().get(2).unwrap(), "ERROR");
    }
}
