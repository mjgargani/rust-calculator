pub fn zero_division(values: &[f64]) -> Result<(), String> {
    if values.iter().any(|&x| x == 0.0) { // .any() é um método que verifica se algum elemento do iterador satisfaz a condição
                                          // 'x == 0.0' verifica se o elemento é zero
        return Err("Divisão por zero não é permitida.".to_string());
    }
    Ok(())
}