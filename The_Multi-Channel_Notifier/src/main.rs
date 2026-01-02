enum NotificationType {
    Email { sender: String, subject: String },
    SMS { provider: String },
    Push { device_token: String },
}

#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

struct NotificationBuilder {
    title: Option<String>,
    message: Option<String>,
    recipient: Option<String>,
    notification_type: Option<NotificationType>,
    priority: Option<Priority>,
}

impl NotificationBuilder {
    fn new(recipient: Option<String>, title: Option<String>, message: Option<String>) -> Self {
        Self {
            title,
            message,
            recipient,
            notification_type: None,
            priority: None,
        }
    }

    fn as_email(mut self, sender: String, subject: String) -> Self {
        self.notification_type = Some(NotificationType::Email { sender, subject });
        self
    }

    fn as_sms(mut self, provider: String) -> Self {
        self.notification_type = Some(NotificationType::SMS { provider });
        self
    }

    fn as_push(mut self, token: String) -> Self {
        self.notification_type = Some(NotificationType::Push {
            device_token: token,
        });
        self
    }

    fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    fn build(self) -> Result<NotificationBuilder, String> {
        let title = self.title.ok_or("Title is required");
        let message = self.message.ok_or("message is required")?;
        let recipient = self.recipient.ok_or("recipient is required")?;
        let notification_type = self
            .notification_type
            .ok_or("notification type is required")?;
        let priority = self.priority.ok_or("priority is required")?;
        let title = match title {
            Ok(val) => val,
            Err(e) => return Err(e.to_string()),
        };
        Ok(NotificationBuilder {
            title: Some(title),
            message: Some(message),
            recipient: Some(recipient),
            notification_type: Some(notification_type),
            priority: Some(priority),
        })
    }
}

fn process_notification(notification: NotificationBuilder) {
    match notification.notification_type {
        Some(NotificationType::Email { sender, subject }) => {
            println!(
                "Sending Email from {} with subject '{}' to {} Title: {}",
                sender,
                subject,
                notification.recipient.unwrap(),
                notification.title.unwrap()
            );
        }
        Some(NotificationType::SMS { provider }) => {
            println!(
                "Sending SMS via {} to {:?}",
                provider, notification.recipient
            );
        }
        Some(NotificationType::Push { device_token }) => {
            println!(
                "Sending Push Notification to device {} for {:?}",
                device_token, notification.recipient
            );
        }
        None => println!("Error"),
    }

    match notification.priority {
        Some(Priority::High) => {
            println!("[URGENT] Alerting all systems!");
        }
        Some(Priority::Medium) | Some(Priority::Low) => {
            println!("Queuing for standard delivery.");
        }
        None => println!("Error"),
    }
}

fn main() {
    let notification = NotificationBuilder::new(
        Some("srivardhan@gmail.com".to_string()),
        None,
        Some("The folowing meeting is for the team building for the new project".to_string()),
    )
    .as_email("Srivardhan".to_string(), "Meeting at 2'o clock".to_string())
    .priority(Priority::High)
    .build();

    match notification {
        Ok(o) => process_notification(o),
        Err(e) => println!("Failed to create order: {}", e),
    }
}
