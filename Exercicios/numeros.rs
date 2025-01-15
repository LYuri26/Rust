use std::io;

fn abreviacao(numero: f64) {
    if numero < 1000.0 {
        println!("O valor é: {}.", numero);
    } else if numero >= 1000.0 && numero < 1000000.0 {
        println!("O número é: {:.1} K.", numero / 1000.0);
    } else if numero >= 1000000.0 && numero < 1000000000.0 {
        println!("O número é: {:.1} M.", numero / 1000000.0);
    } else if numero >= 1000000000.0 && numero < 1000000000000.0 {
        println!("O número é: {:.1} B.", numero / 1000000000.0);
    } else {
        println!("Valor inválido.");
    }
}

fn main() {
    println!("Digite um número:");

    let mut input = String::new();

    // Lê a entrada e trata erros de leitura
    io::stdin().read_line(&mut input).unwrap();

    // Converte a entrada para f64, tratando erros de conversão
    let numero: f64 = input.trim().parse().unwrap();

    // Chama a função para exibir a abreviação
    abreviacao(numero);
}
