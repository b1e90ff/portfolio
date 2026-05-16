use anyhow::{Context, Result, bail};
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncTransport, Message, Tokio1Executor};

use crate::config::SmtpSettings;

#[derive(Clone)]
pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Mailbox,
}

impl Mailer {
    pub fn new(settings: &SmtpSettings) -> Result<Self> {
        let creds = Credentials::new(settings.username.clone(), settings.password.clone());

        let builder = if settings.starttls {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&settings.host)
                .context("starttls relay")?
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&settings.host).context("smtps relay")?
        };

        let transport = builder.port(settings.port).credentials(creds).build();

        let from: Mailbox = settings.from.parse().context("parse SMTP_FROM mailbox")?;
        let to: Mailbox = settings.to.parse().context("parse SMTP_TO mailbox")?;

        Ok(Self {
            transport,
            from,
            to,
        })
    }

    pub async fn send_contact(
        &self,
        sender_name: &str,
        sender_email: &str,
        subject: &str,
        body: &str,
    ) -> Result<()> {
        let reply_to: Mailbox = format!("{sender_name} <{sender_email}>")
            .parse()
            .or_else(|_| sender_email.parse::<Mailbox>())
            .context("parse reply-to mailbox")?;

        let message = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .reply_to(reply_to)
            .subject(format!("Portfolio Contact: {subject}"))
            .header(ContentType::TEXT_PLAIN)
            .body(format!(
                "New Contact Request\n\nName: {sender_name}\nEmail: {sender_email}\nSubject: {subject}\n\nMessage:\n{body}\n\n--\nSent via the portfolio contact form.\n"
            ))
            .context("build message")?;

        match self.transport.send(message).await {
            Ok(_) => Ok(()),
            Err(err) => bail!("smtp send failed: {err}"),
        }
    }
}
