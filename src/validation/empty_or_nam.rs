pub fn empty_or_nan(values: &[f64]) -> Result<(), String> {
    if values.is_empty() { // Verifica se o vetor está vazio
        // Retorna um erro se o vetor estiver vazio
        // 'to_string()' converte a string literal em um objeto String
        // 'Err' é um enum que representa um erro
        // 'return' explicita o retorno da função
        return Err("O vetor não pode estar vazio.".to_string());
    }
    for &value in values { // A partir do ponteiro values, percorre cada elemento do vetor
                           // &value é uma referência ao valor atual - um ponteiro para o valor
        if value.is_nan() { // Verifica se o valor é NaN (Not a Number)
            return Err("O vetor não pode conter NaN.".to_string());
        }
    }
    Ok(()) // Retorna Ok se o vetor for válido
    // 'Ok(())' é uma unidade de valor, semelhante a 'void' em outras linguagens, onde '()' representa um valor vazio e
    // 'Ok' é um enum que representa um resultado bem-sucedido
    // 'Result' é um enum que pode ser 'Ok' ou 'Err', representando o resultado de uma operação
    // 'Result<(), String>' significa que a função retorna um resultado que pode ser Ok (sem valor) ou Err (com uma mensagem de erro)
}