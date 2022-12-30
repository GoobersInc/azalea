//! Tell Mojang you're joining a multiplayer server.
//!
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;
use crate::Proxy;

#[derive(Debug, Error)]
pub enum SessionServerError {
    #[error("Error sending HTTP request to sessionserver: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Multiplayer is not enabled for this account")]
    MultiplayerDisabled,
    #[error("This account has been banned from multiplayer")]
    Banned,
    #[error("Unknown sessionserver error: {0}")]
    Unknown(String),
    #[error("Unexpected response from sessionserver (status code {status_code}): {body}")]
    UnexpectedResponse { status_code: u16, body: String },
}

#[derive(Deserialize)]
pub struct ForbiddenError {
    pub error: String,
    pub path: String,
}

/// Tell Mojang's servers that you are going to join a multiplayer server,
/// which is required to join online-mode servers. The server ID is an empty
/// string.
pub async fn join(
    access_token: &str,
    public_key: &[u8],
    private_key: &[u8],
    uuid: &Uuid,
    server_id: &str,
    proxy: Option<Proxy>,
) -> Result<(), SessionServerError> {

    let mut client = reqwest::Client::new();

    if proxy.is_some() {
        let unwrapped_proxy = proxy.unwrap();
        let mut client_proxy = reqwest::Proxy::all(format!("socks5://{host}:{port}", host = unwrapped_proxy.address.ip(), port = unwrapped_proxy.address.port()))?;

        if unwrapped_proxy.username.is_some() && unwrapped_proxy.password.is_some() {
            client_proxy = client_proxy.clone().basic_auth(&unwrapped_proxy.username.unwrap(), &unwrapped_proxy.password.unwrap());
        }

        client = reqwest::Client::builder()
            .proxy(client_proxy)
            .build()
            .unwrap();
    }

    let server_hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
        server_id.as_bytes(),
        public_key,
        private_key,
    ));

    let mut encode_buffer = Uuid::encode_buffer();
    let undashed_uuid = uuid.simple().encode_lower(&mut encode_buffer);

    let data = json!({
        "accessToken": access_token,
        "selectedProfile": undashed_uuid,
        "serverId": server_hash
    });
    let res = client
        .post("https://sessionserver.mojang.com/session/minecraft/join")
        .json(&data)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::NO_CONTENT => Ok(()),
        reqwest::StatusCode::FORBIDDEN => {
            let forbidden = res.json::<ForbiddenError>().await?;
            match forbidden.error.as_str() {
                "InsufficientPrivilegesException" => Err(SessionServerError::MultiplayerDisabled),
                "UserBannedException" => Err(SessionServerError::Banned),
                _ => Err(SessionServerError::Unknown(forbidden.error)),
            }
        }
        status_code => {
            let body = res.text().await?;
            Err(SessionServerError::UnexpectedResponse {
                status_code: status_code.as_u16(),
                body,
            })
        }
    }
}
