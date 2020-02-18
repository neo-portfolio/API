use std::collections::HashMap;

use nalgebra::DMatrix;
use serde::Serialize;

use crate::models::company::{Company, CompanyOrCorrelation};
use crate::models::queries;
use crate::neo4j::structures::{QueryResponse, QueryResponseData};

#[derive(Serialize)]
pub struct Weight {
    symbol: String,
    weight: f64,
}

fn find_or_add(indexes: &mut HashMap<String, usize>, symbol: &String, last_index: &mut usize) -> usize {
    match indexes.get(symbol.as_str()) {
        Some(index) => index.clone(),
        None => {
            indexes.insert(symbol.clone(), last_index.clone());
            let temp = last_index.clone();
            *last_index += 1;
            temp
        }
    }
}


fn dimensions(resp: &Vec<QueryResponseData<CompanyOrCorrelation>>) -> (usize, usize) {
    // n*(n+1) = len + n
    // n**2 - 2*n - len = 0
    // Quadratic -> (2 + sqrt(4 + 4 * len)) / 2 = n
    let len: usize = resp.len();
    let delta = ((4 + 4 * len) as f64).sqrt();
    let dim: usize = ((delta + 2f64) / 2f64).floor() as usize;
    let vec_dim = len + dim;
    assert_eq!(dim * dim, vec_dim);
    (dim, vec_dim)
}

fn response_to_matrix(resp: &Vec<QueryResponseData<CompanyOrCorrelation>>) -> (DMatrix<f64>, DMatrix<f64>, Vec<String>) {
    let mut indexes: HashMap<String, usize> = HashMap::new();
    let mut last_index: usize = 0;
    let (dim, vec_dim) = dimensions(&resp);
    let mut vec: Vec<f64> = Vec::with_capacity(vec_dim);
    vec.resize(vec_dim, 0f64);
    let mut last: String = string!("");
    let mut mu_vector: Vec<f64> = Vec::with_capacity(dim);
    let mut symbol_vector: Vec<String> = Vec::with_capacity(dim);
    for relationship in resp {
        let row: &Vec<CompanyOrCorrelation> = relationship.row();
        let s1: &Company = row[0].as_company();
        let corr: &f64 = row[1].as_double();
        let s2: &Company = row[2].as_company();
        let i1: usize = find_or_add(&mut indexes, &s1.symbol, &mut last_index);
        let i2: usize = find_or_add(&mut indexes, &s2.symbol, &mut last_index);
        vec[i1 * dim + i2] = corr.clone();
        if last != s1.symbol {
            vec[i1 * dim + i1] = s1.sd;
            mu_vector.push(s1.sd);
            symbol_vector.push(s1.symbol.clone());
            last = s1.symbol.clone();
        }
    }
    (DMatrix::from_vec(dim, dim, vec), DMatrix::from_vec(dim, 1, mu_vector), symbol_vector)
}

pub async fn optimize_portfolio(symbols: &Vec<String>) -> Vec<Weight> {
    let response: QueryResponse<CompanyOrCorrelation> = queries::companies_and_correlations(symbols).await;
    let data = response.data();
    let (matrix, mu_vector, symbols) = response_to_matrix(data);
    let inv = match matrix.pseudo_inverse(0f64) {
        Ok(inv) => inv,
        Err(e) => panic!("{}", e)
    };
    let weights: DMatrix<f64> = &inv * &mu_vector;
    let mut out: Vec<Weight> = Vec::with_capacity(mu_vector.len());
    for i in 0..weights.len() {
        out.push(Weight { symbol: symbols[i].clone(), weight: weights[i] });
    }
    out
}