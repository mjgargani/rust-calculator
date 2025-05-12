use super::validation::empty_or_nan;
use super::validation::zero_division;
// Ou 'use crate::validation::empty_or_nan;' | 'use crate::validation::zero_division;'

pub fn div(values: &[f64]) -> f64 {
    if let Err(e) = empty_or_nan(values) {
        panic!("{}", e);
    }
    if let Err(e) = zero_division(values) {
        panic!("{}", e);
    }
    values.iter().fold(1.0, |acc, &x| acc / x)
    // 'acc' é o acumulador, que começa em 1.0, tem que começar em 1.0 porque estamos fazendo uma divisão
    //  se começássemos em 0.0, o resultado seria sempre 0.0
    // 'acc / x' é a operação de divisão que estamos realizando
}