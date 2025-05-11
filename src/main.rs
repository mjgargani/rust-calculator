enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(op: Operation, numbers: &Vec<f64>) -> f64 {
    match op {
        Operation::Add => numbers.iter().sum(),
        Operation::Subtract => numbers.iter().map(|x| -x).sum(),
        Operation::Multiply => numbers.iter().fold(1.0, |acc, x| acc * x),
        Operation::Divide => {
            if numbers.len() < 2 {
                return f64::NAN;
            }
            numbers.iter().skip(1).fold(numbers[0], |acc, x| acc / x)
        }
    }
}

fn main() {
    let values = vec![2.2, 3.0, 4.4, 5.0];

    println!("Values: {:?}", values);
    println!("Add: {:.2}", calculate(Operation::Add, &values));
    println!("Subtract: {:.2}", calculate(Operation::Subtract, &values));
    println!("Multiply: {:.2}", calculate(Operation::Multiply, &values));
    println!("Divide: {:.2}", calculate(Operation::Divide, &values));
}
