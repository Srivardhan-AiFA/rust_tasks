use crate::notifier::Notifier;

pub struct NotificationService<T: Notifier> {
    notifier: T,
    max_retries: u8,
}

impl<T: Notifier> NotificationService<T> {
    pub fn new(notifier: T) -> Self {
        Self {
            notifier,
            max_retries: 2,
        }
    }

    pub fn notify(&self, recipient: &str, message: &str) -> Result<String, String> {
        let mut attempts = 0;

        while attempts <= self.max_retries {
            match self.notifier.send(recipient, message) {
                Ok(()) => {
                    return Ok("Sent successfully".to_string());
                }
                Err(err) => {
                    attempts += 1;

                    if attempts > self.max_retries {
                        return Err(format!(
                            "Failed after {} retries: {}",
                            self.max_retries, err
                        ));
                    }
                }
            }
        }

        Err("Unexpected failure".to_string())
    }
}
