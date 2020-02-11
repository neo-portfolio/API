use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponseMeta {
    deleted: bool,
    id: u64,
    #[serde(rename = "type")]
    _type: String
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponseData<T> {
    row: Vec<T>,
    meta: Vec<QueryResponseMeta>
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponseFields<T> {
    columns: Vec<String>,
    data: Vec<QueryResponseData<T>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse<T> {
    results: Vec<QueryResponseFields<T>>
}