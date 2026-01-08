mod notifier;
mod providers;
mod service;

use providers::email::EmailNotifier;
use providers::push::PushNotifier;
use providers::sms::SMSNotifier;
use service::NotificationService;
fn main() {
    let email_service = NotificationService::new(EmailNotifier);
    let is_email_send_sucessfully = email_service.notify(
        "srivardhan@gmail.com",
        "This is an reminder email about the meeting we have on 9th of Friday",
    );
    let email_status = match is_email_send_sucessfully {
        Ok(s) => {
            format!("{}", s)
        }
        Err(e) => format!("{}", e),
    };
    println!("{}", email_status);

    let sms_service = NotificationService::new(SMSNotifier);
    let is_sms_send_sucessfully = sms_service.notify("9988773293", "OTP sent sucessfully");
    let sms_status = match is_sms_send_sucessfully {
        Ok(s) => {
            format!("{}", s)
        }
        Err(e) => format!("{}", e),
    };
    println!("{}", sms_status);

    let push_service = NotificationService::new(PushNotifier);
    let is_push_send_sucessfully = push_service.notify(
        "Srivardhan",
        "How are you sending the response to the open search",
    );
    let push_staus = match is_push_send_sucessfully {
        Ok(s) => format!("{}", s),
        Err(e) => format!("{}", e),
    };
    println!("{}", push_staus)
}
