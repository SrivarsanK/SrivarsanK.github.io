// Vercel Serverless Function: contact
use vercel_runtime::{run, service_fn, Error, Request, ResponseBody};
use http::{Response, StatusCode};
use http_body_util::BodyExt;
use shared::ContactForm;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header::ContentType, SinglePart};

mod email_template;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenvy::dotenv();
    run(service_fn(handler)).await
}

pub async fn handler(req: Request) -> Result<Response<ResponseBody>, Error> {
    // Handle CORS preflight
    if req.method() == "OPTIONS" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body(ResponseBody::from(""))?);
    }

    // Only allow POST
    if req.method() != "POST" {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Access-Control-Allow-Origin", "*")
            .body(ResponseBody::from("Method not allowed"))?);
    }

    // Deserialize JSON
    let body_bytes = req.into_body().collect().await?.to_bytes();
    let form: ContactForm = match serde_json::from_slice(&body_bytes) {
        Ok(f) => f,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Access-Control-Allow-Origin", "*")
                .body(ResponseBody::from(format!("Invalid JSON body: {}", e)))?);
        }
    };

    // Get SMTP credentials from environment variables
    let smtp_username = std::env::var("SMTP_USERNAME").unwrap_or_default();
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap_or_default();

    if smtp_username.is_empty() || smtp_password.is_empty() {
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Access-Control-Allow-Origin", "*")
            .body(ResponseBody::from("Missing SMTP credentials"))?);
    }

    // Replace spaces in the app password just in case
    let clean_password = smtp_password.replace(" ", "");

    let html_body = email_template::build_html_email(&form);

    let email = match Message::builder()
        .from(format!("Portfolio Contact <{}>", smtp_username).parse()?)
        .to(smtp_username.parse()?)
        .subject(format!("Portfolio Contact: {} - {}", form.name, form.subject))
        .singlepart(
            SinglePart::builder()
                .content_type(ContentType::TEXT_HTML)
                .body(html_body)
        ) {
            Ok(m) => m,
            Err(e) => {
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(ResponseBody::from(format!("Failed to build email: {}", e)))?);
            }
        };

    let creds = Credentials::new(smtp_username.clone(), clean_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .body(ResponseBody::from("Email sent successfully!"))?),
        Err(e) => {
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Access-Control-Allow-Origin", "*")
                .body(ResponseBody::from(format!("Could not send email: {:?}", e)))?)
        }
    }
}
