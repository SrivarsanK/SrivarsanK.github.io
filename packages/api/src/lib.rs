//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[post("/api/contact")]
pub async fn send_contact_email(form: ContactForm) -> Result<String, ServerFnError> {
    let api_key = &std::env::var("RESEND_API_KEY").unwrap_or_default();
    
    let client = reqwest::Client::new();
    let resend_req = serde_json::json!({
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
        .await
        .map_err(|e| ServerFnError::new(format!("Request failed: {}", e)))?;

    if response.status().is_success() {
        Ok("Email sent successfully!".to_string())
    } else {
        let err_text = response.text().await.unwrap_or_default();
        Err(ServerFnError::new(format!("Resend API error: {}", err_text)))
    }
}
