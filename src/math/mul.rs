use super::validation::empty_or_nan;
// Ou 'use crate::validation::empty_or_nan;'

pub fn mul(values: &[f64]) -> f64 {
    if let Err(e) = empty_or_nan(values) {
        panic!("{}", e);
    }
    values.iter().product() // 'product()' é um método de agregação sobre iteradores que multiplica todos os elementos do iterador
}