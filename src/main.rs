#![allow(unused)]
use std::sync::Arc;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue,
        Method
    },
    routing::{post, get},
    Extension,
    Router
};

use config::Config;
use db::DBClient;
use dotenv::dotenv;
use handler::{
    create_capsule,
    get_all_capsules,
    get_capsule_by_public_id
};

use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

mod config;
mod dtos;
mod error;
mod db;
mod handler;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

fn main() {
    println!("Hello, world!");
}
