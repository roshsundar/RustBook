/*
To demonstrate RefCell<T> we have a scenario.

The LimitTracker tracks a value and sends messages based on how close it is to a maximum.
i.e. Data limit for a phone.

Applications that use this would impl the Messenger trait to actually send the message how they desire.
i.e. Email, text, etc.
*/

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where
    T: Messenger
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        LimitTracker { 
            messenger, 
            value: 0, 
            max 
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You've used up 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/*
For the test, we need to create a mock messenger to test that the correct message is sent out.
Interior mutability is often useful in these cases.
*/

#[cfg(test)]
mod incorrect_test {
    use super::*;

    // Record the messages sent out through the mock messenger
    struct MockMessenger {
        sent_messages: Vec<String>
    }
    
    // Create a new mock messenger
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { 
                sent_messages: vec![] 
            }
        }
    }

    // Attempt to impl the Messenger trait to make the mock messenger an actual Messenger
    impl Messenger for MockMessenger{
        fn send(&self, msg: &str) {
            // self.sent_messages.push(String::from(msg)); // !✕ err: &self is an immutable referece, can't mutate its data
            // The above could be fixed by changing the trait to use fn send(&mut self, msg: &str);
            // But we don't want to change the trait just for testing.
            // This is where RefCell<T> would help.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);
        
        // Set the value above 75% of the max
        limit_tracker.set_value(80);

        // The mock messenger should've logged the appropriate message

        assert_eq!(mock_messager.sent_messages[0], "Warning: You've used up over 75% of your quota!")
    }
}


#[cfg(test)]
mod correct_test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // Wrap the list in a RefCell<T>
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { 
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, msg: &str) {
            // borrow_mut() gets a mutable reference to the interior value of the RefCell<T>, Vec<String> in this case.
            // This allows us to modify it by adding the generated msg, even though we only have an immutable reference to the mock messenger
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        // borrow() to get the Vec<String>
        assert_eq!(mock_messager.sent_messages.borrow()[0], "Warning: You've used up over 75% of your quota!")
    }
}