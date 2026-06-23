#![allow(unused)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{query_as, Error, Pool, Postgres};

use crate::dtos::Capsule;
