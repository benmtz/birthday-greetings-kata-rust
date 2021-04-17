use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::transport::smtp::authentication::Credentials;

use crate::core::model::employee::Employee;
use crate::core::ports::send_greeting::SendGreeting;

use derive_builder::Builder;

#[derive(Debug, Builder, Clone)]
pub struct EmailGreetingAdapter {
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    from: String,
}

impl SendGreeting for EmailGreetingAdapter {

    fn to(&self, e: &Employee) -> anyhow::Result<()> {
        
        let email = Message::builder()
            .from(self.from.parse().unwrap())
            .to(format!("{} {} <{}>",e.first_name, e.last_name, e.email).parse().unwrap())
            .subject("Happy birthday !")
            .body(format!("Happy birthday, dear {}", e.first_name))
            .unwrap();

        let creds = Credentials::new(
            self.smtp_username.to_string(),
            self.smtp_password.to_string()
        );

        let mailer = SmtpTransport::builder_dangerous(&self.smtp_server)
            .port(self.smtp_port)
            .credentials(creds)
            .build();
        
        // FIXME proper error handling
        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::Error::new(e))
       }
    }
}
