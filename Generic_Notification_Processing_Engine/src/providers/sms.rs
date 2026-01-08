use crate::notifier::Notifier;

pub struct SMSNotifier;

impl Notifier for SMSNotifier {
    fn send(&self, recipient: &str, message: &str) -> Result<(), String> {
        println!("To: '{}' Message: '{}'", recipient, message);
        Ok(())
    }
}
