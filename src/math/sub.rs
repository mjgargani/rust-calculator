use super::validation::empty_or_nan;
// Ou 'use crate::validation::empty_or_nan;'

pub fn sub(values: &[f64]) -> f64 {
    if let Err(e) = empty_or_nan(values) {
        panic!("{}", e);  
    }
    values.iter().fold(0.0, |acc, &x| acc - x)
    // 'fold()' é um método de agregação que aplica uma função acumuladora a cada elemento do iterador
    // 'acc' é o acumulador, que começa em 0.0
    // 'x' é o elemento atual do iterador
    // 'acc - x' é a operação de subtração que estamos realizando
}