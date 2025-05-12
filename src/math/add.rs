use super::validation::empty_or_nan;
// Ou 'use crate::validation::empty_or_nan;'

pub fn add(values: &[f64]) -> f64 {
    if let Err(e) = empty_or_nan(values) { // Verifica se o vetor é válido
        panic!("{}", e);  // Se não for, gera um pânico com a mensagem de erro (Aqui estou usando a propriedade 'unwind' do Rust)
                          // 'panic!' é uma macro que gera um pânico, nesse caso, o unwind vai gerar um erro mas não abortar o programa
                          // 'e' é a mensagem de erro retornada pela função 'empty_or_nan'
                          // '{}' é o delimitador de bloco, que indica onde a variável 'e' deve ser inserida na string,
                          // poderia ser algo como "Erro: {}", mas não me parece necessário
    
    // 'iter()' retorna um iterador sobre os elementos do slice
    // 'sum()' é um método de agregação sobre iteradores
    values.iter().sum()
}