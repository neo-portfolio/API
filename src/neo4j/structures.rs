use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponseMeta {
    deleted: bool,
    id: u64,
    #[serde(rename = "type")]
    _type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponseData<T> {
    row: Vec<T>,
    #[serde(skip)]
    meta: Vec<QueryResponseMeta>
}

impl<T> QueryResponseData<T> {
    pub fn row(&self) -> &Vec<T> {
        &self.row
    }
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

impl<T> QueryResponse<T> {
    pub fn data(&self) -> &Vec<QueryResponseData<T>> {
        &self.results[0].data
    }
}