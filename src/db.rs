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

#[async_trait]
pub trait TableExt {
    async fn create_capsule (
        &self,
        name: &str,
        email: &str,
        title: &str,
        message: &str,
        unlock_at: DateTime<Utc>,
        public_id: &str
    ) -> Result<Capsule, Error>;

    async fn get_all_capsules(&self) -> Result<Vec<Capsule>, Error>;