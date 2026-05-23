use shared::ContactForm;

/// Builds a beautiful HTML email body from a ContactForm submission.
pub fn build_html_email(form: &ContactForm) -> String {
    let escaped_name = html_escape(&form.name);
    let escaped_email = html_escape(&form.email);
    let escaped_subject = html_escape(&form.subject);
    let escaped_message = html_escape(&form.message).replace('\n', "<br>");

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body style="margin:0;padding:0;background-color:#0f172a;font-family:'Segoe UI',Roboto,'Helvetica Neue',Arial,sans-serif;">
  <table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="background-color:#0f172a;padding:32px 16px;">
    <tr>
      <td align="center">
        <table role="presentation" width="600" cellpadding="0" cellspacing="0" style="max-width:600px;width:100%;">

          <!-- Header -->
          <tr>
            <td style="background:linear-gradient(135deg,#06b6d4,#8b5cf6);padding:32px 40px;border-radius:16px 16px 0 0;">
              <table role="presentation" width="100%" cellpadding="0" cellspacing="0">
                <tr>
                  <td>
                    <span style="font-size:14px;color:rgba(255,255,255,0.7);letter-spacing:2px;text-transform:uppercase;">New Message</span>
                    <h1 style="margin:8px 0 0;font-size:24px;font-weight:700;color:#ffffff;">Portfolio Contact</h1>
                  </td>
                  <td align="right" valign="top">
                    <div style="width:48px;height:48px;background:rgba(255,255,255,0.15);border-radius:12px;text-align:center;line-height:48px;font-size:24px;">
                      ✉
                    </div>
                  </td>
                </tr>
              </table>
            </td>
          </tr>

          <!-- Body -->
          <tr>
            <td style="background-color:#1e293b;padding:0;">

              <!-- Sender Info Card -->
              <table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="border-bottom:1px solid rgba(148,163,184,0.15);">
                <tr>
                  <td style="padding:28px 40px;">
                    <table role="presentation" width="100%" cellpadding="0" cellspacing="0">
                      <tr>
                        <!-- Avatar -->
                        <td width="52" valign="top">
                          <div style="width:48px;height:48px;background:linear-gradient(135deg,#06b6d4,#8b5cf6);border-radius:50%;text-align:center;line-height:48px;font-size:20px;font-weight:700;color:#ffffff;">
                            {initial}
                          </div>
                        </td>
                        <td style="padding-left:16px;">
                          <div style="font-size:18px;font-weight:600;color:#f1f5f9;margin-bottom:4px;">{name}</div>
                          <a href="mailto:{email}" style="font-size:14px;color:#06b6d4;text-decoration:none;">{email}</a>
                        </td>
                      </tr>
                    </table>
                  </td>
                </tr>
              </table>

              <!-- Subject -->
              <table role="presentation" width="100%" cellpadding="0" cellspacing="0" style="border-bottom:1px solid rgba(148,163,184,0.15);">
                <tr>
                  <td style="padding:20px 40px;">
                    <span style="font-size:11px;color:#94a3b8;text-transform:uppercase;letter-spacing:1.5px;font-weight:600;">Subject</span>
                    <div style="font-size:16px;color:#e2e8f0;margin-top:6px;font-weight:500;">{subject}</div>
                  </td>
                </tr>
              </table>

              <!-- Message -->
              <table role="presentation" width="100%" cellpadding="0" cellspacing="0">
                <tr>
                  <td style="padding:28px 40px 36px;">
                    <span style="font-size:11px;color:#94a3b8;text-transform:uppercase;letter-spacing:1.5px;font-weight:600;">Message</span>
                    <div style="margin-top:12px;padding:20px 24px;background-color:rgba(15,23,42,0.5);border-radius:12px;border-left:3px solid #06b6d4;">
                      <p style="margin:0;font-size:15px;line-height:1.7;color:#cbd5e1;">{message}</p>
                    </div>
                  </td>
                </tr>
              </table>

              <!-- Reply Button -->
              <table role="presentation" width="100%" cellpadding="0" cellspacing="0">
                <tr>
                  <td align="center" style="padding:0 40px 36px;">
                    <a href="mailto:{email}?subject=Re: {subject}" style="display:inline-block;padding:14px 36px;background:linear-gradient(135deg,#06b6d4,#8b5cf6);color:#ffffff;font-size:15px;font-weight:600;text-decoration:none;border-radius:10px;letter-spacing:0.5px;">
                      Reply to {name} →
                    </a>
                  </td>
                </tr>
              </table>

            </td>
          </tr>

          <!-- Footer -->
          <tr>
            <td style="background-color:#0f172a;padding:24px 40px;border-top:1px solid rgba(148,163,184,0.1);border-radius:0 0 16px 16px;">
              <table role="presentation" width="100%" cellpadding="0" cellspacing="0">
                <tr>
                  <td>
                    <span style="font-size:12px;color:#475569;">Sent from your portfolio contact form</span>
                  </td>
                  <td align="right">
                    <span style="font-size:12px;color:#475569;">srivarsankannan.com</span>
                  </td>
                </tr>
              </table>
            </td>
          </tr>

        </table>
      </td>
    </tr>
  </table>
</body>
</html>"##,
        initial = escaped_name.chars().next().unwrap_or('?').to_uppercase(),
        name = escaped_name,
        email = escaped_email,
        subject = escaped_subject,
        message = escaped_message,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
