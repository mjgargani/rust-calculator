use crate::math; // Importa o módulo 'math' do crate atual

// Enumeração (enum) de nome 'Operation' para definir operações matemáticas
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Função que recebe uma operação (tipo 'Operation') e um slice de números (f64) e retorna o resultado da operação
fn calculate(op: Operation, numbers: &[f64]) -> f64 {
    // Retorna NaN (Not a Number) se o vetor estiver vazio ou inválido para a operação
    match op {
        // '::' é o operador de acesso para itens estáticos (ex: variantes de enum)
        // 'iter()' retorna um iterador sobre os elementos do slice
        // 'sum()', 'fold()', etc., são métodos de agregação sobre iteradores
        Operation::Add => math::add(numbers),
        Operation::Subtract => math::sub(numbers),
        Operation::Multiply => math::mul(numbers),
        Operation::Divide => math::div(numbers),
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