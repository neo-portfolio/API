use crate::models::queries;
use crate::neo4j::structures::{QueryResponse, QueryResponseData};
use crate::models::company::{CompanyOrCorrelation, Company};
use nalgebra::DMatrix;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Weight {
    symbol: String,
    weight: f64,
}

fn find_or_add(indexes: &mut HashMap::<String, usize>, symbol: &String, last_index: &mut usize) -> usize {
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

fn response_to_matrix(resp: &Vec<QueryResponseData<CompanyOrCorrelation>>) -> (DMatrix<f64>, DMatrix<f64>, Vec<String>) {
    let mut indexes = HashMap::<String, usize>::new();
    let mut last_index: usize = 0;
    let len: usize = resp.len();
    let mut vec: Vec<f64> = Vec::with_capacity(len);
    vec.resize(len, 0f64);
    let mut new = true;
    let mut last: &String = &resp[0].row()[0].as_company().symbol;
    let dim: usize = (len as f64).sqrt() as usize;
    let mut mu_vector: Vec<f64> = Vec::with_capacity(dim + 1);
    let mut symbol_vector: Vec<String> = Vec::with_capacity(dim + 1);
    for relationship in resp {
        let row = relationship.row();
        let s1: &Company = row[0].as_company();
        let corr: &f64 = row[1].as_double();
        let s2: &Company = row[2].as_company();
        let i1: usize = find_or_add(&mut indexes, &s1.symbol, &mut last_index);
        let i2: usize = find_or_add(&mut indexes, &s2.symbol, &mut last_index);
        vec[i1 * dim + i2] = corr.clone();
        if new {
            vec[i1*dim + i1] = s1.sd;
            mu_vector.push(s1.sd);
            symbol_vector.push(s1.symbol.clone());
            new = false;
        }
        else if last != &s1.symbol {
            last = &s1.symbol;
            new = true;
        }
    }
    println!("{:?}", mu_vector);
    println!("{}", dim);
    (DMatrix::from_vec(dim, dim, vec), DMatrix::from_vec(1, dim + 1, mu_vector) , symbol_vector)
}

pub async fn optimize_portfolio(symbols: &Vec<String>) -> Vec<Weight> {
    let response: QueryResponse<CompanyOrCorrelation> = queries::companies_and_correlations(symbols).await;
    let data = response.data();
    let (matrix, mu_vector, symbols) = response_to_matrix(data);
    let inv = match matrix.pseudo_inverse(0f64) {
        Ok(inv) => inv,
        Err(e) => panic!("{}", e)
    };
    let weights: DMatrix<f64> = mu_vector.transpose() * inv * &mu_vector;
    let mut out: Vec<Weight> = Vec::with_capacity(mu_vector.len());
    for i in 0..weights.len() {
        out.push(Weight {symbol: symbols[i].clone(), weight: weights[i]});
    }
    out
}