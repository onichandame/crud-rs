use async_graphql::{connection::CursorType, InputObject};
use base64::{decode, encode};
use sea_orm::QuerySelect;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Cursor {
    pub offset: u64,
}

#[derive(InputObject, Default)]
pub struct Pagination {
    pub first: Option<u64>,
    pub after: Option<String>,
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

impl Pagination {
    pub fn apply_pagination<T: QuerySelect>(
        &self,
        query: T,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        let query = match self.first {
            Some(v) => query.limit(v),
            None => query,
        };
        let query = match self.get_cursor()? {
            Some(v) => query.offset(v.offset),
            None => query,
        };
        Ok(query)
    }
    pub fn has_prev(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.get_cursor()?.map_or(false, |v| v.offset > 0))
    }

    pub fn has_next(
        &self,
        total_count: u64,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(match self.first {
            Some(limit) => self.get_cursor()?.map_or(0, |v| v.offset) + limit < total_count,
            None => false,
        })
    }

    fn get_cursor(&self) -> Result<Option<Cursor>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(match &self.after {
            Some(v) => Some(Cursor::decode_cursor(v)?),
            None => None,
        })
    }
}
