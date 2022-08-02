use serde::de::DeserializeOwned;

use super::schema::Schema;

pub async fn request<TResponse>(schema: &Schema, query: &str) -> TResponse
where
    TResponse: DeserializeOwned,
{
    let response = schema.execute(query).await;
    serde_json::from_value::<TResponse>(response.data.into_json().unwrap()).unwrap()
}
