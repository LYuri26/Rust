use std::io;

fn main() {
    let mut numero1 = String::new();

    println!("Digite o número:");

    io::stdin()
        .read_line(&mut numero1)
        .expect("Erro ao ler a linha");

    // Converte e calcula a soma
    let numero1: i32 = numero1.trim().parse().expect("Número inválido");

    if numero1 % 2 != 0 {
        println!("O número {} é ímpar.", numero1);
    } else {
        println!("O número {} é par.", numero1);
    }
}
