#![allow(unused)]
use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap()
        )
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(
        message: impl Into<String>,
        status: StatusCode
    ) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(
        message: impl Into<String>
    ) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(
        message: impl Into<String>
    ) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unique_constraint_violation(
        message: impl Into<String>
    ) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }