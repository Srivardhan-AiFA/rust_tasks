use crate::notifier::Notifier;

pub struct PushNotifier;

impl Notifier for PushNotifier {
    fn send(&self, recipient: &str, message: &str) -> Result<(), String> {
        println!("To: '{}' Message: '{}'", recipient, message);
        Ok(())
    }
}
