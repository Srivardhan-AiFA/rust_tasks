pub trait Notifier {
    fn send(&self, recipient: &str, message: &str) -> Result<(), String>;
}
