//! 15.5 RefCell<T> and the interior mutability pattern
//! Let's build some kind of quota tracker. The tracker will be instantiated
//! with some kind of mechanism to emit events when usage level changes. The
//! behavior of the mechanism is abstracted into a trait, but the impl will
//! be left to the developers

/// Specifies the interface of the "event emission" mechanism
/// A messenger is expected to be **stateless**!. Hence calling self.emit
/// should not mutate the instance.
pub trait Messenger {
    fn emit(&self, msg: &str);
}

/// The tracker struct itself
pub struct QuotaTracker<'a, T: Messenger> {
    level: usize,
    limit: usize,
    messenger: &'a T,
}

impl<'a, T: Messenger> QuotaTracker<'a, T> {
    /// Instantiate a new tracker given the input limit and messenger object
    pub fn new(limit: usize, messenger: &'a T) -> Self {
        QuotaTracker{ level: 0, limit, messenger }
    }

    /// Set the level and emit the appropriate message
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

    /// A mock messenger for testing purposes. All calls to emit will clone
    /// the message and push it onto a Vector. However, the vector cannot be
    /// owned by the messenger since the messenger cannot mutate itself.
    /// Instead, the messenger will hold *some mutable reference to the
    /// vector*. This is where RefCell<T> and the "interior mutatbility
    /// pattern" comes in.
    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            let messages = RefCell::new(Vec::new());
            return MockMessenger{ messages };
        }
    }

    impl Messenger for MockMessenger {
        fn emit(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn set_quota_levels() {
        let messenger = MockMessenger::new();
        let mut tracker = QuotaTracker::new(10, &messenger);
        tracker.set_level(0);
        tracker.set_level(10);
        tracker.set_level(100);

        let messages_ref = messenger.messages.borrow();
        assert_eq!(messages_ref.get(0), Some(&"OK".to_string()));
        assert_eq!(messages_ref.get(1), Some(&"OK".to_string()));
        assert_eq!(messages_ref.get(2), Some(&"ERROR".to_string()));
    }
}
