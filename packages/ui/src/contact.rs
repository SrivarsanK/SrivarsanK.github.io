use dioxus::prelude::*;

#[component]
pub fn ContactForm() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut subject = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    
    let mut submission_status = use_signal(|| Option::<String>::None);
    let mut is_submitting = use_signal(|| false);
    let mut is_hovered = use_signal(|| false);

    let submit = move |_| async move {
        *is_submitting.write() = true;
        *submission_status.write() = None;
        
        // Basic email validation
        if !email().contains('@') || !email().contains('.') {
            *submission_status.write() = Some("Please enter a valid email address.".to_string());
            *is_submitting.write() = false;
            return;
        }

        let form_data = shared::ContactForm {
            name: name(),
            email: email(),
            subject: subject(),
            message: message(),
        };

        let client = reqwest::Client::new();
        let res = client.post("/api/contact")
            .json(&form_data)
            .send()
            .await;

        let result = match res {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    Err(text)
                }
            }
            Err(e) => Err(e.to_string()),
        };

        match result {
            Ok(_) => {
                *submission_status.write() = Some("Message sent successfully!".to_string());
                *name.write() = String::new();
                *email.write() = String::new();
                *subject.write() = String::new();
                *message.write() = String::new();
            }
            Err(e) => {
                *submission_status.write() = Some(format!("Failed to send: {}", e));
            }
        }
        *is_submitting.write() = false;
    };

    rsx! {
        div { 
            class: "my-2 font-mono leading-relaxed bg-secondary/30 p-4 border border-border rounded-lg max-w-2xl",
            onclick: move |e| e.stop_propagation(),
            div { class: "text-terminal-cyan font-bold mb-4", "┌ CONTACT ME ┐" }
            
            div { class: "flex flex-col sm:flex-row gap-6 mb-4",
                div { class: "flex-1",
                    label { class: "block text-sm text-foreground/80 mb-2", "Name" }
                    input {
                        class: "w-full bg-transparent border border-white/20 rounded-lg px-4 py-2 focus:outline-none focus:border-terminal-cyan transition-colors",
                        style: "color: inherit;",
                        placeholder: "Your name",
                        value: "{name()}",
                        oninput: move |e| *name.write() = e.value(),
                    }
                }
                div { class: "flex-1",
                    label { class: "block text-sm text-foreground/80 mb-2", "Email" }
                    input {
                        class: "w-full bg-transparent border border-white/20 rounded-lg px-4 py-2 focus:outline-none focus:border-terminal-cyan transition-colors",
                        style: "color: inherit;",
                        placeholder: "your@email.com",
                        value: "{email()}",
                        oninput: move |e| *email.write() = e.value(),
                    }
                }
            }
            
            div { class: "mb-4",
                label { class: "block text-sm text-foreground/80 mb-2", "Subject" }
                input {
                    class: "w-full bg-transparent border border-white/20 rounded-lg px-4 py-2 focus:outline-none focus:border-terminal-cyan transition-colors",
                    style: "color: inherit;",
                    placeholder: "What's this about?",
                    value: "{subject()}",
                    oninput: move |e| *subject.write() = e.value(),
                }
            }
            
            div { class: "mb-6",
                label { class: "block text-sm text-foreground/80 mb-2", "Message" }
                textarea {
                    class: "w-full bg-transparent border border-white/20 rounded-lg px-4 py-3 focus:outline-none focus:border-terminal-cyan min-h-[120px] transition-colors",
                    style: "color: inherit; resize: vertical;",
                    placeholder: "Tell us more about your message...",
                    value: "{message()}",
                    oninput: move |e| *message.write() = e.value(),
                }
            }
            
            // ReCAPTCHA Mockup
            div { class: "flex justify-center mb-6",
                div { 
                    class: "border border-border/50 rounded px-4 py-2 flex items-center justify-between w-[300px]",
                    style: "background: rgba(255,255,255,0.05); color: inherit;",
                    div { class: "flex items-center gap-3",
                        input { 
                            r#type: "checkbox", 
                            class: "w-6 h-6 rounded-sm bg-transparent border border-white/20",
                            style: "cursor: pointer;"
                        }
                        span { class: "text-sm", style: "color: inherit;", "I'm not a robot" }
                    }
                    div { class: "flex flex-col items-center",
                        img { src: "https://www.gstatic.com/recaptcha/api2/logo_48.png", class: "w-8 h-8 opacity-80" }
                        span { class: "text-[10px] text-foreground/60 mt-1", "reCAPTCHA" }
                    }
                }
            }
            
            if let Some(msg) = submission_status() {
                div { 
                    class: format!("mb-4 text-center text-sm font-bold {}", if msg.contains("success") { "text-green-400" } else { "text-red-400" }),
                    "{msg}"
                }
            }
            
            button {
                class: "w-full font-bold py-3 px-4 rounded-lg flex items-center justify-center gap-2 transition-all btn-press",
                style: format!(
                    "background-color: {}; color: #000; border: 1px solid {}; cursor: pointer;",
                    if is_hovered() { "var(--terminal-cyan)" } else { "var(--terminal-green)" },
                    if is_hovered() { "var(--terminal-cyan)" } else { "var(--terminal-green)" }
                ),
                disabled: "{is_submitting}",
                onclick: submit,
                onmouseenter: move |_| *is_hovered.write() = true,
                onmouseleave: move |_| *is_hovered.write() = false,
                if is_submitting() {
                    "Sending..."
                } else {
                    "Send Message \u{27A4}"
                }
            }
        }
    }
}
