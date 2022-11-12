//! 15.5: RefCell<T> and the interior mutability pattern
//!
//! Let's implement some kind of quota tracker: it will be instantiated with
//! some upper limit and some "messenger" instance such that a tracker object
//! can automatically calls the messenger object to send some message based on
//! the most recent value

/// Any type that implements the Messenger trait indicates that it can emit
/// some string message somewhere. The type is responsible for providing
/// implementation of how to send the message to its intended destination
pub trait Messenger {
    pub fn send(&self, msg: &str);
}

/// The QuotaTrakcer object will hold an immutable reference to a Messenger
/// object (instead of owning the Messenger object). Any reference to a
/// quota trakcer must be valid for as long as the reference to the messenger
/// is valid
pub struct QuotaTracker<'a, T: Messenger> {
    messenger: &'a T,
    level: usize,
    limit: usize,
}

impl<'a, T> QuotaTracker<'a, T> {
    pub fn new(m: &'a T, limit: usize) -> QuotaTracker {
        QuotaTracker{
            messenger: m,
            level: 0,
            limit,
        }
    }

    pub fn set(&mut self, level: usize) {
        self.level = level;
        let limit = self.limit;

        match level {
            0..limit => self.m.send("OK"),
            _ => self.m.send("ERROR"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn set_quota_level() {
    }
}

