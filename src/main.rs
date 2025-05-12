// Enumeração (enum) de nome 'Operation' para definir operações matemáticas
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Função que recebe uma operação (tipo 'Operation') e um vetor de números (float 64-bit) e retorna o resultado da operação
fn calculate(op: Operation, numbers: &Vec<f64>) -> f64 {
    // Verifica se o vetor está vazio e retorna NaN (Not a Number) se estiver
    match op {
        // O acesso às propriedades de Operation é feito usando o operador '::'
        // é diferente de acessar uma propriedade no JS, com '.', pois é algo executado em tempo de compilação
        // O método 'iter()' retorna um iterador sobre os elementos do vetor, dessa forma, podemos aplicar operações
        // de agregação como 'sum()', 'fold()', etc.
        // '.map()' é muito semelhante ao 'map()' do JS, embora a sintaxe e funcionalidade sejam um pouco diferentes
        // 'iter()' retorna um iterador, enquanto 'map()' aplica uma função a cada elemento do vetor
        // 'fold()' é semelhante ao 'reduce()' do JS, mas com uma sintaxe diferente
        // 'skip()' pula os primeiros n elementos do iterador
        Operation::Add => numbers.iter().sum(),
        Operation::Subtract => numbers.iter().map(|x| -x).sum(),
        Operation::Multiply => numbers.iter().fold(1.0, |acc, x| acc * x),
        Operation::Divide => {
            if numbers.len() < 2 {
                return f64::NAN; // Acessa a propriedade 'NAN' do tipo 'f64', que retorna NaN, o Rust não tem 'undefined' ou 'null'
            }
            numbers.iter().skip(1).fold(numbers[0], |acc, x| acc / x)
        }
    }
}

fn main() { // Todo programa Rust começa pela função main, isso aparentemente é mandatório
    let values = vec![2.2, 3.0, 4.4, 5.0];  // A semântica vec! onde há um <tipo> acompanhado da exclamação '!' representa macros
                                            // A diferença entre uma macro e uma função é que a macro é expandida em tempo de compilação, 
                                            // enquanto a função é executada em tempo de execução
                                            // A macro 'vec!' cria um vetor, nesse caso, de números de ponto flutuante (f64)
                                            // Não há vetores dinamicamente tipados como no JS, o Rust é uma linguagem estaticamente tipada
                                            // O vetor é criado com o tipo 'Vec<f64>', que é um vetor de números de ponto flutuante de 64 bits    

    println!("Values: {:?}", values); // Imprime o vetor de valores, o operador ':?' formata a saída para mostrar o vetor, algo como 'console.log({values})' no JS
    // O operador {:.2} formata o número para duas casas decimais, semelhante ao 'toFixed(2)' do JS
    // O operador '&' é usado para passar uma referência ao vetor, semelhante ao 'const' do JS,
    // é uma referência imutável, o que significa que não podemos modificar o vetor dentro da função,
    // mas podemos ler seus valores, é como se ele estivesse "emprestado" para a função, um ponteiro para o vetor
    // Se usássemos *values, estaríamos passando o vetor inteiro, o nome desse operador é 'dereferencing', 
    // que é o ato de acessar o valor apontado por um ponteiro
    println!("Add: {:.2}", calculate(Operation::Add, &values));
    println!("Subtract: {:.2}", calculate(Operation::Subtract, &values));
    println!("Multiply: {:.2}", calculate(Operation::Multiply, &values));
    println!("Divide: {:.2}", calculate(Operation::Divide, &values));
}
