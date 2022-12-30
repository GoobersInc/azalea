mod auth;
pub mod cache;
pub mod game_profile;
pub mod sessionserver;

use std::net::SocketAddr;
pub use auth::*;

#[derive(Debug, Clone)]
pub struct Proxy {
    pub address: SocketAddr,
    pub username: Option<String>,
    pub password: Option<String>,
}