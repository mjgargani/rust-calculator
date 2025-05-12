mod validation; // Importa o módulo 'validation' do crate atual
mod math; // Importa o módulo 'math' do crate atual
// 'mod' é uma palavra-chave que define um módulo
// 'validation' e 'math' são módulos que contêm funções e tipos relacionados à validação de dados e operações matemáticas, respectivamente

mod enums; // Importa o módulo 'enums' do crate atual
use enums::Operation; // Importa o enum 'Operation' do módulo 'enums'

// Função que recebe uma operação (tipo 'Operation') e um slice de números (f64) e retorna o resultado da operação
fn calculate(op: Operation, values: &[f64]) -> f64 {
    if let Err(e) = validation::empty_or_nan(values) { // Verifica se o vetor é válido
        panic!("{}", e);  // Se não for, gera um pânico com a mensagem de erro (Aqui estou usando a propriedade 'unwind' do Rust)
                          // 'panic!' é uma macro que gera um pânico, nesse caso, o unwind vai gerar um erro mas não abortar o programa
                          // 'e' é a mensagem de erro retornada pela função 'empty_or_nan'
                          // '{}' é o delimitador de bloco, que indica onde a variável 'e' deve ser inserida na string,
                          // poderia ser algo como "Erro: {}", mas não me parece necessário
    }
    // Retorna NaN (Not a Number) se o vetor estiver vazio ou inválido para a operação
    match op {
        // '::' é o operador de acesso para itens estáticos (ex: variantes de enum)
        // 'iter()' retorna um iterador sobre os elementos do slice
        // 'sum()', 'fold()', etc., são métodos de agregação sobre iteradores
        Operation::Add => math::add(values),
        Operation::Subtract => math::sub(values),
        Operation::Multiply => math::mul(values),
        Operation::Divide => math::div(values),
    }
    
}

fn main() { // Todo programa Rust começa pela função main
    let values = vec![2.2, 3.0, 4.4, 5.0];  // 'vec!' é uma macro que cria um vetor alocado dinamicamente (heap). Todas as entradas devem ter o mesmo tipo

    println!("Values: {:?}", values); // '{:?}' ativa o modo de debug para impressão do vetor

    // '&values' passa uma referência imutável do vetor para a função
    // '{:.2}' formata a saída com duas casas decimais, semelhante a 'toFixed(2)' no JS
    println!("Add: {:.2}", calculate(Operation::Add, &values));
    println!("Subtract: {:.2}", calculate(Operation::Subtract, &values));
    println!("Multiply: {:.2}", calculate(Operation::Multiply, &values));
    println!("Divide: {:.2}", calculate(Operation::Divide, &values));
}
// O ChatGPT me chamou de prolixo pra comentar o código ); Enxutei