#![allow(unused)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone)]
pub struct Capsule {
    pub id: Uuid,
    pub public_id: String,
    pub name: String,
    pub email: String,
    pub title: String,
    pub message: String,
    pub unlock_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub is_unlocked: Option<bool>,
    pub email_sent: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCapsuleRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Invalid Email format"))]
    pub email: String,