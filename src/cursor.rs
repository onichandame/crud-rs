use async_graphql::connection::CursorType;
use base64::{decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Cursor {
    pub offset: u64,
}

impl CursorType for Cursor {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn encode_cursor(&self) -> String {
        encode(serde_json::to_string(self).unwrap())
    }
    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(&decode(s)?)?)
    }
}
