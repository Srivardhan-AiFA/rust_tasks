use crate::notifier::Notifier;

pub struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn send(&self, recipient: &str, message: &str) -> Result<(), String> {
        println!("To: '{}' Message: '{}'", recipient, message);
        Err("The Email Service is DOWN!".to_string())
        // Ok(())
    }
}
