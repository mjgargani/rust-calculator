pub fn mul(values: &[f64]) -> f64 {
    values.iter().product() // 'product()' é um método de agregação sobre iteradores que multiplica todos os elementos do iterador
}