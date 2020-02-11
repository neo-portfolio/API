use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    alpha: f64,
    beta: f64,
    expected_returns: f64,
    sd: f64,
    symbol: String
}