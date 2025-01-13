use std::io;

fn multiplicacao(numero1: i32) {
    for i in 0..=10 {
        println!("{}x{}={}", numero1, i, numero1 * i)
    }
}

fn main() {
    let mut numero1 = String::new();

    println!("Digite o número:");

    io::stdin()
        .read_line(&mut numero1)
        .expect("Erro ao ler a linha");

    let numero1: i32 = numero1.trim().parse().expect("Número inválido");

    multiplicacao(numero1);
}
