use super::jwt::encode_token;
use chrono::{Duration, Utc};
use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub async fn send_mail(
    email: &str,
    user_id: i32,
    url: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let exp: usize = (Utc::now() + Duration::days(7)).timestamp() as usize;
    let token = encode_token(user_id, exp).await;
    // Define the HTML content
    let html_content = format!(
        r#"
        <html>
            <body>
                <h1>Hello!</h1>
                <p>To verificate your account click to reference {}/{}</p>
                <p>This is a <strong>test email</strong> from Rust!</p>
            </body>
        </html>
    "#,
        url, token
    );

    let from_email = "gradprus.manager@gmail.com".parse::<Mailbox>().unwrap();
    let to_email = email.parse::<Mailbox>().unwrap();

    // Define the email with HTML part
    let email = Message::builder()
        .from(from_email)
        .to(to_email)
        .subject("Rust Email")
        .multipart(MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())))
        .unwrap();

    // Set up the SMTP client credentials
    let creds = Credentials::new(
        "gradprus.manager@gmail.com".to_string(),
        "hzxijrovvyvjgciu".to_string(),
    );

    // Open a remote connection to the SMTP server with STARTTLS
    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}
