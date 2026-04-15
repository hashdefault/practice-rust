use std::io;

fn main() {
    let mut number = String::from("coisa");

    println!("{number}");
    number.clear();

    io::stdin().read_line(&mut number).expect("Erro");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_err) => 0,
    };

    println!("Numero parseado {}", number);
}
