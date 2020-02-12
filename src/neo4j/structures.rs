use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct QueryResponseMeta {
    deleted: bool,
    id: u64,
    #[serde(rename = "type")]
    _type: String
}

#[derive(Serialize, Deserialize)]
pub struct QueryResponseData<T> {
    row: Vec<T>,
    meta: Vec<QueryResponseMeta>
}

#[derive(Serialize, Deserialize)]
struct QueryResponseFields<T> {
    columns: Vec<String>,
    data: Vec<QueryResponseData<T>>
}

#[derive(Serialize, Deserialize)]
pub struct QueryResponse<T> {
    results: Vec<QueryResponseFields<T>>
}

impl<T> QueryResponse<T> {
    pub fn data(&self) -> &Vec<QueryResponseData<T>> {
        &self.results[0].data
    }
}