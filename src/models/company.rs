use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    alpha: f64,
    beta: f64,
    expected_returns: f64,
    pub sd: f64,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompanyOrCorrelation {
    Company(Company),
    Correlation(f64),
}

impl CompanyOrCorrelation {
    pub fn as_company(&self) -> &Company {
        match self {
            CompanyOrCorrelation::Company(val) => val,
            _ => panic!("Not a string")
        }
    }

    pub fn as_double(&self) -> &f64 {
        match self {
            CompanyOrCorrelation::Correlation(val) => val,
            _ => panic!("Not an double")
        }
    }
}