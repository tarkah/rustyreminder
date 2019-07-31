use failure::Error;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;
use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::Email;
use native_tls::TlsConnector;
use rustyreminder::config::Smtp;
use rustyreminder::excel::Entry;

pub fn send_emails(config: Smtp, entries: Vec<Entry>) -> Result<(), Error> {
    let from = &config.username;
    let host = &config.host;
    let port = &config.port;
    let username = &config.username;
    let password = &config.password;

    let tls_builder = TlsConnector::builder();
    let tls_parameters = ClientTlsParameters::new(host.to_string(), tls_builder.build()?);

    let mut mailer = SmtpClient::new((&host[..], *port), ClientSecurity::Required(tls_parameters))?
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(username.to_string(), password.to_string()))
        .smtp_utf8(true)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    println!("Sending emails...");
    for entry in entries {
        let email = Email::builder()
            .to(from.to_string())
            .from(from.to_string())
            .subject("Reminder")
            .body(&entry.message)
            .build()?;

        // Send the email
        let result = mailer.send(email.into());
        result?;
        println!("Email sent: {:?}", &entry.message);
    }
    Ok(())
}
