#![allow(unused)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{query_as, Error, Pool, Postgres};

use crate::dtos::Capsule;

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}