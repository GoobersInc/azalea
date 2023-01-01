mod auth;
pub mod cache;
pub mod game_profile;
pub mod sessionserver;

pub use auth::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub address: SocketAddr,
    pub username: Option<String>,
    pub password: Option<String>,
}
