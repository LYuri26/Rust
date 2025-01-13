use std::io;

fn main() {
    let mut numero1 = String::new();
    let mut numero2 = String::new();

    println!("Digite o primeiro número:");

    io::stdin()
        .read_line(&mut numero1)
        .expect("Erro ao ler a linha");

    println!("Digite o segundo número:");
    io::stdin()
        .read_line(&mut numero2)
        .expect("Erro ao ler a linha");

    // Converte e calcula a soma
    let numero1: f64 = numero1.trim().parse().expect("Número inválido");
    let numero2: f64 = numero2.trim().parse().expect("Número inválido");

    println!(
        "A soma de {} e {} é {}",
        numero1,
        numero2,
        numero1 + numero2
    );
}
