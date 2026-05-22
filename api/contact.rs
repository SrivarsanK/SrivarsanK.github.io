use vercel_runtime::{run, service_fn, Error, Request, ResponseBody};
use http::{Response, StatusCode};
use http_body_util::BodyExt;
use shared::ContactForm;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

pub async fn handler(req: Request) -> Result<Response<ResponseBody>, Error> {
    // Only allow POST
    if req.method() != "POST" {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(ResponseBody::from("Method not allowed"))?);
    }

    // Deserialize JSON
    let body_bytes = req.into_body().collect().await?.to_bytes();
    let form: ContactForm = match serde_json::from_slice(&body_bytes) {
        Ok(f) => f,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(ResponseBody::from(format!("Invalid JSON body: {}", e)))?);
        }
    };

    // Use RESEND_API_KEY from environment variables
    let api_key = match std::env::var("RESEND_API_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(ResponseBody::from("Missing RESEND_API_KEY environment variable"))?);
        }
    };

    let client = reqwest::Client::new();
    let resend_req = json!({
        "from": "Acme <onboarding@resend.dev>",
        "to": ["srivarsankannan@gmail.com"],
        "subject": format!("Portfolio Contact: {} - {}", form.name, form.subject),
        "html": format!(
            "<p><strong>Name:</strong> {}</p><p><strong>Email:</strong> {}</p><p><strong>Message:</strong></p><p>{}</p>",
            form.name, form.email, form.message
        )
    });

    let response = client.post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&resend_req)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .body(ResponseBody::from("Email sent successfully!"))?)
            } else {
                let err_text = res.text().await.unwrap_or_default();
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(ResponseBody::from(format!("Resend API error: {}", err_text)))?)
            }
        }
        Err(e) => {
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(ResponseBody::from(format!("Failed to send request: {}", e)))?)
        }
    }
}
