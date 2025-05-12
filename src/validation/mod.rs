mod empty_or_nan;
mod zero_division;

pub use empty_or_nan::empty_or_nan;
pub use zero_division::zero_division;
// 'mod' é uma palavra-chave que define um módulo
// 'empty_or_nan' e 'zero_division' são os módulos que estamos importando
// 'pub use' é uma maneira de reexportar os itens dos módulos, tornando-os disponíveis para outros módulos
// 'empty_or_nan' e 'zero_division' são funções que validam os dados de entrada