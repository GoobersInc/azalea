use azalea_buf::McBuf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(McBuf, Debug, Clone)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub name: String,
    pub properties: HashMap<String, ProfilePropertyValue>,
}

impl GameProfile {
    pub fn new(uuid: Uuid, name: String) -> Self {
        GameProfile {
            uuid,
            name,
            properties: HashMap::new(),
        }
    }
}

#[derive(McBuf, Debug, Clone, Deserialize, Serialize)]
pub struct ProfilePropertyValue {
    pub value: String,
    pub signature: Option<String>,
}
