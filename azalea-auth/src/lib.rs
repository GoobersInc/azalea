mod auth;
pub mod cache;
pub mod game_profile;
pub mod sessionserver;

pub use auth::*;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Proxy {
    pub address: SocketAddr,
    pub username: Option<String>,
    pub password: Option<String>,
}
