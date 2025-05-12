pub fn add(values: &[f64]) -> f64 {
    // 'iter()' retorna um iterador sobre os elementos do slice
    // 'sum()' é um método de agregação sobre iteradores
    values.iter().sum()
}