mod fibonacci;
mod prime_number;
mod zigzag;

pub static APPS: [fn(); 3] = [zigzag::main, fibonacci::main, prime_number::main];
