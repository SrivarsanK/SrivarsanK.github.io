// Local development server with CORS support
// Run with: cargo run --bin dev_server
use axum::{Router, Json, http::StatusCode, routing::post};
use tower_http::cors::{CorsLayer, Any};
use shared::ContactForm;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header::ContentType, SinglePart};

mod email_template;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/contact", post(handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Dev API server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Json(form): Json<ContactForm>) -> Result<String, (StatusCode, String)> {
    let smtp_username = std::env::var("SMTP_USERNAME").unwrap_or_default();
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap_or_default();

    if smtp_username.is_empty() || smtp_password.is_empty() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Missing SMTP credentials".to_string()));
    }

    let clean_password = smtp_password.replace(" ", "");

    let html_body = email_template::build_html_email(&form);

    let email = Message::builder()
        .from(format!("Portfolio Contact <{}>", smtp_username).parse().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid from address: {}", e)))?)
        .to(smtp_username.parse().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Invalid to address: {}", e)))?)
        .subject(format!("Portfolio Contact: {} - {}", form.name, form.subject))
        .singlepart(
            SinglePart::builder()
                .content_type(ContentType::TEXT_HTML)
                .body(html_body)
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to build email: {}", e)))?;

    let creds = Credentials::new(smtp_username.clone(), clean_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok("Email sent successfully!".to_string()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Could not send email: {:?}", e))),
    }
}
