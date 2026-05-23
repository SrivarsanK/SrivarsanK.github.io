use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    let _ = dotenvy::from_filename("../.env");
    
    let smtp_username = std::env::var("SMTP_USERNAME").unwrap_or_else(|_| "srivarsankannan@gmail.com".to_string());
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "ipjg lrcd dmxl ylsz".to_string());
    let clean_password = smtp_password.replace(" ", "");

    let email = Message::builder()
        .from(format!("Portfolio Contact <{}>", smtp_username).parse().unwrap())
        .to(smtp_username.parse().unwrap())
        .subject("Test Email from Portfolio Setup")
        .body("This is a test email sent using the configured Gmail SMTP credentials.\n\nEverything is working perfectly!".to_string())
        .unwrap();

    let creds = Credentials::new(smtp_username.clone(), clean_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    println!("Sending test email...");
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}
