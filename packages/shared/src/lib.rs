//! This crate contains all shared types.

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}
