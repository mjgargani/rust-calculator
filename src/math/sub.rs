pub fn sub(values: &[f64]) -> f64 {
    values.iter().fold(0.0, |acc, &x| acc - x)
    // 'fold()' é um método de agregação que aplica uma função acumuladora a cada elemento do iterador
    // 'acc' é o acumulador, que começa em 0.0
    // 'x' é o elemento atual do iterador
    // 'acc - x' é a operação de subtração que estamos realizando
}